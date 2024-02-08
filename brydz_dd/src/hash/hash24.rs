use brydz_core::player::side::Side;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StateHash24 {
    //bytes: [u8;3] // 6 top cards in each suit 6 * 4 = 24
    hash: u32
    //4 bits
}

impl StateHash24 {
    /*pub fn new(bytes: [u8;3]) -> Self{
        Self{bytes}
    }*/
    pub fn new(hash: u32) -> Self{
        Self{hash}
    }
    pub fn get_hash(&self) -> u32{
        self.hash
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct StateHash24EntryDistinguish {
    //bytes: [u8; 4] // 8 lower cards (2-9) 8*4 = 28
    hash_rest: u32,
    playing_side: Side,
}

impl StateHash24EntryDistinguish {
    pub fn new(hash_rest: u32, playing_side: Side) -> Self{
       Self{hash_rest, playing_side}
    }
    pub fn get_rest(&self) -> u32{
        self.hash_rest
    }
    /*
    pub fn new(bytes: [u8;4]) -> Self{
        Self{bytes}
    }*/
}
//one overlaps