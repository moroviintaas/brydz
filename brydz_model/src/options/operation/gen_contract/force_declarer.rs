use clap::ValueEnum;
use brydz_core::player::side::Side;
use crate::error::BrydzSimError;
use crate::error::GenError::ConvForceDeclarerNoToSide;

#[derive(ValueEnum, Clone, Debug)]
pub enum ForceDeclarer{
    DontForce,
    ForceNorth,
    ForceEast,
    ForceSouth,
    ForceWest
}

impl TryFrom<&ForceDeclarer> for Side{
    type Error = BrydzSimError;

    fn try_from(value: &ForceDeclarer) -> Result<Self, Self::Error> {
        match value{
            ForceDeclarer::DontForce => Err(BrydzSimError::Gen(ConvForceDeclarerNoToSide)),
            ForceDeclarer::ForceNorth => Ok(Side::North),
            ForceDeclarer::ForceEast => Ok(Side::East),
            ForceDeclarer::ForceSouth => Ok(Side::South),
            ForceDeclarer::ForceWest => Ok(Side::West)
        }
    }
}