mod gen;
mod simulation;

use ron::Error;
use amfiteatr_rl::tch::TchError;
//use tensorflow::{SaveModelError, Status};
use brydz_core::error::BridgeCoreError;
use brydz_core::amfiteatr::spec::ContractDP;
pub use gen::*;
pub use simulation::*;
use amfiteatr_core::error::{AmfiteatrError, WorldError};
use amfiteatr_rl::error::AmfiteatrRlError;
use crate::error::BrydzModelError::Amfiteatr;

#[derive(Debug,  thiserror::Error)]
pub enum BrydzModelError {
    #[error("Custom error {0}")]
    Custom(String),
    #[error("Error in game generation: {0}")]
    Gen(GenError),
    #[error("Error in game setup: {0}")]
    Simulation(SimulationError),
    //#[error("Error during playing game: {0}")]
    //Game(BridgeCoreError),
    #[error("Error in sztorm framework: {0}")]
    Amfiteatr(AmfiteatrError<ContractDP>),
    #[error("Error in sztorm Reinforcement Learning framework: {0}")]
    AmfiteatrRL(AmfiteatrRlError<ContractDP>),
    //#[error("Tensorflow Error {0}")]
    //TensorflowStatus(Status),
    //#[error("SaveModel Error {0}")]
    //SaveModel(SaveModelError),
    #[error("LoadModel Error {0}")]
    Tch(TchError),
    #[error("Ron Error {0}")]
    Ron(ron::error::Error),
    #[error("IO Error {0}")]
    IO(String),
    #[error("Locking mutex: {0}")]
    Mutex(String),

}

impl From<BridgeCoreError> for BrydzModelError {
    fn from(value: BridgeCoreError) -> Self {
        Self::Amfiteatr(AmfiteatrError::Game(value))
    }
}


impl From<AmfiteatrError<ContractDP>> for BrydzModelError {
    fn from(value: AmfiteatrError<ContractDP>) -> Self {
        Self::Amfiteatr(value)
    }
}
impl From<AmfiteatrRlError<ContractDP>> for BrydzModelError {
    fn from(value: AmfiteatrRlError<ContractDP>) -> Self {
        Self::AmfiteatrRL(value)
    }
}

impl From<WorldError<ContractDP>> for BrydzModelError {
    fn from(value: WorldError<ContractDP>) -> Self {
        Amfiteatr(AmfiteatrError::World(value))
    }
}


impl From<ron::error::Error> for BrydzModelError {
    fn from(value: Error) -> Self {
        BrydzModelError::Ron(value)
    }
}
impl From<std::io::Error> for BrydzModelError {
    fn from(value: std::io::Error) -> Self {
        BrydzModelError::IO(format!("{}", value))
    }
}
/*
impl From<tensorflow::Status> for BrydzSimError{
    fn from(value: Status) -> Self {
        Self::TensorflowStatus(value)
    }
}

 */
impl From<TchError> for BrydzModelError {
    fn from(value: TchError) -> Self {
        Self::Tch(value)
    }
}