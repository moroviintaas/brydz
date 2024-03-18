use std::collections::HashMap;
use std::sync::{Arc, Mutex};
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
use brydz_core::contract::{Contract, ContractParameters};
use brydz_core::deal::DescriptionDeckDeal;
use brydz_core::player::side::{Side, SideMap};
use brydz_core::player::side::Side::{East, North, South, West};
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{ContractInfoSetSeed, PolicyTypeSelect};
use crate::SimContractParams;


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
    pub test_declarer: Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub test_whist: Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    pub test_offside: Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>,
    



    inactive_declarer_comm: StdEnvironmentEndpoint<ContractDP>,
    inactive_whist_comm: StdEnvironmentEndpoint<ContractDP>,
    inactive_offside_comm: StdEnvironmentEndpoint<ContractDP>,

    test_vectors: Vec<SimContractParams>,
    initial_deal: SimContractParams,
}




impl DynamicBridgeModel{

    fn swap_defense(&mut self){
        let ws = self.env.state().whist_side();
        let os = self.env.state().offside_side();
        std::mem::swap(self.env.comms_mut().get_mut(&ws).unwrap(), &mut self.inactive_whist_comm);
        std::mem::swap(self.env.comms_mut().get_mut(&os).unwrap(), &mut self.inactive_offside_comm);
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

    pub fn prepare_episode(&mut self, seed: (ContractParameters, DescriptionDeckDeal), testing: Testing) -> Result<(), AmfiRLError<ContractDP>>{

        let old_declarer_side = self.env.state().declarer_side();
        let new_declarer_side = seed.0.declarer();
        self.rotate_environment_comms(old_declarer_side, new_declarer_side);

        let seed_refs = (&seed.0, &seed.1);
        self.env.reseed(seed_refs)?;

        let dummy_side = seed_refs.0.dummy();
        self.dummy.reseed((&dummy_side, seed_refs.0, seed_refs.1))?;

        let declarer_side = seed_refs.0.declarer();
        let whist_side = seed_refs.0.whist();
        let offside_side = seed_refs.0.offside();



        match testing{
            Testing::Declarer => {
                // testing declarer
                self.declarer.lock().unwrap().reseed((&declarer_side, seed_refs.0, seed_refs.1))?;
                self.test_whist.lock().unwrap().reseed((&whist_side, seed_refs.0, seed_refs.1))?;
                self.test_offside.lock().unwrap().reseed((&offside_side, seed_refs.0, seed_refs.1))?;
                //let
            }
            Testing::Defenders => {
                self.test_declarer.lock().unwrap().reseed((&declarer_side, seed_refs.0, seed_refs.1))?;
                self.whist.lock().unwrap().reseed((&whist_side, seed_refs.0, seed_refs.1))?;
                self.offside.lock().unwrap().reseed((&offside_side, seed_refs.0, seed_refs.1))?;
            }
            Testing::None => {
                self.declarer.lock().unwrap().reseed((&declarer_side, seed_refs.0, seed_refs.1))?;
                self.whist.lock().unwrap().reseed((&whist_side, seed_refs.0, seed_refs.1))?;
                self.offside.lock().unwrap().reseed((&offside_side, seed_refs.0, seed_refs.1))?;
            }
        }



        Ok(())
    }

    pub fn run_episode(&mut self, testing: Testing){

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
    }
    pub fn run_learning_episode(&mut self, seed: ()) -> Result<(), AmfiRLError<ContractDP>>{

        //self.env.re

        Ok(())
    }
}