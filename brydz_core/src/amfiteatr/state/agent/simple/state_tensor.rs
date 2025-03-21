use amfiteatr_core::error::ConvertError;
use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, SimpleConvertToTensor};
use crate::amfiteatr::state::{ContractAgentInfoSetSimple, ContractInfoSetConvertDense1, ContractInfoSetConvertDense1Normalised, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric};



//  0000:   ROLE {declarer: 0.0, whist: 1.0, dummy: 2.0, offside: 3.0}
//  0001:   CONTRACT_SUIT {C: 0.0, D: 1.0, H: 2.0, S: 3.0, NT:4.0}
//  0002:   CONTRACT_VALUE: as float (1..=7)
//  0003:   DOUBLING {no: 0.0, double: 1.0, redouble: 2.0}
//  0004:   DECLARER_INIT_DISTRIBUTION [52]
//  0056:   WHIST_INIT_DISTRIBUTION [52]
//  0108:   DUMMY_INIT_DISTRIBUTION [52]
//  0160:   OFFSIDE_INIT_DISTRIBUTION [52]
//  0212:   CURRENT_DUMMY_CARDS [52]
//  0264:   CURRENT_OWN_CARDS [52]
//  0316:   TRICKS [TRICK_NUMBER * TRICK_REPRESENTATION_SIZE]
//              representing trick: [DECLARER[S,F], WHIST[S,F], DUMMY[S,F], OFFSIDE[S,F]] (-1.0, -1.0) for non yet
//  0420:
impl SimpleConvertToTensor<ContractAgentInfoSetSimple> for ContractInfoSetConvertDense1 {

    fn make_tensor(&self, t: &ContractAgentInfoSetSimple) -> Tensor {
        use crate::amfiteatr::state::contract_state_converter_common::*;

        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut state_repr, t);
        /*
        for i in DECLARER_DIST_OFFSET..CURRENT_DUMMY_CARDS{
            state_repr[i] = 0.25;
        }

         */
        for byte_repr in state_repr.iter_mut().take(CURRENT_DUMMY_CARDS).skip(DECLARER_DIST_OFFSET){
            *byte_repr = 0.25;
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}

impl SimpleConvertToTensor<ContractAgentInfoSetSimple> for ContractInfoSetConvertDense1Normalised {
    fn make_tensor(&self, t: &ContractAgentInfoSetSimple) -> Tensor {
        use crate::amfiteatr::state::contract_state_converter_common::*;

        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params_n(&mut state_repr, t);
        /*
        for i in DECLARER_DIST_OFFSET..CURRENT_DUMMY_CARDS{
            state_repr[i] = 0.25;
        }

         */
        for repr_byte in state_repr.iter_mut().take(CURRENT_DUMMY_CARDS).skip(DECLARER_DIST_OFFSET){
            *repr_byte = 0.25;
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks_n(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}



impl ContextEncodeTensor<ContractInfoSetConvertDense1> for ContractAgentInfoSetSimple{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertDense1) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}



/// ```
/// use amfiteatr_rl::tch::Tensor;
/// use brydz_core::bidding::Bid;
/// use brydz_core::cards::trump::TRUMP_CLUBS;
/// use brydz_core::contract::{Contract, ContractParameters};
/// use brydz_core::player::side::Side;
/// use brydz_core::player::side::Side::{East, North, South, West};
/// use brydz_core::amfiteatr::state::{ContractAgentInfoSetSimple, ContractInfoSetConvertSparse, ContractStateUpdate};
/// use brydz_core::amfiteatr::state::ContractAction::{PlaceCard, ShowHand};
/// use karty::card_set;
/// use karty::cards::*;
/// use amfiteatr_core::agent::InformationSet;
/// use amfiteatr_rl::tensor_data::ContextEncodeTensor;
/// let card_set = card_set!(
///     THREE_CLUBS, FOUR_CLUBS, FIVE_CLUBS, NINE_CLUBS,
///     QUEEN_CLUBS, KING_CLUBS, ACE_CLUBS, TWO_DIAMONDS,
///     FOUR_DIAMONDS, QUEEN_DIAMONDS, TEN_SPADES, KING_SPADES, ACE_SPADES) ;
/// let dummy_hand = card_set![
///     TEN_CLUBS, JACK_CLUBS, SIX_DIAMONDS, KING_DIAMONDS,
///     ACE_DIAMONDS, TWO_HEARTS, THREE_HEARTS, FIVE_HEARTS,
///     SIX_HEARTS, TEN_HEARTS, JACK_HEARTS, KING_HEARTS, FIVE_SPADES
/// ];
/// let contract_params = ContractParameters::new(North, Bid::init(TRUMP_CLUBS, 3).unwrap());
/// let contract = Contract::new(contract_params);
/// let mut info_set = ContractAgentInfoSetSimple::new(Side::North, card_set, contract, None);
/// info_set.update(ContractStateUpdate::new(East, PlaceCard(ACE_HEARTS))).unwrap();
/// info_set.update(ContractStateUpdate::new(South, ShowHand(dummy_hand))).unwrap();
/// let expected = vec![
///     1.0, 0.0, 0.0, 0.0, //declarer
///     1.0, 0.0, 0.0, 0.0, 0.0, // trump: clubs
///     3.0/7.0, 0.0,    //value 3, doubling: no
///     0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // own clubs
///     1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // own diwamonds
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // own hearts
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // own spades
///
///     0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // left clubs
///     0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, // left diamonds
///     0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0, // left hearts - (ace placed)
///     0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.0, // left spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, //partner clubs,
///     0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, //partner diamonds,
///     1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, //partner hearts
///     0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner spades
///
///     0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // right clubs
///     0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, // right diamonds
///     0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0, // right hearts
///     0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.0, // right spades
///
///     0.0, 0.0, 1.0, 0.0, // called heart
///     0.0, 1.0, 0.0, 0.0, // trick is started by player positioned one to the left
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, //left placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card spades
/// ];
/// let v: Vec<f32> = info_set.to_tensor(&ContractInfoSetConvertSparse{}).try_into().unwrap();
/// assert_eq!(v, expected);
/// info_set.update(ContractStateUpdate::new(North, PlaceCard(TWO_HEARTS))).unwrap();
/// info_set.update(ContractStateUpdate::new(West, PlaceCard(FOUR_SPADES))).unwrap();
/// let expected = vec![
///     1.0, 0.0, 0.0, 0.0, //declarer
///     1.0, 0.0, 0.0, 0.0, 0.0, // trump: clubs
///     3.0/7.0, 0.0,    //value 3, doubling: no
///     0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0, // own clubs
///     1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // own diwamonds
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // own hearts
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, // own spades
///
///     0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // left clubs
///     0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, // left diamonds
///     0.0, 0.0, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.5, 0.0, 0.0, // left hearts - (ace placed)
///     0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.0, // left spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, //partner clubs,
///     0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, //partner diamonds,
///     0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, //partner hearts
///     0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner spades
///
///     0.5, 0.0, 0.0, 0.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // right clubs
///     0.0, 0.5, 0.0, 0.5, 0.0, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, // right diamonds
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // right hearts
///     0.5, 0.5, 0.0, 0.0, 0.5, 0.5, 0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.0, // right spades
///
///     0.0, 0.0, 1.0, 0.0, // called heart
///     0.0, 1.0, 0.0, 0.0, // trick is started by player positioned one to the left
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //own placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, //left placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //left placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card diam
///     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card hearts (A)
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //partner placed card spades
///
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card clubs
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card diam
///     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card hearts (A)
///     0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, //right placed card spades
/// ];
///  let v: Vec<f32> = info_set.to_tensor(&ContractInfoSetConvertSparse{}).try_into().unwrap();
///  assert_eq!(v, expected);
/// ```
impl ContextEncodeTensor<ContractInfoSetConvertSparse> for ContractAgentInfoSetSimple{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertSparse) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertSparseHistoric> for ContractAgentInfoSetSimple{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertSparseHistoric) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertDense1Normalised> for ContractAgentInfoSetSimple{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertDense1Normalised) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

