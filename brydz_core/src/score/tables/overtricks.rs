use karty::suits::{Suit};
use crate::bidding::Doubling;
use crate::contract::ContractParametersGen;
use crate::cards::trump::TrumpGen;
use crate::score::calculation::ScoreIngredient;

pub struct PointsOverTrick{
    pub not_doubled_clubs: i32,
    pub not_doubled_diamonds: i32,
    pub not_doubled_hearts: i32,
    pub not_doubled_spades: i32,
    pub not_doubled_nt: i32,
    pub doubled_not_vulnerable: i32,
    pub doubled_vulnerable: i32,
    pub redoubled_not_vulnerable: i32,
    pub redoubled_vulnerable: i32,
}

impl PointsOverTrick{
    /// Calculates points for taken overtricks.
    /// # Examples:
    /// ```
    /// use brydz_core::contract::ContractParametersGen;
    /// use brydz_core::player::side::Side::North;
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::bidding::Doubling::{Redouble, Double};
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::cards::trump::TrumpGen::NoTrump;
    /// use brydz_core::score::tables::{POINTS_OVER_TRICK};
    /// use karty::suits::Suit::Hearts;
    /// let contract = ContractParametersGen::new(North, Bid::init(TrumpGen::Colored(Hearts), 2).unwrap(),);
    /// let points_table = POINTS_OVER_TRICK;
    /// assert_eq!(points_table.points(&contract, 8 ,false), 0);
    /// assert_eq!(points_table.points(&contract, 10 ,false), 60);
    /// let contract = ContractParametersGen::new_d(North, Bid::init(TrumpGen::Colored(Hearts), 2).unwrap(), Double);
    /// assert_eq!(points_table.points(&contract, 7 ,false), 0);
    /// assert_eq!(points_table.points(&contract, 10 ,false), 200);
    /// assert_eq!(points_table.points(&contract, 11 ,true), 600);
    /// let contract = ContractParametersGen::new_d(North, Bid::init(TrumpGen::Colored(Hearts), 2).unwrap(), Redouble);
    /// assert_eq!(points_table.points(&contract, 12 ,true), 1600);
    ///
    /// ```
    pub fn points(&self, contract: &ContractParametersGen<Suit>, taken: u8, vulnerable: bool) -> i32 {

        let number_of_overtricks = taken.saturating_sub(contract.bid().number_normalised());
        (number_of_overtricks as i32) * match contract.doubling() {
            Doubling::None => match contract.bid().trump() {
                TrumpGen::Colored(Suit::Clubs) => self.not_doubled_clubs,
                TrumpGen::Colored(Suit::Diamonds) => self.not_doubled_diamonds,
                TrumpGen::Colored(Suit::Hearts) => self.not_doubled_hearts,
                TrumpGen::Colored(Suit::Spades) => self.not_doubled_spades,
                TrumpGen::NoTrump => self.not_doubled_nt
            }
            Doubling::Double => match vulnerable {
                true => self.doubled_vulnerable,
                false => self.doubled_not_vulnerable
            }
            Doubling::Redouble => match vulnerable {
                true => self.redoubled_vulnerable,
                false => self.redoubled_not_vulnerable
            }
        }
    }
}

impl ScoreIngredient<Suit> for PointsOverTrick{
    fn calculate(&self, contract: &ContractParametersGen<Suit>, taken: u8, vulnerability: bool) -> i32 {
        self.points(contract, taken, vulnerability)
    }
}
pub const POINTS_OVER_TRICK: PointsOverTrick = PointsOverTrick{
    not_doubled_clubs: 20,
    not_doubled_diamonds: 20,
    not_doubled_hearts: 30,
    not_doubled_spades: 30,
    not_doubled_nt: 30,
    doubled_not_vulnerable: 100,
    doubled_vulnerable: 200,
    redoubled_not_vulnerable: 200,
    redoubled_vulnerable: 400
};