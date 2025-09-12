use serde::{Deserialize, Serialize};
use brydz_core::deal::BiasedHandDistribution;


#[derive(Deserialize, Serialize, Clone)]
#[allow(clippy::large_enum_variant)]
pub enum DistributionTemplate {
    Simple,
    Suspect(BiasedHandDistribution)
}

impl From<DistributionTemplate> for BiasedHandDistribution{
    fn from(value: DistributionTemplate) -> Self {
        match value{
            DistributionTemplate::Simple => BiasedHandDistribution::default(),
            DistributionTemplate::Suspect(d) => d
        }
    }
}