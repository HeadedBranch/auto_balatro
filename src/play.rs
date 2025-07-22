use remotro::{
    balatro::{
        hud::Hud,
        menu::Deck::Plasma,
        play::Play,
        jokers::JokerKind::*,
        deck::{
            PlayingCard,
            Rank::*, 
            Rank,
            Enhancement::*, 
            CardEdition::*,
            Seal::*,
        },
    }
};

fn get_scored_cards<'a>(selected: &[&'a PlayingCard]) -> Vec<&'a PlayingCard> {
    let mut scored: Vec<&'a PlayingCard> = Vec::new();
    for card in selected {
        if card.enhancement == Some(Stone) {
            scored.push(&card);
            continue;
        }
    }
    scored
}

pub fn score_hand(play: Play) -> f64 {
    let mut selected: Vec<&PlayingCard> = Vec::new();
    let mut hand: Vec<&PlayingCard> = Vec::new();
    for card in play.hand() {
        if let Some(card_data) = &card.card {
            if card.selected {
                selected.push(&card_data);
            } else {
                if let Some(Red) = card_data.seal {
                    hand.push(&card_data);
                }
                hand.push(&card_data);
            }
        }
    }
    if selected.is_empty() {
        return 0.0
    }
    let mut chips = play.poker_hand().unwrap().chips as f64;
    let mut mult = play.poker_hand().unwrap().mult as f64;
    let scored = if play.jokers().iter().any(|joker| joker.kind == Splash) {
        selected.into_iter().collect()
    } else {
        get_scored_cards(&selected)
    };
    for card in scored {
        match card.enhancement {
            Some(e) => match e {
                Bonus => {chips+=30.0},
                Glass => {mult*=2.0},
                Mult => {mult+=4.0},
                Lucky => {mult+=20.0},
                Stone => {chips+=50.0},
                _ => {},
            },
            None => {}
        }
        match card.edition {
            Some(e) => match e {
                Foil => {chips+=50.0},
                Holographic => {chips+=10.0},
                Polychrome => {mult*=1.5}
            },
            None => {},
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
            Joker => { mult+=4.0 },
            _ => {},
        }
    }
    if play.run_info().deck == Plasma {
        mult = ((chips+mult)/2.0).floor();
        chips = mult
    }
    chips*mult
}

fn get_chips_from_rank(rank:Rank) -> f64 {
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
        King => 10.0
    }
}
