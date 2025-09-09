use remotro::balatro::deck::Suit::{Clubs, Spades};
use remotro::balatro::{
    deck::{
        CardEdition::*,
        Enhancement::*,
        PlayingCard,
        Rank::{self, *},
        Seal::*,
    },
    hud::Hud,
    jokers::JokerKind::*,
    menu::Deck::Plasma,
    play::{
        Play,
        PokerHandKind::{self, *},
    },
};

fn get_scored_cards<'a>(
    selected: &mut Vec<&'a PlayingCard>,
    hand_type: &PokerHandKind,
    shortcut: bool,
    // TODO: Add support for smeared joker
) -> Vec<&'a PlayingCard> {
    const FIVE_CARD_HANDS: [PokerHandKind; 4] = [FiveOfAKind, FlushFive, FlushHouse, FullHouse];
    // Hands that always require 5 cards to exist
    let mut scored: Vec<&'a PlayingCard> = Vec::new();
    if FIVE_CARD_HANDS.contains(hand_type) {
        return selected.to_vec();
    };
    for card in &mut *selected {
        if card.enhancement == Some(Stone) {
            scored.push(card);
        }
    }
    selected.sort_unstable_by_key(|c| c.rank);
    match hand_type {
        HighCard => {
            if let Some(card) = selected.last() {
                scored.push(card);
            }
        }
        Pair | ThreeOfAKind | FourOfAKind => {
            let count = match hand_type {
                Pair => 2,
                ThreeOfAKind => 3,
                FourOfAKind => 4,
                _ => unreachable!(),
            };
            if let Some(i) = selected
                .windows(count)
                .position(|w| w.iter().all(|c| c.rank == w[0].rank))
            {
                scored.extend_from_slice(&selected[i..i + count]);
            }
        }
        TwoPair => {
            if let Some((i, j)) = selected
                .windows(2)
                .enumerate()
                .filter(|(_, w)| w[0].rank == w[1].rank)
                .map(|(i, _)| i)
                .collect::<Vec<_>>()
                .windows(2)
                .next()
                .map(|w| (w[0], w[1]))
            {
                scored.extend_from_slice(&selected[i..i + 2]);
                scored.extend_from_slice(&selected[j..j + 2]);
            }
        }
        Straight => {
            if selected.windows(2).all(|w| w[1].rank == w[0].rank.next()) {
                scored.extend_from_slice(selected); // 5 card Straight
            } else if shortcut
                && selected
                    .windows(2)
                    .all(|w| w[1].rank == w[0].rank.next().next())
            {
                scored.extend_from_slice(selected); // 5 card Straight with shortcut
            } else if let Some(i) = selected
                .windows(4)
                .position(|w| w.windows(2).all(|w| w[1].rank == w[0].rank.next()))
            {
                scored.extend_from_slice(&selected[i..i + 4]); // 4 card straight (Four Fingers)
            } else if shortcut
                && let Some(i) = selected
                    .windows(4)
                    .position(|w| w.windows(2).all(|w| w[1].rank <= w[0].rank.next().next()))
            {
                scored.extend_from_slice(&selected[i..i + 4]); // 4 card straight with shortcut
            }
        }
        Flush => 'block: {
            if selected.iter().all(|c| c.suit == selected[0].suit) {
                scored.extend_from_slice(selected);
                break 'block;
            }
            selected.sort_unstable_by_key(|a| a.suit);
            if let Some(i) = selected
                .windows(4)
                .position(|w| w.iter().all(|c| c.suit == w[0].suit))
            {
                scored.extend_from_slice(&selected[i..i + 4]);
            }
        }
        StraightFlush => 'block: {
            if selected.windows(2).all(|w| w[1].rank <= w[0].rank.next())
                && selected.iter().all(|c| c.suit == selected[0].suit)
            {
                scored.extend_from_slice(selected);
                break 'block;
            }
            selected.sort_by_key(|c| c.suit);
        }
        _ => unreachable!(),
    }
    scored
}

pub fn score_hand(play: &Play) -> f64 {
    let mut selected: Vec<&PlayingCard> = Vec::new();
    let mut hand: Vec<&PlayingCard> = Vec::new();
    for card in play.hand() {
        if let Some(card_data) = &card.card {
            if card.selected {
                selected.push(card_data);
            } else {
                if let Some(Red) = card_data.seal {
                    hand.push(card_data);
                }
                hand.push(card_data);
            }
        }
    }
    if selected.is_empty() {
        return 0.0;
    }
    let mut chips = play.poker_hand().unwrap().chips as f64;
    let mut mult = play.poker_hand().unwrap().mult as f64;
    let scored = if play.jokers().iter().any(|joker| joker.kind == Splash) {
        selected.into_iter().collect()
    } else {
        get_scored_cards(
            &mut selected,
            &play.poker_hand().unwrap().kind,
            play.jokers().iter().any(|j| j.kind == FourFingers),
        )
    };
    for card in scored {
        if let Some(e) = card.enhancement {
            match e {
                Bonus => chips += 30.0,
                Glass => mult *= 2.0,
                Mult => mult += 4.0,
                Lucky => mult += 20.0,
                Stone => chips += 50.0,
                _ => {}
            }
        }
        if let Some(e) = card.edition {
            match e {
                Foil => chips += 50.0,
                Holographic => chips += 10.0,
                Polychrome => mult *= 1.5,
            }
        }
        chips += get_chips_from_rank(card.rank);
    }
    for card in hand {
        if card.enhancement == Some(Steel) {
            mult *= 1.5
        }
        //RaisedFist => mult += get_chips_from_rank(play.hand().iter().filter(|c| !c.selected).min_by_key(|c| c.card.as_ref().unwrap().rank).unwrap().card.as_ref().unwrap().rank),
    }
    for joker in play.jokers() {
        match joker.kind {
            Joker => mult += 4.0,
            Jolly => { if play.poker_hand().unwrap().kind == Pair { mult += 8.0; } }
            Zany => { if play.poker_hand().unwrap().kind == ThreeOfAKind { mult += 12.0; } }
            Mad => { if play.poker_hand().unwrap().kind == TwoPair { mult += 10.0; } }
            Crazy => { if play.poker_hand().unwrap().kind == Straight { mult += 12.0; } }
            Droll => { if play.poker_hand().unwrap().kind == Flush { mult += 10.0; } }
            Sly => { if play.poker_hand().unwrap().kind == Pair { chips += 50.0; } }
            Wily => { if play.poker_hand().unwrap().kind == ThreeOfAKind { chips += 100.0; } }
            Clever => { if play.poker_hand().unwrap().kind == TwoPair { chips += 80.0; } }
            Devious => { if play.poker_hand().unwrap().kind == Straight { chips += 100.0; } }
            Crafty => { if play.poker_hand().unwrap().kind == Flush { chips += 80.0; } }
            Half => { if play.hand().iter().filter(|c| c.selected).count() <= 3 { mult += 20.0 } }
            Stencil {xmult} => mult *= xmult as f64,
            Ceremonial {mult: jmult} => mult += jmult as f64,
            Banner => chips += 30.0*play.discards() as f64,
            MysticSummit => { if play.discards() == 0 { mult += 15.0 } }
            LoyaltyCard {left} => { if left == 0 { mult *= 4.0 } }
            Misprint => mult += 23.0,
            SteelJoker {xmult} => mult *= xmult,
            Abstract {mult:jmult} => mult += jmult as f64,
            GrosMichel => mult += 15.0,
            Supernova => mult += play.run_info().poker_hands.high_card.played as f64,
            Blackboard => { if play.hand().iter().filter(|c| !c.selected).all(|c| c.card.as_ref().unwrap().suit == Spades || c.card.as_ref().unwrap().suit == Clubs) { mult *= 3.0 } }
            Runner {chips: jchips} => chips += jchips as f64,
            IceCream {chips: jchips} => chips += jchips as f64,
            BlueJoker {chips: jchips} => chips += jchips as f64,
            Constellation {xmult} => mult *= xmult,
            GreenJoker {mult: jmult} => mult += jmult as f64,
            TodoList {poker_hand} => { if poker_hand == play.poker_hand().unwrap().kind { todo!("Implement money gain")}}
            Cavendish => mult *= 3.0,
            CardSharp => todo!(),
            RedCard {mult:jmult} => mult += jmult as f64,
            Square {chips: jchips} => chips += jchips as f64,
            Madness {xmult} => mult *= xmult,
            Vampire {xmult} => mult *= xmult,
            Hologram {xmult} => mult *= xmult,
            Obelisk {xmult} => mult *= xmult,
            _ => {}
        }
    }
    if play.run_info().deck == Plasma {
        mult = ((chips + mult) / 2.0).floor();
        chips = mult
    }
    println!("{:?} {chips} {mult}", play.poker_hand().unwrap().kind);
    chips * mult
}

fn get_chips_from_rank(rank: Rank) -> f64 {
    match rank {
        Ace => 11.0,
        Two => 2.0,
        Three => 3.0,
        Four => 4.0,
        Five => 5.0,
        Six => 6.0,
        Seven => 7.0,
        Eight => 8.0,
        Nine => 9.0,
        Ten => 10.0,
        Jack => 10.0,
        Queen => 10.0,
        King => 10.0,
    }
}
