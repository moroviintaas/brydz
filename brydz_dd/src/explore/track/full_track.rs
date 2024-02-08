use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use crate::explore::ExploreOutput;
use crate::explore::track::TrackStep;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameTrack{
    steps: Vec<TrackStep>,
    leaf_value: ExploreOutput
}

impl PartialOrd<Self> for GameTrack {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GameTrack{
    fn cmp(&self, other: &Self) -> Ordering {
        self.leaf_value.cmp(&other.leaf_value)
    }
}

impl GameTrack{
    pub fn new(leaf_value: ExploreOutput) -> Self{
        Self{steps: Vec::new(), leaf_value}
    }
    pub fn push(&mut self, step: TrackStep){
        self.steps.push(step)
    }
    pub fn steps(&self) -> &Vec<TrackStep>{
        &self.steps
    }
    pub fn leaf_value(&self) -> ExploreOutput{
        self.leaf_value
    }
    pub fn unchecked_leaf_value_u8(&self) -> u8{
        match self.leaf_value{
            ExploreOutput::Number(n) => n,
            inf => panic!("Converting from ExploreOutput to u8, when {inf:?}")
        }
    }
}

impl Display for GameTrack{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        //let mut line_num = 1usize;
        for i in 0..self.steps.len(){
            let side = self.steps[self.steps.len()-i-1].side();
            write!(f, "{i:8}. ({side:?}): "  )?;
            match self.steps[self.steps.len()-i-1].hint() {
                Some(h) =>write!(f, "\t{h:#} "),
                None => write!(f, "")
            }?;
            write!(f, "[")?;
            let group_results = self.steps[self.steps.len()-i-1].results();
            for j in 0..group_results.len().saturating_sub(1){
                write!(f, "{:#} -> {}; ", group_results[j].card_pack(), group_results[j].value(side))?;

            }
            if let Some(last) = group_results.last(){
                write!(f, "{:#} -> {} ", last.card_pack(), last.value(side))?;
            }
            writeln!(f, "]")?;

        }
        write!(f, "")

    }
}