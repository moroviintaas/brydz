use brydz_core::contract::TrickGen;
use brydz_core::cards::trump::TrumpGen;
use karty::cards::{Card2SGen, Card};
use karty::figures::Figure;
use karty::suits::Suit;


fn debug_solve_trick(){
    println!("{}", std::mem::size_of::<TrickGen<Card>>());
    println!("{}", std::mem::size_of::<Card2SGen<Figure, Suit>>());
    println!("{}", std::mem::size_of::<TrumpGen<Suit>>());

}

fn main(){
    debug_solve_trick();
}