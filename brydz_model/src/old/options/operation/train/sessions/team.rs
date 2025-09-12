
#[derive(Copy, Clone, Debug)]
pub enum Team{
    Contractors,
    Defenders
}
impl Team{
    pub fn opposite(&self) -> Team{
        match self{
            Team::Contractors => Team::Defenders,
            Team::Defenders => Team::Contractors
        }
    }
}
