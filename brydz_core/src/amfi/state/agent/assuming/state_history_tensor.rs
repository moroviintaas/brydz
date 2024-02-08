use std::cmp::Ordering;
use karty::cards::{ DECK_SIZE, STANDARD_DECK_CDHS};
use karty::hand::{HandTrait};
use karty::symbol::CardSymbol;
use crate::cards::trump::TrumpGen;
use crate::contract::ContractMechanics;
use crate::amfi::state::agent::assuming::ContractAgentInfoSetAssuming;
use crate::amfi::state::BuildStateHistoryTensor;


impl BuildStateHistoryTensor for ContractAgentInfoSetAssuming{
    fn contract_params(&self) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.0; DECK_SIZE+1];
        result[0] = self.contract().contract_spec().bid().number() as f32;
        match self.contract().contract_spec().bid().trump(){
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

    fn prediction(&self, relative_side: u8) -> [f32; DECK_SIZE + 1] {
        let mut result = [0.0; DECK_SIZE+1];
        for card in STANDARD_DECK_CDHS{
            result[card.usize_index()] = self.distribution_assumption()[self.side().next_i(relative_side)][&card].into()
        }
        result[DECK_SIZE] = 1.0;
        result

    }

    fn actual_cards(&self) -> [f32; DECK_SIZE + 1] {
        let mut cards = [0.0;DECK_SIZE+1];
        /*for suit in SUITS{
            for figure in FIGURES{

            }
        }*/
        for c in STANDARD_DECK_CDHS{
            if self.hand().contains(&c){
                cards[c.usize_index()] = 1.0;
            } else {
                cards[c.usize_index()] = 0.0;
            }


        }
        cards[DECK_SIZE] = 1.0;
        cards
    }

    fn actual_dummy_cards(&self) -> [f32; DECK_SIZE + 1] {
        match self.dummy_hand(){
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
        match self.contract().completed_tricks().len().cmp(&trick_number){
            Ordering::Less => {
                [0.0;DECK_SIZE+1]

            }
            Ordering::Equal => {
                let mut mask = [0.0; DECK_SIZE+1];
                if let Some(c) = self.contract().current_trick()[self.side().next_i(relative_side)] {
                    mask[c.usize_index()] = 1.0;
                    mask[DECK_SIZE] = 1.0;
                }
                mask
            }
            Ordering::Greater => {
                let trick = self.contract().completed_tricks()[trick_number];
                let card = trick[self.side(). next_i(relative_side)].unwrap();
                let mut mask = [0.0; DECK_SIZE+1];
                mask[card.usize_index()] = 1.0;
                mask[DECK_SIZE] = 1.0;
                mask
            }
        }
    }
}