use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use ron::ser::{PrettyConfig, to_string_pretty};
use brydz_core::bidding::{Bid, Doubling};
use brydz_core::cards::trump::{Trump, TrumpGen};
use brydz_core::contract::ContractParameters;
use brydz_core::deal::{BiasedHandDistribution, ContractGameDescription, DealDistribution, fair_bridge_deal};
use brydz_core::player::side::Side;
use karty::set::CardSetStd;
use karty::random::RandomSymbol;
use karty::suits::Suit;
use crate::error::BrydzModelError;
use crate::error::GenError::LowerBoundOverUpper;
use std::io::Write;
use rand::distributions::Distribution;

mod options;
pub use options::*;



//pub fn random_contract_with_declarer(rng: &mut ThreadRng) -> Result<SimContractParams>

fn generate_single_contract(params: &GenContractOptions, rng: &mut ThreadRng) -> Result<ContractGameDescription, BrydzModelError>{

    if params.min_contract > params.max_contract {
        return Err(BrydzModelError::Gen(LowerBoundOverUpper {lower: params.min_contract, upper: params.max_contract }))
    }

    let contract_value = rng.gen_range(params.min_contract..=params.max_contract);
    let trump: Trump = match params.trump_limit{
        Subtrump::All => TrumpGen::<Suit>::random(rng),
        Subtrump::Colored => {
            Trump::Colored(Suit::random(rng))
        },
        Subtrump::NoTrump => Trump::NoTrump
    };
    let contract_declarer: Side = match params.force_declarer {
        ForceDeclarer::DontForce => Side::random(rng),
        _ => Side::try_from(&params.force_declarer).unwrap(),
    };



    let doubling = match params.choice_doubling{
        ChoiceDoubling::Any => *[Doubling::None, Doubling::Redouble, Doubling::Redouble].choose(rng).unwrap(),
        ChoiceDoubling::No => Doubling::None,
        ChoiceDoubling::Double => Doubling::Double,
        ChoiceDoubling::Redouble => Doubling::Redouble,
    };
    let contract_parameters = ContractParameters::new_d(contract_declarer,
                                        Bid::init(trump, contract_value).unwrap(),
                                        doubling);


    let (template, cards) = match params.deal_method{
        DealMethod::Fair => (DealDistribution::Fair, fair_bridge_deal::<CardSetStd>()),

        DealMethod::Biased => {
            let mut rng = thread_rng();
            let distribution: BiasedHandDistribution = rng.gen();
            let cards = distribution.sample(&mut rng);
            (DealDistribution::Biased(Box::new(distribution)), cards)
        }
    };

    Ok(ContractGameDescription::new(contract_parameters, template, cards))


}

pub fn generate_contracts(params: &GenContractOptions) -> Result<Vec<ContractGameDescription>, BrydzModelError>{
    let repeat = params.game_count as usize;
    let mut rng = thread_rng();
    let mut game_params: Vec<ContractGameDescription> = Vec::with_capacity(repeat);
    for _ in 0..repeat{
        game_params.push(generate_single_contract(params, &mut rng)?);
    }
    Ok(game_params)

}

pub fn gen2(gen_options: &GenContractOptions) -> Result<(), BrydzModelError>{
    let my_config = PrettyConfig::new()
        .depth_limit(4)
        // definitely superior (okay, just joking)
        .indentor("\t".to_owned());
    let contracts = generate_contracts(gen_options).unwrap();

    match &gen_options.output_file{
        None => {
            println!("{}", to_string_pretty(&contracts, my_config).unwrap())
        }
        Some(file) => {
            let mut output = std::fs::File::create(file).unwrap();
            write!(output, "{}", to_string_pretty(&contracts, my_config).unwrap()).unwrap()
        }
    };
    Ok(())
}