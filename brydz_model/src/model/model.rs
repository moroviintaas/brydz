use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::path::PathBuf;
use rand::seq::IteratorRandom;
use amfiteatr_core::agent::{AutomaticAgent, MultiEpisodeAutoAgent, PolicyAgent, ReseedAgent, TracingAgent};
use amfiteatr_core::comm::StdEnvironmentEndpoint;
use amfiteatr_core::env::{EpochSummaryGen, GameSummaryGen, HashMapEnvironment, ReseedEnvironment, RoundRobinPenalisingUniversalEnvironment, StatefulEnvironment};
use amfiteatr_core::error::{AmfiteatrError, CommunicationError};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::ContractEnvStateComplete;
use brydz_core::bidding::Bid;
use brydz_core::cards::trump::{Trump, TRUMPS};
use brydz_core::contract::ContractParameters;
use brydz_core::deal::{ContractGameDescription, DealDistribution};
use brydz_core::error::ContractErrorGen;
use brydz_core::player::side::{Side, SideMap, SIDES};
use crate::generate::generate_contracts;
use crate::model::agent::BAgent;
use crate::options::contract::{ModelConfig, TestSet};
use crate::options::contract_generation::{ChoiceDoubling, ForceDeclarer, GenContractOptions, Subtrump};
use crate::options::{DataFormat, DealMethod};
use karty::random::RandomSymbol;
use rand::distr::Distribution;
use amfiteatr_rl::policy::LearningNetworkPolicyGeneric;

pub struct GameModel{

    agent_north: BAgent,
    agent_south: BAgent,
    agent_east: BAgent,
    agent_west: BAgent,

    env: HashMapEnvironment<ContractDP, ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,

    config: ModelConfig,

    learn_set_biased_game_distributions: Option<Vec<DealDistribution>>,
    test_set_contracts: Vec<ContractGameDescription>,
    thread_pool: Option<rayon::ThreadPool>,

}

impl GameModel{

    fn play_single_game(&mut self) -> Result<(), AmfiteatrError<ContractDP>> {
        let (tx, rx) = std::sync::mpsc::channel();
        match &self.thread_pool {
            Some(pool) => {
                pool.scope(|s| {
                    s.spawn(|_| {
                        let result = self.env.run_round_robin_with_rewards_penalise(|_, _| -10);
                        tx.send(result).unwrap();
                    });
                    s.spawn(|_| {
                        self.agent_north.agent_mut().run().unwrap();
                    });
                    s.spawn(|_| {
                        self.agent_east.agent_mut().run().unwrap();
                    });
                    s.spawn(|_| {
                        self.agent_south.agent_mut().run().unwrap();
                    });
                    s.spawn(|_| {
                        self.agent_west.agent_mut().run().unwrap();
                    });
                });
            },
            None => {
                todo!()
            }
        }
        rx.recv().map_err(|e|{
            AmfiteatrError::Communication {
                source: CommunicationError::RecvErrorUnspecified(format!("Environment export result."))
            }
        })?
    }

    pub fn play_one_game(&mut self, seed: &ContractGameDescription) -> anyhow::Result<GameSummaryGen<ContractDP>>{
        self.env.reseed(seed)?;
        self.agent_east.agent_mut().reseed((&Side::East, seed))?;
        self.agent_north.agent_mut().reseed((&Side::North, seed))?;
        self.agent_west.agent_mut().reseed((&Side::West, seed))?;
        self.agent_south.agent_mut().reseed((&Side::South, seed))?;

        let game_result = self.play_single_game()?;


        let mut summary = GameSummaryGen::<ContractDP>::from(self.env.state());
        summary.set_violating_agent(self.env.game_violator().copied());


        //let violator = ;



        Ok(summary)

    }

    pub fn play_train_epoch(&mut self) -> anyhow::Result<EpochSummaryGen<ContractDP>> {

        self.clean_trajectories()?;
        let mut rng = rand::rng();
        let mut summaries = Vec::with_capacity(self.config.number_of_games_in_epoch);

        for i in 0..self.config.number_of_games_in_epoch {
            let declarer = match self.config.force_declarer_when_rand{
                None => *SIDES.iter().choose(&mut rng).unwrap(),
                Some(s) => s
            };
            let bid_h = (0..3).choose(&mut rng).unwrap();
            let trump = Trump::random(&mut rng);
            let parameters = ContractParameters::new(declarer, Bid::init(trump, bid_h)?);


            let seed = match &self.learn_set_biased_game_distributions{
                None => {
                    let cards = DealDistribution::Fair.sample(&mut rng);
                    ContractGameDescription::new(parameters, DealDistribution::Fair, cards)

                }
                Some(v) => {
                    let d = v.choose(&mut rng).unwrap_or(&DealDistribution::Fair);
                    let cards  = d.sample(&mut rng);
                    ContractGameDescription::new(parameters, DealDistribution::Fair, cards)
                }
            };
            let summary = self.play_one_game(&seed)?;
            summaries.push(summary);
            log::trace!("Finishing game {i} in epoch");


        }
        let epoch_summary = EpochSummaryGen::new(summaries);
        Ok(epoch_summary)

    }

    pub fn run_test_epoch(&mut self) -> anyhow::Result<EpochSummaryGen<ContractDP>> {

        self.clean_trajectories()?;

        let mut rng = rand::rng();
        let mut summaries = Vec::with_capacity(self.config.number_of_games_in_epoch);
        for i in 0..self.test_set_contracts.len() {

            let seed = self.test_set_contracts[i].clone();


            let summary = self.play_one_game(&seed)?;
            summaries.push(summary);
            log::trace!("Finishing game {i} in training epoch");


        }
        let epoch_summary = EpochSummaryGen::new(summaries);
        Ok(epoch_summary)
    }

    pub fn clean_trajectories(&mut self) -> anyhow::Result<()>{
        self.agent_east.agent_mut().clear_episodes()?;
        self.agent_north.agent_mut().clear_episodes()?;
        self.agent_west.agent_mut().clear_episodes()?;
        self.agent_south.agent_mut().clear_episodes()?;
        Ok(())
    }

    pub fn train_agent_on_trajectory(&mut self, side: Side) -> anyhow::Result<()>{
         let agent_ref = match side{
             Side::East => self.agent_east.agent_mut(),
             Side::South => self.agent_south.agent_mut(),
             Side::West => self.agent_west.agent_mut(),
             Side::North => self.agent_north.agent_mut(),
         };

        let trajectories = agent_ref.take_episodes();

        let mut policy = agent_ref.policy_mut();

        policy.train(&trajectories[..])?;

        Ok(())
    }


}

impl TryFrom<ModelConfig> for GameModel{
    type Error = anyhow::Error;

    fn try_from(config: ModelConfig) -> Result<Self, Self::Error> {

        let (comm_env_n, comm_north) = StdEnvironmentEndpoint::new_pair();
        let (comm_env_e, comm_east) = StdEnvironmentEndpoint::new_pair();
        let (comm_env_s, comm_south) = StdEnvironmentEndpoint::new_pair();
        let (comm_env_w, comm_west) = StdEnvironmentEndpoint::new_pair();
        let agent_north = BAgent::build(config.agents.north.clone(), Side::North, comm_north)?;
        let agent_east = BAgent::build(config.agents.east.clone(), Side::East, comm_east)?;
        let agent_west = BAgent::build(config.agents.west.clone(), Side::West, comm_west)?;
        let agent_south = BAgent::build(config.agents.south.clone(), Side::South, comm_south)?;

        let mut hm_comm = HashMap::new();
        hm_comm.insert(Side::North, comm_env_n);
        hm_comm.insert(Side::East, comm_env_e);
        hm_comm.insert(Side::South, comm_env_s);
        hm_comm.insert(Side::West, comm_env_w);

        let env = HashMapEnvironment::new(ContractEnvStateComplete::default(), hm_comm);


        let thread_pool = rayon::ThreadPoolBuilder::new().build().unwrap();

        let test_set_contracts = match config.test_set{
            TestSet::Saved(ref path) => {
                let s = std::fs::read_to_string(&path)?;
                let v: Vec<ContractGameDescription> = ron::from_str(&s)?;
                v
            },
            TestSet::New(n) => {

                let contracts_options = GenContractOptions{
                    game_count: n as u64,
                    .. GenContractOptions::default()
                };
                generate_contracts(&contracts_options)?
            }
        };
        let learn_set_biased_game_distributions = match config.game_deal_biases{
            None => None,
            Some(ref bias_path) => {
                let s = std::fs::read_to_string(&bias_path)?;
                let v: Vec<DealDistribution> = ron::from_str(&s)?;
                Some(v)
            }
        };



        Ok(Self{
            agent_north,
            agent_south,
            agent_east,
            agent_west,
            env,
            config,
            learn_set_biased_game_distributions,
            test_set_contracts,
            thread_pool: Some(thread_pool),
        })


    }
}