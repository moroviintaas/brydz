use std::thread;
use brydz_core::amfi::spec::ContractDP;
use amfiteatr_core::agent::AutomaticAgent;
use amfiteatr_core::env::RoundRobinUniversalEnvironment;
use crate::options::operation::train::{DummyAgent, SimpleEnv};

pub fn single_play<D: AutomaticAgent<ContractDP> + Send,
WHIST: AutomaticAgent<ContractDP>+ Send,
OFFSIDE: AutomaticAgent<ContractDP>+ Send>(ready_env: &mut SimpleEnv,
                                           ready_declarer: &mut D,
                                           ready_whist: &mut WHIST,
                                           ready_offside: &mut OFFSIDE,
                                           ready_dummy: &mut DummyAgent){

    thread::scope(|s|{
        s.spawn(||{
            ready_env.run_round_robin_uni_rewards().unwrap();
        });
        s.spawn(||{
            ready_declarer.run().unwrap();
        });

        s.spawn(||{
            ready_whist.run().unwrap();
        });

        s.spawn(||{
            ready_offside.run().unwrap();
        });

        s.spawn(||{
            ready_dummy.run().unwrap();
        });
    });
}