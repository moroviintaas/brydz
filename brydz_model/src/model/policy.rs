use amfiteatr_core::agent::Policy;
use amfiteatr_core::error::AmfiteatrError;
use amfiteatr_core::scheme::Scheme;
use amfiteatr_rl::policy::{PolicyDiscretePPO, PolicyMaskingDiscretePPO};
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractInfoSetEncoding, ContractInformationSet};

pub enum ContractPolicy{
    Ppo(PolicyDiscretePPO<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>),
    MaskedPpo(PolicyMaskingDiscretePPO<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>),

}

impl Policy<ContractDP> for ContractPolicy {
    type InfoSetType = ContractInformationSet;

    fn select_action(&self, state: &Self::InfoSetType) -> Result<<ContractDP as Scheme>::ActionType, AmfiteatrError<ContractDP>> {
        match self{
            ContractPolicy::Ppo(ppos) => ppos.select_action(state),
            ContractPolicy::MaskedPpo(ppos) => ppos.select_action(state),
        }
    }
}