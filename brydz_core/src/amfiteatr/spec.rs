use amfiteatr_core::scheme::Scheme;
use crate::error::BridgeCoreError;
use crate::player::side::Side;
use crate::amfiteatr::state::{ContractAction, ContractStateUpdate};

#[derive(Clone, Copy, Debug)]
pub struct ContractDP {

}

impl Scheme for ContractDP {
    type ActionType = ContractAction;
    type GameErrorType = BridgeCoreError;
    type UpdateType = ContractStateUpdate;
    type AgentId = Side;
    type UniversalReward = i32;
}