use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::options::operation::train::{DeviceSelect, InfoSetTypeSelect, InfoSetWayToTensorSelect};
use crate::options::operation::train::sessions::{PolicyParams};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AgentConfiguration{
    pub info_set_type: InfoSetTypeSelect,
    pub info_set_conversion_type: InfoSetWayToTensorSelect,
    pub policy_params: PolicyParams,
    pub var_load_path: Option<PathBuf>,
    pub var_store_path: Option<PathBuf>,
    pub device: DeviceSelect
}

impl Default for AgentConfiguration{
    fn default() -> Self {
        AgentConfiguration{
            info_set_type: InfoSetTypeSelect::Simple,
            info_set_conversion_type: InfoSetWayToTensorSelect::Sparse,
            policy_params: Default::default(),
            var_load_path: None,
            var_store_path: None,
            device: DeviceSelect::Cpu,
        }
    }
}
/*
impl AgentConfiguration{

    pub fn new_network() ->
}


 */

