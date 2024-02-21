use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::error::TensorRepresentationError;
use karty::cards::Card2SymTrait;
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{ConvertToTensor, ConversionToTensor};
use crate::amfiteatr::state::ContractAction;

#[derive(Default)]
pub struct ContractActionWayToTensor{

}

impl ConversionToTensor for ContractActionWayToTensor{
    fn desired_shape(&self) -> &'static [i64] {
        &[2]
    }
}

impl ConvertToTensor<ContractActionWayToTensor> for ContractAction{
    fn try_to_tensor(&self, _way: &ContractActionWayToTensor) -> Result<Tensor, TensorRepresentationError> {
        match self{
            ContractAction::ShowHand(_) => {panic!("Not prepared to convert show hand to tensor")}
            ContractAction::PlaceCard(c) => {
                let v = vec![c.suit().usize_index() as f32, c.figure().usize_index() as f32];
                Ok(Tensor::from_slice(&v[..]))
            }
        }
    }
}