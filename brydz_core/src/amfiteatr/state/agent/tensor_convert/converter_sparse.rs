use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::tensor_data::{SimpleConvertToTensor, ConversionToTensor};
use crate::player::side::SIDES;

use crate::amfiteatr::state::{ContractInfoSet};

#[derive(Default)]
pub struct ContractInfoSetConvertSparse{}

impl ConversionToTensor for ContractInfoSetConvertSparse{
    fn desired_shape(&self) -> &'static [i64] {
        &[contract_state_sparse_convert_with_init_assumption::STATE_REPR_SIZE as i64;1]
    }
}

pub(crate) mod contract_state_sparse_convert_with_init_assumption{
    // contract spec section
    //  0000-0003:      ROLE (1.0 | 0.0) [Declarer, Whist, Dummy, Offside]
    //  0004-0008:      CONTRACT_SUIT [Clubs, Diamonds, Hearts, Spades, NT] e.g 0.0,0.0,1.0,0.0,0.0 means H
    //  0009-0009:      CONRACT_VALUE: value/7 (normalise to 0..1)
    //  0010-0010:      DOUBLING: 0.0 (no); 0.5 (double); 1.0 (redouble)
    // initial assumption section
    //  0011-0062:      this player init distr [52]
    //  0063-0114:      next player ini assumption [52]
    //  0115-0166:      partner initial assumption [52]
    //  0177-0218:      prev player initial assumption [52]
    //  0219-0270:      own cards
    //  0271-0322:      next player cards
    //  0223-0374:      partner cards
    //  0375-0426:      previous player cards
    //  0427-0478       cards in game
    //  current trick data
    //  0479-0479:      called suit [None, Clubs, Diamonds, Hearts, Spades]
    // ... todo

        // contract spec section
    //  0000-0003:      ROLE (1.0 | 0.0) [Declarer, Whist, Dummy, Offside]
    //  0004-0008:      CONTRACT_TRUMP [Clubs, Diamonds, Hearts, Spades, NT] e.g 0.0,0.0,1.0,0.0,0.0 means H
    //  0009-0009:      CONRACT_VALUE: value/7 (normalise to 0..1)
    //  0010-0010:      DOUBLING: 0.0 (no); 0.5 (double); 1.0 (redouble)
    // card distribution section - hints for ownership (may be bad as probabilities)
    //  0011-0062:      own cards
    //  0063-0114:      next player cards
    //  0115-0166:      partner cards
    //  0167-0218:      previous player cards
    //  current trick data
    //  0219-0222:      called suit [Clubs, Diamonds, Hearts, Spades]
    //  0223-0226:      trick starting side [own, next, partner, right]
    //  0227-0278:      card placed  own [52 - 1.0 to flag card]
    //  0279-0330:      card placed left [52 - 1.0 to flag card]
    //  0331-0382:      card placed partner [52 - 1.0 to flag card]
    //  0383-0334:      card placed right [52 - 1.0 to flag card]

    use karty::cards::{STANDARD_DECK};
    use karty::symbol::CardSymbol;
    use crate::bidding::Doubling;
    use crate::cards::trump::TrumpGen;
    use crate::contract::ContractMechanics;
    use crate::player::side::Side;
    use crate::amfiteatr::state::ContractInfoSet;
    

    pub const STATE_REPR_SIZE: usize = RIGHT_CARD_PLACED_OFFSET + SPARSE_DECK_SIZE;
    pub const SPARSE_DECK_SIZE: usize = 52;

    pub const CONTRACT_ROLE_OFFSET: usize = 0;
    pub const CONTRACT_TRUMP_OFFSET: usize = CONTRACT_ROLE_OFFSET + 4;
    pub const CONTRACT_VALUE_OFFSET: usize = CONTRACT_TRUMP_OFFSET + 5;
    pub const DOUBLING_OFFSET: usize = CONTRACT_VALUE_OFFSET + 1;
    pub const CARD_SET_OFFSET: usize = DOUBLING_OFFSET + 1;
    pub const LEFT_CARD_SET_OFFSET: usize = CARD_SET_OFFSET + SPARSE_DECK_SIZE;
    pub const PARTNER_CARD_SET_OFFSET: usize = LEFT_CARD_SET_OFFSET + SPARSE_DECK_SIZE;
    pub const RIGHT_CARD_SET_OFFSET: usize = PARTNER_CARD_SET_OFFSET + SPARSE_DECK_SIZE;
    pub const CALLED_SUIT_OFFSET: usize = RIGHT_CARD_SET_OFFSET + SPARSE_DECK_SIZE;
    pub const TRICK_STARTING_SIDE_OFFSET: usize = CALLED_SUIT_OFFSET + 4;
    pub const OWN_CARD_PLACED_OFFSET: usize = TRICK_STARTING_SIDE_OFFSET + 4;
    #[allow(dead_code)]
    pub const LEFT_CARD_PLACED_OFFSET: usize = OWN_CARD_PLACED_OFFSET + SPARSE_DECK_SIZE;
    #[allow(dead_code)]
    pub const PARTNER_CARD_PLACED_OFFSET: usize = LEFT_CARD_PLACED_OFFSET + SPARSE_DECK_SIZE;
    #[allow(dead_code)]
    pub const RIGHT_CARD_PLACED_OFFSET: usize = PARTNER_CARD_PLACED_OFFSET + SPARSE_DECK_SIZE;

    #[cfg(test)]
    mod tests{
        use crate::amfiteatr::state::contract_state_sparse_convert_with_init_assumption::{*};

        #[test]
        fn own_trick_offset(){
            assert_eq!(OWN_CARD_PLACED_OFFSET, 227)
        }
        #[test]
        fn left_card_offset(){
            assert_eq!(LEFT_CARD_PLACED_OFFSET, 279);
        }
        #[test]
        fn cardset_offset(){
            assert_eq!(CARD_SET_OFFSET, 11);
        }
        #[test]
        fn called_suit_offset(){
            assert_eq!(CALLED_SUIT_OFFSET, 219);
        }
    }
    #[inline]
    pub fn write_contract_params<T: ContractInfoSet>(state_repr: &mut [f32], state: &T){
        let u = state.side() -  state.contract_data().declarer();
        state_repr[u as usize] = 1.0;

        let t = match state.contract_data().contract_spec().bid().trump(){
            TrumpGen::Colored(c) => c.usize_index(),
            TrumpGen::NoTrump => 4,
        };
        state_repr[CONTRACT_TRUMP_OFFSET+t] = 1.0;
        state_repr[CONTRACT_VALUE_OFFSET] = state.contract_data().contract_spec().bid().number() as f32 / 7.0;
        state_repr[DOUBLING_OFFSET] = match state.contract_data().contract_spec().doubling(){
            Doubling::None => 0.0,
            Doubling::Double => 0.5,
            Doubling::Redouble => 1.0
        };

    }

    #[inline]
    pub fn write_card_hold_probability_hints<T: ContractInfoSet>(state_repr: &mut [f32], state: &T, side: Side){
        let side_diff = (side - state.side()) as usize;
        let offset = CARD_SET_OFFSET + (side_diff * SPARSE_DECK_SIZE);
        for card in STANDARD_DECK{
            state_repr[offset + card.usize_index()] = state.hint_card_probability_for_player(side, &card);
        }

    }
    #[inline]
    pub fn write_called_suit<T: ContractInfoSet>(state_repr: &mut [f32], state: &T){
        let u = CALLED_SUIT_OFFSET + match state.contract_data().current_trick().called_suit(){
            None => {
                return;
            }
            Some(s) => s.usize_index()
        };
        state_repr[u] = 1.0;

    }
    #[inline]
    pub fn write_trick_starter<T: ContractInfoSet>(state_repr: &mut [f32], state: &T){

        let u = TRICK_STARTING_SIDE_OFFSET
            + (state.contract_data().current_trick().first_player_side() - state.side()) as usize;
        state_repr[u] = 1.0;

    }

    #[inline]
    pub fn write_placed_card_in_tricks<T: ContractInfoSet>(state_repr: &mut [f32], state: &T){

        //let offset = LEFT_CARD_PLACED_OFFSET + ((side_diff-1) * SPARSE_DECK_SIZE);

        for i in 0..4{
            match state.contract_data().current_trick()[state.side().next_i(i)]{
                None => {}
                Some(c) => {
                    let offset = OWN_CARD_PLACED_OFFSET + ((i as usize) * SPARSE_DECK_SIZE) + c.usize_index();
                    state_repr[offset] = 1.0;
                }
            }
        }
    }




}

impl<T: ContractInfoSet> SimpleConvertToTensor<T> for ContractInfoSetConvertSparse{
    fn make_tensor(&self, t: &T) -> Tensor {
        use crate::amfiteatr::state::contract_state_sparse_convert_with_init_assumption::*;
        let mut buffer = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut buffer, t);
        for side in SIDES{
            write_card_hold_probability_hints(&mut buffer, t, side);
        }

        write_called_suit(&mut buffer, t);
        write_trick_starter(&mut buffer, t);
        write_placed_card_in_tricks(&mut buffer, t);
        Tensor::from_slice(&buffer[..])
    }
}
/*
impl<T: ContractInfoSet> ConvertToTensor<ContractInfoSetConvertSparse> for T {

    fn to_tensor(&self, _way: &ContractInfoSetConvertSparse) -> Tensor {
        use crate::sztorm::state::contract_state_sparse_convert_with_init_assumption::*;
        let mut buffer = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut buffer, &self);
        write_card_hold_probability_hints(&mut buffer, &self);
        write_called_suit(&mut buffer, &self);
        write_trick_starter(&mut buffer, &self);
        write_placed_card_in_tricks(&mut buffer, &self);



    }
}

 */



