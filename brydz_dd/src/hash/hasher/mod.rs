mod nohash;
//mod hash_28_a;
pub mod hash24;

use std::fmt::Debug;
//pub use nohash::*;
//pub use hash_28_a::*;
use crate::hash::Label;

use crate::node::TrickNode;
pub trait NodeHasher {
    type HashType: TryInto<usize> + Debug;
    type LabelType: Label;


    fn hash(node: &TrickNode) -> Self::HashType;
    fn label(node: &TrickNode) -> Self::LabelType;
    fn hash_and_label(node: &TrickNode) -> (Self::HashType, Self::LabelType);
    //fn reconstruct(&self, hash: &Self::HashType, label: &Self::LabelType, side: Side) -> TrickNode;
    fn count_cards(hash: &Self::HashType, label: &Self::LabelType) -> u32;

}