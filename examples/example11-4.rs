use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Suit {
    Heart,
    Club,
    Spade,
    Diamond,
}

struct Card {
    rank: u8,
    suit: Suit,
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self.rank {
            x @ 2..=10 => x.to_string(),
            11 => "J".to_string(),
            12 => "Q".to_string(),
            13 => "K".to_string(),
            14 => "A".to_string(),
            _ => panic!("number {:?} is not a valid card number", &self.rank),
        };

        f.debug_struct("Card")
            .field("rank", &rank)
            .field("suit", &self.suit)
            .finish()
    }
}

impl Card {
    fn new(rank: u8, suit: Suit) -> Result<Self, &'static str> {
        match rank {
            x @ 2..=14 => Ok(Card { rank: x, suit }),
            _ => Err("name must be in the range of 2 - 14"),
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank).reverse()
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum PokerHandType {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug)]
struct PokerHand {
    cards: Vec<Card>,
    poker_hand_type: Option<PokerHandType>,
}

impl PokerHand {
    fn new() -> Self {
        PokerHand {
            cards: Vec::new(),
            poker_hand_type: None,
        }
    }

    fn add_card(&mut self, card: Card) -> Result<(), &'static str> {
        // Our card hand limit is 5.
        if self.cards.len() == 5 {
            return Err("Max hand limit is 5");
        }

        self.cards.push(card);
        Ok(())
    }

    fn card_rank_histogram(&self) -> Vec<(u8, usize)> {
        let mut map = HashMap::new();

        for card in &self.cards {
            map.entry(card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut results: Vec<(u8, usize)> = map
            .iter()
            .map(|(k, v)| (k.clone(), v.clone() as usize))
            .collect();
        results.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)).reverse());

        results
    }

    fn have_flush(&self) -> bool {
        let mut result = true;

        let suit = &self.cards[0].suit;
        for card in &self.cards[1..] {
            if card.suit != *suit {
                result = false;
                break;
            }
        }

        result
    }
}

fn main() {
    let mut hand = PokerHand::new();
    hand.add_card(Card::new(2, Suit::Heart).unwrap()).unwrap();
    hand.add_card(Card::new(9, Suit::Heart).unwrap()).unwrap();
    hand.add_card(Card::new(5, Suit::Heart).unwrap()).unwrap();
    hand.add_card(Card::new(8, Suit::Heart).unwrap()).unwrap();
    hand.add_card(Card::new(6, Suit::Heart).unwrap()).unwrap();

    assert!(hand.have_flush());

    println!("{:#?} is a flush? => {}", hand, hand.have_flush());
}
