
use arrayvec::ArrayVec;
use itertools::Itertools;
use std::ops::Index;
use rand::rng;
use rand::seq::SliceRandom;
use karty::cards::Card2SGen;
use karty::figures::{FIGURES, Figure};
use karty::suits::{SUITS, Suit};
use karty::cards::DECK_SIZE;




#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Deck{
    cards: ArrayVec<Card2SGen<Figure, Suit>, DECK_SIZE>
    //cards: ArrayVec<ArrayVec<Card<F,S>, {F::NUMBER_OF_FIGURES}>, S::NUMBER_OF_SUITS>
}

impl Deck{



    pub fn new_sorted_by_suits() -> Self{
        let v: ArrayVec<Card2SGen<Figure, Suit>, DECK_SIZE> = ArrayVec::from_iter(SUITS.into_iter().rev()
            .cartesian_product(FIGURES.into_iter().rev())
            .map(|(s,f)| Card2SGen::new(f, s)));

        Self{cards: v}
    }
    pub fn new_sorted_by_figures() -> Self{
        let v: ArrayVec<Card2SGen<Figure, Suit>, DECK_SIZE> = ArrayVec::from_iter(FIGURES.into_iter().rev()
            .cartesian_product(SUITS.into_iter().rev())
            .map(|(s,f)| Card2SGen::new(s, f)));

        Self{cards: v}
    }


    pub fn at(&self, index: usize) -> &Card2SGen<Figure, Suit> {
        &self.cards[index]
    }

    pub fn shuffle(&mut self){
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn cards(&self) -> &ArrayVec<Card2SGen<Figure, Suit>, DECK_SIZE>{
        &self.cards
    }

}

impl IntoIterator for Deck{
    type Item = Card2SGen<Figure, Suit>;
    type IntoIter = arrayvec::IntoIter<Self::Item, DECK_SIZE>;

    fn into_iter(self) -> Self::IntoIter {
        self.cards.into_iter()
    }
}

impl Index<usize> for Deck{
    type Output = Card2SGen<Figure, Suit>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cards[index]
    }
}

