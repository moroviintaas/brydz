use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{PolicyParams, PolicyTypeSelect};

#[derive(Clone)]
pub struct AgentConfiguration{
    pub info_set_type: InfoSetTypeSelect,
    pub info_set_conversion_type: InfoSetWayToTensorSelect,
    pub policy_params: PolicyParams,
}
/*
impl AgentConfiguration{

    pub fn new_network() ->
}


 */

