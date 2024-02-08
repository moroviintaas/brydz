use karty::hand::{CardSet};


pub trait HandInfo{
    //fn side(&self) -> Side;
    fn own_cards(&self) -> CardSet;
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone)]
pub struct HandInfoSimple {
    //side: Side,
    own_cards: CardSet,
}

impl HandInfoSimple{
    //pub fn new(side: Side, cards: CardSet) -> Self{
    pub fn new(cards: CardSet) -> Self{
        Self{own_cards: cards}
    }
}

impl HandInfo for HandInfoSimple{
    /*fn side(&self) -> Side {
        self.side
    }

     */

    fn own_cards(&self) -> CardSet {
        self.own_cards
    }
}






/*
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CardDistribution {

    side_probabilities: SideMap<FuzzyCardSet>
    //cards_probs: Vec<SideMap<f64>>
}

impl Default for CardDistribution{
    fn default() -> Self {
        Self{side_probabilities: SideMap::new_symmetric(
            FuzzyCardSet::new_from_f32_derive_sum(SuitMap::new_symmetric([0.25;13])).unwrap()) }
    }
}

impl Index<Side> for CardDistribution {
    type Output = FuzzyCardSet;

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index]
    }
}

 */
/*
impl Index<(Side, usize)> for CardDistribution {
    type Output = f32;

    fn index(&self, index: (Side, usize)) -> &Self::Output {
        &self.side_probabilities[&index.0].probabilities()[index.1]
    }
}*/
