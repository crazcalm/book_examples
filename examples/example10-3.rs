use std::fmt;

#[derive(Debug)]
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

fn main() {
    for rank in 2..=14 {
        println!("{:?}", Card::new(rank, Suit::Heart).unwrap());
    }
}
