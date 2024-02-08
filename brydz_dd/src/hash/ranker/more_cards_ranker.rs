
use crate::hash::NodeHasher;
use crate::hash::ranker::HashRanker;

#[derive(Clone, Copy, Debug)]
pub struct MoreCardsRanker{

}

impl<Hasher: NodeHasher> HashRanker<Hasher> for MoreCardsRanker{
    fn assess(hash: &<Hasher as NodeHasher>::HashType, label: &<Hasher as NodeHasher>::LabelType) -> i64 {
        Hasher::count_cards(hash, label) as i64
    }
    /*fn rank_compare(&self, hash: &<Hasher as NodeHasher>::HashType, label_1: &<Hasher as NodeHasher>::LabelType, label_2: &<Hasher as NodeHasher>::LabelType) -> Ordering {
        Hasher::count_cards(hash, label_1).cmp(&Hasher::count_cards(hash, label_2))
    }*/
}