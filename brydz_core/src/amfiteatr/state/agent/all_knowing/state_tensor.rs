use amfiteatr_core::agent::InformationSet;
use amfiteatr_core::error::{AmfiteatrError, ConvertError};
use amfiteatr_rl::MaskingInformationSetAction;
use amfiteatr_rl::tch::Tensor;
use karty::cards::{Card, DECK_SIZE, STANDARD_DECK_CDHS};
use karty::set::CardSet;
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, SimpleConvertToTensor};
use crate::amfiteatr::spec::ContractDP;
use crate::contract::ContractMechanics;
use crate::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractAction, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractInfoSet, ContractInfoSetConvertDense1, ContractInfoSetConvertSparse, ContractInfoSetConvertSparseHistoric};
use crate::amfiteatr::state::contract_state_converter_common::{DECLARER_DIST_OFFSET, STATE_REPR_SIZE, write_contract_params, write_current_dummy, write_current_hand, write_tricks};

impl SimpleConvertToTensor<ContractAgentInfoSetAllKnowing> for ContractInfoSetConvertDense1 {

    fn make_tensor(&self, t: &ContractAgentInfoSetAllKnowing) -> Tensor {


        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut state_repr, t);
        let declarer_side = t.contract_data().declarer();
        for side_index in 0usize..4{
            for card in Card::iterator(){
                //let proba = t.distribution_assumption()[declarer_side.next_i(side_index as u8)][&card].into();
                if t.initial_deal()[&declarer_side.next_i(side_index as u8)].contains(&card){
                    state_repr[DECLARER_DIST_OFFSET + (DECK_SIZE*side_index) + card.usize_index()] = 1.0;
                }

            }
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}

impl ContextEncodeTensor<ContractInfoSetConvertDense1> for ContractAgentInfoSetAllKnowing{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertDense1) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertSparse> for ContractAgentInfoSetAllKnowing{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertSparse) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

impl ContextEncodeTensor<ContractInfoSetConvertSparseHistoric> for ContractAgentInfoSetAllKnowing{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertSparseHistoric) -> Result<Tensor, ConvertError> {
        Ok(way.make_tensor(self))
    }
}

impl MaskingInformationSetAction<ContractDP, ActionPlaceCardConvertion1D> for ContractAgentInfoSetAllKnowing{

    /// ```
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{Contract, ContractMechanics, ContractParametersGen};
    /// use brydz_core::player::side::{Side, SideMap};
    /// use karty::cards::{ACE_SPADES, KING_HEARTS, KING_SPADES, TWO_SPADES};
    /// use karty::set::CardSetStd;
    /// use karty::suits::Suit;
    /// use std::str::FromStr;
    /// use rand::{rng, Rng};
    /// use rand::distr::StandardUniform;
    /// use amfiteatr_core::agent::InformationSet;
    /// use amfiteatr_rl::MaskingInformationSetAction;
    /// use brydz_core::amfiteatr::state::{ActionPlaceCardConvertion1D, ContractAction, ContractAgentInfoSetAllKnowing, ContractAgentInfoSetAssuming, ContractAgentInfoSetSimple};
    /// use brydz_core::deal::BiasedHandDistribution;
    /// let mut contract = Contract::new(
    ///     ContractParametersGen::new(Side::West, Bid::init(TrumpGen::Colored(Suit::Hearts), 1).unwrap(),));
    /// contract.insert_card(Side::North, KING_SPADES).unwrap();
    /// contract.insert_card(Side::East, TWO_SPADES).unwrap();
    /// let south_deck = CardSetStd::from_str("AT86.KJT93.4T.2A").unwrap();
    /// let south_info_set = ContractAgentInfoSetAllKnowing::new(Side::South, SideMap {
    ///     north: CardSetStd::from_str("KQJ.87.832.JT843").unwrap(),
    ///     east: CardSetStd::from_str("932.A42.KQ976.K9").unwrap(),
    ///     south: south_deck,
    ///     west: CardSetStd::from_str("754.Q65.AJ5.Q765").unwrap(),
    /// }, contract);
    ///
    /// //assert!(south_info_set.is_action_valid(&ContractAction::PlaceCard(ACE_SPADES)));
    /// //assert!(!south_info_set.is_action_valid(&ContractAction::PlaceCard(KING_HEARTS)));
    /// let masks_t = south_info_set.try_build_mask(&ActionPlaceCardConvertion1D{}).unwrap();
    /// let masks: Vec<bool> = Vec::try_from(masks_t).unwrap();
    /// assert_eq!(&masks[..], &[
    ///     false, false, false, false, false, false, false, false, false, false, false, false, false,
    ///     false, false, false, false, false, false, false, false, false, false, false, false, false,
    ///     false, false, false, false, false, false, false, false, false, false, false, false, false,
    ///     false, false, false, false, true, false, true, false, true, false, false, false, true,
    ///     ]);
    /// ```
    fn try_build_mask(&self, _ctx: &ActionPlaceCardConvertion1D) -> Result<Tensor, AmfiteatrError<ContractDP>> {

        let action_masks: Vec<bool> = STANDARD_DECK_CDHS.iter()
            .map(|c| self.is_action_valid(&ContractAction::PlaceCard(*c))).collect();

        Ok(Tensor::from_slice(action_masks.as_slice()))
    }
}