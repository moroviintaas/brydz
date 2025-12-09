use amfiteatr_core::error::ConvertError;
use amfiteatr_rl::tch::Tensor;
use karty::cards::{Card, Card2SymTrait};
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{ContextDecodeIndexI64, ContextEncodeIndexI64, ContextEncodeTensor, TensorDecoding, TensorEncoding, TensorIndexI64Encoding};
use crate::amfiteatr::state::ContractAction;

#[derive(Default)]
pub struct ContractActionWayToTensor{

}

impl TensorEncoding for ContractActionWayToTensor{
    fn desired_shape(&self) -> &'static [i64] {
        &[2]
    }
}

impl ContextEncodeTensor<ContractActionWayToTensor> for ContractAction{
    fn try_to_tensor(&self, _way: &ContractActionWayToTensor) -> Result<Tensor, ConvertError> {
        match self{
            ContractAction::ShowHand(_) => {panic!("Not prepared to convert shows hand to tensor")}
            ContractAction::PlaceCard(c) => {
                let v = [c.suit().usize_index() as f32, c.figure().usize_index() as f32];
                Ok(Tensor::from_slice(&v[..]))
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ActionPlaceCardConvertion1D{}

impl TensorEncoding for ActionPlaceCardConvertion1D{
    fn desired_shape(&self) -> &'static [i64] { &[1]}
}

impl TensorDecoding for ActionPlaceCardConvertion1D{
    fn expected_input_shape(&self) -> &[i64] {
        &[1]
    }
}

impl TensorIndexI64Encoding for ActionPlaceCardConvertion1D{
    fn min(&self) -> i64 {
        0
    }

    fn limit(&self) -> i64 {
        52
    }
}

impl ContextDecodeIndexI64<ActionPlaceCardConvertion1D> for  ContractAction{
    fn try_from_index(index: i64, _encoding: &ActionPlaceCardConvertion1D) -> Result<Self, ConvertError> {
        let card = Card::from_usize_index(index as usize)
            .map_err(|e| ConvertError::ConvertFromTensor {
                origin: format!("index: {index}"),
                context: "Converting action index to card instance".to_string() })?;

        Ok(ContractAction::PlaceCard(card))
    }
}

impl ContextEncodeIndexI64<ActionPlaceCardConvertion1D> for  ContractAction{
    fn try_to_index(&self, encoding: &ActionPlaceCardConvertion1D) -> Result<i64, ConvertError> {
        match self{
            ContractAction::ShowHand(_) => Err(ConvertError::ConvertToTensor {
                origin: "".to_string(),
                context: format!("Converting ShowHand action to tensor in context {encoding:?}"),
            }),
            ContractAction::PlaceCard(c) => {
                Ok(c.usize_index() as i64)
            }
        }
    }
}