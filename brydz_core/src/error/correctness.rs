use std::fmt::{Debug, Display};
use std::error::Error;
#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
pub enum Correctness<Entry: Debug + Display + Clone, E: Error>{
    Correct(Entry),
    Wrong(Entry, E)

}