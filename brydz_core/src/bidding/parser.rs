use nom::branch::alt;
use nom::bytes::complete::{tag_no_case};
use nom::character::complete::{digit1, space0};
use nom::IResult;
use nom::sequence::{delimited, separated_pair};
use crate::bidding::bid::Bid;
use crate::cards::trump::TrumpGen;
use nom::error::ErrorKind;
use karty::suits::parse::parse_suit;
use karty::suits::Suit;


///Parses no trump strict
///```
/// use brydz_core::bidding::parser::parse_nt;
/// use brydz_core::cards::trump::TrumpGen;
/// assert_eq!(parse_nt("nt "), Ok((" ", TrumpGen::NoTrump)));
/// assert_eq!(parse_nt("notrump"), Ok(("", TrumpGen::NoTrump)));
/// assert_eq!(parse_nt("n"), Ok(("", TrumpGen::NoTrump)));
/// ```
pub fn parse_nt(s: &str) -> IResult<&str, TrumpGen<Suit>>{
    alt((tag_no_case("no_trump"), tag_no_case("notrump"), tag_no_case("nt"), tag_no_case("n")))(s)
        .map(|(i,_)| (i, TrumpGen::NoTrump ))
}
///Parses no trump (delimited)
///```
/// use brydz_core::bidding::parser::parse_nt_delimited;
/// use brydz_core::cards::trump::TrumpGen;
/// assert_eq!(parse_nt_delimited("\tnt \t"), Ok(("", TrumpGen::NoTrump)));
/// assert_eq!(parse_nt_delimited("  notrump\t"), Ok(("", TrumpGen::NoTrump)));
/// assert_eq!(parse_nt_delimited("  n "), Ok(("", TrumpGen::NoTrump)));
/// ```
pub fn parse_nt_delimited(s: &str) -> IResult<&str, TrumpGen<Suit>>{
    delimited(space0, parse_nt, space0)(s)
}

/// Parses colored trump
/// ```
/// use brydz_core::bidding::parser::parse_trump_colored;
/// use karty::suits::Suit::{Spades, Hearts};
/// use brydz_core::cards::trump::TrumpGen;
/// assert_eq!(parse_trump_colored("hjik"), Ok(("jik", TrumpGen::Colored(Hearts))));
/// assert_eq!(parse_trump_colored("spadesorsth"), Ok(("orsth", TrumpGen::Colored(Spades))));
/// ```
pub fn parse_trump_colored(s: &str) -> IResult<&str, TrumpGen<Suit>>{
    parse_suit(s).map(|(r, s)| (r, TrumpGen::Colored(s)))
}

/// Parses trump
/// ```
/// use brydz_core::bidding::parser::{parse_nt, parse_trump_colored};
/// use karty::suits::Suit::{Spades, Hearts};
/// use brydz_core::cards::trump::TrumpGen;
/// assert_eq!(parse_trump_colored("hjik"), Ok(("jik", TrumpGen::Colored(Hearts))));
/// assert_eq!(parse_trump_colored("spadesorsth"), Ok(("orsth", TrumpGen::Colored(Spades))));
/// assert_eq!(parse_nt("notrump\t"), Ok(("\t", TrumpGen::NoTrump)));
/// ```
pub fn parse_trump(s: &str) -> IResult<&str, TrumpGen<Suit>>{
    alt((parse_trump_colored, parse_nt))(s)
}
/// parses bid
/// ```
/// use brydz_core::bidding::parser::parse_bid;
/// use brydz_core::bidding::Bid;
/// use karty::suits::Suit::Clubs;
/// use brydz_core::cards::trump::TrumpGen;
/// use nom::error::ErrorKind;
/// assert_eq!(parse_bid("3c"), Ok(("", Bid::init(TrumpGen::Colored(Clubs), 3).unwrap())));
/// assert_eq!(parse_bid("7nt "), Ok((" ", Bid::init(TrumpGen::NoTrump, 7).unwrap())));
/// assert_eq!(parse_bid("0hearts"), Err(nom::Err::Error(nom::error::Error::new("0hearts", ErrorKind::Digit))));
/// assert_eq!(parse_bid("q2spades"), Err(nom::Err::Error(nom::error::Error::new("q2spades", ErrorKind::Digit))));
/// assert_eq!(parse_bid("8spades"), Err(nom::Err::Error(nom::error::Error::new("8spades", ErrorKind::Digit))));
/// assert_eq!(parse_bid("h"), Err(nom::Err::Error(nom::error::Error::new("h", ErrorKind::Digit))));
/// ```
pub fn parse_bid(s: &str) -> IResult<&str, Bid<Suit>>{
    match separated_pair(digit1, space0, parse_trump)(s){
        Ok((remains, (digs, trump))) => match digs.parse::<u8>(){
            Ok(n) => Bid::init(trump, n).map_or_else(
                |_| Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit))),
                | bid|  Ok((remains, bid))),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(s, ErrorKind::Digit)))
        },
        Err(e) => Err(e)
    }
        //.map(|(i, (digs, trump))| )
}
