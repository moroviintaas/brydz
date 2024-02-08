mod gen;
mod simulation;

use ron::Error;
use amfiteatr_rl::tch::TchError;
//use tensorflow::{SaveModelError, Status};
use brydz_core::error::BridgeCoreError;
use brydz_core::amfi::spec::ContractDP;
pub use gen::*;
pub use simulation::*;
use amfiteatr_core::error::{AmfiError, WorldError};
use amfiteatr_rl::error::AmfiRLError;
use crate::error::BrydzSimError::Amfi;

#[derive(Debug,  thiserror::Error)]
pub enum BrydzSimError{
    #[error("Custom error {0}")]
    Custom(String),
    #[error("Error in game generation: {0}")]
    Gen(GenError),
    #[error("Error in game setup: {0}")]
    Simulation(SimulationError),
    //#[error("Error during playing game: {0}")]
    //Game(BridgeCoreError),
    #[error("Error in sztorm framework: {0}")]
    Amfi(AmfiError<ContractDP>),
    #[error("Error in sztorm Reinforcement Learning framework: {0}")]
    SztormRL(AmfiRLError<ContractDP>),
    //#[error("Tensorflow Error {0}")]
    //TensorflowStatus(Status),
    //#[error("SaveModel Error {0}")]
    //SaveModel(SaveModelError),
    #[error("LoadModel Error {0}")]
    Tch(TchError),
    #[error("Ron Error {0}")]
    Ron(ron::error::Error),
    #[error("IO Error {0}")]
    IO(std::io::Error),

}

impl From<BridgeCoreError> for BrydzSimError{
    fn from(value: BridgeCoreError) -> Self {
        Self::Amfi(AmfiError::Game(value))
    }
}


impl From<AmfiError<ContractDP>> for BrydzSimError{
    fn from(value: AmfiError<ContractDP>) -> Self {
        Self::Amfi(value)
    }
}
impl From<AmfiRLError<ContractDP>> for BrydzSimError{
    fn from(value: AmfiRLError<ContractDP>) -> Self {
        Self::SztormRL(value)
    }
}

impl From<WorldError<ContractDP>> for BrydzSimError{
    fn from(value: WorldError<ContractDP>) -> Self {
        Amfi(AmfiError::World(value))
    }
}


impl From<ron::error::Error> for BrydzSimError{
    fn from(value: Error) -> Self {
        BrydzSimError::Ron(value)
    }
}
impl From<std::io::Error> for BrydzSimError{
    fn from(value: std::io::Error) -> Self {
        BrydzSimError::IO(value)
    }
}
/*
impl From<tensorflow::Status> for BrydzSimError{
    fn from(value: Status) -> Self {
        Self::TensorflowStatus(value)
    }
}

 */
impl From<TchError> for BrydzSimError{
    fn from(value: TchError) -> Self {
        Self::Tch(value)
    }
}