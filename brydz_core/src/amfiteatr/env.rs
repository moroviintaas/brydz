use crate::player::side::{Side, SideMap, SIDES};
use crate::amfiteatr::state::{
    ContractAction,
    ContractState};
use log::warn;
use amfiteatr_core::{comm::BidirectionalEndpoint};
use amfiteatr_core::env::{
    BroadcastingEndpointEnvironment,
    CommunicatingEndpointEnvironment,
    EnvironmentStateSequential,
    EnvironmentStateUniScore,
    EnvironmentWithAgents,
    ScoreEnvironment,
    StatefulEnvironment};
use amfiteatr_core::domain::{AgentMessage, DomainParameters, EnvironmentMessage, Reward};
use amfiteatr_core::error::AmfiteatrError;
use crate::amfiteatr::spec::ContractDP;

pub struct ContractEnv<S: EnvironmentStateSequential<ContractDP> + ContractState, C: BidirectionalEndpoint>{
    state: S,
    comm: SideMap<C>,
    penalties: SideMap<<ContractDP as DomainParameters>::UniversalReward>
}

impl<
    S: EnvironmentStateSequential<ContractDP> + ContractState,
    C: BidirectionalEndpoint>
ContractEnv<S, C>{
    pub fn new(state: S, comm: SideMap<C>) -> Self{
        Self{
            state,
            comm,
            penalties: SideMap::new_symmetric(
                <ContractDP as DomainParameters>::UniversalReward::neutral())
        }
    }
    pub fn replace_state(&mut self, state: S){
        self.state = state;
    }

    pub fn comms_mut(&mut self) -> &mut SideMap<C>{
        &mut self.comm
    }
}

impl<
    S: EnvironmentStateSequential<ContractDP> + ContractState,
    C: BidirectionalEndpoint<
        OutwardType=EnvironmentMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
CommunicatingEndpointEnvironment<ContractDP> for ContractEnv< S, C>{

    type CommunicationError = C::Error;
    //type AgentId = Side;

    fn send_to(
        &mut self,
        agent_id: &Side,
        message: EnvironmentMessage<ContractDP>)
        -> Result<(), Self::CommunicationError> {

        self.comm[agent_id].send(message)
    }

    fn blocking_receive_from(&mut self, agent_id: &Side) -> Result<AgentMessage<ContractDP>, Self::CommunicationError> {
        self.comm[agent_id].receive_blocking()
    }

    fn nonblocking_receive_from(&mut self, agent_id: &Side) -> Result<Option<AgentMessage<ContractDP>>, Self::CommunicationError> {
        self.comm[agent_id].receive_non_blocking()
    }
}

impl<S: EnvironmentStateSequential<ContractDP> + ContractState,
    C: BidirectionalEndpoint<
        OutwardType=EnvironmentMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
BroadcastingEndpointEnvironment<ContractDP> for ContractEnv<S, C>
where <C as BidirectionalEndpoint>::OutwardType: Clone{

    fn send_to_all(&mut self, message: EnvironmentMessage<ContractDP>) -> Result<(), Self::CommunicationError> {
        for s in SIDES{
            match self.comm[&s].send(message.clone()){
                Ok(_) => {},
                Err(_e) => warn!("Failed sending to {s:}")
            }
        }
        Ok(())
    }
}

impl<
    S: EnvironmentStateSequential<ContractDP> + ContractState,
    C: BidirectionalEndpoint>
EnvironmentWithAgents<ContractDP> for ContractEnv<S, C>{

    type PlayerIterator = [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        SIDES
    }
}

impl<
    S: EnvironmentStateSequential<ContractDP> + ContractState + ContractState,
    C: BidirectionalEndpoint>
StatefulEnvironment<ContractDP> for ContractEnv<S, C>
where S: EnvironmentStateSequential<ContractDP> {
    type State = S;
    //type Updates = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &Side, action: &ContractAction)
        -> Result<<Self::State as EnvironmentStateSequential<ContractDP>>::Updates, AmfiteatrError<ContractDP>> {

        self.state.forward(*agent, *action).map_err(|e|AmfiteatrError::Game{source: e})
    }
}


impl<
    S: EnvironmentStateSequential<ContractDP>
        + ContractState + EnvironmentStateUniScore<ContractDP> ,
    C: BidirectionalEndpoint>
ScoreEnvironment<ContractDP> for ContractEnv<S, C>
where S: EnvironmentStateSequential<ContractDP> {
    fn process_action_penalise_illegal(
        &mut self,
        agent: &<ContractDP as DomainParameters>::AgentId,
        action: &<ContractDP as DomainParameters>::ActionType,
        penalty_reward: <ContractDP as DomainParameters>::UniversalReward)

        -> Result<
            <<Self as StatefulEnvironment<ContractDP>>::State as EnvironmentStateSequential<ContractDP>>::Updates, AmfiteatrError<ContractDP>> {



        self.state.forward(*agent, *action).map_err(|e|{
            self.penalties[agent] += &penalty_reward;
            AmfiteatrError::Game{source: e}
        })


    }

    fn actual_state_score_of_player(&self, agent: &<ContractDP as DomainParameters>::AgentId) -> <ContractDP as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

    fn actual_penalty_score_of_player(&self, agent: &<ContractDP as DomainParameters>::AgentId) -> <ContractDP as DomainParameters>::UniversalReward {
        self.penalties[agent]
    }

    fn actual_score_of_player(&self, agent: &Side) -> <ContractDP as DomainParameters>::UniversalReward {
        self.state.state_score_of_player(agent)
    }

}


pub struct ContractProcessor{

}

