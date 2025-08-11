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
) -> Vec<&'a PlayingCard> {
    const FIVE_CARD_HANDS: [PokerHandKind; 4] = [FiveOfAKind, FlushFive, FlushHouse, FullHouse];
    let mut scored: Vec<&'a PlayingCard> = Vec::new();
    if FIVE_CARD_HANDS.contains(hand_type) {
        return selected.to_vec();
    };
    for card in &mut *selected {
        if card.enhancement == Some(Stone) {
            scored.push(card);
            continue;
        }
    }
    selected.sort_unstable_by_key(|a| a.rank);
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
                println!("{:?}", &selected[i..i + count]);
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
                scored.extend_from_slice(selected);
            }
            if let Some(i) = selected
                .windows(4)
                .position(|w| w.windows(2).all(|w| w[1].rank == w[0].rank.next()))
            {
                scored.extend_from_slice(&selected[i..i + 4]); // TODO Add support for shortcut
            }
        }
        Flush => {
            if selected.iter().all(|c| c.suit == selected[0].suit) {
                scored.extend_from_slice(selected);
            }
            selected.sort_unstable_by_key(|a| a.suit);
            if let Some(i) = selected
                .windows(4)
                .position(|w| w.iter().all(|c| c.suit == w[0].suit))
            {
                scored.extend_from_slice(&selected[i..i + 4]);
            }
        }
        _ => {} // TODO Add support for StraightFlush
    }
    scored
}

pub fn score_hand(play: Play) -> f64 {
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
        get_scored_cards(&mut selected, &play.poker_hand().unwrap().kind)
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
    }
    for joker in play.jokers() {
        match joker.kind {
            Joker => mult += 4.0,

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
