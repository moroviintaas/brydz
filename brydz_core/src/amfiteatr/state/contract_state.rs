use karty::hand::CardSet;
use crate::contract::{Contract, ContractMechanics};
use crate::player::side::Side;

pub trait ContractState{

    fn dummy_side(&self) -> Side{
        self.contract_data().dummy()
    }
    fn current_side(&self) -> Side{
        self.contract_data().current_side()
    }
    fn is_turn_of_dummy(&self) -> bool{
        self.dummy_side() == self.current_side()
    }
    fn dummy_hand(&self) -> Option<&CardSet>;
    fn contract_data(&self) -> &Contract;

    fn declarer_side(&self) -> Side{
        self.contract_data().declarer()
    }

    fn whist_side(&self) -> Side{
        self.contract_data().declarer().next()
    }

    fn offside_side(&self) -> Side{
        self.contract_data().declarer().next_i(3)
    }
}

impl<T: ContractState> ContractState for Box<T>{


    fn dummy_hand(&self) -> Option<&CardSet> {
        self.as_ref().dummy_hand()
    }

    fn contract_data(&self) -> &Contract {
        self.as_ref().contract_data()
    }
}