use tch::Tensor;
use karty::cards::Card;
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{TensorBuilder, TensorInterpreter};
use crate::amfi::state::{BuildStateHistoryTensor, ContractAction, ContractAgentInfoSetSimple, ConvertError};
use crate::amfi::state::agent::assuming::ContractAgentInfoSetAssuming;

pub struct ContractStateHistConverter{

}

impl TensorBuilder<ContractAgentInfoSetSimple> for ContractStateHistConverter{
    type Error = ConvertError;

    fn build_tensor(&self, t: &ContractAgentInfoSetSimple) -> Result<Tensor, Self::Error> {
        Ok(t.state_history_tensor())
    }
}


impl TensorBuilder<ContractAgentInfoSetAssuming> for ContractStateHistConverter{
    type Error = ConvertError;

    fn build_tensor(&self, t: &ContractAgentInfoSetAssuming) -> Result<Tensor, Self::Error> {
        Ok(t.state_history_tensor())
    }
}

pub struct ContractActionInterpreter{}

impl TensorInterpreter<ContractAction> for ContractActionInterpreter{
    type Error = ConvertError;

    fn interpret_tensor(&self, tensor: &Tensor) -> Result<ContractAction, Self::Error> {
        let av: Vec<i64> = tensor.try_into().unwrap();
        Ok(ContractAction::PlaceCard(Card::from_usize_index(av[0] as usize).unwrap()))
    }
}
