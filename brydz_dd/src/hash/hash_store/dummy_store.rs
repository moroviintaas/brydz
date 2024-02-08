use crate::hash::NodeStoreTrait;
use crate::node::TrickNode;

#[derive(Copy, Clone, Default, Debug)]
pub struct DummyNodeStore{

}



impl NodeStoreTrait for DummyNodeStore{
    fn get_value(&self, _node: &TrickNode) -> Option<u8> {
        None
    }

    fn store_value(&mut self, _node: &TrickNode, _value: u8) {
        
    }
}