use util::read_input;
use std::collections::{HashSet, VecDeque};
use std::iter::Iterator;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Card {
    value: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    fn score(&self) -> usize {
        self.cards.iter().rev().enumerate().map(|(i, card)| (i + 1) * card.value).sum()
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn draw(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }

    fn place_on_bottom(&mut self, card: Card) {
        self.cards.push_back(card);
    }

    fn sub_deck(&self, n: usize) -> Deck {
        Deck {
            cards: self.cards.iter().cloned().take(n).collect(),
        }
    }
}

fn combat(player_1_deck: &Deck, player_2_deck: &Deck) -> usize {
    let mut player_1_deck = player_1_deck.clone();
    let mut player_2_deck = player_2_deck.clone();

    loop {
        if player_1_deck.is_empty() {
            return player_2_deck.score();
        }
        if player_2_deck.is_empty() {
            return player_1_deck.score();
        }
        
        let player_1_card = player_1_deck.draw().unwrap();
        let player_2_card = player_2_deck.draw().unwrap();

        if player_1_card.value > player_2_card.value {
            player_1_deck.place_on_bottom(player_1_card);
            player_1_deck.place_on_bottom(player_2_card);
        } else {
            player_2_deck.place_on_bottom(player_2_card);
            player_2_deck.place_on_bottom(player_1_card);
        }
    }
}

enum Winner {
    Player1,
    Player2,
}

fn recursive_combat_internal(player_1_deck: &mut Deck, player_2_deck: &mut Deck) -> Winner {
    let mut prev_decks = HashSet::new();

    loop {
        if prev_decks.contains(&(player_1_deck.clone(), player_2_deck.clone())) {
            return Winner::Player1;
        }
        prev_decks.insert((player_1_deck.clone(), player_2_deck.clone()));

        if player_1_deck.is_empty() {
            return Winner::Player2;
        }
        if player_2_deck.is_empty() {
            return Winner::Player1;
        }
        
        let player_1_card = player_1_deck.draw().unwrap();
        let player_2_card = player_2_deck.draw().unwrap();

        let winner = if player_1_deck.len() >= player_1_card.value && player_2_deck.len() >= player_2_card.value {
            recursive_combat_internal(&mut player_1_deck.sub_deck(player_1_card.value), &mut player_2_deck.sub_deck(player_2_card.value))
        } else if player_1_card.value > player_2_card.value {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => {
                player_1_deck.place_on_bottom(player_1_card);
                player_1_deck.place_on_bottom(player_2_card);
            }
            Winner::Player2 => {
                player_2_deck.place_on_bottom(player_2_card);
                player_2_deck.place_on_bottom(player_1_card);
            }
        }
    }
}

fn recursive_combat(player_1_deck: &Deck, player_2_deck: &Deck) -> usize {
    let mut player_1_deck = player_1_deck.clone();
    let mut player_2_deck = player_2_deck.clone();

    match recursive_combat_internal(&mut player_1_deck, &mut player_2_deck) {
        Winner::Player1 => player_1_deck.score(),
        Winner::Player2 => player_2_deck.score(),
    }
}

fn parse_player_deck(iter: &mut Iterator<Item = &String>) -> Deck {
    let mut cards = VecDeque::new();
    for line in iter {
        if line.starts_with("Player") {
            continue;
        }
        if line.is_empty() {
            break;
        }
        cards.push_back(Card {value: usize::from_str_radix(line, 10).unwrap()});
    }
    Deck {cards}
}

fn parse_players(input: &[String]) -> (Deck, Deck) {
    let mut iter = input.iter();
    (parse_player_deck(&mut iter), parse_player_deck(&mut iter))
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let input = read_input::<String>(&args[1]).collect::<Vec<String>>();

    let (player_1_deck, player_2_deck) = parse_players(&input);
    println!("{}", combat(&player_1_deck, &player_2_deck));
    println!("{}", recursive_combat(&player_1_deck, &player_2_deck));
}
