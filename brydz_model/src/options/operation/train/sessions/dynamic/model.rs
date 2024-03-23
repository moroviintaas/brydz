use std::collections::HashMap;
use std::fs;
use std::iter::Sum;
use std::ops::{Add, Deref, Div, Index};
use std::path::Path;
use std::sync::{Arc, Mutex};
use enum_map::{enum_map, EnumMap};
use log::{debug, trace};
use rand::distributions::{Distribution, Standard};
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand_distr::num_traits::Signed;
use amfiteatr_core::agent::{AgentGen, AutomaticAgent, RandomPolicy, ReinitAgent, ReseedAgent};
use amfiteatr_core::comm::{EnvironmentMpscPort, StdEnvironmentEndpoint};
use amfiteatr_core::env::{BasicEnvironment, HashMapEnvironment, ReseedEnvironment, RoundRobinPenalisingUniversalEnvironment, StatefulEnvironment};
use amfiteatr_rl::agent::{RlSimpleLearningAgent, RlSimpleTestAgent};
use amfiteatr_rl::error::AmfiRLError;
use brydz_core::amfiteatr::comm::ContractAgentSyncComm;
use brydz_core::amfiteatr::env::ContractEnv;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractDummyState, ContractEnvStateComplete, ContractState};
use brydz_core::contract::{Contract, ContractMechanics, ContractParameters};
use brydz_core::deal::{ContractGameDescription, DealDistribution, DescriptionDeckDeal};
use brydz_core::player::axis::RoleAxis;
use brydz_core::player::role::PlayRole;
use brydz_core::player::side::{Side, SideMap};
use brydz_core::player::side::Side::{East, North, South, West};
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{ContractInfoSetSeed, ContractInfoSetSeedLegacy, PolicyTypeSelect};
use crate::error::BrydzModelError;
use crate::options::operation::generate::{GenContractOptions, generate_biased_deal_distributions};

type BrydzDynamicAgent = Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>;


#[derive(Default, Debug)]
pub struct RolePayoffSummary {
    pub scores: EnumMap<PlayRole, f64>,
}

impl Add<&RolePayoffSummary> for &RolePayoffSummary{
    type Output = RolePayoffSummary;

    fn add(self, rhs: &RolePayoffSummary) -> Self::Output {
        RolePayoffSummary{
            scores: enum_map! {
                PlayRole::Declarer => self.scores[PlayRole::Declarer] + rhs.scores[PlayRole::Declarer],
                PlayRole::Whist => self.scores[PlayRole::Whist] + rhs.scores[PlayRole::Whist],
                PlayRole::Offside => self.scores[PlayRole::Offside] + rhs.scores[PlayRole::Offside],
                PlayRole::Dummy => self.scores[PlayRole::Dummy] + rhs.scores[PlayRole::Dummy],

            }
        }
    }
}

impl<'a> Sum<&'a RolePayoffSummary> for RolePayoffSummary{
    fn sum<I: Iterator<Item=&'a Self>>(iter: I) -> Self {
        iter.fold(Default::default(), |acc, x|{
            &acc + x
        } )
    }
}

impl Div<usize> for &RolePayoffSummary{
    type Output = RolePayoffSummary;

    fn div(self, rhs: usize) -> Self::Output {
        let df = rhs as f64;
        RolePayoffSummary{
            scores: enum_map! {
                PlayRole::Declarer => self.scores[PlayRole::Declarer] / df,
                PlayRole::Whist => self.scores[PlayRole::Whist] / df,
                PlayRole::Offside => self.scores[PlayRole::Offside] /df,
                PlayRole::Dummy => self.scores[PlayRole::Dummy] / df

            }
        }
    }
}


#[derive(Clone, Copy)]
pub enum Testing{
    Declarer,
    Defenders,
    None
}


pub struct DynamicBridgeModel{
    
    //pub env: ContractEnv<ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,
    pub env: HashMapEnvironment<ContractDP, ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,
    pub declarer: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,

    pub whist: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub offside: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub dummy: AgentGen<ContractDP, RandomPolicy<ContractDP, ContractDummyState>, ContractAgentSyncComm>,
    pub test_declarer: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub test_whist: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub test_offside: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    



    pub(crate) inactive_declarer_comm: StdEnvironmentEndpoint<ContractDP>,
    pub(crate) inactive_whist_comm: StdEnvironmentEndpoint<ContractDP>,
    pub(crate) inactive_offside_comm: StdEnvironmentEndpoint<ContractDP>,

    pub(crate) test_vectors: Vec<ContractGameDescription>,
    pub(crate) initial_deal: ContractGameDescription,
}




impl DynamicBridgeModel{




    fn swap_defense(&mut self){
        let ws = self.env.state().whist_side();
        let os = self.env.state().offside_side();
        std::mem::swap(self.env.comms_mut().get_mut(&ws).unwrap(), &mut self.inactive_whist_comm);
        std::mem::swap(self.env.comms_mut().get_mut(&os).unwrap(), &mut self.inactive_offside_comm);
    }
    fn swap_declarer(&mut self){
        let ds = self.env.state().declarer_side();
        std::mem::swap(self.env.comms_mut().get_mut(&ds).unwrap(), &mut self.inactive_declarer_comm);
    }

    fn swap_for_test(&mut self,  tested_role_axis: RoleAxis){
        match tested_role_axis{
            RoleAxis::Declarers => {
                self.swap_defense();
            }
            RoleAxis::Defenders => {
                self.swap_declarer();
            }
        }
    }

    pub fn load_test_games_from_file(&mut self, file: impl AsRef<Path>) -> Result<(), BrydzModelError>{
        let test_str = fs::read_to_string(&file)
            .map_err(|e| BrydzModelError::IO(format!("Failed reading file input {:?} as test vectors ({e:})", &file.as_ref())))?;
        let set:  Vec<ContractGameDescription> = ron::de::from_str(&test_str)
            .map_err(|e| BrydzModelError::IO(format!("Failed converting input of file {:?} as test vectors ({e:})", &file.as_ref())))?;

        self.test_vectors = set;
        Ok(())
    }
    pub fn generate_test_games(&mut self, rng: &mut ThreadRng, number: usize) -> Result<(), AmfiRLError<ContractDP>>{
        todo!()
    }

    fn rotate_environment_comms(&mut self, side_before: Side, side_after: Side){
        //let rhs = side_after - side_before

        let mut sm = SideMap::new_with_fn_mut(|s| self.env.comms_mut().remove(&s).unwrap());

        sm.rotate(side_before, side_after);

        let (n,e,s,w) = sm.destruct_start_with(North);
        self.env.comms_mut().insert(North, n);
        self.env.comms_mut().insert(East, e);
        self.env.comms_mut().insert(West, w);
        self.env.comms_mut().insert(South, s);
    }

    fn episode_result(&self) -> RolePayoffSummary{

        RolePayoffSummary{
            scores: enum_map! {
                PlayRole::Declarer => self.env.state().contract_data().total_tricks_taken_role_axis(PlayRole::Declarer) as f64,
                PlayRole::Whist => self.env.state().contract_data().total_tricks_taken_role_axis(PlayRole::Whist) as f64,
                PlayRole::Offside => self.env.state().contract_data().total_tricks_taken_role_axis(PlayRole::Offside) as f64,
                PlayRole::Dummy => self.env.state().contract_data().total_tricks_taken_role_axis(PlayRole::Dummy) as f64,

            }
        }
    }

    pub fn prepare_episode(&mut self, seed: &ContractGameDescription, testing: Testing) -> Result<(), AmfiRLError<ContractDP>>{

        let old_declarer_side = self.env.state().declarer_side();
        let new_declarer_side = seed.parameters().declarer();


        //let seed_refs = (seed.0, seed.1);
        self.env.reseed(seed)?;


        let dummy_side = seed.parameters().dummy();
        self.dummy.reseed((&dummy_side, seed))?;

        let declarer_side = seed.parameters().declarer();
        let whist_side = seed.parameters().whist();
        let offside_side = seed.parameters().offside();

        debug!("After game reseed: North: {:#}, East: {:#}, South: {:#}, West: {:#}. Declarer is on: {}.",
            self.env.state()[North], self.env.state()[East], self.env.state()[South], self.env.state()[West],
            self.env.state().contract_data().declarer()
        );

        match testing{
            Testing::Declarer => {
                // testing declarer
                self.declarer.lock().unwrap().reseed((&declarer_side, seed))?;
                self.test_whist.lock().unwrap().reseed((&whist_side, seed))?;
                self.test_offside.lock().unwrap().reseed((&offside_side, seed))?;
                //let
            }
            Testing::Defenders => {
                self.test_declarer.lock().unwrap().reseed((&declarer_side, seed))?;
                self.whist.lock().unwrap().reseed((&whist_side, seed))?;
                self.offside.lock().unwrap().reseed((&offside_side, seed))?;
            }
            Testing::None => {
                self.declarer.lock().unwrap().reseed((&declarer_side, seed))?;
                self.whist.lock().unwrap().reseed((&whist_side, seed))?;
                self.offside.lock().unwrap().reseed((&offside_side, seed))?;
            }
        }
        self.rotate_environment_comms(old_declarer_side, new_declarer_side);


        Ok(())
    }

    pub fn run_episode(&mut self, testing: Testing) -> Result<(), BrydzModelError>{

        std::thread::scope(|s|{
            s.spawn(|| self.env.run_round_robin_with_rewards_penalise(-100));

            s.spawn(|| self.dummy.run());
            match testing{
                Testing::Declarer => {
                    s.spawn(|| self.declarer.lock().unwrap().run_rewarded());
                    s.spawn(|| self.test_whist.lock().unwrap().run_rewarded());
                    s.spawn(|| self.test_offside.lock().unwrap().run_rewarded());
                }
                Testing::Defenders => {
                    s.spawn(|| self.test_declarer.lock().unwrap().run_rewarded());
                    s.spawn(|| self.whist.lock().unwrap().run_rewarded());
                    s.spawn(|| self.offside.lock().unwrap().run_rewarded());
                }
                Testing::None => {
                    s.spawn(|| self.declarer.lock().unwrap().run_rewarded());
                    s.spawn(|| self.whist.lock().unwrap().run_rewarded());
                    s.spawn(|| self.offside.lock().unwrap().run_rewarded());
                }
            }

        });
        Ok(())
    }

    fn set_explore_agent(&self, agent: &Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>, explore: bool)
    -> Result<(), BrydzModelError>{
        agent.lock()
            .map_err(|e|BrydzModelError::Mutex("Failed locking mutex preparing agent in tests".into()))?
            .set_exploration(explore);

        Ok(())
    }
    fn set_explore_all(&self, explore: bool) -> Result<(), BrydzModelError>{
        self.set_explore_agent(&self.declarer, explore)?;
        self.set_explore_agent(&self.whist, explore)?;
        self.set_explore_agent(&self.offside, explore)?;
        Ok(())
    }

    pub fn run_test_series(&mut self, tested_role_axis: RoleAxis) -> Result<RolePayoffSummary, BrydzModelError>{

        let mut result = RolePayoffSummary::default();
        let testing_sides = match tested_role_axis{
            RoleAxis::Declarers => Testing::Declarer,
            RoleAxis::Defenders => Testing::Defenders,
        };
        self.set_explore_all(false)?;
        let mut test_vectors = Vec::with_capacity(0);
        std::mem::swap(&mut test_vectors, &mut self.test_vectors);

        self.swap_for_test(tested_role_axis);
        for t in &test_vectors{

            self.prepare_episode(t, testing_sides)?;
            self.run_episode(testing_sides)?;

            let r =  self.episode_result();
            trace!("Result for test episode: {:?} ", r);
            result = &result + &r;

        }
        result = &result / test_vectors.len();
        debug!("Average results after test: {:?}", result);
        self.swap_for_test(tested_role_axis);
        std::mem::swap(&mut test_vectors, &mut self.test_vectors);
        self.set_explore_all(true)?;



        Ok(result)
    }


    fn clear_trajectories(&mut self) -> Result<(), BrydzModelError>{
        self.declarer.lock().unwrap().clear_episodes();
        self.whist.lock().unwrap().clear_episodes();
        self.offside.lock().unwrap().clear_episodes();
        self.test_declarer.lock().unwrap().clear_episodes();
        self.test_whist.lock().unwrap().clear_episodes();
        self.test_offside.lock().unwrap().clear_episodes();


        Ok(())
    }


    pub fn play_learning_episode(&mut self, seed: &ContractGameDescription) -> Result<(), BrydzModelError>{

        let mut rng = thread_rng();
        //let role:

        self.prepare_episode(seed, Testing::None)?;
        self.run_episode(Testing::None)?;
        self.declarer.lock().unwrap().store_episode();
        self.whist.lock().unwrap().store_episode();
        self.offside.lock().unwrap().store_episode();

        Ok(())
    }
    pub fn play_learning_episode_one_learner(&mut self, seed: &ContractGameDescription, role: PlayRole) -> Result<(), BrydzModelError>{

        let roles_to_disable_exploring = match role{
            PlayRole::Whist => [PlayRole::Declarer, PlayRole::Offside],
            PlayRole::Declarer | PlayRole::Dummy => [PlayRole::Whist, PlayRole::Offside],
            PlayRole::Offside => [PlayRole::Whist, PlayRole::Declarer]
        };
        for r in roles_to_disable_exploring{
            self.set_exploration_for_agent(r, false)?;
        }

        self.prepare_episode(seed, Testing::None)?;
        self.run_episode(Testing::None)?;
        self.declarer.lock().unwrap().store_episode();
        self.whist.lock().unwrap().store_episode();
        self.offside.lock().unwrap().store_episode();

        for r in roles_to_disable_exploring{
            self.set_exploration_for_agent(r, true)?;
        }
        Ok(())
    }

    pub(crate) fn store_agents_var(&self, agent: &BrydzDynamicAgent, file: & impl AsRef<Path>) -> Result<(), BrydzModelError>{
        let g = agent.lock().map_err(|e|{
            BrydzModelError::Mutex("Failed locking agent for borrowing varstore to save to file".into())
        })?;

        let var = g.get_var_store();
        var.save(file).map_err(|e|{
            BrydzModelError::Tch(e)
        })
    }
    pub fn learning_epoch(&mut self, number_of_games: usize) -> Result<(), BrydzModelError>{




        self.clear_trajectories()?;
        let distributions = generate_biased_deal_distributions(number_of_games as u64);
        let mut rng = thread_rng();

        let learning_choices = [PlayRole::Declarer, PlayRole::Offside, PlayRole::Whist];
        let explorer = learning_choices.choose(&mut rng).unwrap();





        for d in distributions.into_iter(){
            let contract_params = Standard{}.sample(&mut rng);
            let cards = d.sample(&mut rng);
            let description = ContractGameDescription::new(
                contract_params, d, cards);

            self.play_learning_episode_one_learner(&description, explorer.clone())?;

        }
        debug!("Played {} games in epoch", number_of_games);
        self.declarer.lock().unwrap().simple_apply_experience()?;
        self.whist.lock().unwrap().simple_apply_experience()?;
        self.offside.lock().unwrap().simple_apply_experience()?;

        Ok(())
    }

    fn get_learning_agent(&self, role: PlayRole) -> Option<&BrydzDynamicAgent>{
        match role{
            PlayRole::Whist => Some(&self.whist),
            PlayRole::Declarer => Some(&self.declarer),
            PlayRole::Offside => Some(&self.offside),
            PlayRole::Dummy => None,
        }
    }

    fn set_exploration_for_agent(&mut self, role: PlayRole, exploring: bool) -> Result<(), BrydzModelError>{
        match self.get_learning_agent(role){
            Some(a) => {
                let mut g = a.lock().map_err(|e|{
                    BrydzModelError::Mutex(format!("Locking agent to switch on/off learning: {e:}"))
                })?;
                g.set_exploration(exploring);
                Ok(())
            },
            None => {
                Ok(())
            }
        }
    }
}

