pub struct SideMapKeyIter<'a>{
    keys: [&'a Side; 4],
    index: usize,
}

pub struct SideMapIter<'a, V: 'a>{
    keys: [Side; 4],
    map: &'a SideMap<V>,
    index: usize
}

impl<'a> Default for SideMapKeyIter<'a>{
    fn default() -> Self {
        SideMapKeyIter{
            keys: [&North, &East, &West, &South],
            index: 0
        }
    }
}

impl<'a, V:'a> SideMapIter<'a, V>{
    pub fn new(map: &'a SideMap<V>) -> Self{
        Self{
            keys: [North, East, West, South],
            index: 0,
            map
        }
    }
}

impl<'a> Iterator for SideMapKeyIter<'a>{
    type Item = &'a Side;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index{
            n@ 0..=3 => {
                self.index +=1;
                Some(self.keys[n])
            },
            _ => None
        }
    }
}
impl<'a, V: 'a> Iterator for SideMapIter<'a, V>{
    type Item = (Side, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self.index{
            n@ 0..=3 => {
                self.index +=1;
                Some((self.keys[n], &self.map[&self.keys[n]]))
            },
            _ => None
        }
    }
}

/// ```
/// use brydz_core::player::side::SideMap;
/// use sztorm::SMap;
/// let map = SideMap::new_symmetric(true);
/// assert!(map.all(|v| *v ))
/// ```
impl<'a, V: 'a> SMap<'a, Side, V> for SideMap<V>
where Self: 'a{
    type Iter = SideMapIter<'a, V>;

    fn get_value(&self, key: Side) -> Option<&V> {
        Some(&self[&key])
    }

    fn get_mut_value(&mut self, key: Side) -> Option<&mut V> {
        Some(&mut self[&key])
    }

    fn iter(&'a self) -> Self::Iter {
        SideMapIter::new(&self)
    }
}

