use crate::hash::NodeHasher;
use crate::node::TrickNode;

pub struct Hash28A{

}

impl NodeHasher for Hash28A{
    type HashType = usize;
    type LabelType = [u8;3];

    fn hash(&self, node: &TrickNode) -> Self::HashType {
        let cards_still_in_game = node.flatten_hands();

        let high7cards = ((cards_still_in_game >> SHIFT_HIGH_CLUBS) & MASK_HIGH_CLUBS) ^
            ((cards_still_in_game >> SHIFT_HIGH_DIAMONDS) & MASK_HIGH_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_HIGH_HEARTS) & MASK_HIGH_HEARTS) ^
            ((cards_still_in_game >> SHIFT_HIGH_SPADES) & MASK_HIGH_SPADES);

        let lower6cards = ((cards_still_in_game >> SHIFT_LOWER_CLUBS) & MASK_LOWER_CLUBS) ^
            ((cards_still_in_game >> SHIFT_LOWER_DIAMONDS) & MASK_LOWER_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_LOWER_HEARTS) & MASK_LOWER_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_SPADES) & MASK_LOWER_SPADES);

        (high7cards ^ (lower6cards<<3)) as usize
    }


    fn label(&self, node: &TrickNode) -> Self::LabelType {
        let cards_still_in_game = node.flatten_hands();
        let lower6cards = ((cards_still_in_game >> SHIFT_LOWER_CLUBS) & MASK_LOWER_CLUBS) ^
            ((cards_still_in_game >> SHIFT_LOWER_DIAMONDS) & MASK_LOWER_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_LOWER_HEARTS) & MASK_LOWER_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_SPADES) & MASK_LOWER_SPADES);

        let mut result = [0u8;3];
        result[0] = (lower6cards & 0xff) as u8; //little endian
        result[1] = ((lower6cards >> 8) & 0xff) as u8;
        result[2] = ((lower6cards >> 16) & 0xff) as u8;

        result
    }

    fn hash_and_label(&self, node: &TrickNode) -> (Self::HashType, Self::LabelType) {
        let cards_still_in_game = node.flatten_hands();

        let high7cards = ((cards_still_in_game >> SHIFT_HIGH_CLUBS) & MASK_HIGH_CLUBS) ^
            ((cards_still_in_game >> SHIFT_HIGH_DIAMONDS) & MASK_HIGH_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_HIGH_HEARTS) & MASK_HIGH_HEARTS) ^
            ((cards_still_in_game >> SHIFT_HIGH_SPADES) & MASK_HIGH_SPADES);

        let lower6cards = ((cards_still_in_game >> SHIFT_LOWER_CLUBS) & MASK_LOWER_CLUBS) ^
            ((cards_still_in_game >> SHIFT_LOWER_DIAMONDS) & MASK_LOWER_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_LOWER_HEARTS) & MASK_LOWER_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_SPADES) & MASK_LOWER_SPADES);

        let mut label = [0u8;3];
        label[0] = (lower6cards & 0xff) as u8; //little endian
        label[1] = ((lower6cards >> 8) & 0xff) as u8;
        label[2] = ((lower6cards >> 16) & 0xff) as u8;



        ((high7cards ^ (lower6cards<<3)) as usize, label)
    }
}

const MASK_HIGH_CLUBS: u64 =            0x7f;
const MASK_HIGH_DIAMONDS: u64 =       0x3f80;
const MASK_HIGH_HEARTS:u64 =        0x1fc000;
const MASK_HIGH_SPADES: u64 =     0x0fe00000;

const MASK_LOWER_CLUBS: u64 =           0x3f;
const MASK_LOWER_DIAMONDS: u64 =      0x0fc0;
const MASK_LOWER_HEARTS: u64 =      0x03f000;
const MASK_LOWER_SPADES: u64 =    0x00fc0000;


const SHIFT_HIGH_CLUBS: usize = 6;
const SHIFT_HIGH_DIAMONDS: usize = 12; // 13 to 0, but 6 to be left for clubs and 7 to cut lower
const SHIFT_HIGH_HEARTS: usize = 18 ;
const SHIFT_HIGH_SPADES: usize = 24;

const SHIFT_LOWER_CLUBS: usize = 0;
const SHIFT_LOWER_DIAMONDS: usize = 7;
const SHIFT_LOWER_HEARTS: usize = 14;
const SHIFT_LOWER_SPADES: usize = 21;


#[cfg(test)]
mod tests{
    use brydz_core::deal::fair_bridge_deal;
    use brydz_core::karty::hand::{HandTrait, StackHand};
    use brydz_core::player::side::Side::North;
    use crate::hash::{Hash28A, NodeHasher};
    //use crate::hash::{PartialHash, StateHash24, StateHash24EntryDistinguish};
    use crate::node::TrickNode;


    #[test]
    fn check_full_hands_hash(){
        let mut trick_node = TrickNode::new(fair_bridge_deal::<StackHand>(), North);
        let node_hasher = Hash28A{};

        assert_eq!(node_hasher.hash_and_label(&trick_node), (0x08000007, [0xff, 0xff, 0xff]));



    }


}