use amfiteatr_core::agent::{AgentStepView, AgentTrajectory, Policy};
use amfiteatr_core::error::AmfiteatrError;
use amfiteatr_core::scheme::Scheme;
use amfiteatr_rl::error::AmfiteatrRlError;
use amfiteatr_rl::policy::{LearnSummary, LearningNetworkPolicy, LearningNetworkPolicyGeneric, PolicyDiscreteA2C, PolicyDiscretePPO, PolicyMaskingDiscreteA2C, PolicyMaskingDiscretePPO};
use amfiteatr_rl::tch::Tensor;
use brydz_core::amfiteatr::spec::ContractDP;
use brydz_core::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractInfoSetEncoding, ContractInformationSet};

pub enum ContractPolicy{
    Ppo(PolicyDiscretePPO<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>),
    MaskedPpo(PolicyMaskingDiscretePPO<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>),
    A2C(PolicyDiscreteA2C<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>),
    MaskedA2C(PolicyMaskingDiscreteA2C<ContractDP, ContractInformationSet, ContractInfoSetEncoding, ActionPlaceCardConvertion1D>)
}

impl Policy<ContractDP> for ContractPolicy {
    type InfoSetType = ContractInformationSet;

    fn select_action(&self, state: &Self::InfoSetType) -> Result<<ContractDP as Scheme>::ActionType, AmfiteatrError<ContractDP>> {
        if state.is_dummy(){
            return Ok(<ContractDP as Scheme>::ActionType::ShowHand(*state.show_hand()))
        }
        match self{
            ContractPolicy::Ppo(ppos) => ppos.select_action(state),
            ContractPolicy::MaskedPpo(ppos) => ppos.select_action(state),
            ContractPolicy::A2C(a2c) => a2c.select_action(state),
            ContractPolicy::MaskedA2C(a2c) => a2c.select_action(state),
        }
    }
}

impl LearningNetworkPolicyGeneric<ContractDP> for ContractPolicy{
    type Summary = LearnSummary;

    fn switch_explore(&mut self, enabled: bool) {
        match self{
            ContractPolicy::Ppo(ppos) => ppos.switch_explore(enabled),
            ContractPolicy::MaskedPpo(policy) => policy.switch_explore(enabled),
            ContractPolicy::A2C(policy) => policy.switch_explore(enabled),
            ContractPolicy::MaskedA2C(policy) => policy.switch_explore(enabled),
        }
    }

    fn train_generic<R: Fn(&AgentStepView<ContractDP, <Self as Policy<ContractDP>>::InfoSetType>) -> Tensor>(&mut self, trajectories: &[AgentTrajectory<ContractDP, <Self as Policy<ContractDP>>::InfoSetType>], reward_f: R) -> Result<Self::Summary, AmfiteatrRlError<ContractDP>> {
        match self{
            ContractPolicy::Ppo(policy) => policy.train_generic(trajectories, reward_f),
            ContractPolicy::MaskedPpo(policy) => policy.train_generic(trajectories, reward_f),
            ContractPolicy::A2C(policy) => policy.train_generic(trajectories, reward_f),
            ContractPolicy::MaskedA2C(policy) => policy.train_generic(trajectories, reward_f),
        }
    }
}