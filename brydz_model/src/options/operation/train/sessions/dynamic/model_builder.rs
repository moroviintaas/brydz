use std::collections::HashMap;
use std::fmt::Debug;
use std::fs;
use std::path::{Path};
use std::sync::{Arc, Mutex};

use rand::thread_rng;
use amfiteatr_core::agent::{AgentGen, InformationSet, PresentPossibleActions, RandomPolicy, TracingAgentGen};
use amfiteatr_core::comm::{StdEnvironmentEndpoint};
use amfiteatr_core::domain::Renew;
use amfiteatr_core::env::{HashMapEnvironment, StatefulEnvironment};
use amfiteatr_rl::agent::{RlSimpleLearningAgent};
use amfiteatr_rl::error::AmfiteatrRlError;
use amfiteatr_rl::policy::{ActorCriticPolicy, QLearningPolicy, QSelector, TrainConfig};
use amfiteatr_rl::tch::{nn, Tensor};
use amfiteatr_rl::tch::nn::{Adam, OptimizerConfig, VarStore};
use amfiteatr_rl::tensor_data::{ConversionToTensor, CtxTryIntoTensor};
use amfiteatr_rl::torch_net::{A2CNet, NeuralNetTemplate, QValueNet, TensorA2C};
use brydz_core::amfiteatr::comm::ContractAgentSyncComm;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractActionWayToTensor, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple, ContractDummyState, ContractEnvStateComplete, ContractInfoSetConvert420, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric, ContractState};
use brydz_core::contract::{Contract, ContractMechanics};
use brydz_core::deal::{ContractGameDescription, DescriptionDeckDeal};

use brydz_core::player::side::Side;
use crate::error::BrydzModelError;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{AgentConfiguration, ContractInfoSetSeed, DynamicBridgeModel, PolicyTypeSelect};

#[derive(Copy, Clone, Debug)]
pub enum AgentRole{
    Declarer,
    Whist,
    Offside,
    TestDeclarer,
    TestWhist,
    TestOffside
}



pub struct DynamicBridgeModelBuilder{

    env: HashMapEnvironment<ContractDP, ContractEnvStateComplete, StdEnvironmentEndpoint<ContractDP>>,
    declarer: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    whist: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    offside: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,

    pub dummy: AgentGen<ContractDP, RandomPolicy<ContractDP, ContractDummyState>, ContractAgentSyncComm>,

    test_declarer: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    test_whist: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,
    test_offside: Option<Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>>,

    inactive_declarer_comm: Option<StdEnvironmentEndpoint<ContractDP>>,
    inactive_whist_comm: Option<StdEnvironmentEndpoint<ContractDP>>,
    inactive_offside_comm: Option<StdEnvironmentEndpoint<ContractDP>>,

    test_vectors: Vec<ContractGameDescription>,

    initial_deal: ContractGameDescription,


}

impl DynamicBridgeModelBuilder{
    pub fn new() -> Self{
        let mut rng = thread_rng();

        let contract = ContractGameDescription::new_fair_random(&mut rng);

        let _contract_params = contract.parameters();
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
            inactive_declarer_comm: None,
            inactive_whist_comm: None,
            inactive_offside_comm: None,
            test_vectors: Vec::new(),
            initial_deal: contract,
        }
    }

    /*
    fn create_info_set<IS: From<(&Side, &ContractParameters, &DescriptionDeckDeal)>>(args: (&Side, &ContractParameters, &DescriptionDeckDeal))
    -> IS
    
     */


    pub fn create_agent_q_policy<
        InfoSet: InformationSet<ContractDP> + Debug + CtxTryIntoTensor<IS2T> + PresentPossibleActions<ContractDP>,
        IS2T: ConversionToTensor>
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, is2t: IS2T)
        -> Result<QLearningPolicy<ContractDP, InfoSet, IS2T, ContractActionWayToTensor>, AmfiteatrRlError<ContractDP>>{

    // Result<(Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvEndpoint<ContractDP>), AmfiRLError<ContractDP>>{
        
        //let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();
        
        //let description = self.initial_deal.distribution();

        let input_shape: i64 = match agent_configuration.info_set_conversion_type{
            InfoSetWayToTensorSelect::_420 => {
                ContractInfoSetConvert420::default().desired_shape().iter().sum()
            },
            InfoSetWayToTensorSelect::Sparse => {
                ContractInfoSetConvertSparse::default().desired_shape().iter().sum()
            },
            InfoSetWayToTensorSelect::SparseHistoric => {
                ContractInfoSetConvertSparseHistoric::default().desired_shape().iter().sum()
            },
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
                        .add_fn(|xs| xs.tanh());
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
        let optimiser = Adam::from(agent_configuration.policy_params.optimizer_params).build(&var_store, agent_configuration.policy_params.learning_rate)?;
        let net = QValueNet::new(var_store, net, );
        Ok(QLearningPolicy::new(net, optimiser, is2t, ContractActionWayToTensor::default(), QSelector::EpsilonGreedy(0.1), TrainConfig{ gamma: agent_configuration.policy_params.gamma }))

        /*


         */
    }

    pub fn create_agent_a2c_policy<
        InfoSet: InformationSet<ContractDP> + Debug + CtxTryIntoTensor<IS2T> + PresentPossibleActions<ContractDP>,
        IS2T: ConversionToTensor>
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, is2t: IS2T)
     -> Result<ActorCriticPolicy<ContractDP, InfoSet, IS2T>, AmfiteatrRlError<ContractDP>>{

        // Result<(Arc<Mutex<dyn for<'a> RlSimpleTestAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvEndpoint<ContractDP>), AmfiRLError<ContractDP>>{

        //let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();

        //let description = self.initial_deal.distribution();

        let input_shape: i64 = match agent_configuration.info_set_conversion_type{
            InfoSetWayToTensorSelect::_420 => {
                ContractInfoSetConvert420::default().desired_shape().iter().sum()
            },
            InfoSetWayToTensorSelect::Sparse => {
                ContractInfoSetConvertSparse::default().desired_shape().iter().sum()
            },
            InfoSetWayToTensorSelect::SparseHistoric => {
                ContractInfoSetConvertSparseHistoric::default().desired_shape().iter().sum()
            }
        };
        let hidden_layers = &agent_configuration.policy_params.hidden_layers;
        let network_pattern = NeuralNetTemplate::new(|path| {
            let mut seq = nn::seq();
            let mut last_dim = None;
            if !hidden_layers.is_empty(){
                let mut ld = hidden_layers[0];

                last_dim = Some(ld);
                seq = seq.add(nn::linear(path / "INPUT", input_shape, ld, Default::default()));

                for i in 1..hidden_layers.len(){
                    let ld_new = hidden_layers[i];
                    seq = seq.add(nn::linear(path / &format!("h_{:}", i+1), ld, ld_new, Default::default()))
                        .add_fn(|xs| xs.tanh());

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
            let device = path.device();
            {move |xs: &Tensor|{
                if seq.is_empty(){
                    TensorA2C{critic: xs.apply(&critic), actor: xs.apply(&actor)}
                } else {
                    let xs = xs.to_device(device).apply(&seq);
                    TensorA2C{critic: xs.apply(&critic), actor: xs.apply(&actor)}
                }
            }}
        });

        let net = network_pattern.get_net_closure();
        let optimiser = Adam::from(agent_configuration.policy_params.optimizer_params).build(&var_store, agent_configuration.policy_params.learning_rate)?;
        let net = A2CNet::new(var_store, net, );
        //Ok(ActorCriticPolicy::new(net, optimiser, is2t, ContractActionWayToTensor::default(), TrainConfig{ gamma: 0.999 }))
        Ok(ActorCriticPolicy::new(net, optimiser, is2t, TrainConfig {gamma: agent_configuration.policy_params.gamma }))
        /*


         */
    }

    fn create_dyn_agent_l3<
        InfoSet: InformationSet<ContractDP> + Debug + CtxTryIntoTensor<IS2T> + PresentPossibleActions<ContractDP>
         + for<'a> Renew<ContractDP, (&'a Side, &'a ContractGameDescription)> + Clone + 'static,
        IS2T: ConversionToTensor + 'static,
        >
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, info_set: InfoSet, is2t: IS2T)
    -> Result<(Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvironmentEndpoint<ContractDP> ), BrydzModelError>{
        let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();
        match agent_configuration.policy_params.select_policy{

            PolicyTypeSelect::Q => {
                let policy = self.create_agent_q_policy(&agent_configuration, var_store, is2t)?;
                Ok((Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy))), env_endpoint))
            }
            PolicyTypeSelect::A2C => {
                let policy = self.create_agent_a2c_policy(&agent_configuration, var_store, is2t)?;
                Ok((Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy))), env_endpoint))
            }
        }
    }

    fn create_dyn_agent_l2<
        InfoSet: InformationSet<ContractDP> + Debug + PresentPossibleActions<ContractDP>
        + CtxTryIntoTensor<ContractInfoSetConvertSparse> + CtxTryIntoTensor<ContractInfoSetConvertSparseHistoric>
        + CtxTryIntoTensor<ContractInfoSetConvert420>
        + for<'a> Renew<ContractDP, (&'a Side, &'a ContractGameDescription)> + Clone + 'static,
    >
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, info_set: InfoSet)
     -> Result<(Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvironmentEndpoint<ContractDP> ), BrydzModelError>{
        match agent_configuration.info_set_conversion_type{
            InfoSetWayToTensorSelect::_420 => self.create_dyn_agent_l3(agent_configuration, var_store, info_set, ContractInfoSetConvert420::default()),
            InfoSetWayToTensorSelect::Sparse => self.create_dyn_agent_l3(agent_configuration, var_store, info_set, ContractInfoSetConvertSparse::default()),
            InfoSetWayToTensorSelect::SparseHistoric => self.create_dyn_agent_l3(agent_configuration, var_store, info_set, ContractInfoSetConvertSparseHistoric::default()),
        }
    }

    fn create_dyn_agent_l1
    (&self, agent_configuration: &AgentConfiguration, var_store: VarStore, side: Side)
     -> Result<(Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvironmentEndpoint<ContractDP> ), BrydzModelError>{

        let description = ContractGameDescription::new(
            self.env.state().contract_data().contract_spec().clone(),
            self.initial_deal.distribution().clone(),
            self.initial_deal.cards().clone());

        match agent_configuration.info_set_type{
            InfoSetTypeSelect::Simple => {
                let info_set = ContractAgentInfoSetSimple::from((&side, &description));
                self.create_dyn_agent_l2(agent_configuration, var_store, info_set)
            }
            InfoSetTypeSelect::Assume => {
                let info_set = ContractAgentInfoSetAssuming::from((&side, &description));
                self.create_dyn_agent_l2(agent_configuration, var_store, info_set)
            }
            InfoSetTypeSelect::Complete => {
                let info_set = ContractAgentInfoSetAllKnowing::from((&side, &description));
                self.create_dyn_agent_l2(agent_configuration, var_store, info_set)
            }
        }
    }

/*
    fn create_dynamic_agent(&self, agent_configuration: &AgentConfiguration, var_store: VarStore, side: Side)
    -> Result<(Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>>, StdEnvironmentEndpoint<ContractDP>), AmfiRLError<ContractDP>>
    {
        let (env_endpoint, agent_endpoint ) = StdEnvironmentEndpoint::new_pair();

        /*
        let description = DescriptionDeckDeal{
            probabilities: self.initial_deal.distribution().clone(),
            cards: self.initial_deal.cards().clone(),
        };

         */

        let description = ContractGameDescription::new(
            self.env.state().contract_data().contract_spec().clone(),
            self.initial_deal.distribution().clone(),
            self.initial_deal.cards().clone());






        let agent: Arc<Mutex<dyn for<'a> RlSimpleLearningAgent<ContractDP, ContractInfoSetSeed<'a>>>> = match  agent_configuration.info_set_conversion_type {
            InfoSetWayToTensorSelect::_420 => {

                match agent_configuration.info_set_type{
                    InfoSetTypeSelect::Simple => {
                        let info_set = ContractAgentInfoSetSimple::from((&side, &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Assume => {
                        let info_set = ContractAgentInfoSetAssuming::from((&side, &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Complete => {
                        let info_set = ContractAgentInfoSetAllKnowing::from((&side, &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvert420::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                }


            },

            InfoSetWayToTensorSelect::Sparse => {
                match agent_configuration.info_set_type{
                    InfoSetTypeSelect::Simple => {
                        let info_set = ContractAgentInfoSetSimple::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Assume => {
                        let info_set = ContractAgentInfoSetAssuming::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Complete => {
                        let info_set = ContractAgentInfoSetAllKnowing::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparse::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                }
            },
            InfoSetWayToTensorSelect::SparseHistoric => {
                match agent_configuration.info_set_type{
                    InfoSetTypeSelect::Simple => {
                        let info_set = ContractAgentInfoSetSimple::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparseHistoric::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Assume => {
                        let info_set = ContractAgentInfoSetAssuming::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparseHistoric::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                    InfoSetTypeSelect::Complete => {
                        let info_set = ContractAgentInfoSetAllKnowing::from((&side,  &description));
                        let policy = self.create_agent_q_policy(agent_configuration, var_store, ContractInfoSetConvertSparseHistoric::default())?;
                        Arc::new(Mutex::new(TracingAgentGen::new(info_set, agent_endpoint, policy)))
                    }
                }
            },

        };
        Ok((agent, env_endpoint))


    }


 */

    pub fn with_agent(mut self, agent_configuration: &AgentConfiguration, place: AgentRole) -> Result<Self, BrydzModelError>{

        let side = match place{
            AgentRole::Declarer | AgentRole::TestDeclarer => self.initial_deal.parameters().declarer(),
            AgentRole::Whist | AgentRole::TestWhist => self.initial_deal.parameters().whist(),
            AgentRole::Offside | AgentRole::TestOffside => self.initial_deal.parameters().offside()
        };
        let var_store = match agent_configuration.var_load_path {
            None => VarStore::new(agent_configuration.device.into()),
            Some(ref s) => {
                let mut vs = VarStore::new(agent_configuration.device.into());
                vs.load(s)?;
                vs
            }
        };
        //let (agent, comm) = self.create_dynamic_agent(agent_configuration, var_store, side)?;
        let (agent, comm) = self.create_dyn_agent_l1(agent_configuration, var_store, side)?;


        match place{
            AgentRole::TestDeclarer | AgentRole::TestOffside | AgentRole::TestWhist => {
                agent.lock().map_err(|e|{
                    BrydzModelError::Mutex(format!("Locking agent in builder {e:}"))
                })?.set_exploration(false)
            },
            _ => {}
        }





        match place{
            AgentRole::Declarer => {}
            AgentRole::Whist => {}
            AgentRole::Offside => {}
            AgentRole::TestDeclarer => {}
            AgentRole::TestWhist => {}
            AgentRole::TestOffside => {}
        }


        match place{
            AgentRole::Declarer => {
                self.declarer = Some(agent);
                self.env.comms_mut().insert(side, comm);
            }
            AgentRole::Whist => {
                self.whist = Some(agent);
                self.env.comms_mut().insert(side, comm);
            }
            AgentRole::Offside => {
                self.offside = Some(agent);
                self.env.comms_mut().insert(side, comm);
            }
            AgentRole::TestDeclarer => {
                self.test_declarer = Some(agent);
                self.inactive_declarer_comm = Some(comm);
            }
            AgentRole::TestWhist => {
                self.test_whist = Some(agent);
                self.inactive_whist_comm = Some(comm);
            }
            AgentRole::TestOffside => {
                self.test_offside = Some(agent);
                self.inactive_offside_comm = Some(comm);
            }
        }
        Ok(self)
    }



    /*
    pub fn with_test_vectors(mut self, path: &PathBuf) -> Result<Self, AmfiRLError<ContractDP>>{
        let test_str = fs::read_to_string(path)
            .map_err(|e| AmfiRLError::IO(format!("Error opening file: {path:?}")))?;
        let set = ron::de::from_str(&test_str)
            .map_err(|e| AmfiRLError::IO(format!("Error reading tensors from file: {path:?} (error: {e:})")))?;
        self.test_vectors = set;
        Ok(self)
    }

     */

    pub fn with_test_games_from_file(mut self, file: impl AsRef<Path>) -> Result<Self, BrydzModelError>{
        let test_str = fs::read_to_string(&file)
            .map_err(|e| BrydzModelError::IO(format!("Failed reading file input {:?} as test vectors ({e:})", file.as_ref())))?;
        let set:  Vec<ContractGameDescription> = ron::de::from_str(&test_str)
            .map_err(|e| BrydzModelError::IO(format!("Failed converting input of file {:?} as test vectors ({e:})", file.as_ref())))?;

        self.test_vectors = set;
        Ok(self)
    }


    pub fn build(self) -> Result<DynamicBridgeModel, AmfiteatrRlError<ContractDP>>{


        Ok(DynamicBridgeModel{
            env: self.env,
            declarer: self.declarer.unwrap(),
            whist: self.whist.unwrap(),
            offside: self.offside.unwrap(),
            dummy: self.dummy,
            test_declarer: self.test_declarer.unwrap(),
            test_whist: self.test_whist.unwrap(),
            test_offside: self.test_offside.unwrap(),
            inactive_declarer_comm: self.inactive_declarer_comm.unwrap(),
            inactive_whist_comm: self.inactive_whist_comm.unwrap(),
            inactive_offside_comm: self.inactive_offside_comm.unwrap(),
            test_vectors: self.test_vectors,
            initial_deal: self.initial_deal,
        })
    }




}

/*
macro_rules! apply_info_set {
    (t: ty) => {
        let info_set = t::from((&side, self.initial_deal.cards(), description));
        Arc::new(Mutex::new(info_set, agent_endpoint, ))
    }
}



 */
