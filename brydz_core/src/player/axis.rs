#[derive(Debug, Eq, PartialEq,  Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Axis{
    NorthSouth,
    EastWest
}