use std::cmp::Ordering;
use karty::cards::Card2SymTrait;
use karty::suits::{Suit};
use crate::contract::{ContractMechanics};
use crate::error::{BridgeCoreErrorGen, ContractErrorGen};
use crate::player::axis::Axis;
use crate::score::calculation::ScoreIngredient;
use crate::score::ScoreTracker;
use crate::score::tables::{PENALTY_UNDER_TRICK, POINTS_CONTRACTED_TRICK, POINTS_OVER_TRICK, POINTS_PREMIUM_CONTRACT, POINTS_PREMIUM_SPORT, POINTS_SLAM};


#[derive(Debug, Copy, Clone, Default)]
pub struct ScoreTableSport {
    ns_score: i32,
    ew_score: i32,
    ns_vulnerability: bool,
    ew_vulnerability: bool

}

impl ScoreTableSport{
    pub fn new(ns_vulnerability: bool, ew_vulnerability: bool) -> Self{
        Self{ns_score: 0, ew_score: 0, ns_vulnerability, ew_vulnerability}
    }

}


impl<Co: ContractMechanics<Card = Crd>, Crd: Card2SymTrait<Suit =Suit>> ScoreTracker<Co, Crd>
for ScoreTableSport{

    fn winner_axis(&self) -> Option<Axis> {
        match self.ew_score.cmp(&self.ns_score){
            Ordering::Less => Some(Axis::NorthSouth),
            Ordering::Equal => None,
            Ordering::Greater => Some(Axis::EastWest)
        }
    }

    /// # Example:
    /// ```
    /// use brydz_core::bidding::Bid;
    /// use brydz_core::cards::trump::TrumpGen;
    /// use brydz_core::contract::{ContractParametersGen, ContractMechanics, Contract};
    /// use brydz_core::player::axis::Axis::NorthSouth;
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use brydz_core::score::ScoreTracker;
    /// use brydz_core::score::sport::ScoreTableSport;
    /// use karty::suits::Suit::{Diamonds, Hearts};
    /// use karty::cards::*;
    /// use karty::figures::Figure;
    /// use karty::suits::Suit;
    /// let mut score = ScoreTableSport::new(false, false);
    /// let mut deal = Contract::new(ContractParametersGen::new(South, Bid::init(TrumpGen::Colored(Diamonds), 3).unwrap()));
    /// deal.insert_card(West, ACE_CLUBS).expect("Error inserting in deal 0.");
    /// deal.insert_card(North, THREE_CLUBS).expect("Error inserting card 1.");
    /// deal.insert_card(East, FOUR_CLUBS).expect("Error inserting card  2.");
    /// deal.insert_card(South, TWO_CLUBS).expect("Error inserting card  3.");
    ///
    /// deal.insert_card(West, JACK_CLUBS).expect("Error inserting card  4.");
    /// deal.insert_card(North, TEN_CLUBS).expect("Error inserting card  5.");
    /// deal.insert_card(East, SEVEN_CLUBS).expect("Error inserting card 6.");
    /// deal.insert_card(South, FIVE_CLUBS).expect("Error inserting card 7.");
    ///
    /// deal.insert_card(West, NINE_SPADES).expect("Error inserting card  8.");
    /// deal.insert_card(North, JACK_SPADES).expect("Error inserting card  9.");
    /// deal.insert_card(East, KING_SPADES).expect("Error inserting card  10.");
    /// deal.insert_card(South, ACE_SPADES).expect("Error inserting card  11.");
    ///
    /// deal.insert_card(South, ACE_DIAMONDS).expect("Error inserting card  12.");
    /// deal.insert_card(West, SIX_DIAMONDS).expect("Error inserting card  13.");
    /// deal.insert_card(North, TWO_DIAMONDS).expect("Error inserting card  14.");
    /// deal.insert_card(East, JACK_DIAMONDS).expect("Error inserting card  15.");
    ///
    /// deal.insert_card(South, FIVE_DIAMONDS).expect("Error inserting card  16.");
    /// deal.insert_card(West, SEVEN_DIAMONDS).expect("Error inserting card  17.");
    /// deal.insert_card(North, TEN_DIAMONDS).expect("Error inserting card  18.");
    /// deal.insert_card(East, TWO_HEARTS).expect("Error inserting card  19.");
    ///
    /// deal.insert_card(North, QUEEN_SPADES).expect("Error inserting card  20.");
    /// deal.insert_card(East, FOUR_SPADES).expect("Error inserting card  21.");
    /// deal.insert_card(South, TWO_SPADES).expect("Error inserting card  22.");
    /// deal.insert_card(West, THREE_SPADES).expect("Error inserting card  23.");
    ///
    /// deal.insert_card(North, FIVE_SPADES).expect("Error inserting card  24.");
    /// deal.insert_card(East, SEVEN_SPADES).expect("Error inserting card  25.");
    /// deal.insert_card(South, TEN_SPADES).expect("Error inserting card  26.");
    /// deal.insert_card(West, EIGHT_SPADES).expect("Error inserting card  27.");
    ///
    /// deal.insert_card(South, ACE_HEARTS).expect("Error inserting card  28.");
    /// deal.insert_card(West, FIVE_HEARTS).expect("Error inserting card  29.");
    /// deal.insert_card(North, THREE_HEARTS).expect("Error inserting card  30.");
    /// deal.insert_card(East, SIX_HEARTS).expect("Error inserting card  31.");
    ///
    /// deal.insert_card(South, FOUR_HEARTS).expect("Error inserting card  32.");
    /// deal.insert_card(West, SEVEN_HEARTS).expect("Error inserting card  33.");
    /// deal.insert_card(North, QUEEN_HEARTS).expect("Error inserting card  34.");
    /// deal.insert_card(East, EIGHT_HEARTS).expect("Error inserting card  35.");
    ///
    /// deal.insert_card(North, KING_HEARTS).expect("Error inserting card  36.");
    /// deal.insert_card(East, NINE_HEARTS).expect("Error inserting card  37.");
    /// deal.insert_card(South, SIX_SPADES).expect("Error inserting card  38.");
    /// deal.insert_card(West, JACK_HEARTS).expect("Error inserting card  39.");
    ///
    /// deal.insert_card(North, THREE_DIAMONDS).expect("Error inserting card  40.");
    /// deal.insert_card(East, TEN_HEARTS).expect("Error inserting card  41.");
    /// deal.insert_card(South, KING_DIAMONDS).expect("Error inserting card  42.");
    /// deal.insert_card(West, SIX_CLUBS).expect("Error inserting card  43.");
    ///
    /// deal.insert_card(South, QUEEN_DIAMONDS).expect("Error inserting card  44.");
    /// deal.insert_card(West, EIGHT_CLUBS).expect("Error inserting card  45.");
    /// deal.insert_card(North, FOUR_DIAMONDS).expect("Error inserting card  46.");
    /// deal.insert_card(East, QUEEN_CLUBS).expect("Error inserting card  47.");
    ///
    /// deal.insert_card(South, NINE_DIAMONDS).expect("Error inserting card  48.");
    /// deal.insert_card(West, NINE_CLUBS).expect("Error inserting card  49.");
    /// deal.insert_card(North, EIGHT_DIAMONDS).expect("Error inserting card  50.");
    /// deal.insert_card(East, KING_CLUBS).expect("Error inserting card  51.");
    ///
    /// score.update(&mut deal).unwrap();
    ///
    ///
    ///
    ///
    /// //60 + 40 + 50 zapis czesciowy
    /// assert_eq!(<ScoreTableSport as ScoreTracker<Contract, Card>>::points(&score, &NorthSouth), 150);
    /// //assert_eq!(score.points(&NorthSouth), 150);
    ///
    ///
    /// ```
    fn update(&mut self, deal: &Co) -> Result<(), BridgeCoreErrorGen<Crd>> {
        if deal.is_completed(){
            let axis = deal.contract_spec().declarer().axis();
            let vulnerability = match axis{
                Axis::EastWest => self.ew_vulnerability,
                Axis::NorthSouth => self.ns_vulnerability
            };
            let defender_vulnerability = match axis{
                Axis::EastWest => self.ns_vulnerability,
                Axis::NorthSouth => self.ew_vulnerability
            };
            let taken = deal.total_tricks_taken_axis(axis) as u8;
            let contracted_points = POINTS_CONTRACTED_TRICK.calculate(deal.contract_spec(), taken, false);
            let overtrick_bonus = POINTS_OVER_TRICK.points(deal.contract_spec(), taken, vulnerability);
            let slam_bonus = POINTS_SLAM.points(deal.contract_spec(), taken, vulnerability);
            let premium_game_points = POINTS_PREMIUM_SPORT.points(contracted_points, vulnerability);
            let premium_contract_points = POINTS_PREMIUM_CONTRACT.points(deal.contract_spec(), taken);
            let penalty_undertricks = match PENALTY_UNDER_TRICK.penalty_checked(deal.contract_spec(), taken, defender_vulnerability){
                Ok(points) => points,
                Err(e) => return Err(BridgeCoreErrorGen::Score(e))
            };
            println!("contracted: {}, overtrick: {}", &contracted_points, &overtrick_bonus);

            let declarer_axis_score = contracted_points+ overtrick_bonus + slam_bonus
                + premium_game_points + premium_contract_points;
            let defender_axis_score = penalty_undertricks;

            match axis{
                Axis::NorthSouth => {
                    self.ns_score += declarer_axis_score;
                    self.ew_score += defender_axis_score;
                }
                Axis::EastWest => {
                    self.ns_score += defender_axis_score;
                    self.ew_score += declarer_axis_score;
                }
            }
            Ok(())


        }
        else{
            Err(BridgeCoreErrorGen::Contract(ContractErrorGen::DealIncomplete))
        }
    }

    fn points(&self, axis: &Axis) -> i32 {
        match axis{
            Axis::EastWest => self.ew_score,
            Axis::NorthSouth => self.ns_score
        }
    }
}

