#[derive(thiserror::Error, Debug, Clone)]
pub enum GenError{
    #[error("Converting ForceDeclarer::No to Side")]
    ConvForceDeclarerNoToSide,
    #[error("Lower bound ({lower:?})for contract value is grater than upper ({upper:?})")]
    LowerBoundOverUpper{
        lower: u8,
        upper: u8
    },

}