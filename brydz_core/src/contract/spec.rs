
//use serde::{Deserialize, Serialize};
use karty::suits::{Suit, SuitTrait};
use crate::error::BiddingErrorGen::{DoubleAfterDouble, DoubleAfterReDouble, ReDoubleAfterReDouble, ReDoubleWithoutDouble};
use crate::bidding::{Doubling};
use crate::player::side::Side;
use crate::bidding::Bid;
use crate::error::BiddingErrorGen;
use crate::player::role::PlayRole;


#[derive(Debug, Eq, PartialEq,  Clone)]
#[cfg_attr(all(feature = "serde_derive", not(feature = "serde_dedicate")), derive(serde::Serialize, serde::Deserialize))]
pub struct ContractParametersGen<SU: SuitTrait> {
    declarer: Side,
    bid: Bid<SU>,
    doubling: Doubling
}

pub type ContractParameters = ContractParametersGen<Suit>;

impl<SU: SuitTrait> ContractParametersGen<SU> {
    pub fn new_d(owner: Side, bid: Bid<SU>, doubling: Doubling) -> Self{
        Self{bid, doubling, declarer: owner }
    }
    pub fn new(player: Side, bid: Bid<SU>) -> Self{
        Self{ declarer: player, bid, doubling: Doubling::None}
    }
    pub fn bid(&self) -> &Bid<SU>{
        &self.bid
    }
    pub fn doubling(&self) -> Doubling{
        self.doubling
    }
    pub fn declarer(&self) -> Side{
        self.declarer
    }
    pub fn whist(&self) -> Side{
        self.declarer.next()
    }
    pub fn dummy(&self) -> Side{
        self.declarer.next_i(2)
    }
    pub fn offside(&self) -> Side{
        self.declarer.next_i(3)
    }

    pub fn double(&mut self) -> Result<(), BiddingErrorGen<SU>>{
        match self.doubling{
            Doubling::None => {
                self.doubling = Doubling::Double;
                Ok(())
            },
            Doubling::Double => Err(DoubleAfterDouble),
            Doubling::Redouble => Err(DoubleAfterReDouble)
        }
    }

    pub fn redouble(&mut self) -> Result<(), BiddingErrorGen<SU>>{
        match self.doubling{
            Doubling::Double => {
                self.doubling = Doubling::Redouble;
                Ok(())
            },
            Doubling::Redouble => Err(ReDoubleAfterReDouble),
            Doubling::None => Err(ReDoubleWithoutDouble)
        }
    }

    pub fn map_side_to_role(&self, side: Side) -> PlayRole{
        let i = side - self.declarer;
        PlayRole::Declarer.next_i(i)
    }

    pub fn map_role_to_side(&self, role: PlayRole) -> Side{
        let i = (role - PlayRole::Declarer).index();
        self.declarer.next_i(i)
    }

}

#[cfg(feature = "serde_dedicate")]
mod serde_for_contract_spec{
    use std::fmt::Formatter;
    use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
    use serde::de::{MapAccess, SeqAccess, Visitor};
    use serde::ser::SerializeStruct;
    use karty::suits::Suit;
    use crate::contract::ContractParametersGen;

    impl Serialize for ContractParametersGen<Suit>{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let mut state = serializer.serialize_struct("contract", 3)?;
            state.serialize_field("declarer", &self.declarer)?;
            state.serialize_field("bid", &self.bid)?;
            state.serialize_field("doubling", &self.doubling)?;
            state.end()
        }
    }

    impl<'de> Deserialize<'de> for ContractParametersGen<Suit>{
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            #[derive(Deserialize)]
            #[serde(field_identifier, rename_all = "lowercase")]
            enum Field { Declarer, Bid, Doubling }
            struct ContractSpecVisitor;
            impl<'de> Visitor<'de> for ContractSpecVisitor{
                type Value = ContractParametersGen<Suit>;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("Expected struct with fields [Declarer(Side), Bid(Bid<Suit>), Doubling]")
                }
                fn visit_seq<V>(self, mut seq: V) -> Result<ContractParametersGen<Suit>, V::Error>
                where V: SeqAccess<'de> {
                    let declarer = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                    let bid = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                    let doubling = seq.next_element()?
                        .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                    Ok(ContractParametersGen::<Suit>::new_d(declarer, bid, doubling))

                }

                fn visit_map<V>(self, mut map: V) -> Result<ContractParametersGen<Suit>, V::Error>
                where
                    V: MapAccess<'de>,
                {
                    let mut declarer_op = None;
                    let mut bid_op = None;
                    let mut doubling_op = None;

                    while let Some(key) = map.next_key()?{
                        match key {
                            Field::Declarer => {
                                if declarer_op.is_some(){
                                    return Err(de::Error::duplicate_field("declarer"));
                                }
                                declarer_op = Some(map.next_value()?);
                            }
                            Field::Bid => {
                                if bid_op.is_some(){
                                    return Err(de::Error::duplicate_field("bid"));
                                }
                                bid_op = Some(map.next_value()?);
                            }
                            Field::Doubling => {
                                if doubling_op.is_some(){
                                    return Err(de::Error::duplicate_field("doubling"));
                                }
                                doubling_op = Some(map.next_value()?);
                            }
                        }
                    }
                    let declarer = declarer_op.ok_or_else(|| de::Error::missing_field("declarer"))?;
                    let bid = bid_op.ok_or_else(|| de::Error::missing_field("bid"))?;
                    let doubling = doubling_op.ok_or_else(|| de::Error::missing_field("doubling"))?;
                    Ok(ContractParametersGen::<Suit>::new_d(declarer, bid, doubling))
                }
            }
            const FIELDS: & [& str] = &["declarer", "bid", "doubling"];
            deserializer.deserialize_struct("contract", FIELDS, ContractSpecVisitor)
        }
    }
}

#[cfg(test)]
mod tests{
    use karty::suits::Suit;
    use karty::suits::Suit::Diamonds;
    use crate::bidding::Bid;
    use crate::bidding::Doubling::{Double, Redouble};
    use crate::cards::trump::TrumpGen;
    use crate::contract::ContractParametersGen;
    use crate::player::side::Side::{East, West};

    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn serialize_contract_spec(){
        let contract_1 = ContractParametersGen::new_d(
            East,
            Bid::init(TrumpGen::Colored(Diamonds), 4).unwrap(),
            Redouble
        );
        assert_eq!(ron::to_string(&contract_1).unwrap(), "(declarer:East,bid:(trump:Diamonds,number:4),doubling:Redouble)");

    }

    #[test]
    #[cfg(feature = "serde_dedicate")]
    fn deserialize_contract_spec(){
        let contract_1 = ContractParametersGen::new_d(
            West,
            Bid::init(TrumpGen::NoTrump, 6).unwrap(),
            Double
        );
        assert_eq!(ron::from_str::<ContractParametersGen<Suit>>("(declarer:West, doubling:Double, bid: (trump:NoTrump,number:6))").unwrap(), contract_1);

    }

    #[test]
    #[cfg(all(feature = "serde_derive", not(feature = "serde_dedicate")))]
    fn serialize_contract_spec_derive() {
        let contract_1 = ContractParametersGen::new_d(
            East,
            Bid::init(TrumpGen::Colored(Diamonds), 4).unwrap(),
            Redouble
        );
        assert_eq!(ron::to_string(&contract_1).unwrap(), "(declarer:East,bid:(trump:Colored(Diamonds),number:4),doubling:Redouble)");
    }

    #[test]
    #[cfg(all(feature = "serde_derive", not(feature = "serde_dedicate")))]
    fn deserialize_contract_spec_derive(){
        let contract_1 = ContractParametersGen::new_d(
            West,
            Bid::init(TrumpGen::NoTrump, 6).unwrap(),
            Double
        );
        assert_eq!(ron::from_str::<ContractParametersGen<Suit>>("(declarer:West, doubling:Double, bid: (trump:NoTrump,number:6))").unwrap(), contract_1);

    }
}