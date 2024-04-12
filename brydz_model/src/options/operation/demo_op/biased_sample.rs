use rand::{Rng, thread_rng};
use brydz_core::deal::BiasedHandDistribution;
use brydz_core::player::side::Side::North;
use brydz_core::amfiteatr::spec::ContractDP;
use karty::cards::ACE_SPADES;
use karty::set::CardSet;
use amfiteatr_core::error::AmfiteatrError;

pub fn test_sample_biased_deal_crossing() -> Result<(), AmfiteatrError<ContractDP>>{
    let mut trng = thread_rng();
    let distribution: BiasedHandDistribution = trng.gen();
    let tries = 10000;
    let mut ace_spades_north = Vec::with_capacity(tries);


    let proba_ace_spades = distribution.card_probabilities(&ACE_SPADES);
    let proba_ace_spades_north = f32::from(proba_ace_spades[&North]);
    
    for _ in 0..tries{
        match  distribution.sample_deal_crossing(&mut trng){
            Ok(card_deal) => {
                match card_deal[&North].contains(&ACE_SPADES){
                    true => {ace_spades_north.push(1.0);},
                    false => {
                        ace_spades_north.push(0.0);
                    }
                }
            },
            Err(e) => {
                panic!("{:?}", e)
            }
        }

    }

    let sum = ace_spades_north.iter().copied().sum::<f64>();
    let count = ace_spades_north.len();

    println!("Expected probability: {}", proba_ace_spades_north);
    println!("Empiric probability: {}", sum/(count as f64));
    
    
    Ok(())

}

pub fn test_sample_biased_deal_single() -> Result<(), AmfiteatrError<ContractDP>>{
    let mut trng = thread_rng();
    let distribution: BiasedHandDistribution = trng.gen();
    let tries = 10000;
    let mut ace_spades_north = Vec::with_capacity(tries);


    let proba_ace_spades = distribution.card_probabilities(&ACE_SPADES);
    let proba_ace_spades_north = f32::from(proba_ace_spades[&North]);

    for _ in 0..tries{
        match  distribution.sample_deal_single_try(&mut trng){
            Ok(card_deal) => {
                match card_deal[&North].contains(&ACE_SPADES){
                    true => {ace_spades_north.push(1.0);},
                    false => {
                        ace_spades_north.push(0.0);
                    }
                }
            },
            Err(e) => {
                panic!("{:?}", e)
            }
        }

    }

    let sum = ace_spades_north.iter().copied().sum::<f64>();
    let count = ace_spades_north.len();

    println!("Expected probability: {}", proba_ace_spades_north);
    println!("Empiric probability: {}", sum/(count as f64));


    Ok(())
}