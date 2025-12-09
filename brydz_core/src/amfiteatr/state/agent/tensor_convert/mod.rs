//mod state_with_hist_conv;
mod converter_dense1;
mod converter_sparse;
mod converter_sparse_historic;

use amfiteatr_rl::tensor_data::TensorEncoding;
//pub use state_with_hist_conv::*;
pub use converter_dense1::*;
pub use converter_sparse::*;
pub use converter_sparse_historic::*;


pub enum ContractInfoSetEncoding{
    Dense1(ContractInfoSetConvertDense1),
    Sparse(ContractInfoSetConvertSparse),
    SparseHistoric(ContractInfoSetConvertSparseHistoric),
}

impl TensorEncoding for ContractInfoSetEncoding {
    fn desired_shape(&self) -> &[i64] {
        match self{
            ContractInfoSetEncoding::Dense1(a) => a.desired_shape(),
            ContractInfoSetEncoding::Sparse(a) => a.desired_shape(),
            ContractInfoSetEncoding::SparseHistoric(a) => a.desired_shape(),
        }
    }
}