use std::cmp::min;
use std::thread;
use log::debug;
use rand::distributions::{Distribution};
use rand::prelude::ThreadRng;
use rand::{Rng, thread_rng};
use rand_distr::Geometric;
use tch::Kind::Float;
use tch::Tensor;
use brydz_core::contract::{Contract, ContractMechanics, ContractParameters};
use brydz_core::deal::{BiasedHandDistribution, fair_bridge_deal};
use brydz_core::meta::HAND_SIZE;
use brydz_core::player::axis::Axis::{EastWest, NorthSouth};
use brydz_core::player::side::Side::{East, North, South, West};
use brydz_core::player::side::SideMap;
use brydz_core::amfi::agent::TracingContractAgent;
use brydz_core::amfi::comm::{ContractAgentSyncComm, ContractEnvSyncComm};
use brydz_core::amfi::env::ContractEnv;
use brydz_core::amfi::spec::ContractDP;
use brydz_core::amfi::state::{BuildStateHistoryTensor, ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateMin, CreatedContractInfoSet, StateWithSide};
use karty::hand::CardSet;
use amfiteatr_core::agent::{AutomaticAgent, PolicyAgent, RandomPolicy, TracingAgent, ReinitAgent, ScoringInformationSet, InformationSet, PresentPossibleActions};
use crate::{ContractStateHistQPolicy, EEPolicy, single_play};
use crate::error::BrydzSimError;
use amfiteatr_core::env::{RoundRobinUniversalEnvironment, StatefulEnvironment};
use crate::options::operation::train::{load_var_store, random_contract_params, SequentialB, TrainOptions};


const LEARNING_RATE: f64 = 1e-4;

pub(crate) type QNetStateHistAgent<St> = TracingContractAgent<ContractAgentSyncComm, EEPolicy<ContractStateHistQPolicy<St>>>;
pub(crate) type DummyAgent2 = TracingContractAgent<ContractAgentSyncComm, RandomPolicy<ContractDP, ContractDummyState>>;
pub(crate) type SimpleEnv2 = ContractEnv<ContractEnvStateMin, ContractEnvSyncComm>;



pub fn train_episode_state_hist<
    St: ScoringInformationSet<ContractDP,
        RewardType=i32> + BuildStateHistoryTensor + Clone
    + StateWithSide
    + PresentPossibleActions<ContractDP>
    + Send>(
    ready_env: &mut SimpleEnv2,
    ready_declarer: &mut QNetStateHistAgent<St>,
    ready_whist: &mut QNetStateHistAgent<St>,
    ready_offside: &mut QNetStateHistAgent<St>,
    ready_dummy: &mut DummyAgent2,
    rng: &mut ThreadRng, geo: &mut Geometric) -> Result<(), BrydzSimError>
//where for<'a> f32: From<&'a <St as InformationSet<ContractProtocolSpec>>::RewardType>
{

    let step_start_explore = min(geo.sample(rng), HAND_SIZE as u64);
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
            let (state, action, reward ) = agent.game_trajectory().list()[i].s_a_r_subjective();
            //accumulated_reward += &Into::<f32>::into(reward);
            accumulated_reward += reward as f32;//f32::from(reward);
            debug!("Applying train vector for {} (accumulated reward: {})", agent.id(), accumulated_reward);
            let t = state.state_history_tensor().f_flatten(0,1).unwrap();
            let ta = Tensor::from_slice(&action.sparse_representation());
            let input = tch::Tensor::cat(&[t,ta], 0);

            //let optimiser = agent.policy_mut().internal_policy_mut().optimizer_mut();
            let q = (agent.policy_mut().internal_policy_mut().model())(&input);
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
    Ok(())


}

fn run_test_set2<St: CreatedContractInfoSet + BuildStateHistoryTensor + StateWithSide + Send + Clone
        + PresentPossibleActions<ContractDP>>(
    env: &mut SimpleEnv2,
    declarer: &mut QNetStateHistAgent<St>,
    whist: &mut QNetStateHistAgent<St>,
    offside: &mut QNetStateHistAgent<St>,
    dummy: &mut DummyAgent2,
    test_params: &[(ContractParameters, SideMap<CardSet>)])-> Result<SideMap<f64>, BrydzSimError>{


    let mut sum_north_south = 0.0;
    let mut sum_east_west =0.0;
    for (param, cards) in test_params{
        renew_world2(param.to_owned(), cards.to_owned(), env, declarer, whist, offside, dummy)?;
        single_play(env, declarer, whist, offside, dummy);
        sum_north_south += env.state().contract().total_tricks_taken_axis(NorthSouth) as f64;
        sum_east_west += env.state().contract().total_tricks_taken_axis(EastWest) as f64;
    }
    sum_north_south /= test_params.len() as f64;
    sum_east_west /= test_params.len() as f64;

    Ok(SideMap::new(sum_north_south, sum_east_west, sum_north_south, sum_east_west))
}

fn run_test_set2_with_assumption<
    St: CreatedContractInfoSet + BuildStateHistoryTensor + StateWithSide + Send + Clone
        +PresentPossibleActions<ContractDP>>(
    env: &mut SimpleEnv2,
    declarer: &mut QNetStateHistAgent<St>,
    whist: &mut QNetStateHistAgent<St>,
    offside: &mut QNetStateHistAgent<St>,
    dummy: &mut DummyAgent2,
    test_params: &[(ContractParameters, SideMap<CardSet>, BiasedHandDistribution)])-> Result<SideMap<f64>, BrydzSimError>{


    //let card_distribution: BiasedHandDistribution = thread_rng().generate();
    let mut sum_north_south = 0.0;
    let mut sum_east_west =0.0;
    for (param, cards, biased_card_distribution) in test_params{
        renew_world2_with_assumption(
            param.to_owned(),
            cards.to_owned(), env, declarer, whist, offside, dummy,
            biased_card_distribution.clone())?;
        single_play(env, declarer, whist, offside, dummy);
        sum_north_south += env.state().contract().total_tricks_taken_axis(NorthSouth) as f64;
        sum_east_west += env.state().contract().total_tricks_taken_axis(EastWest) as f64;
    }
    sum_north_south /= test_params.len() as f64;
    sum_east_west /= test_params.len() as f64;

    Ok(SideMap::new(sum_north_south, sum_east_west, sum_north_south, sum_east_west))
}


fn renew_world2<
    St: CreatedContractInfoSet + BuildStateHistoryTensor + StateWithSide + Send + Clone
        + PresentPossibleActions<ContractDP>
>(contract_params: ContractParameters, cards: SideMap<CardSet>,
               env: &mut SimpleEnv2,
               declarer: &mut QNetStateHistAgent<St>, whist: &mut QNetStateHistAgent<St>, offside: &mut QNetStateHistAgent<St>,
               dummy: &mut DummyAgent2) -> Result<(), BrydzSimError>{
    let contract = Contract::new(contract_params);
    let dummy_side = contract.dummy();
    env.replace_state(ContractEnvStateMin::new(contract.clone(), None));
    declarer.reinit(St::create_new(*declarer.id(), cards[&declarer.id()], contract.clone(), None, Default::default()));
    whist.reinit(St::create_new(*whist.id(), cards[&whist.id()], contract.clone(), None, Default::default()));
    offside.reinit(St::create_new(*offside.id(), cards[&offside.id()], contract.clone(), None, Default::default()));
    dummy.reinit(ContractDummyState::new(dummy_side, cards[&dummy_side], contract));


    Ok(())

}

#[allow(clippy::too_many_arguments)]
fn renew_world2_with_assumption<
    St: CreatedContractInfoSet + BuildStateHistoryTensor + StateWithSide + Send + Clone
         + PresentPossibleActions<ContractDP>
>(contract_params: ContractParameters, cards: SideMap<CardSet>,
               env: &mut SimpleEnv2,
               declarer: &mut QNetStateHistAgent<St>, whist: &mut QNetStateHistAgent<St>, offside: &mut QNetStateHistAgent<St>,
               dummy: &mut DummyAgent2, distribution_assumption: BiasedHandDistribution) -> Result<(), BrydzSimError>{
    let contract = Contract::new(contract_params);
    let dummy_side = contract.dummy();
    env.replace_state(ContractEnvStateMin::new(contract.clone(), None));
    declarer.reinit(St::create_new(*declarer.id(), cards[&declarer.id()], contract.clone(), None, distribution_assumption.clone()));
    whist.reinit(St::create_new(*whist.id(), cards[&whist.id()], contract.clone(), None, distribution_assumption.clone()));
    offside.reinit(St::create_new(*offside.id(), cards[&offside.id()], contract.clone(), None, distribution_assumption));
    dummy.reinit(ContractDummyState::new(dummy_side, cards[&dummy_side], contract));


    Ok(())

}

pub fn train_session2<St: InformationSet<ContractDP> + BuildStateHistoryTensor + Send>(
    train_options: &TrainOptions,
    sequential_gen: &SequentialB) -> Result<(), BrydzSimError>{
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

    let mut policy_declarer_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let mut policy_whist_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let mut policy_offside_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE, sequential_gen));

    let policy_declarer = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let policy_whist = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let policy_offside = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE, sequential_gen));
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

    let mut declarer = QNetStateHistAgent::new(North, initial_state_declarer, comm_north, policy_declarer);
    let mut whist = QNetStateHistAgent::new(East, initial_state_whist, comm_east, policy_whist);
    let mut offside = QNetStateHistAgent::new(West, initial_state_offside, comm_west, policy_offside);
    let mut dummy = DummyAgent2::new(South,initial_state_dummy, comm_south, policy_dummy);

    let mut env = SimpleEnv2::new(env_state, comm_association);

    // Before training

    let test_results = run_test_set2(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
    println!("Test set run before training:\n\tDeclarer:\t{:}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
    //info!()


    for e in 0..train_options.epochs{

        for _g in 0..train_options.games{
            let contract_params = random_contract_params(North, &mut rng);
            renew_world2(contract_params, fair_bridge_deal(), &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy)?;
            train_episode_state_hist( &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &mut rng, &mut geo)?;
        }

        println!("Epoch {}", e+1);
        //demo_op declarer
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);
        let test_results = run_test_set2(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDeclarer vs reference defenders:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);

        std::mem::swap(declarer.policy_mut(), &mut policy_declarer_ref);
        let test_results = run_test_set2(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDefenders vs reference declarer:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_declarer_ref);




    }

    Ok(())
}


pub fn train_session2_with_assumption<St: InformationSet<ContractDP> + BuildStateHistoryTensor + Send>(
    train_options: &TrainOptions,
    sequential_gen: &SequentialB) -> Result<(), BrydzSimError>{
    let mut rng = thread_rng();
    //let test_set: Vec<(ContractParameters, SideMap<CardSet>)> = Range::new(0..train_options.tests_set_size)
    //   .map().collect
    let declarer_side = North;


    let mut test_set = Vec::with_capacity(train_options.tests_set_size as usize);
    for _i in 0..train_options.tests_set_size{
        let card_distribution: BiasedHandDistribution = rng.gen();
        let card_set = card_distribution.sample(&mut rng);
        let parameters = random_contract_params(declarer_side, &mut rng);
        test_set.push((parameters, card_set, card_distribution.clone()));
    }


    let mut geo = Geometric::new(0.25).unwrap();



    let card_deal = fair_bridge_deal();//card_distribution.sample(&mut rng);

    let mut policy_declarer_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let mut policy_whist_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let mut policy_offside_ref = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE, sequential_gen));

    let policy_declarer = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.declarer_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let policy_whist = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.whist_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let policy_offside = EEPolicy::new(ContractStateHistQPolicy::new(load_var_store(train_options.offside_load.as_ref())?, LEARNING_RATE, sequential_gen));
    let policy_dummy = RandomPolicy::<ContractDP, ContractDummyState>::new();



    let contract = Contract::new(random_contract_params(declarer_side, &mut rng));

    let (comm_env_north, comm_north) = ContractEnvSyncComm::new_pair();
    let (comm_env_east, comm_east) = ContractEnvSyncComm::new_pair();
    let (comm_env_west, comm_west) = ContractEnvSyncComm::new_pair();
    let (comm_env_south, comm_south) = ContractEnvSyncComm::new_pair();
    let comm_association = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);

    //let card_deal = fair_bridge_deal::<CardSet>();
    let initial_state_declarer = ContractAgentInfoSetSimple::new(declarer_side, card_deal[&North], contract.clone(), None);
    let initial_state_whist = ContractAgentInfoSetSimple::new(declarer_side.next(), card_deal[&East], contract.clone(), None);
    let initial_state_offside = ContractAgentInfoSetSimple::new(declarer_side.prev(), card_deal[&West], contract.clone(), None);
    let initial_state_dummy = ContractDummyState::new(declarer_side.partner(), card_deal[&South], contract.clone());
    let env_state = ContractEnvStateMin::new(contract, None);

    let mut declarer = QNetStateHistAgent::new(North, initial_state_declarer, comm_north, policy_declarer);
    let mut whist = QNetStateHistAgent::new(East, initial_state_whist, comm_east, policy_whist);
    let mut offside = QNetStateHistAgent::new(West, initial_state_offside, comm_west, policy_offside);
    let mut dummy = DummyAgent2::new(South, initial_state_dummy, comm_south, policy_dummy);

    let mut env = SimpleEnv2::new(env_state, comm_association);

    // Before training

    let test_results = run_test_set2_with_assumption(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
    println!("Test set run before training:\n\tDeclarer:\t{:}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
    //info!()


    for e in 0..train_options.epochs{

        for _g in 0..train_options.games{
            let card_distribution: BiasedHandDistribution = rng.gen();
            let contract_params = random_contract_params(North, &mut rng);
            renew_world2_with_assumption(contract_params, card_distribution.sample(&mut rng), &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, card_distribution.clone())?;
            train_episode_state_hist( &mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &mut rng, &mut geo)?;
        }

        println!("Epoch {}", e+1);
        //demo_op declarer
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);
        let test_results = run_test_set2_with_assumption(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDeclarer vs reference defenders:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_whist_ref);
        std::mem::swap(offside.policy_mut(), &mut policy_offside_ref);

        std::mem::swap(declarer.policy_mut(), &mut policy_declarer_ref);
        let test_results = run_test_set2_with_assumption(&mut env, &mut declarer, &mut whist, &mut offside, &mut dummy, &test_set)?;
        println!("\nDefenders vs reference declarer:\n\tDeclarer:\t{}\n\tDefenders:\t{}", test_results[&North], test_results[&East]);
        std::mem::swap(whist.policy_mut(), &mut policy_declarer_ref);




    }

    Ok(())
}

