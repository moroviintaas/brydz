use amfiteatr_rl::tch::Tensor;
use amfiteatr_rl::tensor_data::{ConversionToTensor, SimpleConvertToTensor};
use crate::amfiteatr::state::{ContractInfoSet};
use crate::player::side::SIDES;

#[derive(Default)]
pub struct ContractInfoSetConvertSparseHistoric{}

impl ConversionToTensor for ContractInfoSetConvertSparseHistoric {
    fn desired_shape(&self) -> &[i64] {
        &[contract_state_sparse_with_history::STATE_REPR_SIZE_WITH_HISTORY as i64;1]
    }
}
pub(crate) mod contract_state_sparse_with_history{
    use karty::symbol::CardSymbol;
    use crate::amfiteatr::state::contract_state_converter_common::STATE_REPR_SIZE;
    use crate::amfiteatr::state::contract_state_sparse_convert_with_init_assumption::{RIGHT_CARD_PLACED_OFFSET, SPARSE_DECK_SIZE};
    use crate::amfiteatr::state::ContractInfoSet;
    use crate::contract::ContractMechanics;

    pub const  STATE_REPR_SIZE_WITH_HISTORY: usize = STATE_REPR_SIZE + (SPARSE_DECK_SIZE * 4 * 13);

    const CONTRACT_HISTORY_OFFSET: usize = RIGHT_CARD_PLACED_OFFSET + SPARSE_DECK_SIZE;

    pub fn write_tricks<T: ContractInfoSet>(state_repr: &mut [f32], state: &T){
        let own = state.side();
        for t in 0..state.contract_data().completed_tricks().len(){
            for p in 0..4{
                if let Some(c) = state.contract_data().completed_tricks()[t][own.next_i(p)]{
                    state_repr[CONTRACT_HISTORY_OFFSET + (((4*t)+p as usize)*SPARSE_DECK_SIZE) + c.usize_index()] = 1.0;
                }
            }


        }
    }

}



impl<T: ContractInfoSet> SimpleConvertToTensor<T> for ContractInfoSetConvertSparseHistoric{

    fn make_tensor(&self, t: &T) -> Tensor {
        use crate::amfiteatr::state::contract_state_sparse_with_history::*;
        use crate::amfiteatr::state::contract_state_sparse_convert_with_init_assumption::*;
        let mut buffer = [0f32; STATE_REPR_SIZE_WITH_HISTORY];
        write_contract_params(&mut buffer, t);
        for side in SIDES{
            write_card_hold_probability_hints(&mut buffer, t, side);
        }

        write_called_suit(&mut buffer, t);
        write_trick_starter(&mut buffer, t);
        write_placed_card_in_tricks(&mut buffer, t);
        write_tricks(&mut buffer, t);
        Tensor::from_slice(&buffer[..])
    }
}