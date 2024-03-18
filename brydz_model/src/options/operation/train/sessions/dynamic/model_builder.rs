use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use rand::thread_rng;
use amfiteatr_core::agent::{AgentGen, EvaluatedInformationSet, InformationSet, PresentPossibleActions, RandomPolicy, TracingAgentGen};
use amfiteatr_core::comm::{StdAgentEndpoint, StdEnvironmentEndpoint};
use amfiteatr_core::env::HashMapEnvironment;
use amfiteatr_rl::agent::{RlSimpleLearningAgent, RlSimpleTestAgent};
use amfiteatr_rl::error::AmfiRLError;
use amfiteatr_rl::policy::{QLearningPolicy, QSelector, TrainConfig};
use amfiteatr_rl::tch::{nn, Tensor};
use amfiteatr_rl::tch::nn::{OptimizerConfig, VarStore};
use amfiteatr_rl::tensor_data::{ConversionToTensor, CtxTryIntoTensor};
use amfiteatr_rl::torch_net::{NeuralNetTemplate, QValueNet};
use brydz_core::amfiteatr::comm::ContractAgentSyncComm;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractActionWayToTensor, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateComplete, ContractInfoSetConvert420, ContractInfoSetConvertSparse};
use brydz_core::contract::{Contract, ContractParameters};
use brydz_core::deal::DescriptionDeckDeal;
use brydz_core::player::role::PlayRole;
use brydz_core::player::side::Side;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{AgentConfiguration, ContractInfoSetSeed, PolicyTypeSelect};
use crate::SimContractParams;

pub struct DynamicBridgeModelBuilder{

    env: HashMapEnvironment<ContractDP, ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,
    declarer: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    whist: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    offside: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,

    pub dummy: AgentGen<ContractDP, RandomPolicy<ContractDP, ContractDummyState>, ContractAgentSyncComm>,

    test_declarer: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    test_whist: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    test_offside: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,

    test_vectors: Option<Vec<SimContractParams>>,

    initial_deal: SimContractParams,


}

impl DynamicBridgeModelBuilder{
    pub fn new() -> Self{
        let mut rng = thread_rng();

        let contract = SimContractParams::new_fair_random(&mut rng);

        let contract_params = contract.parameters();
        let deal_description = DescriptionDeckDeal{
            probabilities: contract.distribution().clone(),
            cards: contract.cards().clone(),
        };
        let mut hm  = HashMap::new();
        let dummy_side = contract.parameters().dummy();
        let (comm_env_dummy, comm_dummy_env) = StdEnvironmentEndpoint::new_pair();
        let dummy = AgentGen::new(
            ContractDummyState::new(
                dummy_side, contract.cards()[&dummy_side], Contract::new(contract.parameters().clone())
            ),
            comm_dummy_env, RandomPolicy::new()
        );
        hm.insert(dummy_side, comm_env_dummy);

        let env = HashMapEnvironment::new(ContractEnvStateComplete::from((contract.parameters(), &deal_description)), hm);
        Self{
            env,
            declarer: None,
            whist: None,
            offside: None,
            dummy,
            test_declarer: None,
            test_whist: None,
            test_offside: None,
            test_vectors: None,
            initial_deal: contract,
        }
    }

    /*
    fn create_info_set<IS: From<(&Side, &ContractParameters, &DescriptionDeckDeal)>>(args: (&Side, &ContractParameters, &DescriptionDeckDeal))
    -> IS
    
     */


    pub fn create_agent_q_policy<
        InfoSet: EvaluatedInformationSet<ContractDP> + Debug + CtxTryIntoTensor<IS2T> + PresentPossibleActions<ContractDP>,
        IS2T: ConversionToTensor>
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, is2t: IS2T)
        -> Result<QLearningPolicy<ContractDP, InfoSet, IS2T, ContractActionWayToTensor>, AmfiRLError<ContractDP>>{

    // Result<(Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvEndpoint<ContractDP>), AmfiRLError<ContractDP>>{
        
        //let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();
        
        //let description = self.initial_deal.distribution();

        let input_shape: i64 = match agent_configuration.info_set_conversion_type{
            InfoSetWayToTensorSelect::_420 => {
                ContractInfoSetConvert420::default().desired_shape().iter().sum()
            },
            InfoSetWayToTensorSelect::Sparse => {
                ContractInfoSetConvertSparse::default().desired_shape().iter().sum()
            }
        };
        let hidden_layers = &agent_configuration.policy_params.hidden_layers;
        let network_pattern = NeuralNetTemplate::new(|path| {
            let mut seq = nn::seq();
            let mut last_dim = None;
            if !hidden_layers.is_empty(){
                let mut ld = hidden_layers[0];

                last_dim = Some(ld);
                seq = seq.add(nn::linear(path / "INPUT", input_shape+2, ld, Default::default()));

                for i in 0..hidden_layers.len(){
                    let ld_new = hidden_layers[i];
                    seq = seq.add(nn::linear(path / &format!("h_{:}", i+1), ld, ld_new, Default::default()))
                        .add_fn(|xs| xs.relu());
                    ld = ld_new;
                    last_dim = Some(ld);
                }
            }
            if let Some(ld) = last_dim{
                seq = seq.add(nn::linear(path / "Q", ld, 1, Default::default()));
            } else {
                seq = seq.add(nn::linear(path / "Q", input_shape+2, 1, Default::default()));
            }
            let device = path.device();
            {move |xs: &Tensor|{
                xs.to_device(device).apply(&seq)
            }}
        });

        let net = network_pattern.get_net_closure();
        let optimiser = agent_configuration.policy_params.optimizer_params.build(&var_store, agent_configuration.policy_params.learning_rate)?;
        let net = QValueNet::new(var_store, net, );
        Ok(QLearningPolicy::new(net, optimiser, is2t, ContractActionWayToTensor::default(), QSelector::EpsilonGreedy(0.1), TrainConfig{ gamma: 0.99 }))

        /*


         */
    }



    /*
    fn create_dynamic_agent(&self, agent_configuration: &AgentConfiguration, var_store: VarStore, side: Side)
    -> Result<(Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvironmentEndpoint<ContractDP>), AmfiRLError<ContractDP>>
    {
        let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();

        let description = DescriptionDeckDeal{
            probabilities: self.initial_deal.distribution().clone(),
            cards: self.initial_deal.cards().clone(),
        };



        let agent: Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>> = match  agent_configuration.info_set_conversion_type {
            InfoSetWayToTensorSelect::_420 => {
                /*
                match agent_configuration.info_set_type{
                    InfoSetTypeSelect::Simple => {
                        let info_set = ContractAgentInfoSetSimple::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Assume => {
                        let info_set = ContractAgentInfoSetAssuming::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Complete => {
                        let info_set = ContractAgentInfoSetAllKnowing::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                }

                 */
                todo!()

            },

            InfoSetWayToTensorSelect::Sparse => {
                match agent_configuration.info_set_type{
                    InfoSetTypeSelect::Simple => {
                        let info_set = ContractAgentInfoSetSimple::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        //todo!()
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Assume => {
                        let info_set = ContractAgentInfoSetAssuming::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        //Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                        todo!()
                    }
                    InfoSetTypeSelect::Complete => {
                        let info_set = ContractAgentInfoSetAllKnowing::from((&side, self.initial_deal.parameters(), &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        //Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                        todo!()
                    }
                }
            }

        };
        Ok((agent, env_endpoint))


    }

     */



}

/*
macro_rules! apply_info_set {
    (t: ty) => {
        let info_set = t::from((&side, self.initial_deal.cards(), description));
        Arc::new(Mutex::new(info_set, agent_endpoint, ))
    }
}



 */
