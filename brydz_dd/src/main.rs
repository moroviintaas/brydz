use brydz_core::{
    contract::{Contract, ContractParametersGen, ContractMechanics},
    karty::{card_set, cards::*, suits::Suit::*, },
    bidding::Bid,
    cards::trump::Trump,
    player::side::{SideMap, Side::*}};
use brydz_dd::{node::TrickNode};
use brydz_dd::explore::{BinaryExplorer, ExploreOutput, Explorer, ExplorerStateUpdate};
use brydz_dd::actions::{DistinctCardGrouper, NeighbourCardGrouper};
use brydz_dd::hash::{DummyNodeStore, HashArrayNodeStore};
use brydz_dd::hash::hash24::Hash24;
use brydz_dd::hash::ranker::MoreCardsRanker;
//use brydz_core::karty::stack_hand;

fn _setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
#[allow(dead_code)]
fn dbg_explore(){

    let contract = Contract::new(
    ContractParametersGen::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS],
        card_set!(ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS),
        card_set![ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS],
        card_set![KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS ]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node ).unwrap();
    let mut explorer2 = explorer.clone();
    let mut explorer3 = explorer.clone();
    let result = explorer.explore_actions(ExploreOutput::MinusInfinity, ExploreOutput::Infinity).unwrap();
    let result_t = explorer2.explore_and_track_actions(ExploreOutput::MinusInfinity, ExploreOutput::Infinity).unwrap();
    let result_h = explorer3.hint().unwrap();
    println!("Result: {result:?}");
     println!("Result: \n{result_t}");
    println!("Hint: {result_h}");
    explorer3.update(ExplorerStateUpdate::PlaceCard(KING_CLUBS)).unwrap();
    let result_h = explorer3.hint().unwrap();
    println!("Hint: {result_h}");
}
/*
fn example_8_cards(){
    let contract = Contract::new(
    ContractSpec::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        stack_hand![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, EIGHT_HEARTS, TEN_SPADES, SEVEN_SPADES, NINE_DIAMONDS],
        stack_hand![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS, SEVEN_DIAMONDS, QUEEN_CLUBS, NINE_CLUBS, SEVEN_CLUBS, NINE_HEARTS],
        stack_hand![ACE_HEARTS, JACK_HEARTS, KING_SPADES, EIGHT_SPADES, KING_DIAMONDS, QUEEN_DIAMONDS, EIGHT_DIAMONDS, JACK_CLUBS],
        stack_hand![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES, SEVEN_HEARTS, EIGHT_CLUBS]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::new_checked(contract, node, ExploreOutput::MinusInfinity, ExploreOutput::Infinity, DistinctCardGrouper{}).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {}", result_h);
}
*/

#[allow(dead_code)]
fn example_6_cards(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {result_h}");
}

#[allow(dead_code)]
fn example_6_cards_n(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<NeighbourCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {result_h}");
}

#[allow(dead_code)]
fn example_6_cards_concurrent(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let result_h = explorer.hint_concurrent().unwrap();
    println!("Hint: {result_h}");
    //explorer.update_state(ExplorerStateUpdate::PlaceCard(JACK_CLUBS)).unwrap();
    //let result_h = explorer.hint().unwrap();
    //println!("Placed: {:#}.\n Hint: {}", JACK_CLUBS, result_h);
}

#[allow(dead_code)]
fn example_6_cards_binary(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = BinaryExplorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node, 3).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {result_h}");
}

#[allow(dead_code)]
fn example_7_cards(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS, EIGHT_HEARTS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS, EIGHT_SPADES],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS, EIGHT_DIAMONDS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES, EIGHT_CLUBS]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {result_h}");
    //explorer.update_state(ExplorerStateUpdate::PlaceCard(JACK_CLUBS)).unwrap();
    //let result_h = explorer.hint().unwrap();
    //println!("Placed: {:#}.\n Hint: {}", JACK_CLUBS, result_h);
}

#[allow(dead_code)]
fn example_7_cards_hash(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS, EIGHT_HEARTS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS, EIGHT_SPADES],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS, EIGHT_DIAMONDS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES, EIGHT_CLUBS]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, HashArrayNodeStore<Hash24<3>, MoreCardsRanker,0x1000000,8 >>
        ::new_checked(contract, node).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Hint: {result_h}");
    explorer.update(ExplorerStateUpdate::PlaceCard(JACK_CLUBS)).unwrap();
    let result_h = explorer.hint().unwrap();
    println!("Placed: {JACK_CLUBS:#}.\n Hint: {result_h}");
}
#[allow(dead_code)]
fn example_7_cards_concurrent(){
    let contract = Contract::new(
    ContractParametersGen::new(East, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_CLUBS, KING_CLUBS, TEN_CLUBS, TEN_HEARTS, TEN_SPADES,  NINE_DIAMONDS, EIGHT_HEARTS],
        card_set![ACE_DIAMONDS, JACK_DIAMONDS, TEN_DIAMONDS,  QUEEN_CLUBS, NINE_CLUBS,  NINE_HEARTS, EIGHT_SPADES],
        card_set![ACE_HEARTS, JACK_HEARTS, KING_SPADES,  KING_DIAMONDS, QUEEN_DIAMONDS,  JACK_CLUBS, EIGHT_DIAMONDS],
        card_set![KING_HEARTS, QUEEN_HEARTS, ACE_SPADES, QUEEN_SPADES, JACK_SPADES, NINE_SPADES, EIGHT_CLUBS]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = Explorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node).unwrap();
    let result_h = explorer.hint_concurrent().unwrap();
    println!("Hint: {result_h}");
}
#[allow(dead_code)]
fn debug_binary_explorer_4(){
    let contract = Contract::new(
    ContractParametersGen::new(West, Bid::init(Trump::Colored(Diamonds), 1).unwrap()));
    let hands = SideMap::new(
        card_set![ACE_SPADES, QUEEN_SPADES, JACK_CLUBS, KING_CLUBS],
        card_set!(ACE_HEARTS, KING_DIAMONDS, KING_HEARTS, JACK_DIAMONDS),
        card_set![ACE_DIAMONDS, ACE_CLUBS, QUEEN_HEARTS, QUEEN_CLUBS],
        card_set![KING_SPADES, QUEEN_DIAMONDS, JACK_SPADES, JACK_HEARTS ]);
    let node = TrickNode::new_checked(hands, contract.current_side()).unwrap();
    let mut explorer = BinaryExplorer::<DistinctCardGrouper, DummyNodeStore>::new_checked(contract, node, 1).unwrap();
    //let mut explorer2 = explorer.clone();
    //let mut explorer3 = explorer.clone();
    let result = explorer.hint().unwrap();
    println!("Result: {result:#}");
    explorer.update(ExplorerStateUpdate::PlaceCard(JACK_CLUBS)).unwrap();
    println!("Result: {:#}", explorer.hint().unwrap());
    explorer.update(ExplorerStateUpdate::PlaceCard(KING_HEARTS)).unwrap();
    println!("Result: {:#}", explorer.hint().unwrap());
    explorer.update(ExplorerStateUpdate::PlaceCard(ACE_CLUBS)).unwrap();
    println!("Result: {:#}", explorer.hint().unwrap());
    explorer.update(ExplorerStateUpdate::PlaceCard(QUEEN_DIAMONDS)).unwrap();
    println!("Result: {:#}", explorer.hint().unwrap());
    //let result_t = explorer2.explore_and_track_actions().unwrap();
    //let result_h = explorer3.hint().unwrap();

}

fn main() {
    //setup_logger().unwrap();
    //dbg_explore();
    //example_7_cards_concurrent();
    //example_7_cards();
    example_6_cards_n();
    //example_6_cards_concurrent();
    //example_6_cards_binary();

    //debug_binary_explorer_4()
}