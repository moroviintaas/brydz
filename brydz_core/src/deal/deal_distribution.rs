use std::boxed::Box;
use rand::distr::Distribution;
use rand::Rng;
use karty::set::CardSetStd;
use crate::deal::{BiasedHandDistribution, distribute_standard_deck_on_4};
use crate::player::side::SideMap;


#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive( Clone, Debug)]
pub enum DealDistribution{
    Fair,
    Biased(Box<BiasedHandDistribution>),
}

impl Distribution<SideMap<CardSetStd>> for DealDistribution{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SideMap<CardSetStd> {
        match self{
            DealDistribution::Biased(distr) => {
                distr.as_ref().sample(rng)
            },
            DealDistribution::Fair => {
                distribute_standard_deck_on_4(rng)

            }
        }
    }
}

