/*mod hash24;
pub use hash24::*;
mod hash_table;
pub use hash_table::*;

pub trait PartialHash<Hsh: Sized, EntryDistinguish: Sized>{


    fn partial_hash(&self) -> Hsh;
    fn entry_distinguish(&self) -> EntryDistinguish;
    fn check(&self, hash: &Hsh, entry: &EntryDistinguish) -> bool;
    //fn reconstruct(hash: &Hsh, distinct: &Dist) -> Self;
}

*/

mod hasher;
mod hash_store;
mod hash_entry;
mod label;
pub mod ranker;


pub use hasher::*;
pub use label::*;
pub use hash_store::*;
pub use hash_entry::*;


