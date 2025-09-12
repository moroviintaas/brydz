//mod contract_net;
pub mod qnet_synthetic;
//mod qnet_state_hist;
mod explore_exploit_policy;

//pub use contract_net::*;

use amfiteatr_rl::tch::{nn, Tensor};
use amfiteatr_rl::tch::nn::Sequential;
pub use qnet_synthetic::*;
//pub use qnet_state_hist::*;
pub use explore_exploit_policy::*;

type Model = Box<dyn Fn(&Tensor) -> Tensor + Send>;


pub fn tch_model(p: &nn::Path, sequential: Sequential) -> Model {
    let device = p.device();
    Box::new(move | xs | {
        xs.to_device(device).apply(&sequential)
    })
}

