use std::str::FromStr;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::error::ErrorKind;
use nom::IResult;
use nom::multi::{fold_many0, many, many0};
use nom::sequence::{delimited, pair, tuple};
use karty::cards::Card;
use karty::figures::{Figure, parse_figure};
use karty::suits::SuitMap;
use karty::symbol::CardSymbol;
use crate::error::FuzzyCardSetErrorGen;
use crate::amfiteatr::state::FuzzyCardSet;
use super::card_probability::FProbability;
use nom::Parser;

pub fn parse_proba_prefix(s: &str) -> IResult<&str, FProbability>{
    delimited(
        tag("[0."),
        digit1,
        tag("]")

    ).parse(s).and_then(|(rem, frac)|{
        let s = "0.".to_owned() + frac;
        match s.parse::<f32>(){
            Ok(proba) => {
                if !(0.0..=1.0).contains(&proba){
                    //Ok((rem, FProbability::Bad(proba)))
                    Err(nom::Err::Error(nom::error::Error::new("Bad probsbility (p<0 or p>1)", ErrorKind::Digit)))
                    //panic!("Bad probability");
                } else if proba == 0.0{
                    Ok((rem, FProbability::Zero))
                } else if proba == 1.0{
                    Ok((rem, FProbability::One))
                } else{
                    Ok((rem, FProbability::Uncertain(proba)))
                }
            }
            Err(_e) => {
                //panic!("Error paring float");
                Err(nom::Err::Failure(nom::error::Error::new("Failed parsing float from str", nom::error::ErrorKind::Digit)))
            }
        }



    })
}

pub fn parse_1_prefix(s: &str) -> IResult<&str, FProbability>{

    (
        tag("[1."),
        many0(tag("0")),
        tag("]")

        ).parse(s).and_then(|(rem, _) | Ok((rem, FProbability::One)))

}

pub fn parse_uncertain_figure(s: &str) -> IResult<&str, (FProbability, Figure)> {
    pair(parse_proba_prefix, parse_figure).parse(s)//(s).map(|(rem, (proba, fig))|)
}

pub fn parse_verbose_certain_figure(s: &str) -> IResult<&str, (FProbability, Figure)>{
    pair(parse_1_prefix, parse_figure).parse(s)//(s).map(|(rem, (proba, fig))|)
}

pub fn parse_certain_figure(s: &str) -> IResult<&str, (FProbability, Figure)>{
    parse_figure(s).map(|(rem, fig)| (rem, (FProbability::One, fig)))
}

pub fn parse_probable_figure(s: &str) -> IResult<&str, (FProbability, Figure)>{
    alt((parse_uncertain_figure, parse_certain_figure, parse_verbose_certain_figure)).parse(s)

}


pub fn parse_fuzzy_card_set(s: &str) -> IResult<&str, FuzzyCardSet>{
    type ProbaArray = [FProbability; Figure::SYMBOL_SPACE];
    (
        fold_many0(
            parse_probable_figure,
            ProbaArray::default,
            |mut set: ProbaArray, (probability, fig)|{
                set[fig.usize_index()] = probability;
                set
            }
        ),
        tag("."),
        fold_many0(
            parse_probable_figure,
            ProbaArray::default,
            |mut set: ProbaArray, (probability, fig)|{
                set[fig.usize_index()] = probability;
                set
            }
        ),
        tag("."),
        fold_many0(
            parse_probable_figure,
            ProbaArray::default,
            |mut set: ProbaArray, (probability, fig)|{
                set[fig.usize_index()] = probability;
                set
            }
        ),
        tag("."),
        fold_many0(
            parse_probable_figure,
            ProbaArray::default,
            |mut set: ProbaArray, (probability, fig)|{
                set[fig.usize_index()] = probability;
                set
            }
        )
        ).parse(s).map(|(rem, (s, _, h, _, d, _, c))|
        (rem, FuzzyCardSet::new_derive_sum(SuitMap::new(s, h, d, c)).unwrap()))
}


impl FromStr for FuzzyCardSet{
    type Err = FuzzyCardSetErrorGen<Card>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_fuzzy_card_set(s){
            Ok((_rem, cs)) => Ok(cs),
            Err(_e) => Err(FuzzyCardSetErrorGen::Parse)
        }
    }
}

#[cfg(test)]
mod tests{
    use std::str::FromStr;
    use approx::assert_abs_diff_eq;
    use nom::IResult;
    use karty::figures::F10;
    use crate::amfiteatr::state::{FProbability, FuzzyCardSet, parse_probable_figure};
    use karty::cards::*;

    #[test]
    fn parse_uncertain_figure_from_str(){
        let input = "[0.3]Taa";
        let x = parse_probable_figure(input);
        assert_eq!(x , IResult::Ok(("aa", (FProbability::Uncertain(0.3), F10))));
    }

    #[test]
    fn parse_certain_figure_from_str(){
        let input = "Taa";
        let x = parse_probable_figure(input);
        assert_eq!(x , IResult::Ok(("aa", (FProbability::One, F10))));
    }

    #[test]
    fn fuzzy_card_set_from_str_correct(){
        let card_set = FuzzyCardSet::from_str("[0.4]AT86[0.6]2.KJT93.4T.2A").unwrap();
        assert_abs_diff_eq!(f32::from(card_set[&ACE_SPADES]), 0.4, epsilon=0.001);
        assert_abs_diff_eq!(f32::from(card_set[&TWO_DIAMONDS]), 0.0, epsilon=0.001);
        assert_abs_diff_eq!(f32::from(card_set[&TWO_SPADES]), 0.6, epsilon=0.001);
        assert_abs_diff_eq!(card_set.sum_probabilities(), 13.0, epsilon=0.001);
    }
}