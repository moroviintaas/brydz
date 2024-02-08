pub trait Label: Eq + PartialEq + Clone + Copy{
    fn unused() -> Self;
}





impl Label for [u8;3]{
    fn unused() -> Self {
        [0,0,0]
    }
}

impl Label for [u8;4]{
    fn unused() -> Self {
        [0,0,0,0]
    }
}
impl Label for u32{
    fn unused() -> Self {
        u32::MAX
    }
}