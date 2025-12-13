use std::collections::HashMap;
use std::path::PathBuf;
use amfiteatr_core::agent::{AutomaticAgent, ReseedAgent};
use amfiteatr_core::comm::StdEnvironmentEndpoint;
use amfiteatr_core::env::{HashMapEnvironment, ReseedEnvironment, RoundRobinPenalisingUniversalEnvironment};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::ContractEnvStateComplete;
use brydz_core::deal::{ContractGameDescription, DealDistribution};
use brydz_core::player::side::{Side, SideMap};
use crate::generate::generate_contracts;
use crate::model::agent::BAgent;
use crate::options::contract::{ModelConfig, TestSet};
use crate::options::contract_generation::{ChoiceDoubling, ForceDeclarer, GenContractOptions, Subtrump};
use crate::options::{DataFormat, DealMethod};

pub struct GameModel{

    agent_north: BAgent,
    agent_south: BAgent,
    agent_east: BAgent,
    agent_west: BAgent,

    env: HashMapEnvironment<ContractDP, ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,

    config: ModelConfig,

    learn_set_biased_game_distributions: Option<Vec<DealDistribution>>,
    test_set_contracts: Option<Vec<ContractGameDescription>>,
    thread_pool: Option<rayon::ThreadPool>,

}

impl GameModel{

    fn play_single_game(&mut self) -> anyhow::Result<()> {
        match &self.thread_pool{
            Some(pool) => {
                pool.scope(|s|{
                    s.spawn(|_|{
                        self.env.run_round_robin_with_rewards_penalise(|_,_| -10).unwrap();
                    });
                    s.spawn(|_|{
                        self.agent_north.agent_mut().run().unwrap();
                    });
                    s.spawn(|_|{
                        self.agent_east.agent_mut().run().unwrap();
                    });
                    s.spawn(|_|{
                        self.agent_south.agent_mut().run().unwrap();
                    });
                    s.spawn(|_|{
                        self.agent_west.agent_mut().run().unwrap();
                    });

                });

            },
            None => {
                todo!()
            }
        }
        Ok(())
    }

    pub fn play_one_game(&mut self, seed: &ContractGameDescription) -> anyhow::Result<()>{
        self.env.reseed(seed)?;
        self.agent_east.agent_mut().reseed((&Side::East, seed))?;
        self.agent_north.agent_mut().reseed((&Side::North, seed))?;
        self.agent_west.agent_mut().reseed((&Side::West, seed))?;
        self.agent_south.agent_mut().reseed((&Side::South, seed))?;

        self.play_single_game()?;


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
                Some(v)
            },
            TestSet::New(n) => {

                let contracts_options = GenContractOptions{
                    game_count: n as u64,
                    .. GenContractOptions::default()
                };
                Some(generate_contracts(&contracts_options)?)
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