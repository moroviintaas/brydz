use serde::{Deserialize, Serialize};
use crate::player::axis::Axis;


#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct ScoreTable {
    score_ns_above_line: i64,
    score_ns_below_line: i64,
    score_ew_above_line: i64,
    score_ew_below_line: i64
}

impl ScoreTable {
    pub fn new(ns_above: i64, ns_below:i64, ew_above: i64, ew_below: i64) -> Self{
        Self{score_ns_below_line: ns_below, score_ns_above_line: ns_above,
        score_ew_above_line: ew_above, score_ew_below_line: ew_below}
    }

    pub fn above(&self, axis: &Axis) -> i64{
        match axis{
            Axis::NorthSouth => self.score_ns_above_line,
            Axis::EastWest => self.score_ew_above_line
        }
    }

    pub fn below(&self, axis: &Axis) -> i64{
        match axis{
            Axis::NorthSouth => self.score_ns_below_line,
            Axis::EastWest => self.score_ew_below_line
        }
    }

    pub fn add_above(&mut self, axis: &Axis, score: i64){
        match axis{
            Axis::NorthSouth => { self.score_ns_above_line += score},
            Axis::EastWest => { self.score_ew_above_line += score}
        }
    }

    pub fn add_below(&mut self, axis: &Axis, score: i64){
        match axis{
            Axis::NorthSouth => { self.score_ns_below_line += score},
            Axis::EastWest => { self.score_ew_below_line += score}
        }
    }
}

impl Default for ScoreTable {
    fn default() -> Self {
        ScoreTable::new(0, 0, 0, 0)
    }
}


