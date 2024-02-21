use amfiteatr_core::comm::StdEndpoint;
use amfiteatr_core::error::CommunicationError;
use amfiteatr_core::domain::{AgentMessage, EnvironmentMessage};
use crate::amfiteatr::spec::ContractDP;

pub type ContractAgentSyncComm = StdEndpoint<AgentMessage<ContractDP>, EnvironmentMessage<ContractDP>, CommunicationError<ContractDP>>;
pub type ContractEnvSyncComm = StdEndpoint<EnvironmentMessage<ContractDP>, AgentMessage<ContractDP>, CommunicationError<ContractDP>>;