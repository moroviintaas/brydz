use std::fs;
use rand::prelude::{Distribution};
use rand::thread_rng;
use amfiteatr_rl::tch::{nn, Tensor};
use amfiteatr_rl::tch::nn::{Adam, VarStore};
use brydz_core::contract::{ContractRandomizer};
use brydz_core::deal::{ContractGameDescription, DealDistribution, DescriptionDeckDeal};

use brydz_core::player::side::{Side, SideMap};
use brydz_core::amfiteatr::comm::{ContractAgentSyncComm, ContractEnvSyncComm};
use brydz_core::amfiteatr::env::ContractEnv;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::*;
use amfiteatr_core::agent::*;
use amfiteatr_rl::policy::ActorCriticPolicy;
use amfiteatr_rl::error::AmfiteatrRlError;
use amfiteatr_rl::tensor_data::{ConversionToTensor};
use amfiteatr_rl::torch_net::{A2CNet, NeuralNetTemplate, TensorActorCritic};
use crate::options::operation::train::sessions::{ContractInfoSetForLearning, TSession};
use crate::options::operation::train::TrainOptions;


pub type ContractA2CLocalAgent<ISW, S> = TracingAgentGen<
    ContractDP,
    ActorCriticPolicy<
        ContractDP,
        S,
        ISW>,
    ContractAgentSyncComm>;

pub struct TestAgents<P: Policy<ContractDP>>
where P: Policy<ContractDP, InfoSetType= ContractAgentInfoSetAllKnowing>{
    pub declarer: AgentGen<ContractDP, P, ContractAgentSyncComm>,
    pub whist: AgentGen<ContractDP, P, ContractAgentSyncComm>,
    pub offside: AgentGen<ContractDP, P, ContractAgentSyncComm>,
}

#[allow(clippy::type_complexity)]
pub fn t_session_a2c_symmetric<
    InfoSet: ContractInfoSetForLearning<W2T> + Clone,
    W2T: ConversionToTensor + Default,
>(
    //declarer_policy: QLearningPolicy<ContractDP, DIS, DISW2T, ContractActionWayToTensor>,
    //whist_policy: QLearningPolicy<ContractDP, WIS, WISW2T, ContractActionWayToTensor>,
    //offside_policy: QLearningPolicy<ContractDP, OIS, OISW2T, ContractActionWayToTensor>,
    options: &TrainOptions,
) -> Result<TSession<
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    ActorCriticPolicy<ContractDP, InfoSet, W2T>,
    W2T, W2T, W2T, W2T, W2T, W2T,

>, AmfiteatrRlError<ContractDP>>
where InfoSet: InformationSet<ContractDP>{

    let mut rng = thread_rng();
    let contract_params = ContractRandomizer::default().sample(&mut rng);
    let deal_description = DescriptionDeckDeal{
        probabilities: DealDistribution::Fair,
        cards: DealDistribution::Fair.sample(&mut rng)
    };

    let test_set = options.test_set.as_ref().map(|path|{
        let test_str = fs::read_to_string(path).unwrap();
        let set: Vec<ContractGameDescription> = ron::de::from_str(&test_str).unwrap();
        set
    });

    let network_pattern = NeuralNetTemplate::new(|path| {
        let mut seq = nn::seq();
        let mut last_dim = None;
        let tensor_repr = W2T::default();
        let input_shape = tensor_repr.desired_shape().iter().sum();
        if !options.hidden_layers.is_empty(){
            let mut ld = options.hidden_layers[0];

            last_dim = Some(ld);
            seq = seq.add(nn::linear(path / "INPUT", input_shape, ld, Default::default()));

            for i in 1..options.hidden_layers.len(){
                let ld_new = options.hidden_layers[i];
                seq = seq.add(nn::linear(path / &format!("h_{:}", i+1), ld, ld_new, Default::default()))
                    .add_fn(|xs| xs.relu());

                ld = ld_new;
                last_dim = Some(ld);
            }
        }
        let (actor, critic) = match last_dim{
            None => {
                (nn::linear(path / "al", input_shape, 52, Default::default()),
                 nn::linear(path / "cl", input_shape, 1, Default::default()))
            }
            Some(ld) => {
                (nn::linear(path / "al", ld, 52, Default::default()),
                 nn::linear(path / "cl", ld, 1, Default::default()))
            }
        };
        /*
        if let Some(ld) = last_dim{
            seq = seq.add(nn::linear(path / "Q", ld, 1, Default::default()));
        } else {
            seq = seq.add(nn::linear(path / "Q", W2T::desired_shape()[0], 1, Default::default()));
        }*/

        let device = path.device();
        {move |xs: &Tensor|{
            if seq.is_empty(){
                TensorActorCritic {critic: xs.apply(&critic), actor: xs.apply(&actor)}
            } else {
                let xs = xs.to_device(device).apply(&seq);
                TensorActorCritic {critic: xs.apply(&critic), actor: xs.apply(&actor)}
            }
        }}
    });


    let (comm_env_decl, comm_decl_env) = ContractEnvSyncComm::new_pair();
    let (comm_env_whist, comm_whist_env) = ContractEnvSyncComm::new_pair();
    let (comm_env_dummy, comm_dummy_env) = ContractEnvSyncComm::new_pair();
    let (comm_env_offside, comm_offside_env) = ContractEnvSyncComm::new_pair();
    let (_, comm_decl_test_env) = ContractEnvSyncComm::new_pair();
    let (_, comm_whist_test_env) = ContractEnvSyncComm::new_pair();
    let (_, comm_offside_test_env) = ContractEnvSyncComm::new_pair();

    let mut declarer_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());
    if let Some(p) =  &options.declarer_load{
        declarer_net.var_store_mut().load(p)?;
    }
    let mut whist_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());
    if let Some(p) =  &options.whist_load{
        whist_net.var_store_mut().load(p)?;
    }
    let mut offside_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());
    if let Some(p) =  &options.offside_load{
        offside_net.var_store_mut().load(p)?;
    }
    let mut declarer_test_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());
    let mut whist_test_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());
    let mut offside_test_net = A2CNet::new(VarStore::new(options.device.map()), network_pattern.get_net_closure());

    declarer_test_net.var_store_mut().copy(declarer_net.var_store())?;
    whist_test_net.var_store_mut().copy(whist_net.var_store())?;
    offside_test_net.var_store_mut().copy(offside_net.var_store())?;

    let declarer_optimiser = declarer_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();
    let whist_optimiser = whist_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();
    let offside_optimiser = offside_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();
    let declarer_test_optimiser = declarer_test_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();
    let whist_test_optimiser = whist_test_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();
    let offside_test_optimiser = offside_test_net.build_optimizer(Adam::default(), options.learning_rate).unwrap();

    let declarer_policy: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(declarer_net, declarer_optimiser, W2T::default(), options.into());
    let whist_policy: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(whist_net, whist_optimiser, W2T::default(), options.into());
    let offside_policy: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(offside_net, offside_optimiser, W2T::default(), options.into());

    let declarer_policy_test: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(declarer_test_net, declarer_test_optimiser, W2T::default(), options.into());
    let whist_policy_test: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(whist_test_net, whist_test_optimiser, W2T::default(), options.into());
    let offside_policy_test: ActorCriticPolicy<ContractDP, InfoSet, W2T>  =
        ActorCriticPolicy::new(offside_test_net, offside_test_optimiser, W2T::default(), options.into());


    let declarer = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer(), &contract_params, &deal_description)),
        comm_decl_env, declarer_policy);



    let whist = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer().next_i(1), &contract_params, &deal_description)),
        comm_whist_env, whist_policy);

    let offside = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer().next_i(3), &contract_params, &deal_description)),
        comm_offside_env, offside_policy);


    let test_declarer = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer(), &contract_params, &deal_description)),
        comm_decl_test_env, declarer_policy_test);


    let test_whist = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer().next_i(1), &contract_params, &deal_description)),
        comm_whist_test_env, whist_policy_test);

    let test_offside = ContractA2CLocalAgent::new(
        InfoSet::from((&contract_params.declarer().next_i(3), &contract_params, &deal_description)),
        comm_offside_test_env, offside_policy_test);

    let dummy = AgentGen::new(
        ContractDummyState::from((&contract_params.declarer().next_i(2), &contract_params, &deal_description)), comm_dummy_env, RandomPolicy::new(), );

    let (north_comm, east_comm, south_comm, west_comm) = match contract_params.declarer() {
        Side::East => (comm_env_offside, comm_env_decl, comm_env_whist, comm_env_dummy),
        Side::South => (comm_env_dummy, comm_env_offside, comm_env_decl, comm_env_whist),
        Side::West => (comm_env_whist, comm_env_dummy, comm_env_offside, comm_env_decl),
        Side::North => ( comm_env_decl, comm_env_whist, comm_env_dummy, comm_env_offside),
    };
    let environment = ContractEnv::new(
        ContractEnvStateComplete::from((&contract_params, &deal_description)),
        SideMap::new(north_comm, east_comm, south_comm, west_comm));

    Ok(TSession::_new(
        environment,
        declarer,
        whist,
        offside,
        dummy,
        test_declarer,
        test_whist,
        test_offside,
        test_set,
    ))

}