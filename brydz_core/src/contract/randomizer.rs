use rand::distributions::{Distribution, Uniform as RandUni};

use rand::{Rng};
use rand::seq::SliceRandom;
use statrs::distribution::Multinomial;
use karty::suits::Suit;
use karty::symbol::CardSymbol;
use crate::bidding::{Bid, Doubling};
use crate::cards::trump::Trump;
use crate::contract::ContractParameters;
use crate::player::side::{Side, SIDES};

pub struct ContractRandomizer{
    //min_contract: u8,
    //max_contract: u8,
    contract_value_distr: RandUni<u8>,
    trump_distribution: Multinomial,
    declarer_side: Option<Side>,
    doubling: Option<Doubling>



}

impl Default for ContractRandomizer{
    fn default() -> Self {
        Self{
            contract_value_distr: RandUni::new(1,8),
            trump_distribution: Multinomial::new(&[1.0, 1.0, 1.0, 1.0, 1.0], 1).unwrap(),

            declarer_side: None,
            doubling: None,
        }
    }

}

impl ContractRandomizer{
    /*
    pub fn new(min_contract: u8, max_contract: u8, clubs_p: f32, diamonds_p: f32,
        hearts_p: f32, spades_p: f32, nt_p: f32) -> Self{

    }

     */
    pub fn new(min_contract: u8, max_contract: u8, probabilities: &[f64;5], declarer_side: Option<Side>, doubling: Option<Doubling>) -> Self{
        Self{
            contract_value_distr: RandUni::new(min_contract, max_contract+1),
            trump_distribution: Multinomial::new(&probabilities[..], 1).unwrap(),
            declarer_side,
            doubling
        }
    }

}

impl Distribution<ContractParameters> for ContractRandomizer{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ContractParameters {
        let v = self.contract_value_distr.sample(rng);
        let t = match self.trump_distribution.sample(rng)[0] as usize{
            n @ 0..=3 => Trump::Colored(Suit::from_usize_index(n).unwrap()),
            4 => Trump::NoTrump,
            a => panic!("This should not happen. Trump should be in [0..=4] (4 is for NoTrump). It was {a:}."),
        };
        let s = if let Some(side) = self.declarer_side{
            side
        } else{
            *SIDES.choose(rng).unwrap()
        };
        let d = if let Some(doubling) = self.doubling{
            doubling
        } else {
            *[Doubling::None, Doubling::Double, Doubling::Redouble].choose(rng).unwrap()
        };

        ContractParameters::new_d(s, Bid::init(t, v).unwrap(), d)

    }
}