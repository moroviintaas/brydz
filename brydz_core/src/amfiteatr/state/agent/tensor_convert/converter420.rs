use amfiteatr_rl::tensor_data::{ConversionToTensor};

/// ```
/// use brydz_core::bidding::{Bid, Doubling};
/// use brydz_core::cards::trump::TrumpGen;
/// use brydz_core::contract::{Contract, ContractParameters};
/// use brydz_core::contract::SmartTrickSolver::Trump;
/// use brydz_core::player::side::Side::*;
/// use brydz_core::amfiteatr::state::{ContractAgentInfoSetSimple, ContractInfoSetConvert420, ContractStateUpdate};
/// use brydz_core::amfiteatr::state::ContractAction::PlaceCard;
/// use karty::card_set;
/// use karty::suits::Suit::Diamonds;
/// use karty::cards::*;
/// use amfiteatr_core::agent::InformationSet;
/// use amfiteatr_rl::tensor_data::SimpleConvertToTensor;
/// let final_bid = Bid::init(TrumpGen::Colored(Diamonds), 3).unwrap();
/// let contract_spec = ContractParameters::new_d(East, final_bid, Doubling::Double);
/// let contract = Contract::new(contract_spec);
/// let whist_hand = card_set!(TWO_CLUBS, SIX_CLUBS, SEVEN_CLUBS, FIVE_DIAMONDS, SEVEN_DIAMONDS,
///     NINE_DIAMONDS, TEN_DIAMONDS, JACK_DIAMONDS, FOUR_HEARTS, SIX_SPADES, SEVEN_SPADES,
///     JACK_SPADES,  QUEEN_SPADES );
/// let dummy_hand = card_set!(EIGHT_CLUBS, THREE_DIAMONDS, EIGHT_DIAMONDS, SEVEN_HEARTS,
///     EIGHT_HEARTS, NINE_HEARTS, QUEEN_HEARTS, ACE_HEARTS, TWO_SPADES, THREE_SPADES, FOUR_SPADES,
///     EIGHT_SPADES, NINE_SPADES);
/// let mut whist_state = ContractAgentInfoSetSimple::new(South, whist_hand, contract, Some(dummy_hand));
/// whist_state.update(ContractStateUpdate::new(South, PlaceCard(JACK_SPADES))).unwrap();
/// whist_state.update(ContractStateUpdate::new(West, PlaceCard(TWO_SPADES))).unwrap();
/// let tensor = ContractInfoSetConvert420{}.make_tensor(&whist_state);
/// let v: Vec<f32> = tensor.try_into().unwrap();
/// assert_eq!(v[0], 1.0);
/// assert_eq!(v[1], 1.0);
/// assert_eq!(v[2], 3.0);
/// assert_eq!(v[3], 1.0);
/// assert_eq!(v[4], 0.25);
/// assert_eq!(v[117], 0.25);
/// assert_eq!(v[212], 0.0); //dummy does not have TWO_CLUBS
/// assert_eq!(v[218], 1.0); //dummy has 8 Clubs
/// assert_eq!(v[226], 1.0); //dummy has 3 diamonds
/// assert_eq!(v[264], 1.0); //2 clubs in hand
/// assert_eq!(v[315], 0.0); //A clubs not in hand
/// assert_eq!(v[316], -1.0);
/// assert_eq!(v[317], -1.0);
/// assert_eq!(v[318], 3.0);
/// assert_eq!(v[319], 9.0);
/// assert_eq!(v[320], 3.0);
/// assert_eq!(v[321], 0.0);
/// for i in 322..420{
///     assert_eq!(v[i], -1.0);
/// }
/// ```
#[derive(Default)]
pub struct ContractInfoSetConvert420 {}

impl ConversionToTensor for ContractInfoSetConvert420 {
    fn desired_shape(&self) -> &'static [i64] {
        &[420]
    }
}
#[derive(Default)]
pub struct ContractInfoSetConvert420Normalised {}
impl ConversionToTensor for ContractInfoSetConvert420Normalised{
    fn desired_shape(&self) -> &'static [i64] {
        &[420]
    }
}
/*
impl<S: InformationSet<ContractDP>, T: ConvStateToTensor<S>>
ConvStateToTensor<Box<dyn InformationSet<ContractDP, ActionIteratorType=S::ActionIteratorType>>> for T{
    fn make_tensor(&self, t: &Box<dyn InformationSet<ContractDP, ActionIteratorType=S::ActionIteratorType>>) -> Tensor {
        self.make_tensor(t.as_ref())
    }
}

 */

pub(crate) mod contract_state_converter_common {
    use karty::cards::{Card2SymTrait, DECK_SIZE, STANDARD_DECK_CDHS};
    use karty::figures::Figure;
    use karty::hand::{ HandTrait};
    use karty::suits::Suit;
    use karty::symbol::CardSymbol;
    use crate::bidding::Doubling;
    use crate::contract::ContractMechanics;
    use crate::amfiteatr::state::{ContractInfoSet};

    pub const SPARSE_DECK_SIZE: usize = 52;
    pub const TRICK_REPRESENTATION_SIZE: usize = 2 * 4; //two numbers for suit and figure x 4 5 players
    pub const TRICK_NUMBER: usize = 13;
    pub const CONTRACT_TRUMP_OFFSET: usize = 1;
    pub const CONTRACT_VALUE_OFFSET: usize = CONTRACT_TRUMP_OFFSET + 1;
    pub const DOUBLING_OFFSET: usize = CONTRACT_VALUE_OFFSET + 1;
    pub const DECLARER_DIST_OFFSET: usize = DOUBLING_OFFSET + 1;
    pub const WHIST_DIST_OFFSET: usize = DECLARER_DIST_OFFSET + SPARSE_DECK_SIZE;
    pub const DUMMY_DIST_OFFSET: usize = WHIST_DIST_OFFSET + SPARSE_DECK_SIZE;
    pub const OFFSIDE_DIST_OFFSET: usize = DUMMY_DIST_OFFSET + SPARSE_DECK_SIZE;
    pub const CURRENT_DUMMY_CARDS: usize = OFFSIDE_DIST_OFFSET + SPARSE_DECK_SIZE;
    pub const CURRENT_OWN_CARDS: usize = CURRENT_DUMMY_CARDS + SPARSE_DECK_SIZE;
    pub const TRICKS_OFFSET: usize = CURRENT_OWN_CARDS + SPARSE_DECK_SIZE;
    pub const STATE_REPR_SIZE: usize = TRICKS_OFFSET + 13 * TRICK_REPRESENTATION_SIZE;

    #[inline]
    pub fn write_contract_params<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        state_repr[0] = (state.side() - state.contract_data().contract_spec().declarer()) as f32;
        state_repr[CONTRACT_TRUMP_OFFSET] = state.contract_data().contract_spec().bid().trump().into();
        state_repr[CONTRACT_VALUE_OFFSET] = state.contract_data().contract_spec().bid().number() as f32;
        state_repr[DOUBLING_OFFSET] = match state.contract_data().contract_spec().doubling(){
            Doubling::None => 0.0,
            Doubling::Double => 1.0,
            Doubling::Redouble => 2.0
        };
    }
    #[inline]
    pub fn write_contract_params_n<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        state_repr[0] = (state.side() - state.contract_data().contract_spec().declarer()) as f32;
        state_repr[CONTRACT_TRUMP_OFFSET] = state.contract_data().contract_spec().bid().trump().into();
        state_repr[CONTRACT_VALUE_OFFSET] = state.contract_data().contract_spec().bid().number() as f32;
        state_repr[DOUBLING_OFFSET] = match state.contract_data().contract_spec().doubling(){
            Doubling::None => 0.0,
            Doubling::Double => 0.5,
            Doubling::Redouble => 1.0
        }
    }
    #[inline]
    pub fn write_current_dummy<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        if let Some(dhand) = state.dummy_hand(){
            for card in STANDARD_DECK_CDHS{
                if dhand.contains(&card){
                    state_repr[CURRENT_DUMMY_CARDS + card.usize_index()] = 1.0;
                }
            }
        } else {
            for i in CURRENT_DUMMY_CARDS..CURRENT_DUMMY_CARDS+DECK_SIZE{
                state_repr[i] = -1.0;
            }
        }
    }
    #[inline]
    pub fn write_current_hand<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        for card in STANDARD_DECK_CDHS{
            if state.hand().contains(&card){
                state_repr[CURRENT_OWN_CARDS + card.usize_index()] = 1.0;
            }
        }
    }
    #[inline]
    pub fn write_tricks<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        let declarer_side = state.contract_data().declarer();
        let tricks_done = state.contract_data().completed_tricks().len();
        //setting up completed tricks
        for trick_num in 0..tricks_done{
            let trick = &state.contract_data().completed_tricks()[trick_num];
            for offset in 0..4{

                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2)]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => c.suit().usize_index() as f32
                };
                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2) + 1]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => c.figure().usize_index() as f32
                };
            }

        }
        //setting not completed tricks with -1
        for next_trick_num in tricks_done+1..TRICK_NUMBER{
            for pos in 0..TRICK_REPRESENTATION_SIZE{
                state_repr[TRICKS_OFFSET + (next_trick_num * TRICK_REPRESENTATION_SIZE) + pos] = -1.0;
            }
        }
        //setting current trick
        for offset in 0..4{
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2)]
                = match state.contract_data().current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) => c.suit().usize_index() as f32
            };
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2) + 1]
                = match state.contract_data().current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) => c.figure().usize_index() as f32
            };
        }
    }

    #[inline]
    pub fn write_tricks_n<T: ContractInfoSet>(state_repr: &mut [f32; STATE_REPR_SIZE], state: &T){
        let declarer_side = state.contract_data().declarer();
        let tricks_done = state.contract_data().completed_tricks().len();
        //setting up completed tricks
        for trick_num in 0..tricks_done{
            let trick = &state.contract_data().completed_tricks()[trick_num];
            for offset in 0..4{

                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2)]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => (c.suit().usize_index() as f32 + 1.0)/ Suit::SYMBOL_SPACE as f32
                };
                state_repr[TRICKS_OFFSET + (trick_num * TRICK_REPRESENTATION_SIZE)  + (offset as usize * 2) + 1]
                    = match trick[declarer_side.next_i(offset)]{
                    None => -1.0,
                    Some(c) => (c.figure().usize_index() as f32 + 1.0)  / Figure::SYMBOL_SPACE as f32
                };
            }

        }
        //setting not completed tricks with -1
        for next_trick_num in tricks_done+1..TRICK_NUMBER{
            for pos in 0..TRICK_REPRESENTATION_SIZE{
                state_repr[TRICKS_OFFSET + (next_trick_num * TRICK_REPRESENTATION_SIZE) + pos] = -1.0;
            }
        }
        //setting current trick
        for offset in 0..4{
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2)]
                = match state.contract_data().current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) => (c.suit().usize_index() as f32 + 1.0)/ Suit::SYMBOL_SPACE as f32
            };
            state_repr[TRICKS_OFFSET + (tricks_done * TRICK_REPRESENTATION_SIZE) + (offset as usize * 2) + 1]
                = match state.contract_data().current_trick()[declarer_side.next_i(offset)]{
                None => -1.0,
                Some(c) =>  (c.figure().usize_index() as f32 + 1.0)  / Figure::SYMBOL_SPACE as f32
            };
        }
    }
}


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