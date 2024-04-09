use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use karty::suits::{Suit, SuitTrait};
use crate::cards::trump::TrumpGen;
use crate::error::BiddingErrorGen;
use crate::error::BiddingErrorGen::IllegalBidNumber;
use crate::meta::{HALF_TRICKS, MAX_BID_NUMBER, MIN_BID_NUMBER};



#[cfg(feature="speedy")]
use crate::speedy::{Readable, Writable};

#[cfg_attr(feature = "speedy", derive(Writable, Readable))]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(all(feature = "serde_derive", not(feature = "serde_dedicate")), derive(serde::Serialize, serde::Deserialize))]
pub struct Bid<S: SuitTrait> {
    trump: TrumpGen<S>,
    number: u8
}



/*
#[cfg(feature = "serde")]
impl Serialize for Bid<Suit>{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {

        let sym = match self.trump{
            TrumpGen::Colored(c) => match c{
                Suit::Spades => "S",
                Suit::Hearts => "H",
                Suit::Diamonds => "D",
                Suit::Clubs => "C"
            }
            TrumpGen::NoTrump => "NT"
        };

        serializer.serialize_str(&format!("{}{}", self.number, sym))



        let mut state = serializer.serialize_struct("Bid", 2)?;
        state.serialize_field("trump", )


    }
}*/


pub type BidStd = Bid<Suit>;

impl <S: SuitTrait + Copy> Copy for Bid<S>{}

impl<S: SuitTrait>  Bid<S> {
    pub fn init(trump: TrumpGen<S>, number: u8) -> Result<Self, BiddingErrorGen<S>>{
        match number{
            legit @MIN_BID_NUMBER..=MAX_BID_NUMBER => Ok(Self{trump, number: legit}),
            no_legit => Err(IllegalBidNumber(no_legit))

        }
    }
    pub fn trump(&self) -> &TrumpGen<S>{
        &self.trump
    }
    pub fn number(&self) -> u8{
        self.number
    }
    pub fn number_normalised(&self) -> u8{
        self.number + HALF_TRICKS
    }
}
impl<S: SuitTrait> PartialOrd for Bid<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        /*
        Some(self.number.cmp(&other.number).then_with(|| {
            self.trump.cmp(&other.trump)
        }))

         */
        Some(std::cmp::Ord::cmp(self, other))
    }
}

impl<S: SuitTrait + Display> Display for Bid<S>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        write!(f, "{:?}", self)
    }
}


/// Delivers `Ord` for `Bid`
/// ```
/// use std::cmp::Ordering;
/// use brydz_core::cards::trump::TrumpGen::{Colored, NoTrump};
/// use karty::suits::Suit::*;
/// use brydz_core::bidding::Bid;
/// let bid1 = Bid::init(NoTrump, 2).unwrap();
/// let bid2 = Bid::init(Colored(Spades), 3).unwrap();
/// let bid3 = Bid::init(Colored(Clubs), 3).unwrap();
/// let bid4 = Bid::init(Colored(Hearts), 4).unwrap();
/// let bid5 = Bid::init(Colored(Diamonds), 2).unwrap();
/// assert!(bid1 < bid2);
/// assert!(bid2 > bid3);
/// assert!(bid2 < bid4);
/// assert!(bid1 > bid5);
/// ```
impl<S: SuitTrait> Ord for Bid<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number).then_with(||{
            self.trump.cmp(&other.trump)
        })


    }
}

#[cfg(feature = "serde_dedicate")]
mod serde_dedicate{
    use serde::de;
    use crate::bidding::bid::Suit;
    use crate::bidding::Bid;
    use serde::de::MapAccess;
    use serde::de::SeqAccess;
    use serde::de::Visitor;
    use serde::{Deserializer, Serializer, Deserialize, Serialize};
    use serde::ser::SerializeStruct;
    use std::fmt::Formatter;

    impl Serialize for Bid<Suit>{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let mut state = serializer.serialize_struct("bid", 2)?;
            state.serialize_field("trump", &self.trump)?;
            state.serialize_field("number", &self.number)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for Bid<Suit>{

        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field { Trump, Number }

            struct BidVisitor;
            impl<'de> Visitor<'de> for BidVisitor{
                type Value = Bid<Suit>;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("Expected struct with field Trump(Spades/Hearts/Diamonds/Clubs/NoTrump) and number (0..=7)")
                }
                fn visit_seq<V>(self, mut seq: V) -> Result<Bid<Suit>, V::Error>
                where
                    V: SeqAccess<'de>,
                {
                    let trump = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let number = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    Bid::<Suit>::init(trump, number).map_err(|e| de::Error::custom(&format!("Error deserializing bid: {e:}")[..]))

                }

                fn visit_map<V>(self, mut map: V) -> Result<Bid<Suit>, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut trump_op = None;
                    let mut number_op = None;
                    while let Some(key) = map.next_key()? {
                        match key {
                            Field::Trump => {
                                if trump_op.is_some() {
                                    return Err(de::Error::duplicate_field("trump"));
                                }
                                trump_op = Some(map.next_value()?);
                            }
                            Field::Number => {
                                if number_op.is_some() {
                                    return Err(de::Error::duplicate_field("number"));
                                }
                                number_op = Some(map.next_value()?);
                            }
                        }
                    }
                    let trump = trump_op.ok_or_else(|| de::Error::missing_field("trump"))?;
                    let number = number_op.ok_or_else(|| de::Error::missing_field("number"))?;
                    Bid::<Suit>::init(trump, number).map_err(|e| de::Error::custom(&format!("Error deserializing bid: {e:}")[..]))
                }
            }
            const FIELDS: &[& str] = &["trump", "number"];
            deserializer.deserialize_struct("bid", FIELDS, BidVisitor)
        }
    }
}





pub mod consts {
    use karty::suits::Suit::{Clubs, Diamonds, Hearts, Spades};
    use crate::bidding::Bid;
    use crate::cards::trump::TrumpGen;

    pub const BID_C1: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 1 };
    pub const BID_C2: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 2 };
    pub const BID_C3: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 3 };
    pub const BID_C4: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 4 };
    pub const BID_C5: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 5 };
    pub const BID_C6: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 6 };
    pub const BID_C7: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Clubs), number: 7 };

    pub const BID_D1: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 1 };
    pub const BID_D2: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 2 };
    pub const BID_D3: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 3 };
    pub const BID_D4: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 4 };
    pub const BID_D5: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 5 };
    pub const BID_D6: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 6 };
    pub const BID_D7: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Diamonds), number: 7 };

    pub const BID_H1: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 1 };
    pub const BID_H2: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 2 };
    pub const BID_H3: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 3 };
    pub const BID_H4: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 4 };
    pub const BID_H5: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 5 };
    pub const BID_H6: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 6 };
    pub const BID_H7: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Hearts), number: 7 };

    pub const BID_S1: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 1 };
    pub const BID_S2: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 2 };
    pub const BID_S3: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 3 };
    pub const BID_S4: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 4 };
    pub const BID_S5: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 5 };
    pub const BID_S6: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 6 };
    pub const BID_S7: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::Colored(Spades), number: 7 };

    pub const BID_NT1: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 1 };
    pub const BID_NT2: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 2 };
    pub const BID_NT3: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 3 };
    pub const BID_NT4: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 4 };
    pub const BID_NT5: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 5 };
    pub const BID_NT6: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 6 };
    pub const BID_NT7: Bid<karty::suits::Suit> = Bid { trump: TrumpGen::NoTrump, number: 7 };
}

#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::Hearts;
    use crate::bidding::Bid;
    use crate::cards::trump::TrumpGen;

    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn serialize_bid(){
        let bid_1 = Bid::init(TrumpGen::Colored(Hearts), 2).unwrap();
        let bid_2 = Bid::init(TrumpGen::<Suit>::NoTrump, 4).unwrap();
        assert_eq!(ron::to_string(&bid_1).unwrap(), "(trump:Hearts,number:2)");
        assert_eq!(ron::to_string(&bid_2).unwrap(), "(trump:NoTrump,number:4)");
    }


    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn deserialize_bid(){

        let bid_1 = Bid::init(TrumpGen::Colored(Hearts), 2).unwrap();
        let bid_2 = Bid::init(TrumpGen::<Suit>::NoTrump, 4).unwrap();
        assert_eq!(bid_1, ron::from_str::<Bid<Suit>>("(trump:Hearts,number:2)").unwrap());
        assert_eq!(bid_2, ron::from_str::<Bid<Suit>>("(trump:NoTrump,number:4)").unwrap());
    }
    #[test]
    #[cfg(all(feature = "serde_derive", not(feature = "serde_dedicate")))]
    fn serialize_bid_derive(){
        let bid_1 = Bid::init(TrumpGen::Colored(Hearts), 2).unwrap();
        let bid_2 = Bid::init(TrumpGen::<Suit>::NoTrump, 4).unwrap();
        assert_eq!(ron::to_string(&bid_1).unwrap(), "(trump:Colored(Hearts),number:2)");
        assert_eq!(ron::to_string(&bid_2).unwrap(), "(trump:NoTrump,number:4)");
    }


}