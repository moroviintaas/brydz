use std::fmt::Formatter;
use nom::Finish;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{Error, Visitor};
use smallvec::SmallVec;
use karty::figures::Figure;
use karty::suits::Suit::*;
use karty::symbol::CardSymbol;
use crate::amfi::state::{FProbability, FuzzyCardSet, parse_fuzzy_card_set};

impl Serialize for FuzzyCardSet{
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
            let cards_in_suits = [Spades, Hearts, Diamonds, Clubs].iter().map(|suit|{
                let mut suit_string = String::new();
                for i in (0..self.probabilities[suit].len()).rev(){

                    match self.probabilities[suit][i]{
                        FProbability::One => {
                            suit_string += &Figure::from_usize_index(i).unwrap().repr_char().to_string();
                        }
                        FProbability::Zero => {}
                        FProbability::Uncertain(p) => {
                            suit_string = suit_string + "[" + &format!("{:.2}", p) + "]" + &Figure::from_usize_index(i).unwrap().repr_char().to_string();
                        }
                        FProbability::Bad(p) => {
                            suit_string = suit_string + "[" + &format!("{:.2}", p) + "]" + &Figure::from_usize_index(i).unwrap().repr_char().to_string();
                        }
                    }
                }
                suit_string
            }).collect::<SmallVec<[String;4]>>();
            let result = format!("{}.{}.{}.{}", cards_in_suits[0], cards_in_suits[1], cards_in_suits[2], cards_in_suits[3]);
            serializer.serialize_str(&result)


        }
    }

impl<'de> Deserialize<'de> for FuzzyCardSet{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        struct FuzzyCardSetVisitor;

        impl<'de> Visitor<'de> for FuzzyCardSetVisitor{
            type Value = FuzzyCardSet;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {

                formatter.write_str("Expected string \"<SPADES>.<HEARTS>.<DIAMONDS>.<CLUBS>\" example: [0.4]AT86[0.6]2.KJT93.4T.2A ")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: Error {
                parse_fuzzy_card_set(v).finish()
                    .map(|(_i, cs)|cs  )
                    .map_err(|e| E::custom(format!("Error parsing FuzzyCardSet: {e:}")))
            }
        }

        deserializer.deserialize_str(FuzzyCardSetVisitor)

    }
}

#[cfg(test)]
mod tests{
    use approx::assert_abs_diff_eq;
    use karty::figures::Figure;
    use karty::suits::SuitMap;
    use karty::symbol::CardSymbol;
    use crate::amfi::state::FuzzyCardSet;
    use karty::cards::*;

    type ProbaArrayF32 = [f32; Figure::SYMBOL_SPACE];
    const CARDS_CLUBS: ProbaArrayF32 =       [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    const CARDS_DIAMONDS: ProbaArrayF32 =   [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    const CARDS_HEARTS: ProbaArrayF32 =     [0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3, 0.3];
    const CARDS_SPADES: ProbaArrayF32 =     [0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.35, 0.0];


    #[test]
    fn fuzzy_card_set_serialize(){
        let card_map = SuitMap::new(CARDS_SPADES, CARDS_HEARTS, CARDS_DIAMONDS, CARDS_CLUBS);
        let card_set = FuzzyCardSet::new_check_epsilon(card_map, 13).unwrap();
        assert_eq!(ron::to_string(&card_set).unwrap(),
                   "\"[0.35]K[0.35]Q[0.35]J[0.35]T[0.35]9[0.35]8[0.35]7[0.35]6[0.35]5[0.35]4[0.35]3[0.35]2.\
                   [0.30]A[0.30]K[0.30]Q[0.30]J[0.30]T[0.30]9[0.30]8[0.30]7[0.30]6[0.30]5[0.30]4[0.30]3[0.30]2.\
                   2.\
                   [0.30]A[0.30]K[0.30]Q[0.30]J[0.30]T[0.30]9[0.30]8[0.30]7[0.30]6[0.30]5[0.30]4[0.30]3[0.30]2\"")
    }

    #[test]
    fn fuzzy_card_set_deserialize(){
        let card_set = ron::from_str::<FuzzyCardSet>("\"[0.35]K[0.35]Q[0.35]J[0.35]T[0.35]9[0.35]8[0.35]7[0.35]6[0.35]5[0.35]4[0.35]3[0.35]2.\
                   [0.3]A[0.3]K[0.3]Q[0.3]J[0.3]T[0.3]9[0.3]8[0.3]7[0.3]6[0.3]5[0.3]4[0.3]3[0.3]2.\
                   2.\
                   [0.3]A[0.3]K[0.3]Q[0.3]J[0.3]T[0.3]9[0.3]8[0.3]7[0.3]6[0.3]5[0.3]4[0.3]3[0.3]2\"").unwrap();

        assert_abs_diff_eq!(f32::from(card_set.card_probability(&THREE_HEARTS)), 0.3, epsilon = 0.001);
        assert_abs_diff_eq!(f32::from(card_set.card_probability(&ACE_SPADES)), 0.0, epsilon = 0.001);
        assert_abs_diff_eq!(f32::from(card_set.card_probability(&TWO_DIAMONDS)), 1.0, epsilon = 0.001);
    }
}

