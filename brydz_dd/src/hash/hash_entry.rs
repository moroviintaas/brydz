use crate::hash::Label;

#[derive(Clone, Copy, Debug )]
pub struct HashEntry<L: Label>{
    label: L,
    value: u8
}

impl <L: Label> HashEntry<L>{
    pub fn new(label: L, value: u8) -> Self{
        Self{label, value}
    }
    pub fn label(&self) -> &L{
        &self.label
    }
    pub fn value(&self) -> u8{
        self.value
    }
}



#[cfg(test)]
mod tests{
    use std::mem::size_of;
    use crate::hash::HashEntry;

    #[test]
    fn hash_entry_standard_size(){
        assert_eq!(size_of::<HashEntry<u32>>(), 8);
    }
}