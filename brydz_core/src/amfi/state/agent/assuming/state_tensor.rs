use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::error::TensorRepresentationError;
use karty::cards::{Card, DECK_SIZE};
use karty::symbol::CardSymbol;
use amfiteatr_rl::tensor_data::{ConvertToTensor, SimpleConvertToTensor};
use crate::contract::ContractMechanics;
use crate::amfi::state::agent::assuming::ContractAgentInfoSetAssuming;
use crate::amfi::state::{ContractInfoSet, ContractInfoSetConvert420, ContractInfoSetConvertSparse};
use crate::amfi::state::contract_state_converter_common::{DECLARER_DIST_OFFSET, STATE_REPR_SIZE, write_contract_params, write_current_dummy, write_current_hand, write_tricks};

impl SimpleConvertToTensor<ContractAgentInfoSetAssuming> for ContractInfoSetConvert420 {

    fn make_tensor(&self, t: &ContractAgentInfoSetAssuming) -> Tensor {


        let mut state_repr = [0f32; STATE_REPR_SIZE];
        write_contract_params(&mut state_repr, t);
        let declarer_side = t.contract_data().declarer();
        for side_index in 0usize..4{
            for card in Card::iterator(){
                let proba = t.distribution_assumption()[declarer_side.next_i(side_index as u8)][&card].into();
                state_repr[DECLARER_DIST_OFFSET + (DECK_SIZE*side_index) + card.usize_index()] = proba;
            }
        }

        write_current_dummy(&mut state_repr, t);
        write_current_hand(&mut state_repr, t);
        write_tricks(&mut state_repr, t);


        Tensor::from_slice(&state_repr[..])

    }
}
impl ConvertToTensor<ContractInfoSetConvert420> for ContractAgentInfoSetAssuming{
    fn try_to_tensor(&self, way: &ContractInfoSetConvert420) -> Result<Tensor, TensorRepresentationError> {
        Ok(way.make_tensor(self))
    }
}

impl ConvertToTensor<ContractInfoSetConvertSparse> for ContractAgentInfoSetAssuming{
    fn try_to_tensor(&self, way: &ContractInfoSetConvertSparse) -> Result<Tensor, TensorRepresentationError> {
        Ok(way.make_tensor(self))
    }
}