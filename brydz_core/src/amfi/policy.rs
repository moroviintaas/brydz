use amfiteatr_core::agent::Policy;
use crate::amfi::spec::ContractDP;

pub trait ContractPolicy: Policy<ContractDP>{}

impl<P: Policy<ContractDP>> ContractPolicy for P{}