use std::ops::{Index, IndexMut};
use crate::player::side::{Side, SIDES};
use crate::player::side::Side::{East, North, South, West};
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

/// ```
/// use brydz_core::player::side::SideMap;
/// use karty::cards::Card;
/// assert_eq!(std::mem::size_of::<SideMap<Card>>(), 12)
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[cfg_attr(any(feature = "serde_derive", feature = "serde_dedicate"), derive(serde::Serialize, serde::Deserialize))]
pub struct SideMap<T>{
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T
}

impl<T> SideMap<T>{
    pub fn new(north: T, east: T, south: T, west:T) -> Self{
        Self{north, east, south, west}
    }
    pub fn new_with_fn<F>(f: F) -> Self where F: Fn(Side) -> T{
        Self{
            north: f(North),
            east: f(East),
            south: f(South),
            west: f(West),
        }
    }
    pub fn new_symmetric(sym: T)  -> Self where T: Clone{
        Self{north: sym.clone(), east: sym.clone(), south: sym.clone(), west: sym }
    }
    pub fn and<F: Fn(&T) -> bool >(&self, f:F) -> bool{
        f(&self.north) && f(&self.east) && f(&self.south) && f(&self.west)
    }
    pub fn or<F: Fn(&T) -> bool + Copy>(&self, f:F) -> bool{
        f(&self.north) || f(&self.east) || f(&self.south) || f(&self.west)
    }
    pub fn transform<D, F: FnOnce(&T) -> D + Copy>(&self, f: F) -> SideMap<D>{
        SideMap {north: f(&self.north), south: f(&self.south), east: f(&self.east), west: f(&self.west)}
    }
    pub fn find<F: FnOnce(&T) -> bool + Copy>(&self, f: F) -> Option<Side>{
        for s in SIDES{
            if f(&self[&s]){
                return Some(s)
            }
        }
        None
    }

    pub fn merge<F: Fn(&T, &T) -> T> (&self, f:F) -> T{
        let ns = f(&self.north, &self.south);
        let we = f(&self.west, &self.east);
        f(&ns, &we)
    }

    pub fn destruct(self) -> (T,T,T,T){
    (self.north, self.east, self.south, self.west)
    }
    pub fn destruct_start_with(self, side: Side) -> (T,T,T,T){
        match side{
            East => (self.east, self.south, self.west, self.north),
            South => (self.south, self.west, self.north, self.east),
            West => (self.west, self.north, self.east, self.south),
            North => (self.north, self.east, self.south, self.west)
        }
    }
    pub fn select_best_fit<C: Ord, F: Fn(&T) -> C>(&self, fit: F) -> Side{
        let mut max = fit(&self.north);
        let mut best = North;
        for side in &SIDES[0..]{
            let tmp = fit(&self[side]);
            if  tmp > max{
                best = *side;
                max = tmp;
            }
        }
        best
    }

    /// Rotates map setting `point_before` to `point_after`.
    /// # Example:
    /// ```
    /// use brydz_core::player::side::Side::{East, North, South, West};
    /// use brydz_core::player::side::SideMap;
    /// let mut sm = SideMap::new(0,1,2,3);
    /// //What was at [North] will be now at [East]
    /// sm.rotate(North, East);
    /// assert_eq!(sm, SideMap::new(3,0,1,2));
    /// sm.rotate(North, West);
    /// assert_eq!(sm, SideMap::new(0,1,2,3));
    /// sm.rotate(North, South);
    /// assert_eq!(sm, SideMap::new(2,3,0,1));
    /// ```
    pub fn rotate(&mut self, point_before: Side, point_after: Side){
        let rhs = point_after - point_before;

        match rhs {
            0 => {},
            1 => {
                std::mem::swap(&mut self.north, &mut self.east);
                std::mem::swap(&mut self.north, &mut self.south);
                std::mem::swap(&mut self.north, &mut self.west);
            },
            2 => {
                std::mem::swap(&mut self.north, &mut self.south);
                std::mem::swap(&mut self.east, &mut self.west);
            },
            3 => {
                std::mem::swap(&mut self.north, &mut self.west);
                std::mem::swap(&mut self.north, &mut self.south);
                std::mem::swap(&mut self.north, &mut self.east);
            },
            _ => panic!("Should not happen")
        }

    }

    pub fn fold_on_ref<B, F>(&self, init: B, f: F) -> B
    where
        Self: Sized,
        F: Fn(B, &T) -> B{

        let mut acc = init;
        for side in SIDES{
            acc = f(acc, &self[&side])
        }
        acc
    }

}
impl<T> Index<&Side> for SideMap<T>{
    type Output = T;

    fn index(&self, index: &Side) -> &Self::Output {
        match index{
            East => &self.east,
            South => &self.south,
            West => &self.west,
            North => &self.north
        }
    }
}

impl<T> IndexMut<&Side> for SideMap<T>{
    fn index_mut(&mut self, index: &Side) -> &mut Self::Output {
        match index{
            East => &mut self.east,
            South => &mut self.south,
            West => &mut self.west,
            North => &mut self.north
        }
    }
}

impl<T: Eq> SideMap<T>{
    pub fn are_all_equal(&self) -> bool{
        let t_north = &self.north;
        self.and(|c| c== t_north)
    }
}

