use brydz_core::player::side::{Side};
use brydz_dd::hash::{DummyNodeStore};
use brydz_core::bidding::Bid;
use brydz_core::cards::trump::Trump;
use brydz_core::contract::{Contract, ContractMechanics, ContractParametersGen};
use brydz_core::deal::fair_bridge_partial_deal;
use brydz_core::karty::cards::{Card, Card2SymTrait};
use brydz_core::karty::figures::*;
use brydz_core::karty::hand::CardSet;
use brydz_core::karty::suits::{Suit::*, SUITS};
use brydz_core::player::side::Side::{East, North};
use criterion::{ criterion_group, criterion_main, Criterion, BenchmarkId};
use brydz_dd::explore::{BinaryExplorer};
use brydz_dd::node::TrickNode;
use brydz_dd::actions::{DistinctCardGrouper, ActionOptimiser, NeighbourCardGrouper};

pub fn prepare_explorer<G: ActionOptimiser> (card_supply: Vec<Card>, first_side: Side, north_south_target: u8) -> BinaryExplorer<G, DummyNodeStore>{
    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), first_side);
    let contract = Contract::new(
        ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), north_south_target).unwrap()));
        let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
        BinaryExplorer::<G, DummyNodeStore>::new_checked(contract, node, north_south_target).unwrap()
}


//pub fn binary_hint_target(hands: SideMap<StackHand>, target: u8){
pub fn binary_hint_target(mut explorer: BinaryExplorer<DistinctCardGrouper, DummyNodeStore>){
    
    let _result_h = explorer.hint().unwrap();
}


pub fn bench_binary_series_unoptimised(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Binary_unoptimised_8_to_28");
    for fig_num in [2,3,4,5,6,7]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        for target in 1..=fig_num{
            let parameter_string = format!("{} cards and target {}", fig_num*4, target);
            group.bench_function(BenchmarkId::new("binary_hint_target", parameter_string), |b|{
                b.iter_batched(||{
                    prepare_explorer::<DistinctCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
        }
        
        
    }
}

pub fn bench_binary_series_neighbouring(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Binary_neigbouring_8_to_28");
    for fig_num in [2,3,4,5,6,7]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        for target in 1..=fig_num{
            let parameter_string = format!("{} cards and target {}", fig_num*4, target);
            group.bench_function(BenchmarkId::new("binary_hint_target", parameter_string), |b|{
                b.iter_batched(||{
                    prepare_explorer::<NeighbourCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
        }
        
        
    }
}

pub fn bench_binary_optimalisations_20(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Grouping benchmark using 20 cards");
    for fig_num in [5]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        for target in 1..=fig_num{
            let parameter_string = format!("{} cards and target {}", fig_num*4, target);
            group.bench_function(BenchmarkId::new("unoptimised_binary_hint_target", parameter_string.clone()), |b|{
                b.iter_batched(||{
                    prepare_explorer::<DistinctCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
            group.bench_function(BenchmarkId::new("neighbour_opti_binary_hint_target", parameter_string), |b|{
                b.iter_batched(||{
                    prepare_explorer::<NeighbourCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
        }
        
        
    }
}

pub fn bench_binary_optimalisations_24(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Grouping benchmark using 24 cards");
    for fig_num in [6]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        for target in 1..=fig_num{
            let parameter_string = format!("{} cards and target {}", fig_num*4, target);
            group.bench_function(BenchmarkId::new("unoptimised_binary_hint_target", parameter_string.clone()), |b|{
                b.iter_batched(||{
                    prepare_explorer::<DistinctCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
            group.bench_function(BenchmarkId::new("neighbour_opti_binary_hint_target", parameter_string), |b|{
                b.iter_batched(||{
                    prepare_explorer::<NeighbourCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
        }
        
        
    }
}

pub fn bench_binary_optimalisations_28(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Grouping benchmark using 28 cards");
    for fig_num in [7]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        for target in 1..=fig_num{
            let parameter_string = format!("{} cards and target {}", fig_num*4, target);
            group.bench_function(BenchmarkId::new("unoptimised_binary_hint_target", parameter_string.clone()), |b|{
                b.iter_batched(||{
                    prepare_explorer::<DistinctCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
            group.bench_function(BenchmarkId::new("neigbour_opti_binary_hint_target", parameter_string), |b|{
                b.iter_batched(||{
                    prepare_explorer::<NeighbourCardGrouper>(card_supply.clone(), North, target as u8)
    
                    //let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
                    //binary_hint_target(hands, target as u8);
                }, |mut e| {
                    e.hint().unwrap()

                }, criterion::BatchSize::SmallInput)
            });
        }
        
        
    }
}


//criterion_group!(benches, hint_4_cards_benchmark, hint_5_cards_benchmark, hint_6_cards_benchmark);
//criterion_group!(benches, bench_binary_12_base, bench_binary_16_base, bench_binary_20_base, bench_binary_24_base, bench_binary_28_base);
criterion_group!(comp_from_4_to_28, bench_binary_series_unoptimised, bench_binary_series_neighbouring, );
criterion_group!(groupings, bench_binary_optimalisations_20, bench_binary_optimalisations_24, bench_binary_optimalisations_28);
//criterion_main!(unoptimised, groupings);
//criterion_main!(groupings);
//criterion_main!(comp_from_4_to_28);
criterion_group!(check_on_24, bench_binary_optimalisations_24);
criterion_main!(check_on_24);