use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExploreOutput{
    MinusInfinity,
    Number(u8),
    Infinity
}


impl From<u8> for ExploreOutput{
    fn from(n: u8) -> Self {
        Self::Number(n)
    }
}


impl Debug for ExploreOutput{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MinusInfinity => write!(f, "MinusInfinity"),
            Self::Number(arg0) => write!(f, "{arg0:?}"),
            Self::Infinity => write!(f, "Infinity"),
        }
    }
}

impl Display for ExploreOutput{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            ExploreOutput::MinusInfinity => write!(f, "-∞"),
            ExploreOutput::Number(n) => write!(f, "{n}"),
            ExploreOutput::Infinity => write!(f, "∞"),
        }
    }
}

impl PartialOrd for ExploreOutput{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ExploreOutput{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self{
            ExploreOutput::MinusInfinity => match other {
                ExploreOutput::MinusInfinity => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Less,
            },
            ExploreOutput::Number(n) => match other {
                ExploreOutput::MinusInfinity => std::cmp::Ordering::Greater,
                ExploreOutput::Number(o) => n.cmp(o),
                ExploreOutput::Infinity => std::cmp::Ordering::Less,
            },
            ExploreOutput::Infinity => match other{
                ExploreOutput::Infinity => std::cmp::Ordering::Equal,
                _ => std::cmp::Ordering::Greater

            }

        }
    }
}