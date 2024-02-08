mod dummy_store;
mod hash_array_store;

pub use dummy_store::*;
pub use hash_array_store::*;
use crate::node::TrickNode;

pub trait NodeStoreTrait: Default{
    fn get_value(&self, node: &TrickNode) -> Option<u8>;
    fn store_value(&mut self, node: &TrickNode, value: u8);
}