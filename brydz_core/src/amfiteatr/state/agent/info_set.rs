use crate::amfiteatr::re_export::scheme::Scheme;
use amfiteatr_core::agent::InformationSet;
use amfiteatr_core::error::{AmfiteatrError, ConvertError};
use amfiteatr_core::scheme::Renew;
use amfiteatr_rl::MaskingInformationSetAction;
use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::tensor_data::ContextEncodeTensor;
use crate::amfiteatr::spec::ContractDP;
use crate::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple, ContractInfoSetConvertDense1, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric, ContractInfoSetEncoding};
use crate::deal::ContractGameDescription;
use crate::player::side::Side;

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

impl MaskingInformationSetAction<ContractDP, ActionPlaceCardConvertion1D> for ContractInformationSet{
    fn try_build_mask(&self, ctx: &ActionPlaceCardConvertion1D) -> Result<Tensor, AmfiteatrError<ContractDP>> {

        match self{
            ContractInformationSet::Simple(s) => s.try_build_mask(ctx),
            ContractInformationSet::AllKnowing(s) => s.try_build_mask(ctx),
            ContractInformationSet::Assuming(s) => s.try_build_mask(ctx),
        }

    }
}

impl Renew<ContractDP, (&Side, &ContractGameDescription)> for ContractInformationSet{
    fn renew_from(&mut self, base: (&Side, &ContractGameDescription)) -> Result<(), AmfiteatrError<ContractDP>> {
        match self{
            ContractInformationSet::Simple(s) => s.renew_from(base),
            ContractInformationSet::Assuming(s) => s.renew_from(base),
            ContractInformationSet::AllKnowing(s) => s.renew_from(base),
        }
    }
}