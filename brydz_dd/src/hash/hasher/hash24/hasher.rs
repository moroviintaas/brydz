use crate::hash::NodeHasher;
use crate::node::TrickNode;

pub struct Hash24<const SHIFT_LOWER: i32>{

}

impl<const SHIFT_LOWER: i32> NodeHasher for Hash24<SHIFT_LOWER>{
    type HashType = u32;
    type LabelType = u32;

    fn hash(node: &TrickNode) -> Self::HashType {
        let cards_still_in_game = node.flatten_hands();

        let high6cards = ((cards_still_in_game >> SHIFT_HIGH_6_CLUBS) & MASK_6_CLUBS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_DIAMONDS) & MASK_6_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_HEARTS) & MASK_6_HEARTS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_SPADES) & MASK_6_SPADES);

        let lower6cards = ((cards_still_in_game >> SHIFT_3_TO_8_CLUBS) & MASK_6_CLUBS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_DIAMONDS) & MASK_6_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_HEARTS) & MASK_6_HEARTS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_SPADES) & MASK_6_SPADES);


        let lower_shift = SHIFT_LOWER;
        let lower_shifted = lower6cards<<lower_shift;
        let lower_part  = lower_shifted & 0xffffff;
        let cycled_part = lower_shifted >> 24;



        (high6cards ^ lower_part ^ cycled_part) as u32
    }


    fn label(node: &TrickNode) -> Self::LabelType {
        let cards_still_in_game = node.flatten_hands();
        let lowest7cards = ((cards_still_in_game >> SHIFT_LOWER_7_CLUBS) & MASK_7_CLUBS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_DIAMONDS) & MASK_7_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_HEARTS) & MASK_7_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_SPADES) & MASK_7_SPADES);

        /*let mut result = [0u8;4];
        result[0] = (lower7cards & 0xff) as u8; //little endian
        result[1] = ((lower7cards >> 8) & 0xff) as u8;
        result[2] = ((lower7cards >> 16) & 0xff) as u8;
        result[3] = ((lower7cards >> 24) & 0xff) as u8;

        result
        */
        lowest7cards as u32
    }

    fn hash_and_label(node: &TrickNode) -> (Self::HashType, Self::LabelType) {
        let cards_still_in_game = node.flatten_hands();

        let high6cards = ((cards_still_in_game >> SHIFT_HIGH_6_CLUBS) & MASK_6_CLUBS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_DIAMONDS) & MASK_6_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_HEARTS) & MASK_6_HEARTS) ^
            ((cards_still_in_game >> SHIFT_HIGH_6_SPADES) & MASK_6_SPADES);

        let lower6cards = ((cards_still_in_game >> SHIFT_3_TO_8_CLUBS) & MASK_6_CLUBS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_DIAMONDS) & MASK_6_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_HEARTS) & MASK_6_HEARTS) ^
            ((cards_still_in_game >> SHIFT_3_TO_8_SPADES) & MASK_6_SPADES);

        let lowest7cards = ((cards_still_in_game >> SHIFT_LOWER_7_CLUBS) & MASK_7_CLUBS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_DIAMONDS) & MASK_7_DIAMONDS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_HEARTS) & MASK_7_HEARTS) ^
            ((cards_still_in_game >> SHIFT_LOWER_7_SPADES) & MASK_7_SPADES);


        let lower_shift = SHIFT_LOWER;
        let lower_shifted = lower6cards<<lower_shift;
        let lower_part  = lower_shifted & 0xffffff;
        let cycled_part = lower_shifted >> 24;

        ((high6cards ^ lower_part ^ cycled_part) as u32, lowest7cards as u32)
    }

    fn count_cards(hash: &Self::HashType, label: &Self::LabelType) -> u32 {
        let l = *label as u64;
        let clubs_3_8 = (l >> 1) &      MASK_6_CLUBS;
        let diamonds_3_8 = (l >> 2) & MASK_6_DIAMONDS;
        let hearts_3_8 = (l >> 3) & MASK_6_HEARTS;
        let spades_3_8 =(l >> 4) &  MASK_6_SPADES;

        let cards_3_8 = clubs_3_8 ^ diamonds_3_8 ^ hearts_3_8 ^ spades_3_8;
        let lower_shift = SHIFT_LOWER;
        let lower_shifted = cards_3_8 << lower_shift;
        let cycled_part = lower_shifted >> 24;
        let lower_part  = lower_shifted & 0xffffff;
        let higher6cards = hash  ^ cycled_part as u32 ^ lower_part as u32;

        higher6cards.count_ones() + label.count_ones()
    }
    /*
        fn reconstruct(&self, hash: &Self::HashType, label: &Self::LabelType, side: Side) -> TrickNode {
            let l = *label as u64;
            let clubs_3_8 = (l >> 1) &      MASK_6_CLUBS;
            let diamonds_3_8 = (l >> 2) & MASK_6_DIAMONDS;
            let hearts_3_8 = (l >> 3) & MASK_6_HEARTS;
            let spades_3_8 =(l >> 4) &  MASK_6_SPADES;

            let cards_3_8 = clubs_3_8 ^ diamonds_3_8 ^ hearts_3_8 ^ spades_3_8;
            let lower_shift = SHIFT_LOWER;
            let lower_shifted = cards_3_8 << lower_shift;
            let cycled_part = lower_shifted >> 24;
            let lower_part  = lower_shifted & 0xffffff;
            let higher6cards = (hash as u64) ^ cycled_part ^ lower_part;

            let higher_6_clubs = (higher6cards & MASK_6_CLUBS) << SHIFT_HIGH_6_CLUBS;
            let higher_6_diamonds = (higher6cards & MASK_6_DIAMONDS) << SHIFT_HIGH_6_DIAMONDS;
            let higher_6_hearts = (higher6cards & MASK_6_HEARTS) << SHIFT_HIGH_6_HEARTS;
            let higher_6_spades = (higher6cards & MASK_6_SPADES) << SHIFT_HIGH_6_SPADES;


            todo!()
        }

     */
}



const MASK_6_CLUBS: u64 =            0x3f;
const MASK_6_DIAMONDS: u64 =       0x0fc0;
const MASK_6_HEARTS:u64 =        0x03f000;
const MASK_6_SPADES: u64 =       0xfc0000;

const MASK_7_CLUBS: u64 =           0x7f;
const MASK_7_DIAMONDS: u64 =      0x3f80;
const MASK_7_HEARTS: u64 =      0x1fc000;
const MASK_7_SPADES: u64 =    0x0fe00000;




const SHIFT_HIGH_6_CLUBS: usize = 7;
const SHIFT_HIGH_6_DIAMONDS: usize = 14; // 13 to 0, but 6 to be left for clubs and 7 to cut lower
const SHIFT_HIGH_6_HEARTS: usize = 21 ;
const SHIFT_HIGH_6_SPADES: usize = 28;

const SHIFT_LOWER_7_CLUBS: usize = 0;
const SHIFT_LOWER_7_DIAMONDS: usize = 6; //13 - 7
const SHIFT_LOWER_7_HEARTS: usize = 12; // 26 - 14
const SHIFT_LOWER_7_SPADES: usize = 18;

const SHIFT_3_TO_8_CLUBS: usize = 1;
const SHIFT_3_TO_8_DIAMONDS: usize = 8;  //13 + 1 - 6
const SHIFT_3_TO_8_HEARTS: usize = 15; //26 + 1 - 12
const SHIFT_3_TO_8_SPADES: usize = 22; //39 + 1 - 18

/*
const SHIFT_LOWER_8_CLUBS: usize = 0;
const SHIFT_LOWER_8_DIAMONDS: usize = 5;
const SHIFT_LOWER_8_HEARTS: usize = 10;
const SHIFT_LOWER_8_SPADES: usize = 15;
*/

#[cfg(test)]
mod tests{
    use brydz_core::deal::fair_bridge_deal;
    use brydz_core::karty::hand::{CardSet};
    use brydz_core::player::side::Side::North;
    use crate::hash::{ NodeHasher};
    use crate::hash::hash24::Hash24;
    //use crate::hash::{PartialHash, StateHash24, StateHash24EntryDistinguish};
    use crate::node::TrickNode;


    #[test]
    fn check_full_hands_hash(){
        let mut trick_node = TrickNode::new(fair_bridge_deal::<CardSet>(), North);

        let (hash, label) = Hash24::<3>::hash_and_label(&trick_node);
        assert_eq!((hash, label), (0x000000, 0xfffffff));
        assert_eq!(Hash24::<3>::count_cards(&hash, &label), 52);

        let north_card_1 = trick_node.hands()[&North].into_iter().next().unwrap();
        trick_node.remove_card_current_side(&north_card_1).unwrap();
        let (hash, label) = Hash24::<3>::hash_and_label(&trick_node);
        assert_eq!(Hash24::<3>::count_cards(&hash, &label), 51);




    }


}