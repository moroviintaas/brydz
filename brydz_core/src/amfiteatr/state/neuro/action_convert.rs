use amfiteatr_core::error::ConvertError;
use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::error::TensorRepresentationError;
use karty::cards::Card2SymTrait;
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{ContextEncodeTensor, TensorEncoding};
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