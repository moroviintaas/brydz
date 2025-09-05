use std::cmp::Ordering;
use karty::suits::SuitTrait;
use crate::error::BiddingErrorGen::{BidTooLow, DoubleAfterDouble, DoubleAfterReDouble, DoubleOnSameAxis, DoubleOnVoidCall, ReDoubleAfterReDouble, ReDoubleOnSameAxis, ReDoubleOnVoidCall, ReDoubleWithoutDouble, ViolatedOrder};
use crate::bidding::call::{Call, CallEntry, Doubling};

use crate::bidding::bid::{Bid};
use crate::bidding::declaration_storage::DeclarationStorage;
use crate::contract::ContractParametersGen;
use crate::error::{BiddingErrorGen, Mismatch};

use crate::player::side::Side;



#[derive(Debug, Eq, PartialEq, Copy ,Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AuctionStatus{
    Running(Side),
    Finished,

}


#[derive(Debug, Eq, PartialEq,  Clone)]
pub struct AuctionStack<SU: SuitTrait, DS: DeclarationStorage<SU>>{
    calls_entries: Vec<CallEntry<SU>>,
    current_contract: Option<ContractParametersGen<SU>>,
    declaration_storage: DS,

}

impl<SU: SuitTrait, DS: DeclarationStorage<SU>> AuctionStack<SU, DS>{
    pub fn new() -> Self{
        Self{ calls_entries: Vec::new(), current_contract: None,
            declaration_storage: DS::default()}

    }

    pub fn current_contract(&self) -> Option<&ContractParametersGen<SU>>{
        match &self.current_contract{
            Some(x) => Some(x),
            None => None
        }

    }

    pub fn last_passes(&self) -> u8{
        let mut counter = 0u8;
        for it in self.calls_entries.iter().rev(){
            match it.call(){
                Call::Pass => {
                    counter +=1;
                },
                _ => break
            }
        }
        counter
    }

    pub fn current_bid(&self) -> Option<&Bid<SU>>{
        self.current_contract.as_ref().map(|c| c.bid())
    }

    pub fn add_contract_bid(&mut self, player_side: Side, call: Call<SU>) -> Result<AuctionStatus, BiddingErrorGen<SU>>{
        match self.current_contract{
            None => {
                // First bid, must not be double or redouble
                match call{
                    Call::Pass=> {
                        self.calls_entries.push(CallEntry::new(player_side, call));
                        //self.last_player = Some(player_side);
                        Ok(AuctionStatus::Running(player_side.next()))
                    },
                    Call::NewBid(ref bid) => {
                        self.calls_entries.push(CallEntry::new(player_side, call.to_owned()));

                        match self.declaration_storage.get_declarer(player_side.axis(), bid.trump()){
                            None => {
                                self.declaration_storage.set_declarer(player_side, bid.trump().to_owned());
                                self.current_contract = Some(ContractParametersGen::new(player_side, bid.to_owned(), ));
                            },
                            Some(s) => {
                                self.current_contract = Some(ContractParametersGen::new(s.to_owned(), bid.to_owned(), ));
                            }

                        }
                        Ok(AuctionStatus::Running(player_side.next()))
                    }
                    Call::Double => Err(DoubleOnVoidCall),
                    Call::Redouble => Err(ReDoubleOnVoidCall)


                }
            },
            _ => {
                match player_side{
                    next if next == self.calls_entries.last().unwrap().player_side().next() =>{
                        //good order
                        match call{
                            Call::Pass => match self.last_passes(){
                                0 | 1  => {
                                    self.calls_entries.push(CallEntry::new(player_side, call));
                                    Ok(AuctionStatus::Running(player_side.next()))
                                },
                                _ => {
                                    self.calls_entries.push(CallEntry::new(player_side, call));
                                    Ok(AuctionStatus::Finished)
                                }

                            },
                            Call::NewBid(ref bid) => match bid.cmp( self.current_bid().unwrap()){
                                Ordering::Greater => {

                                    match self.declaration_storage.get_declarer(player_side.axis(), bid.trump()){
                                        None => {
                                            self.declaration_storage.set_declarer(player_side, bid.trump().to_owned());
                                            self.current_contract = Some(ContractParametersGen::new(player_side, bid.to_owned(), ));
                                        },
                                        Some(s) => {
                                            self.current_contract = Some(ContractParametersGen::new(s.to_owned(), bid.to_owned(), ));
                                        }

                                    }
                                    self.calls_entries.push(CallEntry::new(player_side, call));

                                    Ok(AuctionStatus::Running(player_side.next()))
                                }
                                _ => Err(BidTooLow(Mismatch{ expected: self.current_bid().unwrap().to_owned(), found:bid.to_owned()}))

                            },
                            Call::Double => match &self.current_contract.as_ref().unwrap().doubling(){
                                Doubling::None => match  self.current_contract.as_ref().unwrap().declarer().axis(){
                                    same if same ==player_side.axis() => Err(DoubleOnSameAxis),
                                    _different => {
                                        //self.current_contract.as_mut().unwrap().doubling() = Doubling::Double;
                                        self.current_contract.as_mut().unwrap().double()?;
                                        self.calls_entries.push(CallEntry::new(player_side, call));

                                        Ok(AuctionStatus::Running(player_side.next()))
                                    }


                                }
                                Doubling::Double => Err(DoubleAfterDouble),
                                Doubling::Redouble => Err(DoubleAfterReDouble)
                            }
                            Call::Redouble => match &self.current_contract.as_ref().unwrap().doubling(){
                                Doubling::None => Err(ReDoubleWithoutDouble),
                                Doubling::Double => match self.current_contract.as_ref().unwrap().declarer().axis() {
                                    same if same == player_side.axis() => {
                                        //self.current_contract.as_mut().unwrap().doubling = Doubling::ReDouble;
                                        self.current_contract.as_mut().unwrap().redouble()?;
                                        self.calls_entries.push(CallEntry::new(player_side, call));
                                        Ok(AuctionStatus::Running(player_side.next()))

                                    },
                                    _different => Err(ReDoubleOnSameAxis)
                                },
                                Doubling::Redouble => Err(ReDoubleAfterReDouble)

                            }
                        }
                    },
                    found => Err(ViolatedOrder(Mismatch{ expected: self.calls_entries.last().unwrap().player_side().next(), found} ))
                }

            }
        }
    }
}
impl<SU: SuitTrait, DS: DeclarationStorage<SU>> Default for AuctionStack<SU, DS> {
     fn default() -> Self {
         Self::new()
     }
}

#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::{Clubs, Diamonds};
    use crate::cards::trump::TrumpGen::Colored;
    use crate::error::{BiddingErrorGen, Mismatch};
    use crate::error::BiddingErrorGen::{BidTooLow, DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
    use crate::bidding::auction_field::{AuctionStack};
    use crate::bidding::Bid;
    use crate::player::side::Side::{East, North, South, West};
    use crate::bidding::call::{Call, Doubling};
    use crate::bidding::bid::consts::{ BID_C1, BID_C2, BID_C3, BID_S2};
    use crate::bidding::declaration_storage::GeneralDeclarationStorage;
    use crate::contract::ContractParametersGen;

    #[test]
    fn add_bids_legal(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();
        auction_stack.add_contract_bid(South, Call::Pass).unwrap();
        assert_eq!(auction_stack.current_contract, None);
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)
        /*{
            owner: West,
            bid: Bid::create_bid(Colored(Clubs), 1).unwrap(),
            doubling: Doubling::None
        }*/));
        auction_stack.add_contract_bid(North, Call::NewBid(
            Bid::init(Colored(Diamonds), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            North,
            Bid::init(Colored(Diamonds), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();

        auction_stack.add_contract_bid(South, Call::NewBid(
            Bid::init(Colored(Diamonds), 2).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            North,
            Bid::init(Colored(Diamonds), 2).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(West, Call::Double).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            North,
            Bid::init(Colored(Diamonds), 2).unwrap(),
            Doubling::Double)));
        auction_stack.add_contract_bid(North, Call::Redouble).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            North,
            Bid::init(Colored(Diamonds), 2).unwrap(),
            Doubling::Redouble)));

    }

    #[test]
    fn violate_auction_order(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        let r = auction_stack.add_contract_bid(South, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap()));
        assert_eq!(r, Err(BiddingErrorGen::ViolatedOrder(Mismatch{ expected: North, found: South})));

    }

    #[test]
    fn double_after_double(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::Pass).unwrap();
        let r = auction_stack.add_contract_bid(South, Call::Double);
        assert_eq!(r, Err(DoubleAfterDouble));
    }

    #[test]
    fn redouble_after_redouble(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::Redouble).unwrap();
        auction_stack.add_contract_bid(South, Call::Pass).unwrap();
        let r = auction_stack.add_contract_bid(West, Call::Redouble);
        assert_eq!(r, Err(ReDoubleAfterReDouble));
    }

    #[test]
    fn double_after_redouble(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        auction_stack.add_contract_bid(North, Call::Double).unwrap();
        auction_stack.add_contract_bid(East, Call::Redouble).unwrap();
        let r = auction_stack.add_contract_bid(South, Call::Double);
        assert_eq!(r, Err(DoubleAfterReDouble));
    }

    #[test]
    fn redouble_without_double(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 1).unwrap())).unwrap();
        assert_eq!(auction_stack.current_contract, Some(ContractParametersGen::new_d(
            West,
            Bid::init(Colored(Clubs), 1).unwrap(),
            Doubling::None)));
        let r = auction_stack.add_contract_bid(North, Call::Redouble);
        assert_eq!(r, Err(ReDoubleWithoutDouble));
    }

    #[test]
    fn bid_too_low(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(
            Bid::init(Colored(Clubs), 2).unwrap())).unwrap();

        let r = auction_stack.add_contract_bid(North, Call::NewBid(
            Bid::init(Colored(Diamonds), 1).unwrap()));
        assert_eq!(r, Err(BidTooLow(Mismatch{
            expected: Bid::init(Colored(Clubs), 2).unwrap(),
            found: Bid::init(Colored(Diamonds), 1).unwrap() })));
    }

    #[test]
    fn declarer_simple(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(BID_C1)).unwrap();
        auction_stack.add_contract_bid(North, Call::NewBid(BID_C2)).unwrap();
        auction_stack.add_contract_bid(East, Call::NewBid(BID_S2)).unwrap();

        assert_eq!(auction_stack.current_contract().unwrap().declarer(), East);

    }

    #[test]
    fn declarer_partner(){
        let mut auction_stack = AuctionStack::<Suit, GeneralDeclarationStorage<Suit>>::new();
        auction_stack.add_contract_bid(West, Call::NewBid(BID_C1)).unwrap();
        auction_stack.add_contract_bid(North, Call::NewBid(BID_C2)).unwrap();
        auction_stack.add_contract_bid(East, Call::NewBid(BID_C3)).unwrap();

        assert_eq!(auction_stack.current_contract().unwrap().declarer(), West);

    }






}