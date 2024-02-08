
use karty::suits::Suit;
use crate::bidding::Doubling;
use crate::contract::ContractParametersGen;
use crate::cards::trump::TrumpGen;
use crate::meta::HALF_TRICKS;
use crate::score::calculation::ScoreIngredient;

pub struct PointsContractedTrick{
    pub clubs: i32,
    pub diamonds: i32,
    pub hearts: i32,
    pub spades: i32,
    pub nt_first: i32,
    pub nt_next: i32,
    pub doubling_multiplier: i32,
    pub redoubling_multiplier: i32

}
impl PointsContractedTrick{
    /// Calculates points for contracted tricks based on number of taken, does not count overtricks
    /// # Examples:
    /// ```
    /// use brydz_core::contract::ContractParametersGen;
    /// use brydz_core::player::side::Side::North;
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::bidding::Doubling::Redouble;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::cards::trump::TrumpGen::NoTrump;
    /// use brydz_core::score::tables::POINTS_CONTRACTED_TRICK;
    /// use karty::suits::Suit::Hearts;
    /// let contract = ContractParametersGen::new(North, Bid::init(TrumpGen::Colored(Hearts), 2).unwrap(),);
    /// let points_table = POINTS_CONTRACTED_TRICK;
    /// assert_eq!(points_table.points(&contract, 7), 0 );
    /// assert_eq!(points_table.points(&contract, 8), 60 );
    /// assert_eq!(points_table.points(&contract, 9), 60 );
    ///
    /// let contract = ContractParametersGen::new_d(North, Bid::init(NoTrump, 1).unwrap(), Redouble);
    /// assert_eq!(points_table.points(&contract, 6), 0 );
    /// assert_eq!(points_table.points(&contract, 7), 160 );
    /// assert_eq!(points_table.points(&contract, 8), 160 );
    ///
    /// ```

    pub fn points(&self, contract: &ContractParametersGen<Suit>, taken: u8) -> i32{
        let multiplier = match contract.doubling(){
            Doubling::None => 1,
            Doubling::Double => self.doubling_multiplier,
            Doubling::Redouble => self.redoubling_multiplier,
        };
        match contract.bid().trump(){
            TrumpGen::Colored(c) => {

                let number = if contract.bid().number_normalised() <= taken{
                    contract.bid().number()
                } else{
                    0
                };
                i32::from(number) * multiplier * match c{
                    Suit::Spades => &self.spades,
                    Suit::Hearts => &self.hearts,
                    Suit::Diamonds => &self.diamonds,
                    Suit::Clubs => &self.clubs
                }
            }
            TrumpGen::NoTrump => {
                if taken <= HALF_TRICKS{
                    0
                } else{
                    let number = if contract.bid().number_normalised() <= taken{
                        contract.bid().number() - 1
                    }
                    else{
                        0
                    };
                    (self.nt_first + (self.nt_next * i32::from(number))) * multiplier

                }

            }
        }
    }
}

impl ScoreIngredient<Suit> for PointsContractedTrick{
    fn calculate(&self, contract: &ContractParametersGen<Suit>, taken: u8, _vulnerability: bool) -> i32 {
        self.points(contract, taken)
    }
}

pub const POINTS_CONTRACTED_TRICK: PointsContractedTrick = PointsContractedTrick{
    clubs: 20,
    diamonds: 20,
    hearts: 30,
    spades: 30,
    nt_first: 40,
    nt_next: 30,
    doubling_multiplier: 2,
    redoubling_multiplier: 4
};