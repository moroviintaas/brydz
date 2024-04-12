use std::cmp::Ordering;
use karty::cards::{ DECK_SIZE, STANDARD_DECK_CDHS};
use karty::set::{CardSet};
use karty::symbol::CardSymbol;
use crate::cards::trump::TrumpGen;
use crate::contract::ContractMechanics;
use crate::amfiteatr::state::{BuildStateHistoryTensor, ContractAgentInfoSetSimple};

impl BuildStateHistoryTensor for ContractAgentInfoSetSimple{
    fn contract_params(&self) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.0; DECK_SIZE+1];
        result[0] = self.contract.contract_spec().bid().number() as f32;
        match self.contract.contract_spec().bid().trump(){
            TrumpGen::Colored(c) => {
                result[1] = 1.0;
                result[2] = c.usize_index() as f32;
            }
            TrumpGen::NoTrump => {
                result[1] = 0.0;
                result[2] = 0.0;
            }
        };


        result
    }

    fn prediction(&self, _relative_side: u8) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.25; DECK_SIZE+1];
        result[DECK_SIZE] = 0.0;
        result

    }

    fn actual_cards(&self) -> [f32; DECK_SIZE + 1] {
        let mut cards = [0.0;DECK_SIZE+1];
        /*for suit in SUITS{
            for figure in FIGURES{

            }
        }*/
        for c in STANDARD_DECK_CDHS{
            if self.hand.contains(&c){
                cards[c.usize_index()] = 1.0;
            } else {
                cards[c.usize_index()] = 0.0;
            }


        }
        cards[DECK_SIZE] = 1.0;
        cards
    }

    fn actual_dummy_cards(&self) -> [f32; DECK_SIZE + 1] {
        match self.dummy_hand{
            None => [0.0; DECK_SIZE+1],
            Some(dh) => {
                let mut result = [0.0; DECK_SIZE+1];
                for card in STANDARD_DECK_CDHS{
                    if dh.contains(&card){
                        result[card.usize_index()] = 1.0;
                    }
                }
                result[DECK_SIZE] = 1.0;
                result
            }
        }
    }

    fn trick_cards(&self, trick_number: usize, relative_side: u8) -> [f32; DECK_SIZE + 1] {
        match self.contract.completed_tricks().len().cmp(&trick_number){
            Ordering::Less => {
                [0.0;DECK_SIZE+1]

            }
            Ordering::Equal => {
                let mut mask = [0.0; DECK_SIZE+1];
                if let Some(c) = self.contract.current_trick()[self.side.next_i(relative_side)] {
                    mask[c.usize_index()] = 1.0;
                    mask[DECK_SIZE] = 1.0;
                }
                mask
            }
            Ordering::Greater => {
                let trick = self.contract.completed_tricks()[trick_number];
                let card = trick[self.side. next_i(relative_side)].unwrap();
                let mut mask = [0.0; DECK_SIZE+1];
                mask[card.usize_index()] = 1.0;
                mask[DECK_SIZE] = 1.0;
                mask
            }
        }
    }
}

#[cfg(test)]
mod tests{
    /*
    use karty::suits::Suit::Spades;
    use sztorm::learning::ActorCriticPolicy;
    use crate::bidding::Bid;
    use crate::cards::trump::TrumpGen;
    use crate::contract::{Contract, ContractParametersGen};
    use crate::player::side::Side;
    use crate::sztorm::state::ContractAgentInfoSetSimple;

    #[test]
    fn a2c_policy(){
        //let comm_association = SideMap::new(comm_env_north, comm_env_east, comm_env_south, comm_env_west);
        let contract = ContractParametersGen::new(Side::East, Bid::init(TrumpGen::Colored(Spades), 2).unwrap());
        let initial_contract = Contract::new(contract);
        let initial_state_south = ContractAgentInfoSetSimple::new(Side::South, hand_south, initial_contract.clone(), None);
        //let policy = ActorCriticPolicy::
        assert!(false);


    }

 */
}