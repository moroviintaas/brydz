use amfiteatr_core::domain::DomainParameters;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::amfi::state::{ContractAction, ContractStateUpdate};

#[derive(Clone, Copy, Debug)]
pub struct ContractDP {

}

impl DomainParameters for ContractDP {
    type ActionType = ContractAction;
    type GameErrorType = BridgeCoreError;
    type UpdateType = ContractStateUpdate;
    type AgentId = Side;
    type UniversalReward = i32;
}