use clap::ValueEnum;


#[derive(ValueEnum)]
#[derive(Clone, Debug)]
pub enum DealMethod {
    Fair,
    Biased
}
/*
impl Display for DealMethod{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{
            Self::Fair => "fair"
        })
    }
}*/