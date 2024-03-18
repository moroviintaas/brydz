use std::path::PathBuf;
use amfiteatr_rl::tch::Device;
use crate::options::operation::train::{InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{PolicyParams, PolicyTypeSelect};

#[derive(Clone)]
pub struct AgentConfiguration{
    pub info_set_type: InfoSetTypeSelect,
    pub info_set_conversion_type: InfoSetWayToTensorSelect,
    pub policy_params: PolicyParams,
    pub var_store_path: Option<PathBuf>,
    pub device: Device
}
/*
impl AgentConfiguration{

    pub fn new_network() ->
}


 */

