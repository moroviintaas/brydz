use brydz_core::player::side::{SideMap, Side};
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
use brydz_dd::explore::{ Explorer};
use brydz_dd::node::TrickNode;
use brydz_dd::actions::DistinctCardGrouper;

pub fn prepare_explorer(card_supply: Vec<Card>, first_side: Side) -> Explorer<DistinctCardGrouper, DummyNodeStore>{
    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), first_side);
    let contract = Contract::new(
        ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
        let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
        Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap()
}


pub fn alphabeta_hint(hands: SideMap<CardSet>){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));

    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let _result_h = explorer.hint().unwrap();
}

pub fn hint_12_cards(){

    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));

    let card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen], vec![Spades, Hearts, Diamonds, Clubs]).collect();

    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let _result_h = explorer.hint().unwrap();



}

pub fn hint_16_cards(){

    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));

    let card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen, Jack], vec![Spades, Hearts, Diamonds, Clubs]).collect();

    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let _result_h = explorer.hint().unwrap();



}
pub fn hint_20_cards(){

    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));

    let card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen, Jack, F10], vec![Spades, Hearts, Diamonds, Clubs]).collect();

    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let _result_h = explorer.hint().unwrap();



}


pub fn hint_24_cards(){

    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));

    let card_supply: Vec<Card> = Card::card_subset(vec![Ace, King, Queen, Jack, F10, F9], vec![Spades, Hearts, Diamonds, Clubs]).collect();

    let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let _result_h = explorer.hint().unwrap();



}

pub fn explorer_hint(hands: &SideMap<CardSet>){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let node = TrickNode::new_checked(*hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    explorer.hint().unwrap();
}


pub fn hint_range_2_6(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("Number of figures");
    
    for num in [2,3,4, 5, 6]{
        
            
            let parameter_string = format!("{} cards", num*4);
            
            
        group.bench_function(BenchmarkId::new("Alpha beta on ", parameter_string),  |b|{
            b.iter(|| {
                let card_supply: Vec<Card> = Card::card_subset(figures[..num].to_vec(), SUITS).collect();
                let hands = fair_bridge_partial_deal::<CardSet>(card_supply.clone(), North);
                explorer_hint(&hands)
            })
        });
    }
}


/*pub fn bench_12_base(c: &mut Criterion){
    c.bench_function("Benchmark 12 cards", |b| b.iter(|| hint_12_cards()));
}
pub fn bench_16_base(c: &mut Criterion){
    c.bench_function("Benchmark 16 cards", |b| b.iter(|| hint_16_cards()));
}
pub fn bench_20_base(c: &mut Criterion){
    c.bench_function("Benchmark 20 cards", |b| b.iter(|| hint_20_cards()));
}*/
/* 
pub fn bench_20_base(c: &mut Criterion){
    //c.bench_function("Benchmark 24 cards", |b| b.iter(|| hint_24_cards()));
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let card_supply: Vec<Card> = Card::card_subset(figures[..5].to_vec(), SUITS).collect();
    //let mut group = c.benchmark_group("ALphaBeta search 24 cards");
    //let parameter_string = format!("{}", t);
    c.bench_function("AlphaBeta explore 20 cards", |b|{
        b.iter(||{
            
            let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
            alphabeta_hint(hands);
        })
    });
}


pub fn bench_24_base(c: &mut Criterion){
    //c.bench_function("Benchmark 24 cards", |b| b.iter(|| hint_24_cards()));
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let card_supply: Vec<Card> = Card::card_subset(figures[..6].to_vec(), SUITS).collect();
    //let mut group = c.benchmark_group("ALphaBeta search 24 cards");
    //let parameter_string = format!("{}", t);
    c.bench_function("AlphaBeta explore 24 cards", |b|{
        b.iter(||{
            
            let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
            alphabeta_hint(hands);
        })
    });
}

pub fn bench_28_base(c: &mut Criterion){
    //c.bench_function("Benchmark 24 cards", |b| b.iter(|| hint_24_cards()));
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let card_supply: Vec<Card> = Card::card_subset(figures[..7].to_vec(), SUITS).collect();
    //let mut group = c.benchmark_group("ALphaBeta search 24 cards");
    //let parameter_string = format!("{}", t);
    c.bench_function("AlphaBeta explore 28 cards", |b|{
        b.iter(||{
            
            let hands = fair_bridge_partial_deal::<StackHand>(card_supply.clone(), North);
            alphabeta_hint(hands);
        })
    });
}
*/
pub fn bench_alpha_beta_unoptimised(c: &mut Criterion){
    let figures = vec![Ace, King, Queen, Jack, F10, F9, F8, F7, F6, F5, F4, F3, F2];
    let mut group = c.benchmark_group("AlphaBeta_unoptimised_8_to_24");
    for fig_num in [2,3,4,5,6]{
        let card_supply: Vec<Card> = Card::card_subset(figures[..fig_num].to_vec(), SUITS).collect();
        let parameter_string = format!("{} cards", fig_num*4);
        group.bench_function(BenchmarkId::new("alphabeta_hint ", parameter_string), |b|{
            b.iter_batched(||{
                prepare_explorer(card_supply.clone(), North)
            }, | mut e |{
                e.hint().unwrap()
            }, criterion::BatchSize::SmallInput)
        });
    }

}


//criterion_group!(benches, hint_4_cards_benchmark, hint_5_cards_benchmark, hint_6_cards_benchmark);
criterion_group!(unoptimised, bench_alpha_beta_unoptimised);
criterion_group!(range,  hint_range_2_6);
criterion_main!(unoptimised);