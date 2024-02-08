use arrayvec::ArrayVec;
use karty::cards::Card2SymTrait;
use crate::player::side::Side;
use crate::contract::TrickGen;
use crate::player::role::PlayRole;
use crate::meta::QUARTER_SIZE;

#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct Player<Card: Card2SymTrait>{
    id: u8,
    name: String,
    play_role: Option<PlayRole>,
    tricks_taken: ArrayVec<TrickGen<Card>, QUARTER_SIZE>,
    side: Side


}
impl<Card: Card2SymTrait> Player<Card>{
    pub fn new(id: u8, name: String, side: Side) -> Self{
        Self{id, name, play_role: None, tricks_taken: ArrayVec::new(), side}
    }
    pub fn id(&self) -> u8{
        self.id
    }
    pub fn name(&self) -> &str{
        &self.name
    }
    pub fn play_role(&self) -> &Option<PlayRole>{
        &self.play_role
    }
    pub fn tricks_taken(&self) -> &ArrayVec<TrickGen<Card>, QUARTER_SIZE>{
        &self.tricks_taken
    }
}