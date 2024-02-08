use std::marker::PhantomData;
use log::warn;
use brydz_core::player::side::{SideMap};
use crate::error::HashError;
use crate::hash::{HashEntry, Label, NodeHasher, NodeStoreTrait};
use crate::hash::ranker::HashRanker;
use crate::node::TrickNode;


#[derive(Copy, Clone)]
pub struct HashLine<L: Label, const ENTRIES: usize>{
    entries: [HashEntry<L>; ENTRIES],
    sizes: u8

}
impl<L: Label, const ENTRIES: usize> HashLine<L, ENTRIES>{
    pub fn init() -> Self{
        Self{ entries: [HashEntry::new(L::unused(), 0); ENTRIES], sizes: 0 }
    }
    pub fn entries_for_side(&self) -> &[HashEntry<L>]{
        &self.entries[..self.sizes as usize]
    }
    pub fn number_of_entries_for_side(&self) -> u8{
        self.sizes
    }
    pub fn try_insert(&mut self, entry: HashEntry<L>) -> Result<(), HashError>{
        let size = self.sizes as usize;
        if  size < ENTRIES{
            self.entries[size] = entry;
            self.sizes += 1;
            Ok(())
        } else {
            Err(HashError::HashTableFull)
        }
    }
    pub fn get_value(&self,  label:&L) -> Option<u8>{
        for entry in &self.entries{
            if entry.label() == label{
                return Some(entry.value())
            }
        }
        None
    }


}


pub struct HashArrayNodeStore<H: NodeHasher, HR: HashRanker<H>, const LINES: usize, const ENTRIES: usize>{
    hasher: PhantomData<H>,
    hash_ranker: PhantomData<HR>,
    array: Vec<SideMap<HashLine<H::LabelType, ENTRIES>>>,
    //fill_measure: SideMap<[u8;ENTRIES]>,
   // hit_counter: u64,

}
impl<H: NodeHasher, HR: HashRanker<H>, const LINES: usize, const ENTRIES: usize> HashArrayNodeStore<H, HR, LINES, ENTRIES>{
    pub fn init() -> Self{
        let mut v = Vec::with_capacity(LINES);
        for _ in 0..LINES{
            v.push(SideMap::new_symmetric( HashLine::init()));
            //v.push(INITIAL_HASH_LINE);
        }
        Self{
            hasher: PhantomData{},
            hash_ranker: PhantomData{},
            array: v,
            /*fill_measure: SideMap::new_symmetric([0;ENTRIES]),
            hit_counter: 0,*/
        }

    }
}

impl<H: NodeHasher, HR: HashRanker<H>, const LINES: usize, const ENTRIES: usize> Default for HashArrayNodeStore<H, HR, LINES, ENTRIES>{
    fn default() -> Self {
        Self::init()
    }
}

impl<H: NodeHasher, HR: HashRanker<H>, const LINES: usize, const ENTRIES: usize>  NodeStoreTrait for HashArrayNodeStore<H, HR,  LINES, ENTRIES>{
    fn get_value(&self, node: &TrickNode) -> Option<u8> {
        let (hash, label) = H::hash_and_label(node);
         let line = match hash.try_into(){
             Ok(index) => self.array[index],
             Err(_) => {return None}
         }[&node.current_side()];

        /*line.get_value(&label).map(|v| {
            self.hit_counter +=1;
            v
        })

         */
        line.get_value(&label)





    }

    fn store_value(&mut self, node: &TrickNode, value: u8) {
        let (hash, label) = H::hash_and_label(node);
        let mut line = match hash.try_into(){
             Ok(index) => self.array[index],
             Err(_) => {
                 panic!("Error creating usize from hash for node {node:?}.");
             }
        }[&node.current_side()];

        let hash_entry = HashEntry::new(label, value);
        match line.try_insert(hash_entry){
            Ok(_) => {}
            Err(_) => {
                warn!("Trying to store in full hash bucket. Bucket optimisation is not yet implemented")
            }
        }
    }
}


#[cfg(test)]
mod tests{
    use brydz_core::player::side::SideMap;
    use crate::hash::{HashArrayNodeStore, HashEntry, HashLine};
    use crate::hash::hash24::Hash24;
    use crate::hash::ranker::MoreCardsRanker;

    #[test]
    fn assert_option_hash_entry_array_size(){
        assert_eq!(std::mem::size_of::<Option<[HashEntry<u32>;8]>>(), 68)
    }

    #[test]
    fn assert_hash_line_size(){
        assert_eq!(std::mem::size_of::<HashLine<u32, 8>>(), 68)
    }

    #[test]
    fn assert_hash_line_map_size(){
        assert_eq!(std::mem::size_of::<SideMap<HashLine<u32, 8>>>(), 272)
    }

    #[test]
    fn assert_hash_array_size(){
        assert_eq!(std::mem::size_of::<HashArrayNodeStore<Hash24<3>, MoreCardsRanker, 0x1000000, 8>>(), 24)
        //4362076200
    }
    #[test]
    fn assert_hash24_array_size(){
        assert_eq!(std::mem::size_of::<[SideMap<HashLine<u32, 8>>;0x1000000]>(), 4563402752)
        //4362076200
    }


}

//const INITIAL_HASH_LINE: HashLine<u32, 8> = HashLine::<u32, 8>::init();