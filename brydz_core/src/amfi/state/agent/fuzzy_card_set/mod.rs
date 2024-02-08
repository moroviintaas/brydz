mod card_probability;
#[cfg(feature = "serde")]
mod serde;

//#[cfg(feature = "serde")]
//pub use self::serde::*;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "parse")]
pub use parse::*;


pub use card_probability::*;



use std::ops::Index;
use approx::{abs_diff_eq, abs_diff_ne};
use karty::cards::{Card, Card2SymTrait};
use karty::figures::Figure;
use karty::suits::{Suit, SuitMap, SUITS};
use karty::symbol::CardSymbol;
use crate::error::FuzzyCardSetErrorGen;

const FUZZY_CARD_SET_TOLERANCE: f32 = 0.01;
/*
pub(crate) fn is_uncertain(proba: f32) -> bool {
    proba > 0.0 && proba <1.0
}
*/

//#[derive(serde::Serialize, serde::Deserialize, Clone)]
/// Set of cards that can include card with certain probability
#[derive(Clone, Debug)]
pub struct FuzzyCardSet {
    //#[serde(with = "BigArray")]
    //probabilities: [f32; DECK_SIZE],
    probabilities: SuitMap<[FProbability; Figure::SYMBOL_SPACE]>,
    expected_card_number : u8,

}
impl FuzzyCardSet{

    #[allow(dead_code)]
    fn probability_mut(&mut self, card: &Card) -> &mut FProbability{
        &mut self.probabilities[card.suit()][card.figure().usize_index()]
    }

    pub fn empty() -> Self{
        Self{probabilities: SuitMap::new_from_f(|_| Default::default()), expected_card_number: 0}
    }


    pub fn new_from_f32_derive_sum(probabilities: SuitMap<[f32; Figure::SYMBOL_SPACE]>) -> Result<Self, FuzzyCardSetErrorGen<Card>>{
        let mut tmp = SuitMap::new_from_f(|_|[FProbability::Zero; Figure::SYMBOL_SPACE]);
        /*let sum = SUITS.iter().fold(0.0, |acc, x|{
            acc + probabilities[*x].iter().sum::<f32>()

        });*/

        let mut sum = 0.0;
        for s in SUITS{
            for i in 0..probabilities[&s].len(){
                sum += probabilities[&s][i];
                tmp[&s][i] = probabilities[&s][i].try_into()?;
            }
        }

        Ok(Self{probabilities: tmp, expected_card_number: sum.round() as u8})
    }

    fn new_derive_sum(probabilities: SuitMap<[FProbability; Figure::SYMBOL_SPACE]>) -> Result<Self, FuzzyCardSetErrorGen<Card>>{


        let mut global_sum = 0.0;
        for suit in SUITS{
            let mut suit_sum = 0.0f32;
            for i in 0..probabilities[suit].len(){
                match probabilities[suit][i]{
                    FProbability::One => {
                        suit_sum += 1.0;
                        //Ok(())
                    },
                    FProbability::Zero => {
                        //Ok(())
                    },
                    FProbability::Uncertain(p) => {
                        suit_sum += p;
                        //Ok(())
                    },
                    FProbability::Bad(p) => {
                        return Err(FuzzyCardSetErrorGen::BadProbability(p))

                    }
                }
            }
            global_sum += suit_sum
        }
        Ok(Self{probabilities, expected_card_number: global_sum.round() as u8})


    }


    pub fn assert_expected_card_num_with_epsilon(&mut self, expected: u8, epsilon:f32) -> Result<(), FuzzyCardSetErrorGen<Card>>{
        let sum = self.sum_probabilities();
        let expected_f32 = expected as f32;
        if abs_diff_eq!(sum, expected_f32, epsilon=epsilon){
            self.expected_card_number = expected_f32.round() as u8;
            Ok(())
        } else{
            Err(FuzzyCardSetErrorGen::BadProbabilitiesSum{expected: expected_f32, found: sum})
        }
    }

    /*
    fn naturalise_expected(&mut self){
        self.expected_card_number = self.expected_card_number.round();
    }

     */

    pub fn card_probability(&self, card: &Card) -> FProbability{
        self.probabilities[card.suit()][card.figure().usize_index()]
    }



    /// ```
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let cards_spades_bad =  [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35];
    ///
    /// assert!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades_bad, cards_hearts, cards_diamonds, cards_clubs), 13).is_err());
    /// assert_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap().expected_card_number(), 13);
    /// ```
    ///
    pub fn new_check_epsilon(probabilities: SuitMap<[f32; Figure::SYMBOL_SPACE]>, expected_card_number: u8) -> Result<Self, FuzzyCardSetErrorGen<Card>>{
        //let sum = probabilities.iter().sum();
        let mut tmp = SuitMap::new_from_f(|_|[FProbability::Zero; Figure::SYMBOL_SPACE]);
        /*let sum = SUITS.iter().fold(0.0, |acc, x|{
            acc + probabilities[*x].iter().sum::<f32>()

        });*/

        let mut sum = 0.0;
        for s in SUITS{
            for i in 0..probabilities[&s].len(){
                sum += probabilities[&s][i];
                tmp[&s][i] = probabilities[&s][i].try_into()?;
            }
        }

        let expected = expected_card_number as f32;
        if abs_diff_ne!(expected, sum, epsilon = FUZZY_CARD_SET_TOLERANCE){
            return Err(FuzzyCardSetErrorGen::BadProbabilitiesSum{expected: sum, found: expected})
        }
        Ok(Self{probabilities: tmp, expected_card_number })
    }

    pub fn probabilities(&self) -> &SuitMap<[FProbability; Figure::SYMBOL_SPACE]>{
        &self.probabilities
    }
    pub fn expected_card_number(&self) -> u8{
        self.expected_card_number
    }
    pub fn expected_card_number_f32(&self) -> f32{
        self.expected_card_number as f32
    }

    /// ```
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_ones_in_suit(&Clubs), 0);
    /// assert_eq!(fset.count_ones_in_suit(&Diamonds), 1);
    /// assert_eq!(fset.count_ones_in_suit(&Hearts), 0);
    /// assert_eq!(fset.count_ones_in_suit(&Spades), 0);
    /// ```
    ///
    pub fn count_ones_in_suit(&self, suit: &Suit) -> usize{
        /*self.probabilities[suit].iter().fold(0, |acc, x|{
            if *x>= 1.0{
                acc + 1
            } else {
                acc
            }
        })*/
        self.probabilities[suit].iter().filter(|&&x| x == FProbability::One).count()
    }

    pub fn count_ones(&self) -> usize{
        SUITS.iter().map(|s|self.count_ones_in_suit(s)).sum()
    }

    /// ```
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_zeros_in_suit(&Clubs), 0);
    /// assert_eq!(fset.count_zeros_in_suit(&Diamonds), 12);
    /// assert_eq!(fset.count_zeros_in_suit(&Hearts), 0);
    /// assert_eq!(fset.count_zeros_in_suit(&Spades), 1);
    /// ```
    ///
    pub fn count_zeros_in_suit(&self, suit: &Suit) -> usize{
        self.probabilities[suit].iter().filter(|&&x| x == FProbability::Zero).count()
    }

    pub fn count_zeros(&self) -> usize{
        SUITS.iter().map(|s|self.count_zeros_in_suit(s)).sum()
    }

    /// ```
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_eq!(fset.count_uncertain_in_suit(&Clubs), 13);
    /// assert_eq!(fset.count_uncertain_in_suit(&Diamonds), 0);
    /// assert_eq!(fset.count_uncertain_in_suit(&Hearts), 13);
    /// assert_eq!(fset.count_uncertain_in_suit(&Spades), 12);
    /// ```
    pub fn count_uncertain_in_suit(&self, suit: &Suit) -> usize{
        self.probabilities[suit].into_iter().filter(|x| x.is_uncertain() /*0.0 < x && x < 1.0*/).count()
    }

    pub fn count_uncertain(&self) -> usize{
        SUITS.iter().map(|s| self.count_uncertain_in_suit(s)).sum()

    }

    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_abs_diff_eq!(fset.sum_uncertain_in_suit(&Clubs), 3.9, epsilon=0.001);
    /// assert_abs_diff_eq!(fset.sum_uncertain_in_suit(&Diamonds), 0.0, epsilon=0.001);
    /// assert_abs_diff_eq!(fset.sum_uncertain_in_suit(&Hearts), 3.9, epsilon=0.001);
    /// assert_abs_diff_eq!(fset.sum_uncertain_in_suit(&Spades), 4.2, epsilon=0.001);
    /// ```
    pub fn sum_uncertain_in_suit(&self, suit: &Suit) -> f32{
        self.probabilities[suit].into_iter().filter_map(|fp| {
            match fp{
                FProbability::One => None,
                FProbability::Zero => None,
                FProbability::Uncertain(x) => Some(x),
                FProbability::Bad(_) => None
            }
        } /* x>0.0 && x<1.0*/).sum()
    }

    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_abs_diff_eq!(fset.sum_uncertain(), 12.0, epsilon=0.001);

    /// ```
    pub fn sum_uncertain(&self) -> f32{
        SUITS.iter().map(|s|self.sum_uncertain_in_suit(s)).sum()
    }

    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_abs_diff_eq!(fset.sum_probabilities_in_suit(&Clubs), 3.9, epsilon=0.001);
    /// assert_eq!(fset.sum_probabilities_in_suit(&Diamonds), 1.0);
    /// assert_abs_diff_eq!(fset.sum_probabilities_in_suit(&Spades), 4.2, epsilon=0.001);
    ///
    /// ```
    ///
    pub fn sum_probabilities_in_suit(&self, suit: &Suit) -> f32{
        self.probabilities[suit].iter().map(|&x| Into::<f32>::into(x)).sum()
    }

     /// ```
     /// use approx::assert_abs_diff_eq;
     /// use karty::suits::SuitMap;
     /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
     /// use brydz_core::amfi::state::FuzzyCardSet;
     /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
     /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
     /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
     /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
     /// assert_abs_diff_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap().sum_probabilities(), 13.0, epsilon=0.001);
     /// ```
    pub fn sum_probabilities(&self) -> f32{
        SUITS.iter().map(|s| self.sum_probabilities_in_suit(s)).sum()
    }


    pub fn repair_not_fixed(&mut self) -> Result<f32, FuzzyCardSetErrorGen<Card>>{
        let sum_uncertain = self.sum_uncertain();
        let cards_yet_to_take = self.expected_card_number- self.count_ones() as u8;
        let scale = (cards_yet_to_take as f32) / sum_uncertain;
        if cards_yet_to_take == self.count_uncertain() as u8{
            for s in SUITS{
                for i in 0..self.probabilities[&s].len(){
                    if let FProbability::Uncertain(_) = self.probabilities()[&s][i]{
                        self.probabilities[&s][i] = FProbability::One;
                    }
                }
            }
            return Ok(scale)
        }

        //println!("{}", scale);
        for s in SUITS{
            for i in 0..self.probabilities[&s].len(){
                if let FProbability::Uncertain(p ) = self.probabilities()[&s][i]{
                    if p == 0.0 || scale ==0.0 {
                        self.probabilities[&s][i] = FProbability::Zero;
                    } else {
                        self.probabilities[&s][i]  *= scale;
                    }

                } else if let FProbability::Bad(b ) = self.probabilities()[&s][i]{
                    //self.probabilities[&s][i]  *= scale
                    return  Err(FuzzyCardSetErrorGen::BadProbability(b))
                }

            }
        }
        Ok(scale)
    }

    /*
    fn downscale_uncertain_no_check(&mut self, scale: f32) -> Result<(), CardSetError>{
        if scale <= 0.0 || scale >= 1.0{
            return Err(CardSetErrorGen::ForbiddenDownscale(scale));
        }
        for s in SUITS{
            for i in 0..self.probabilities[&s].len(){

                if let FProbability::Uncertain(f) = self.probabilities[&s][i]{
                    self.probabilities[&s][i] *= scale;
                }
            }
        }


        Ok(())
    }

     */

    /// ```
    /// use approx::assert_abs_diff_eq;
    /// use brydz_core::amfi::state::FuzzyCardSet;
    /// use karty::cards::{TWO_DIAMONDS, TWO_HEARTS};
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// let mut fset = FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs), 13).unwrap();
    /// assert_abs_diff_eq!(fset.sum_probabilities(), 13.0, epsilon=0.001);
    /// let scale = fset.set_zero_card_probability(&TWO_HEARTS).unwrap();
    /// assert_abs_diff_eq!(scale, 0.9461, epsilon=0.01);
    /// assert_abs_diff_eq!(fset.sum_probabilities(), 12.0, epsilon=0.001);
    /// //assert_abs_diff_eq!(fset.expected_card_number_f32(), 12.0, epsilon=0.001);
    /// //assert_eq!(fset[&TWO_DIAMONDS], FProbability::One);
    /// //fset.repair_not_fixed();
    /// //assert_abs_diff_eq!(fset.sum_probabilities(), 12.0, epsilon=0.02);
    /// ```
    pub fn set_zero_card_probability(&mut self, card: &Card) -> Result<f32, FuzzyCardSetErrorGen<Card>>{
        match self[card]{
            FProbability::One => {
                self.probabilities[card.suit()][card.figure().usize_index()] = FProbability::Zero;
                self.expected_card_number -= 1;
                Ok(1.0)
            }
            FProbability::Zero => Ok(1.0),
            FProbability::Uncertain(deleted_card_proba_before) => {
                let remaining_probability_to_remove = 1.0 - deleted_card_proba_before;

                let uncertain_sum_before = self.sum_uncertain();
                let new_uncertain = uncertain_sum_before - remaining_probability_to_remove;

                self.probabilities[card.suit()][card.figure().usize_index()] = FProbability::Zero;

                let scale = new_uncertain / uncertain_sum_before;

                /*self.downscale_uncertain_no_check(scale).and_then(|_|{
                    self.expected_card_number -= 1.0;
                    Ok(scale)
                })*/
                self.expected_card_number -= 1;
                match self.repair_not_fixed(){
                    Ok(_) => Ok(scale),
                    Err(e) => {
                        self.expected_card_number += 1;
                        Err(e)
                    }
                }


                /*
                match self.downscale_uncertain_no_check(scale){
                    Ok(_) => {
                        self.expected_card_number -= 1.0;
                        Ok(())
                    }
                    Err(e) => {}
                }

                 */

            }
            FProbability::Bad(p) => Err(FuzzyCardSetErrorGen::BadProbability(p))
        }
        /*
        if self[card] <= 0.0{
            return Ok(1.0)
        }
        if self[card] > 1.0{
            return Err(CardSetError::ProbabilityOverOne(self[card]));
        }

        let deleted_card_proba = self[card];
        //we need to decrase probablityu sum by 1.0, by zeroising_field we decrase it by it, we must decrase 1.0 - this further
        let remaining_proba = 1.0 - self[card];

        let uncertain_sum_before = self.sum_uncertain();
        let new_uncertain = uncertain_sum_before - remaining_proba;

        self.probabilities[card.suit][card.figure.position()] = 0.0;

        //what if there are no uncertain left?
        //this need to be addressed, maybe enum (one, zero, uncertain(f32))?



        let scale = new_uncertain / uncertain_sum_before;
        self.expected_card_number -= 1.0;

        self.downscale_uncertain_no_check(scale)?;



        Ok(scale)
        */
    }

    #[allow(dead_code)]
    fn set_expected(&mut self, expected: u8){
        self.expected_card_number = expected
    }


}


impl Index<&Card> for FuzzyCardSet{
    type Output = FProbability;
    /// ```
    /// use karty::cards::TWO_DIAMONDS;
    /// use karty::suits::SuitMap;
    /// use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    /// use brydz_core::amfi::state::{FProbability, FuzzyCardSet};
    /// let cards_clubs =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_diamonds =    [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    /// let cards_hearts =      [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    /// let cards_spades =      [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];
    /// assert_eq!(FuzzyCardSet::new_check_epsilon(SuitMap::new(cards_spades, cards_hearts, cards_diamonds, cards_clubs),13 ).unwrap()[&TWO_DIAMONDS], FProbability::One);
    /// ```
    fn index(&self, index: &Card) -> &Self::Output {
        &self.probabilities[index.suit()][index.figure().usize_index()]
    }
}


