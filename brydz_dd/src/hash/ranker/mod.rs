mod more_cards_ranker;

use crate::hash::{ NodeHasher};
pub use more_cards_ranker::*;

pub trait HashRanker<NH: NodeHasher>{
    /*fn rank_compare(&self, hash: &NH::HashType, label_1: &NH::LabelType,
                    label_2: &NH::LabelType) -> Ordering;*/

    fn assess(hash: &NH::HashType, label: &NH::LabelType) -> i64;
}