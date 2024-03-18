mod ac_generic;
mod team;
mod q_generic;
mod options;
mod main_session;
mod traits;
mod dynamic;

pub use ac_generic::*;
pub use team::*;
pub use q_generic::*;
pub use options::*;
pub use main_session::*;
pub use traits::*;
pub use dynamic::*;


use clap::Subcommand;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ContractAgentInfoSetAllKnowing, ContractAgentInfoSetSimple, ContractAgentInfoSetAssuming, ContractInfoSetConvert420, ContractInfoSetConvertSparse};
use amfiteatr_core::agent::EvaluatedInformationSet;


use amfiteatr_rl::tensor_data::{FloatTensorReward, ConversionToTensor};
use crate::error::BrydzSimError;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect, TrainOptions};


#[derive(Subcommand)]
pub enum TrainType{
    Simple(TrainOptions)
}


fn create_and_run_learning_a2c_session<
    InfoSet: ContractInfoSetForLearning<W2T> + Clone,
    W2T: ConversionToTensor + Default
>(options: &TrainOptions) -> Result<(), BrydzSimError>
where <InfoSet as EvaluatedInformationSet<ContractDP>>::RewardType: FloatTensorReward{
    let mut session = t_session_a2c_symmetric::<InfoSet, W2T>(options)?;
    session.load_network_params(options)?;
    session.train_all_at_once(
        options.epochs as usize,
        options.games as usize,
        options.tests_set_size as usize,
        None,
        &Default::default(), )?;
    session.save_network_params(options)?;
    Ok(())
}



fn session_a2c_420_repr(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.info_set_select{
        InfoSetTypeSelect::Simple => create_and_run_learning_a2c_session::<ContractAgentInfoSetSimple, ContractInfoSetConvert420>(options),
        InfoSetTypeSelect::Assume => create_and_run_learning_a2c_session::<ContractAgentInfoSetAssuming, ContractInfoSetConvert420>(options),
        InfoSetTypeSelect::Complete => create_and_run_learning_a2c_session::<ContractAgentInfoSetAllKnowing, ContractInfoSetConvert420>(options),
    }

}

fn session_a2c_sparse_repr(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.info_set_select{
        InfoSetTypeSelect::Simple => create_and_run_learning_a2c_session::<ContractAgentInfoSetSimple, ContractInfoSetConvertSparse>(options),
        InfoSetTypeSelect::Assume => create_and_run_learning_a2c_session::<ContractAgentInfoSetAssuming, ContractInfoSetConvertSparse>(options),
        InfoSetTypeSelect::Complete => create_and_run_learning_a2c_session::<ContractAgentInfoSetAllKnowing, ContractInfoSetConvertSparse>(options),
    }

}

fn session_a2c(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.w2t{
        InfoSetWayToTensorSelect::_420 => session_a2c_420_repr(options),
        InfoSetWayToTensorSelect::Sparse => session_a2c_sparse_repr(options)
    }
}

fn create_and_run_learning_q_session<
    InfoSet: ContractInfoSetForLearning<W2T> + Clone,
    W2T: ConversionToTensor + Default
>(options: &TrainOptions) -> Result<(), BrydzSimError>{
    let mut session = t_session_q_symmetric::<InfoSet, W2T>(options)?;
    session.load_network_params(options)?;
    session.train_all_at_once(
        options.epochs as usize,
        options.games as usize,
        options.tests_set_size as usize,
        None,
        &Default::default(),)?;
    session.save_network_params(options)?;
    Ok(())
}

fn session_q_420_repr(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.info_set_select{
        InfoSetTypeSelect::Simple => create_and_run_learning_q_session::<ContractAgentInfoSetSimple, ContractInfoSetConvert420>(options),
        InfoSetTypeSelect::Assume => create_and_run_learning_q_session::<ContractAgentInfoSetAssuming, ContractInfoSetConvert420>(options),
        InfoSetTypeSelect::Complete => create_and_run_learning_q_session::<ContractAgentInfoSetAllKnowing, ContractInfoSetConvert420>(options),
    }

}

fn session_q_sparse_repr(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.info_set_select{
        InfoSetTypeSelect::Simple => create_and_run_learning_q_session::<ContractAgentInfoSetSimple, ContractInfoSetConvertSparse>(options),
        InfoSetTypeSelect::Assume => create_and_run_learning_q_session::<ContractAgentInfoSetAssuming, ContractInfoSetConvertSparse>(options),
        InfoSetTypeSelect::Complete => create_and_run_learning_q_session::<ContractAgentInfoSetAllKnowing, ContractInfoSetConvertSparse>(options),
    }

}

fn session_q(options: &TrainOptions) -> Result<(), BrydzSimError>{
    match options.w2t{
        InfoSetWayToTensorSelect::_420 => session_q_420_repr(options),
        InfoSetWayToTensorSelect::Sparse => session_q_sparse_repr(options)
    }
}

pub fn build_and_run_train_session(agent_type: &AgentType) -> Result<(), BrydzSimError>{
    match agent_type{
        /*
        AgentType::A2C(options) => {
            let mut session = t_session_a2c_symmetric::<ContractAgentInfoSetSimple, ContractInfoSetConvert420Normalised>(options)?;
            session.load_network_params(options)?;
            session.train_all_at_once(options.epochs as usize, options.games as usize, options.tests_set_size as usize, None, &Default::default())?;
            session.save_network_params(options)?;
            //train_session_a2c(options)
        }
        AgentType::Q(options) => {
            let mut session = t_session_q_symmetric::<ContractAgentInfoSetSimple, ContractInfoSetConvert420Normalised>(options)?;
            session.load_network_params(options)?;
            session.train_all_at_once(options.epochs as usize, options.games as usize, options.tests_set_size as usize, None, &Default::default())?;
            session.save_network_params(options)?;
            //train_session_q(options)
        }

         */
        AgentType::A2C(options) => session_a2c(options)?,
        AgentType::Q(options) => session_q(options)?,
        AgentType::Dynamic(options) => run_dynamic_model(options)?

    }
    Ok(())
}