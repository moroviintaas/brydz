use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use smallvec::SmallVec;
use brydz_core::karty::cards::Card;
use brydz_core::meta::{CONTRACT_ACTION_SPACE_BOUND, TOTAL_TRICKS};
use brydz_core::player::axis::Axis;
use brydz_core::player::side::Side;
use crate::actions::CardPack;
use crate::explore::ExploreOutput;
use crate::explore::ExploreOutput::{Infinity, MinusInfinity};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CardPackResult{
    card_pack: CardPack,
    north_south_value: ExploreOutput
}

impl PartialOrd<Self> for CardPackResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardPackResult{
    fn cmp(&self, other: &Self) -> Ordering {
        self.north_south_value.cmp(&other.north_south_value)
    }
}
impl CardPackResult{
    pub fn new(card_pack: CardPack, north_south_value: ExploreOutput) -> Self{
        Self{ card_pack, north_south_value}
    }/*
    pub fn trick_reward(&self) -> u8{
        match self.side.axis(){
            Axis::NorthSouth => self.north_south_value,
            Axis::EastWest => TOTAL_TRICKS - self.north_south_value
        }
    }*/
    pub fn card_pack(&self) -> &CardPack{
        &self.card_pack
    }
    pub fn value(&self, side: Side) -> ExploreOutput{
        match side.axis(){
            Axis::NorthSouth => self.north_south_value,
            Axis::EastWest => match self.north_south_value{
                MinusInfinity => Infinity,
                ExploreOutput::Number(n) => ExploreOutput::Number(TOTAL_TRICKS - n),
                Infinity => MinusInfinity
            }
        }
    }
    pub fn raw_value(&self) -> ExploreOutput{
        self.north_south_value
    }
}

impl Display for CardPackResult{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match f.alternate(){
            true => write!(f, "{:#} -> {:?}", self.card_pack, self.north_south_value),
            false => write!(f, "{} -> {:?}", self.card_pack, self.north_south_value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackStep {
    results: SmallVec<[CardPackResult; CONTRACT_ACTION_SPACE_BOUND]>,
    side: Side,
    hint: Option<Card>

}

impl TrackStep{
    pub fn new(side: Side) -> Self{
        Self{results: SmallVec::new(), side, hint: None}
    }

    pub fn results(&self) -> &SmallVec<[CardPackResult; CONTRACT_ACTION_SPACE_BOUND]>{
        &self.results
    }
    pub fn side(&self) -> Side{
        self.side
    }
    /*
    pub fn best_card(&self) -> Option<Card>{
        match self.side.axis(){
            Axis::NorthSouth => {
                &self.results.into_iter().max()
            }
            Axis::EastWest => {&self.results.into_iter().min()}
        }.map(|cpr| cpr.card_pack().lowest_card())
    }*/
/*
    pub fn best(&self) -> Option<CardPackResult>{
        match self.side.axis(){
            Axis::NorthSouth => {
                &self.results.into_iter().max()
            }
            Axis::EastWest => {&self.results.into_iter().min()}
        }.to_owned()
    }

 */
    pub fn hint(&self) -> Option<&Card>{
        self.hint.as_ref()
    }
    pub fn push(&mut self, result: CardPackResult){
        self.results.push(result)
    }
    pub fn push_and_hint(&mut self, result:CardPackResult){
        self.hint = Some(result.card_pack().lowest_card());
        self.results.push(result);

    }
}

impl Display for TrackStep{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}):", self.side)?;
        match self.hint{
            Some(h) =>write!(f, "\t{h:#}"),
            None => write!(f, "")
        }?;
        write!(f, " [")?;
        let  group_results = self.results();
        for j in 0..group_results.len().saturating_sub(1){
            write!(f, "{:#} -> {}; ", group_results[j].card_pack(), group_results[j].value(self.side))?;

        }
        if let Some(last) = group_results.last(){
            write!(f, "{:#} -> {} ", last.card_pack(), last.value(self.side))?;
        }
        write!(f, "]")



    }
}