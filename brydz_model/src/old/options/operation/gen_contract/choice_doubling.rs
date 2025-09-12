use clap::ValueEnum;
#[derive(ValueEnum, Clone, Debug)]
pub enum ChoiceDoubling{
    Any,
    No,
    Double,
    Redouble
}