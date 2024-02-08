use karty::suits::{Suit};
use crate::contract::ContractParametersGen;
use crate::meta::QUARTER_SIZE;
use crate::score::calculation::ScoreIngredient;

pub struct PointsSlam{
    vulnerable_great_slam: i32,
    vulnerable_small_slam: i32,
    not_vulnerable_great_slam: i32,
    not_vulnerable_small_slam: i32,
}

pub const POINTS_SLAM: PointsSlam = PointsSlam{
    vulnerable_great_slam: 1500,
    vulnerable_small_slam: 750,
    not_vulnerable_great_slam: 1000,
    not_vulnerable_small_slam: 500
};

impl PointsSlam{
    ///
    /// # Args:
    /// `taken: u8` - number of tricks taken (in total)
    /// # Examples:
    /// ```
    /// use brydz_core::contract::ContractParametersGen;
    /// use brydz_core::player::side::Side::North;
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::bidding::Doubling::Redouble;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::cards::trump::TrumpGen::NoTrump;
    /// use brydz_core::score::tables::POINTS_SLAM;
    /// use karty::suits::Suit::Hearts;
    /// let contract = ContractParametersGen::new(North, Bid::init(TrumpGen::Colored(Hearts), 2).unwrap(),);
    /// let points_table = POINTS_SLAM;
    /// assert_eq!(points_table.points(&contract, 13, false), 0);
    /// let contract = ContractParametersGen::new(North, Bid::init(TrumpGen::Colored(Hearts), 6).unwrap(),);
    /// assert_eq!(points_table.points(&contract, 12, false), 500);
    /// assert_eq!(points_table.points(&contract, 12, true), 750);
    /// assert_eq!(points_table.points(&contract, 13, true), 750);
    /// let contract = ContractParametersGen::new_d(North, Bid::init(NoTrump, 7).unwrap(), Redouble);
    /// assert_eq!(points_table.points(&contract, 12, false), 0 );
    /// assert_eq!(points_table.points(&contract, 13, true), 1500 );
    /// assert_eq!(points_table.points(&contract, 13, false), 1000 );
    ///
    /// ```
    pub fn points(&self, contract: &ContractParametersGen<Suit>, taken: u8, vulnerable: bool) -> i32{
        let declared = contract.bid().number_normalised() as usize;
        match declared{
            n if n == QUARTER_SIZE => {
                if taken >= declared as u8{
                    match vulnerable{
                        true => self.vulnerable_great_slam,
                        false => self.not_vulnerable_great_slam
                    }
                }
                else{
                    0
                }
            },
            n1 if n1 == (QUARTER_SIZE - 1) => {
                if taken >= declared as u8{
                    match vulnerable{
                        true => self.vulnerable_small_slam,
                        false => self.not_vulnerable_small_slam
                    }
                }
                else{
                    0
                }
            }
            _ => {0}
        }
    }
}
impl ScoreIngredient<Suit> for PointsSlam{
    fn calculate(&self, contract: &ContractParametersGen<Suit>, taken: u8, vulnerability: bool) -> i32 {
        self.points(contract, taken, vulnerability)
    }
}