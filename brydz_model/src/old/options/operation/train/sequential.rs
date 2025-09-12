use amfiteatr_rl::tch::nn::{Path, Sequential};

pub trait SequentialGen{

    fn build_sequential(&self, path: &Path) -> Sequential;
}

//#[derive(Clone)]
pub struct SequentialBuilder<F: Fn(&Path) -> Sequential> {

    sequential_fn: F
}

impl<F: Fn(&Path) -> Sequential> SequentialBuilder<F>{

    pub fn new(f: F) -> Self{
        Self{sequential_fn: f}
    }
}

impl <F: Fn(&Path) -> Sequential> SequentialGen for SequentialBuilder<F>{
    fn build_sequential(&self, path: &Path) -> Sequential {
        (self.sequential_fn)(path)
    }
}

pub type SequentialB = SequentialBuilder<Box<dyn Fn(&Path) -> Sequential>>;

impl<T: SequentialGen> SequentialGen for Box<T>{
    fn build_sequential(&self, path: &Path) -> Sequential {
        self.as_ref().build_sequential(path)
    }
}