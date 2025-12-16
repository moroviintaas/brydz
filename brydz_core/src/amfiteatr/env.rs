use crate::player::side::{Side, SideMap, SIDES};
use crate::amfiteatr::state::{
    ContractAction,
    ContractState};
use log::warn;
use amfiteatr_core::{comm::BidirectionalEndpoint};
use amfiteatr_core::env::{
    BroadcastingEndpointEnvironment,
    CommunicatingEndpointEnvironment,
    SequentialGameState,
    GameStateWithPayoffs,
    EnvironmentWithAgents,
    ScoreEnvironment,
    StatefulEnvironment};
use amfiteatr_core::scheme::{AgentMessage, Scheme, EnvironmentMessage, Reward};
use amfiteatr_core::error::AmfiteatrError;
use crate::amfiteatr::spec::ContractDP;

pub struct ContractEnv<ST: SequentialGameState<ContractDP> + ContractState, C: BidirectionalEndpoint>{
    state: ST,
    comm: SideMap<C>,
    penalties: SideMap<<ContractDP as Scheme>::UniversalReward>,
    game_violator: Option<Side>,
}

impl<
    ST: SequentialGameState<ContractDP> + ContractState,
    C: BidirectionalEndpoint>
ContractEnv<ST, C>{
    pub fn new(state: ST, comm: SideMap<C>) -> Self{
        Self{
            state,
            comm,
            penalties: SideMap::new_symmetric(
                <ContractDP as Scheme>::UniversalReward::neutral()),
            game_violator: None,
        }
    }
    pub fn replace_state(&mut self, state: ST){
        self.state = state;
    }

    pub fn comms_mut(&mut self) -> &mut SideMap<C>{
        &mut self.comm
    }
}

impl<
    ST: SequentialGameState<ContractDP> + ContractState,
    C: BidirectionalEndpoint<
        OutwardType=EnvironmentMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
CommunicatingEndpointEnvironment<ContractDP> for ContractEnv< ST, C>{

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

impl<ST: SequentialGameState<ContractDP> + ContractState,
    C: BidirectionalEndpoint<
        OutwardType=EnvironmentMessage<ContractDP>,
        InwardType=AgentMessage<ContractDP>>>
BroadcastingEndpointEnvironment<ContractDP> for ContractEnv<ST, C>
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
    ST: SequentialGameState<ContractDP> + ContractState,
    C: BidirectionalEndpoint>
EnvironmentWithAgents<ContractDP> for ContractEnv<ST, C>{

    type PlayerIterator = [Side; 4];

    fn players(&self) -> Self::PlayerIterator {
        SIDES
    }
}

impl<
    ST: SequentialGameState<ContractDP> + ContractState + ContractState,
    C: BidirectionalEndpoint>
StatefulEnvironment<ContractDP> for ContractEnv<ST, C>
where ST: SequentialGameState<ContractDP> {
    type State = ST;
    //type Updates = <[(Side, ContractStateUpdate);4] as IntoIterator>::IntoIter;

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn process_action(&mut self, agent: &Side, action: &ContractAction)
        -> Result<<Self::State as SequentialGameState<ContractDP>>::Updates, AmfiteatrError<ContractDP>> {

        self.state.forward(*agent, *action).map_err(|e|AmfiteatrError::Game{source: e})
    }

    fn game_violator(&self) -> Option<&<ContractDP as Scheme>::AgentId> {
        self.game_violator.as_ref()
    }

    fn set_game_violator(&mut self, game_violator: Option<<ContractDP as Scheme>::AgentId>) {
        self.game_violator = game_violator;
    }
}


impl<
    ST: SequentialGameState<ContractDP>
        + ContractState + GameStateWithPayoffs<ContractDP> ,
    C: BidirectionalEndpoint>
ScoreEnvironment<ContractDP> for ContractEnv<ST, C>
where ST: SequentialGameState<ContractDP> {
    fn process_action_penalise_illegal(
        &mut self,
        agent: &<ContractDP as Scheme>::AgentId,
        action: &<ContractDP as Scheme>::ActionType,
        penalty_reward: <ContractDP as Scheme>::UniversalReward)

        -> Result<
            <<Self as StatefulEnvironment<ContractDP>>::State as SequentialGameState<ContractDP>>::Updates, AmfiteatrError<ContractDP>> {



        self.state.forward(*agent, *action).map_err(|e|{
            self.penalties[agent] += &penalty_reward;
            AmfiteatrError::Game{source: e}
        })


    }

    fn actual_state_score_of_player(&self, agent: &<ContractDP as Scheme>::AgentId) -> <ContractDP as Scheme>::UniversalReward {
        self.state.state_payoff_of_player(agent)
    }

    fn actual_penalty_score_of_player(&self, agent: &<ContractDP as Scheme>::AgentId) -> <ContractDP as Scheme>::UniversalReward {
        self.penalties[agent]
    }

    fn actual_score_of_player(&self, agent: &Side) -> <ContractDP as Scheme>::UniversalReward {
        self.state.state_payoff_of_player(agent)
    }

}


pub struct ContractProcessor{

}

