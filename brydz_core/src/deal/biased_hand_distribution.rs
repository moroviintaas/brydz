use std::ops::Index;
use log::debug;
use rand::distr::StandardUniform;
use rand::prelude::{Distribution};
use rand::Rng;
use rand::seq::SliceRandom;
use smallvec::SmallVec;
use karty::cards::{Card, DECK_SIZE, STANDARD_DECK};
use karty::figures::Figure;
use karty::set::{CardSetStd, CardSet};
use karty::suits::{Suit, SuitMap};
use karty::suits::Suit::Spades;
use karty::symbol::CardSymbol;
use crate::error::FuzzyCardSetErrorGen;
use crate::meta::HAND_SIZE;
use crate::player::side::{Side, SideMap, SIDES};
use crate::player::side::Side::{East, North, South, West};
use crate::amfiteatr::state::{FProbability, FuzzyCardSet};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct BiasedHandDistribution {
    side_probabilities: SideMap<FuzzyCardSet>
}

impl BiasedHandDistribution{

    pub fn card_probabilities(&self, card: &Card) -> SideMap<FProbability>{
        SideMap::new(
                self.side_probabilities[&North].card_probability(card),
                self.side_probabilities[&East].card_probability(card),
                self.side_probabilities[&South].card_probability(card),
                self.side_probabilities[&West].card_probability(card),
        )
    }

    fn pick_side_for_card<R: Rng + ?Sized>(&self, card: &Card, map_of_closed: &SideMap<bool>, rng: &mut R) -> Result<Side, FuzzyCardSetErrorGen<Card>>{
        let card_probabilities = self.card_probabilities(card);
        let top_north = match map_of_closed[&North]{
            true => 0.0,
            false => f32::from(card_probabilities[&North])
        };
        let top_east = match map_of_closed[&East]{
            true => top_north,
            false => top_north + f32::from(card_probabilities[&East])
        };
        let top_south = match map_of_closed[&South]{
            true => top_east,
            false => top_east + f32::from(card_probabilities[&South])
        };
        let top_west = match map_of_closed[&West]{
            true => top_south,
            false => top_south + f32::from(card_probabilities[&West])
        };

        if top_west == 0.0{
            return Err(FuzzyCardSetErrorGen::ImpossibleSideSelection)
        }
        let sample = rng.random_range(0f32..top_west);
        if sample < top_north{
            return Ok(North);
        } else if sample < top_east{
            return  Ok(East)
        } else if sample < top_south{
            return Ok(South);
        } else if sample < top_west{
            return Ok(West)
        }

        Err(FuzzyCardSetErrorGen::ImpossibleSideSelection)


    }

    fn check_if_auto_alloc_uncertain(&self,
                                     side: &Side,
                                     allocated_cards: &SideMap<CardSetStd>,
                                     uncertain_card_nums: &SideMap<u8>)
                                     -> Result<bool, FuzzyCardSetErrorGen<Card>>{

        if allocated_cards[side].len() as u8 == self.side_probabilities[side].expected_card_number(){
            return Ok(false)
        }
        let needed_cards = self.side_probabilities[side].expected_card_number() - allocated_cards[side].len() as u8;
        if uncertain_card_nums[side] < needed_cards{
            return Err(FuzzyCardSetErrorGen::OutOfUncertainCardsForSide(*side));
        }
        if uncertain_card_nums[side] > needed_cards{
            Ok(false)
        }
        else{
            Ok(true)
        }


    }

    fn distribute_uncertain_cards_when_sure(&self,
                                            _side: &Side,
                                            card_set_to_insert: &mut SideMap<CardSetStd>,
                                            used_cards_register: &mut CardSetStd,
                                            numbers_of_uncertain: &mut SideMap<u8>,
                                            cards_with_zero: &SmallVec<[Card; 64]>,
                                            cards_uncertain: &SmallVec<[Card; 64]>)
                                            -> Result<(), FuzzyCardSetErrorGen<Card>>{

        for side in SIDES{
            if self.check_if_auto_alloc_uncertain(&side, card_set_to_insert, numbers_of_uncertain)?{
                self.alloc_all_uncertain_to_side(&side,
                                                 card_set_to_insert ,
                                                 cards_with_zero,
                                                 used_cards_register,
                                                 numbers_of_uncertain)?;
                self.alloc_all_uncertain_to_side(&side,
                                                 card_set_to_insert,
                                                 cards_uncertain,
                                                 used_cards_register,
                                                 numbers_of_uncertain)?;
            }
        }

        Ok(())

    }

    fn alloc_all_uncertain_to_side(&self,
                                   side: &Side,
                                   card_set_to_insert: &mut SideMap<CardSetStd>,
                                   cards_to_pick_from: &SmallVec<[Card; 64]>,
                                   used_cards_register: &mut CardSetStd,
                                   numbers_of_uncertain: &mut SideMap<u8>) -> Result<(), FuzzyCardSetErrorGen<Card>>{

        for c in cards_to_pick_from{

            if !used_cards_register.contains(c){
                if let FProbability::Uncertain(_) = self.side_probabilities[side][c] {
                    card_set_to_insert[side].insert_card(*c)?;
                    used_cards_register.insert_card(*c)?;
                    for s in SIDES{
                        if let FProbability::Uncertain(_) = self.side_probabilities[&s][c] {
                            numbers_of_uncertain[&s] = match numbers_of_uncertain[&s].checked_sub(1){
                                None => {
                                    //panic!("Probably bad use of alloc_all_uncertain_to_side, number of uncertain dropping below 0");
                                    return Err(FuzzyCardSetErrorGen::OutOfUncertainCardsForSide(s))
                                }
                                Some(i) => i
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    //fn set_card_as_used(&self, card: &Card, )


    pub fn sample_deal_single_try<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<SideMap<CardSetStd>, FuzzyCardSetErrorGen<Card>>{
        let mut deal = SideMap::new_symmetric(CardSetStd::empty());
        let mut cards_uncertain: SmallVec<[Card; 64]> = SmallVec::new();
        let mut cards_with_zero: SmallVec<[Card; 64]> = SmallVec::new();
        let mut closed_sides = SideMap::new_symmetric(false);

        let mut cards = STANDARD_DECK;
        cards.shuffle(rng);
        for c in cards{
            let card_probabilities = self.card_probabilities(&c);
            match choose_certain(&card_probabilities)?{
                None => match choose_certain_zero(&card_probabilities)?{
                    None => {
                        cards_uncertain.push(c);

                    }
                    Some(_s) => {
                        cards_with_zero.push(c);

                    }
                }

                Some(side) => {
                    deal[&side].insert_card(c)?;
                    if deal[&side].len() >= HAND_SIZE{
                        closed_sides[&side] = true;
                    }
                }
            }
        }

        for c in cards_with_zero {
            let side = self.pick_side_for_card(&c, &closed_sides, rng)?;
            deal[&side].insert_card(c)?;
            if deal[&side].len() >= HAND_SIZE {
                closed_sides[&side] = true;
            }
        }

        for c in cards_uncertain{
            let side = self.pick_side_for_card(&c, &closed_sides, rng)?;
            deal[&side].insert_card(c)?;
            if deal[&side].len() >= HAND_SIZE {
                closed_sides[&side] = true;
            }
        }

        Ok(deal)
    }

    pub fn sample_deal_crossing<R: Rng + ?Sized>(&self, rng: &mut R) -> Result<SideMap<CardSetStd>, FuzzyCardSetErrorGen<Card>>{




        let mut card_sets = SideMap::new_symmetric(CardSetStd::empty());
        let mut cards_uncertain: SmallVec<[Card; 64]> = SmallVec::new();
        let mut cards_with_zero: SmallVec<[Card; 64]> = SmallVec::new();

        //let mut distributed_card_numbers = SideMap::new_symmetric(0u8);
        let mut cards_distributed = CardSetStd::empty();


        let mut numbers_of_uncertain = SideMap::new_symmetric(0u8);

        let mut closed_sides = SideMap::new_symmetric(false);

        let mut cards = STANDARD_DECK;
        cards.shuffle(rng);


        //phase 1, alloc certain One
        for c in cards{
            let card_probabilities = self.card_probabilities(&c);
            match choose_certain(&card_probabilities)?{
                None => {
                    for side in SIDES{
                        match card_probabilities[&side]{

                            FProbability::Uncertain(_) => {
                                numbers_of_uncertain[&side] +=1;
                            }
                            FProbability::Bad(b) => {
                                return Err(FuzzyCardSetErrorGen::BadProbability(b))
                            }
                            _ => {}
                        }
                    }
                    match choose_certain_zero(&card_probabilities)?{
                        None => {
                            cards_uncertain.push(c);

                        }
                        Some(_s) => {
                            cards_with_zero.push(c);

                        }
                    }

                }
                Some(side) => {
                    card_sets[&side].insert_card(c)?;
                    cards_distributed.insert_card(c)?;
                    if card_sets[&side].len() >= HAND_SIZE{
                        closed_sides[&side] = true;
                    }
                }
            }

        }

        // now if for some side number of remaining uncertain probabilities



        //for side in SIDES

        // phase 2: alloc these with zero

        for c in &cards_with_zero{
            if !cards_distributed.contains(c){
                for side in SIDES{
                    self.distribute_uncertain_cards_when_sure(&side,
                                                              &mut card_sets, &mut cards_distributed,
                                                              &mut numbers_of_uncertain,
                                                              &cards_with_zero,
                                                              &cards_uncertain)?;

                }


                let side = self.pick_side_for_card(c, &closed_sides, rng)?;
                card_sets[&side].insert_card(*c)?;
                cards_distributed.insert_card(*c)?;
                for s in SIDES{
                    if let FProbability::Uncertain(_) = self.side_probabilities[&s][c] {
                        numbers_of_uncertain[&s] = match numbers_of_uncertain[&s].checked_sub(1){
                            None => {
                               return Err(FuzzyCardSetErrorGen::OutOfUncertainCardsForSide(s))
                            }
                            Some(i) => i
                        }
                    }
                }
                if card_sets[&side].len() >= HAND_SIZE{
                    closed_sides[&side] = true;
                }
            }



        }

        for c in cards_uncertain{
            if !cards_distributed.contains(&c){
                let side = self.pick_side_for_card(&c, &closed_sides, rng)?;
                card_sets[&side].insert_card(c)?;
                cards_distributed.insert_card(c)?;
                if card_sets[&side].len() >= HAND_SIZE{
                    closed_sides[&side] = true;
                }
            }

        }

        Ok(card_sets)

    }
}

impl Index<Side> for BiasedHandDistribution {
    type Output = FuzzyCardSet;

    fn index(&self, index: Side) -> &Self::Output {
        &self.side_probabilities[&index]
    }
}



impl Distribution<BiasedHandDistribution> for StandardUniform{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BiasedHandDistribution {

        let mut sides_shuffled = SIDES;
        loop{
            let mut probabilities = SideMap::new_symmetric(SuitMap::new_from_f(|_|[0.0f32; HAND_SIZE]));
            let mut sums_per_side = SideMap::new_symmetric(0.0f32);
            for i in 0..DECK_SIZE-1{
                let s = Suit::from_usize_index(i/13).unwrap();
                let f = i%13;
                //let mut inner_iteration = 0;
                loop{
                    sides_shuffled.shuffle(rng);
                    //inner_iteration += 1;

                    let proba_1:f32 = rng.random_range(0.0..=1.0);
                    let proba_2: f32 = rng.random_range(0.0..=1.0);
                    let proba_3: f32 = rng.random_range(0.0..=1.0);

                    let proba_4: f32 = 1.0 - (proba_1 + proba_2 + proba_3);

                    if proba_4 >=0.0{
                        probabilities[&sides_shuffled[0]][s][f] = proba_1;
                        probabilities[&sides_shuffled[1]][s][f] = proba_2;
                        probabilities[&sides_shuffled[2]][s][f] = proba_3;
                        probabilities[&sides_shuffled[3]][s][f] = proba_4;

                        sums_per_side[&North] += probabilities[&North][s][f];
                        sums_per_side[&East] += probabilities[&East][s][f];
                        sums_per_side[&South] += probabilities[&South][s][f];
                        sums_per_side[&West] += probabilities[&West][s][f];

                        break;
                    }
                    //debug!("For card {i:} resampling")

                }




            }
            if sums_per_side[&North] > 13.0{
                debug!("North with probability_sum over 13: {}", sums_per_side[&North]);
                continue;
            }
            if sums_per_side[&East] > 13.0{
                debug!("East with probability_sum over 13: {}", sums_per_side[&East]);
                continue;
            }
            if sums_per_side[&South] > 13.0{
                debug!("South with probability_sum over 13: {}", sums_per_side[&South]);
                continue;
            }
            if sums_per_side[&West] > 13.0{
                debug!("West with probability_sum over 13: {}", sums_per_side[&West]);
                continue;
            }
            //debug!("Probabilities sum: {:?}", sums_per_side);
            for side in SIDES{
                probabilities[&side][Spades][Figure::SYMBOL_SPACE-1] = 13.0 - sums_per_side[&side];
            }

            return BiasedHandDistribution { side_probabilities: SideMap::new(
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&North]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&East]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&South]).unwrap(),
                FuzzyCardSet::new_from_f32_derive_sum(probabilities[&West]).unwrap()) }

        }


    }
}

impl Default for BiasedHandDistribution{
    fn default() -> Self {
        Self{side_probabilities: SideMap::new_symmetric(
            FuzzyCardSet::new_from_f32_derive_sum(SuitMap::new_symmetric([0.25;13])).unwrap()) }
    }
}



fn choose_certain(probabilities: &SideMap<FProbability>) -> Result<Option<Side>, FuzzyCardSetErrorGen<Card>>{
    for side in SIDES{
        if probabilities[&side] == FProbability::One{
            let probability_sum = probabilities
                .fold_on_ref(0.0f32, |acc, x|{
                    acc + f32::from(*x)
                });
            if probability_sum == 1.0{
                return Ok(Some(side))
            } else {
                return Err(FuzzyCardSetErrorGen::BadProbabilitiesSum{expected: 1.0, found: probability_sum})
            }
        }
    }
    Ok(None)
}

fn choose_certain_zero(probabilities: &SideMap<FProbability>) -> Result<Option<Side>, FuzzyCardSetErrorGen<Card>> {
    for side in SIDES{
        if probabilities[&side] == FProbability::Zero{
            return Ok(Some(side))
        }
    }
    Ok(None)
}



impl Distribution<SideMap<CardSetStd>> for BiasedHandDistribution{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SideMap<CardSetStd> {

        //let mut distribution_

        match self.sample_deal_crossing(rng){
            Ok(p) => p,
            Err(e) => {
                panic!("Error sampling cards from distribution {e:?}")
            }
        }


    }
}