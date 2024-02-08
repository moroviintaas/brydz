use rand::{Rng, thread_rng};
use brydz_core::deal::BiasedHandDistribution;
use brydz_core::player::side::Side::North;
use brydz_core::amfi::spec::ContractDP;
use karty::cards::ACE_SPADES;
use amfiteatr_core::error::AmfiError;

pub fn test_sample_biased_distribution_parameters() -> Result<(), AmfiError<ContractDP>>{
    //setup_logger(LevelFilter::Debug, &None).unwrap();

    let mut trng = thread_rng();
    let tries = 100;
    let mut ace_spades_north = Vec::with_capacity(tries);
    for i in 0..tries{
        let sample: BiasedHandDistribution = trng.gen();
        //println!("{:?}", ron::to_string(&sample));
        print!("\r{:3}/100",i+1);
        ace_spades_north.push(f32::try_from(sample[North][&ACE_SPADES]).unwrap());

    }

    let sum = ace_spades_north.iter().map(|n| *n as f64).sum::<f64>();
    let count = ace_spades_north.len();

    println!("\rMean of probabilities that north has Ace Spades: {}", sum/(count as f64));



    Ok(())

}