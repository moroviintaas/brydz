use crate::amfiteatr::re_export::scheme::Scheme;
use amfiteatr_core::agent::InformationSet;
use amfiteatr_core::error::ConvertError;
use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::tensor_data::ContextEncodeTensor;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple, ContractInfoSetConvertDense1, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric, ContractInfoSetEncoding};

#[derive(Clone, Debug)]
pub enum ContractInformationSet{
    Simple(ContractAgentInfoSetSimple),
    AllKnowing(ContractAgentInfoSetAllKnowing),
    Assuming(ContractAgentInfoSetAssuming)
}

impl InformationSet<ContractDP> for ContractInformationSet{
    fn agent_id(&self) -> &<ContractDP as Scheme>::AgentId {
        match self{
            ContractInformationSet::Simple(a) => a.agent_id(),
            ContractInformationSet::AllKnowing(a) => a.agent_id(),
            ContractInformationSet::Assuming(a) => a.agent_id(),
        }
    }

    fn is_action_valid(&self, action: &<ContractDP as Scheme>::ActionType) -> bool {
        match self{
            ContractInformationSet::Simple(a) => a.is_action_valid(action),
            ContractInformationSet::AllKnowing(a) => a.is_action_valid(action),
            ContractInformationSet::Assuming(a) => a.is_action_valid(action),
        }
    }

    fn update(&mut self, update: <ContractDP as Scheme>::UpdateType) -> Result<(), <ContractDP as Scheme>::GameErrorType> {
        match self{
            ContractInformationSet::Simple(a) => a.update(update),
            ContractInformationSet::AllKnowing(a) => a.update(update),
            ContractInformationSet::Assuming(a) => a.update(update),
        }
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertDense1> for ContractInformationSet{
    fn try_to_tensor(&self, encoding: &ContractInfoSetConvertDense1) -> Result<Tensor, ConvertError> {
        match self{
            ContractInformationSet::Simple(a) => a.try_to_tensor(encoding),
            ContractInformationSet::AllKnowing(a) => a.try_to_tensor(encoding),
            ContractInformationSet::Assuming(a) => a.try_to_tensor(encoding),
        }
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertSparse> for ContractInformationSet{
    fn try_to_tensor(&self, encoding: &ContractInfoSetConvertSparse) -> Result<Tensor, ConvertError> {
        match self{
            ContractInformationSet::Simple(a) => a.try_to_tensor(encoding),
            ContractInformationSet::AllKnowing(a) => a.try_to_tensor(encoding),
            ContractInformationSet::Assuming(a) => a.try_to_tensor(encoding),
        }
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertSparseHistoric> for ContractInformationSet{
    fn try_to_tensor(&self, encoding: &ContractInfoSetConvertSparseHistoric) -> Result<Tensor, ConvertError> {
        match self{
            ContractInformationSet::Simple(a) => a.try_to_tensor(encoding),
            ContractInformationSet::AllKnowing(a) => a.try_to_tensor(encoding),
            ContractInformationSet::Assuming(a) => a.try_to_tensor(encoding),
        }
    }
}

impl ContextEncodeTensor<ContractInfoSetEncoding> for ContractInformationSet{
    fn try_to_tensor(&self, encoding: &ContractInfoSetEncoding) -> Result<Tensor, ConvertError> {
        match encoding{
            ContractInfoSetEncoding::Dense1(c) => self.try_to_tensor(c),
            ContractInfoSetEncoding::Sparse(c) => self.try_to_tensor(c),
            ContractInfoSetEncoding::SparseHistoric(c) => self.try_to_tensor(c),
        }
    }
}