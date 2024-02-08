use std::cmp::{min};
use std::path::PathBuf;
use std::thread;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand_distr::Geometric;
use tch::Device;
use tch::nn::VarStore;
use brydz_core::bidding::{Bid, Doubling};
use brydz_core::cards::trump::TrumpGen;
use brydz_core::contract::{Contract, ContractMechanics, ContractParameters};
use brydz_core::deal::fair_bridge_deal;
use brydz_core::meta::HAND_SIZE;
use brydz_core::player::side::{Side, SideMap};
use brydz_core::player::side::Side::*;
use brydz_core::amfi::agent::TracingContractAgent;
use brydz_core::amfi::comm::{ContractAgentSyncComm, ContractEnvSyncComm};
use brydz_core::amfi::env::{ContractEnv};
use brydz_core::amfi::spec::ContractDP;
use brydz_core::amfi::state::{ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin};
use karty::hand::CardSet;
use karty::random::RandomSymbol;
use karty::suits::Suit;
use crate::{SyntheticContractQNetSimple, EEPolicy};
use crate::error::BrydzSimError;
use rand_distr::Distribution;
use tch::Kind::Float;
use brydz_core::player::axis::Axis::{EastWest, NorthSouth};
use amfiteatr_core::agent::{AgentWithId, AutomaticAgent, PolicyAgent, RandomPolicy, ReinitAgent, TracingAgent};
use amfiteatr_core::env::{RoundRobinUniversalEnvironment, StatefulEnvironment};
use crate::model::single_play;
use crate::options::operation::train::TrainOptions;

const LEARNING_RATE: f64 = 1e-4;

pub(crate) fn load_var_store(path: Option<&PathBuf>) -> Result<VarStore, BrydzSimError>{
    Ok(match path{
        None => VarStore::new(Device::cuda_if_available()),
        Some(path) => {
            let mut vs = VarStore::new(Device::cuda_if_available());
            vs.load(path)?;
            vs
        }
    })
}

pub(crate) type SimpleQnetAgent = TracingContractAgent<ContractAgentSyncComm, EEPolicy<SyntheticContractQNetSimple>>;

pub(crate) type DummyAgent  = TracingContractAgent<ContractAgentSyncComm, RandomPolicy<ContractDP, ContractDummyState>>;
pub(crate) type SimpleEnv = ContractEnv<ContractEnvStateMin, ContractEnvSyncComm>;

pub fn train_on_single_game(ready_env: &mut SimpleEnv,
                            ready_declarer: &mut SimpleQnetAgent,
                            ready_whist: &mut SimpleQnetAgent,
                            ready_offside: &mut SimpleQnetAgent,
                            ready_dummy: &mut DummyAgent, rng: &mut ThreadRng, geo: &mut Geometric) -> Result<(), BrydzSimError>{


    let step_start_explore = min(geo.sample(rng), HAND_SIZE as u64);

    //ready_declarer.policy_mut().set_exploiting_start(step_start_explore*2);
    let _ = &mut ready_declarer.policy_mut().set_exploiting_start(step_start_explore*2);

    ready_whist.policy_mut().set_exploiting_start(step_start_explore);
    ready_offside.policy_mut().set_exploiting_start(step_start_explore);

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

    for agent in [ready_declarer, ready_whist, ready_offside ]{
        let mut accumulated_reward = 0.0;
        for i in (agent.policy().exploitation_start() as usize.. agent.game_trajectory().list().len()).rev(){
            let (state, action, reward ) =  agent.game_trajectory().list()[i].s_a_r_subjective();
            accumulated_reward += reward as f32;
            let t = tch::Tensor::from(state);
            let ta = tch::Tensor::from(action);
            let input = tch::Tensor::cat(&[t,ta], 0);

            //let optimiser = agent.policy_mut().internal_policy_mut().optimizer_mut();
            let q = (agent.policy_mut().internal_policy_mut().model)(&input);
            let q_from_net = tch::Tensor::from_slice(&[accumulated_reward]);

            //println!("{q:} {q_from_net:}");
            let diff = &q-&q_from_net;
            let loss = (&diff * &diff).mean(Float);

            agent.policy_mut().internal_policy_mut().optimizer_mut().zero_grad();
            loss.backward();
            agent.policy_mut().internal_policy_mut().optimizer_mut().step();
            //println!("{loss:}");

        }
    }


    /*
    println!("{:?}", ready_declarer.trace().iter().map(|(s,a,r)|(r)).collect::<Vec<_>>());
    println!("{:?}", ready_whist.trace().iter().map(|(s,a,r)|(r)).collect::<Vec<_>>());
    println!("{:?}", ready_offside.trace().iter().map(|(s,a,r)|(r)).collect::<Vec<_>>());

    println!("{:?}", ready_declarer.policy().get_step_counter());
    println!("{:?}", ready_declarer.policy().exploitation_start());
    */

    //todo!();
    Ok(())
}

fn run_test_set(env: &mut SimpleEnv,
                            declarer: &mut SimpleQnetAgent,
                            whist: &mut SimpleQnetAgent,
                            offside: &mut SimpleQnetAgent,
                            dummy: &mut DummyAgent,
                            test_params: &[(ContractParameters, SideMap<CardSet>)])-> Result<SideMap<f64>, BrydzSimError>{
    let mut sum_north_south = 0.0;
    let mut sum_east_west =0.0;
    for (param, cards) in test_params{
        renew_world(param.to_owned(), cards.to_owned(), env, declarer, whist, offside, dummy)?;
        single_play(env, declarer, whist, offside, dummy);
        sum_north_south += env.state().contract().total_tricks_taken_axis(NorthSouth) as f64;
        sum_east_west += env.state().contract().total_tricks_taken_axis(EastWest) as f64;
    }
    sum_north_south /= test_params.len() as f64;
    sum_east_west /= test_params.len() as f64;

    Ok(SideMap::new(sum_north_south, sum_east_west, sum_north_south, sum_east_west))
}

pub(crate) fn random_contract_params(declarer: Side, rng: &mut ThreadRng) -> ContractParameters{
    let contract_value = rng.gen_range(1..=7);
    let trump = TrumpGen::<Suit>::random(rng);
    let doubling = *[Doubling::None, Doubling::Redouble, Doubling::Redouble].choose(rng).unwrap();

    ContractParameters::new_d(declarer, Bid::init(trump, contract_value).unwrap(), doubling)
}

fn renew_world(contract_params: ContractParameters, cards: SideMap<CardSet>,
               env: &mut SimpleEnv,
               declarer: &mut SimpleQnetAgent, whist: &mut SimpleQnetAgent, offside: &mut SimpleQnetAgent,
               dummy: &mut DummyAgent) -> Result<(), BrydzSimError>{
    let contract = Contract::new(contract_params);
    let dummy_side = contract.dummy();
    env.replace_state(ContractEnvStateMin::new(contract.clone(), None));
    declarer.reinit(ContractAgentInfoSetSimple::new(*declarer.id(), cards[&declarer.id()], contract.clone(), None));
    whist.reinit(ContractAgentInfoSetSimple::new(*whist.id(), cards[&whist.id()], contract.clone(), None));
    offside.reinit(ContractAgentInfoSetSimple::new(*offside.id(), cards[&offside.id()], contract.clone(), None));
    dummy.reinit(ContractDummyState::new(dummy_side, cards[&dummy_side], contract));
    //declarer.reset_trace();
    //whist.reset_trace();
    //offside.reset_trace();


    Ok(())

}

pub fn train_session(train_options: &TrainOptions) -> Result<(), BrydzSimError>{
    let mut rng = thread_rng();
    //let test_set: Vec<(ContractParameters, SideMap<CardSet>)> = Range::new(0..train_options.tests_set_size)
    //   .map().collect
    let declarer_side = North;

    let mut test_set = Vec::with_capacity(train_options.tests_set_size as usize);
    for _i in 0..train_options.tests_set_size{
        let card_set = fair_bridge_deal::<CardSet>();
        let parameters = random_contract_params(declarer_side, &mut rng);
        test_set.push((parameters, card_set));
    }


    let mut geo = Geometric::new(0.25).unwrap();

    let mut policy_declarer_ref = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE));
    let mut policy_whist_ref = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE));
    let mut policy_offside_ref = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE));

    let policy_declarer = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE));
    let policy_whist = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE));
    let policy_offside = EEPolicy::new(SyntheticContractQNetSimple::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE));
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();



    let contract = Contract::new(random_contract_params(declarer_side, &mut rng));

    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();
    let comm_association = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);

    let card_deal = fair_bridge_deal::<CardSet>();
    let initial_state_declarer = ContractAgentInfoSetSimple::new(declarer_side, card_deal[&North], contract.clone(), None);
    let initial_state_whist = ContractAgentInfoSetSimple::new(declarer_side.next(), card_deal[&East], contract.clone(), None);
    let initial_state_offside = ContractAgentInfoSetSimple::new(declarer_side.prev(), card_deal[&West], contract.clone(), None);
    let initial_state_dummy = ContractDummyState::new(declarer_side.partner(), card_deal[&South], contract.clone());
    let env_state = ContractEnvStateMin::new(contract, None);

    let mut declarer = SimpleQnetAgent::new(North, initial_state_declarer, comm_north, policy_declarer);
    let mut whist = SimpleQnetAgent::new(East, initial_state_whist, comm_east, policy_whist);
    let mut offside = SimpleQnetAgent::new(West, initial_state_offside, comm_west, policy_offside);
    let mut dummy = DummyAgent::new(South , initial_state_dummy, comm_south, policy_dummy);

    let mut env = SimpleEnv::new(env_state, comm_association);

    // Before training

    let test_results = run_test_set(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
    println!("Test set run before training:\n\tDeclarer:\t{:}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
    //info!()


    for e in 0..train_options.epochs{

        for _g in 0..train_options.games{
            let contract_params = random_contract_params(North, &mut rng);
            renew_world(contract_params, fair_bridge_deal(), &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy)?;
            train_on_single_game( &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &mut rng, &mut geo)?;
        }

        println!("Epoch {}", e+1);
        //demo_op declarer
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);
        let test_results = run_test_set(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDeclarer vs reference defenders:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);

        std::mem::swap(declarer.policy_mut(), &mut policy_declarer_ref);
        let test_results = run_test_set(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDefenders vs reference declarer:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_declarer_ref);




    }

    Ok(())
}