use std::fmt::Debug;
use brydz_core::contract::ContractParameters;
use brydz_core::deal::DescriptionDeckDeal;
use brydz_core::player::side::Side;
use brydz_core::amfiteatr::comm::{ContractAgentSyncComm};

use brydz_core::amfiteatr::spec::ContractDP;

use amfiteatr_core::agent::{AgentGen, TracingAgentGen, AutomaticAgent, Policy, PolicyAgent, PresentPossibleActions, StatefulAgent, InformationSet};

use amfiteatr_rl::policy::LearningNetworkPolicy;
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, TensorEncoding};

pub trait ContractInfoSetForLearning<ISW: TensorEncoding>:
ContextEncodeTensor<ISW>
+ for<'a> From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>
+ InformationSet<ContractDP>
+ PresentPossibleActions<ContractDP>
+ Debug {}

impl<ISW: TensorEncoding, T: ContextEncodeTensor<ISW>
+ for<'a> From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>
+ InformationSet<ContractDP>
+ PresentPossibleActions<ContractDP>
+ Debug > ContractInfoSetForLearning<ISW> for T{}

pub trait SessionAgentTraitDyn<
    ISW: TensorEncoding,
    P: Policy<ContractDP>
> where <P as Policy<ContractDP>>::InfoSetType: ContractInfoSetForLearning<ISW>
 + for<'a> From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>{}

pub trait SessionAgentTrait<
    ISW: TensorEncoding,
    P: Policy<ContractDP>
> where <P as Policy<ContractDP>>::InfoSetType: ContractInfoSetForLearning<ISW>
 + for<'a> From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>{

    fn create_for_session(
        side: Side,
        contract_params: &ContractParameters,
        deal_description: & DescriptionDeckDeal,
        comm: ContractAgentSyncComm,
        policy: P
    ) -> Self;
}

impl<
    ISW: TensorEncoding,
    P: Policy<ContractDP>
> SessionAgentTrait<ISW, P> for TracingAgentGen<ContractDP, P, ContractAgentSyncComm>
where for<'a> <P as Policy<ContractDP>>::InfoSetType: From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>
+ InformationSet<ContractDP> + ContextEncodeTensor<ISW> + PresentPossibleActions<ContractDP>
{
    fn create_for_session(side: Side, contract_params: &ContractParameters, deal_description: &DescriptionDeckDeal, comm: ContractAgentSyncComm, policy: P) -> Self {
        type IS<P> = <P as Policy<ContractDP>>::InfoSetType;
        TracingAgentGen::new(
            <IS<P>>::from((&side, contract_params, deal_description)),
            comm, policy)
    }
}

impl<
    ISW: TensorEncoding,
    P: Policy<ContractDP>
> SessionAgentTrait<ISW, P> for AgentGen<ContractDP, P, ContractAgentSyncComm>
where for<'a> <P as Policy<ContractDP>>::InfoSetType:
    From<(&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>
    + PresentPossibleActions<ContractDP>
    + InformationSet<ContractDP> + ContextEncodeTensor<ISW>
{
    fn create_for_session(side: Side, contract_params: &ContractParameters, deal_description: &DescriptionDeckDeal, comm: ContractAgentSyncComm, policy: P) -> Self {
        type IS<P> = <P as Policy<ContractDP>>::InfoSetType;
        AgentGen::new(
            <IS<P>>::from((&side, contract_params, deal_description)),
            comm, policy)
    }
}

/*
impl <
    ISW: WayToTensor,
    P: Policy<ContractDP>
> SessionAgent<ISW, P>
where <P as Policy<ContractDP>>::StateType: ContractInfoSetForLearning<ISW>
 + for<'a> ConstructedState<ContractDP, (&'a Side, &'a ContractParameters, &'a DescriptionDeckDeal)>{

}

 */


pub trait ContractLearningAgent: AutomaticAgent<ContractDP>  + PolicyAgent<ContractDP>
where <Self as PolicyAgent<ContractDP>>::Policy: LearningNetworkPolicy<ContractDP>,
<Self as StatefulAgent<ContractDP>>::InfoSetType: InformationSet<ContractDP>{}

impl <T: AutomaticAgent<ContractDP>  + PolicyAgent<ContractDP>>
ContractLearningAgent for T
where <T as PolicyAgent<ContractDP>>::Policy: LearningNetworkPolicy<ContractDP>,
<T as StatefulAgent<ContractDP>>::InfoSetType: InformationSet<ContractDP>
{}


/*
impl<
    P: LearningNetworkPolicy<ContractDP>,
    Comm: > ContractLearningAgent for AgentGenT<ContractDP, P, Comm>
where Comm: CommEndpoint<
        OutwardType = AgentMessage<ContractDP>,
        InwardType = EnvMessage<ContractDP>,
        Error=CommError<ContractDP>>,
    <P as Policy<ContractDP>>::StateType: ScoringInformationSet<ContractDP> + Clone

{}

 */

