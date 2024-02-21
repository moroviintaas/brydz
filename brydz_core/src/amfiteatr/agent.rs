use amfiteatr_core::agent::TracingAgentGen;
use crate::amfiteatr::spec::ContractDP;

pub type TracingContractAgent<C, P> = TracingAgentGen<ContractDP, P, C>;
