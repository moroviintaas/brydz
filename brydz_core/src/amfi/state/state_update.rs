use std::fmt::{Debug, Display, Formatter};
//use sztorm::state::StateUpdate;
use crate::player::side::Side;
use crate::amfi::state::ContractAction;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::manual_slice_size_calculation)]
#[cfg_attr(feature = "speedy", derive(speedy::Writable, speedy::Readable))]
pub struct ContractStateUpdate {
    agent: Side,
    action: ContractAction


}

impl ContractStateUpdate{
    pub fn new(side: Side, action: ContractAction) -> Self{
        Self{agent:side, action}
    }

    pub fn side(&self) -> &Side{
        &self.agent
    }
    pub fn action(&self) -> &ContractAction{
        &self.action
    }
    pub fn into_tuple(self) -> (Side, ContractAction){
        (self.agent, self.action)
    }
}


impl Display for ContractStateUpdate{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "agent {:#}; action: {:#}", &self.agent, &self.action),
            false => write!(f, "agent {}; action: {}", &self.agent, &self.action)
        }
    }
}


/*
impl StateUpdate for ContractStateUpdate{

}

 */

#[cfg(feature = "torch")]
mod tensor{
    use karty::cards::{Card2SymTrait, DECK_SIZE};
    use karty::symbol::CardSymbol;
    use crate::amfi::state::ContractAction;
    const MIN_ACTION_SIZE:usize = 2;

    impl From<&ContractAction> for [u8;MIN_ACTION_SIZE]{
        fn from(value: &ContractAction) -> Self {
            match value{
                ContractAction::ShowHand(_) => [0,0],
                ContractAction::PlaceCard(c) => [c.suit().usize_index() as u8 +1, c.figure().usize_index() as u8 + 1]
            }
        }
    }
    impl From<&ContractAction> for [f32;MIN_ACTION_SIZE]{
        fn from(value: &ContractAction) -> Self {
            match value{
                ContractAction::ShowHand(_) => [0.0,0.0],
                ContractAction::PlaceCard(c) => [c.suit().usize_index() as f32 +1.0, c.figure().usize_index() as f32 + 1.0]
            }
        }
    }

    impl From<&ContractAction> for amfiteatr_rl::tch::Tensor{
        fn from(value: &ContractAction) -> Self {
            amfiteatr_rl::tch::Tensor::from_slice(&Into::<[f32;MIN_ACTION_SIZE]>::into(value))
        }
    }

    impl ContractAction{

        pub fn sparse_representation(&self) -> [f32; DECK_SIZE+1]{
            let mut crd = [0.0; DECK_SIZE+1];

            match self{
                ContractAction::ShowHand(h) => {
                    for c in h.into_iter(){
                        crd[c.usize_index()] = 1.0;
                    }
                    crd[DECK_SIZE] = 0.0;
                }
                ContractAction::PlaceCard(c) => {
                    crd[c.usize_index()] = 1.0;
                    crd[DECK_SIZE] = 1.0;
                }
            }

            crd
        }
    }

}