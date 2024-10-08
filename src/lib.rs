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

/* TODO: Fix in book --

When I compare two hands, I want the greater hand to return Ordering::Greater.
To make sure that happens, I needed to put the "greater" poker hand type at the bottom.
*/
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum PokerHandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

#[derive(Debug, PartialEq, Eq)]
struct PokerHand {
    cards: Vec<Card>,
    poker_hand_type: Option<PokerHandType>,
}

impl PartialOrd for PokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for PokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        /*
        Note: I call `reverse` on the `self.cards.cmp` because Cards default ordering is reversed and made 11 less than 10 so that
        when we sorted our hands, the higher number was in front. Now that we are comparing the values of the hands, we need
        11 to be greater than 10, so we must `reverse` the outcome of comparing the vector of cards. In layman terms, the second
        `reverse` negates the affects of the first `reverse` giving us the default comparison for a vector of numbers.
         */
        self.poker_hand_type
            .clone()
            .expect("Must call sort_hand before sorting")
            .cmp(
                &other
                    .poker_hand_type
                    .clone()
                    .expect("Must call sort_hand_before sorting"),
            )
            .then(self.cards.cmp(&other.cards).reverse())
    }
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

    fn have_straight(&self) -> bool {
        let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.rank.clone()).collect();

        ranks.sort();

        if ranks == [2, 3, 4, 5, 14] {
            true
        } else {
            let mut result = true;

            let mut expected = ranks[0];
            for num in ranks {
                if num != expected {
                    result = false;
                    break;
                }

                expected += 1
            }

            result
        }
    }

    fn set_hand_type(&mut self) -> Result<(), &'static str> {
        if self.cards.len() != 5 {
            return Err("Must have 5 Cards to set hand type");
        }

        self.poker_hand_type = None;

        let card_rank_histogram = self.card_rank_histogram();

        // Check Poker hand Types that have multiple cards of the same rank
        if card_rank_histogram[0].1 == 4 {
            self.poker_hand_type = Some(PokerHandType::FourOfAKind);
        } else if card_rank_histogram[0].1 == 3 && card_rank_histogram[1].1 == 2 {
            self.poker_hand_type = Some(PokerHandType::FullHouse);
        } else if card_rank_histogram[0].1 == 3 && card_rank_histogram[1].1 == 1 {
            self.poker_hand_type = Some(PokerHandType::ThreeOfAKind);
        } else if card_rank_histogram[0].1 == 2 && card_rank_histogram[1].1 == 2 {
            self.poker_hand_type = Some(PokerHandType::TwoPair);
        } else if card_rank_histogram[0].1 == 2 && card_rank_histogram[1].1 == 1 {
            self.poker_hand_type = Some(PokerHandType::OnePair)
        }

        // Check to see if we should return early
        if self.poker_hand_type.is_some() {
            return Ok(());
        }

        match (self.have_straight(), self.have_flush()) {
            (false, false) => self.poker_hand_type = Some(PokerHandType::HighCard),
            (false, true) => self.poker_hand_type = Some(PokerHandType::Flush),
            (true, false) => self.poker_hand_type = Some(PokerHandType::Straight),
            (true, true) => {
                let mut ranks: Vec<u8> = self.cards.iter().map(|card| card.rank.clone()).collect();
                ranks.sort();
                if ranks[0] == 10 {
                    self.poker_hand_type = Some(PokerHandType::RoyalFlush);
                } else {
                    self.poker_hand_type = Some(PokerHandType::StraightFlush);
                }
            }
        }

        match self.poker_hand_type {
            None => Err("We unable to figure out your poker hand type"),
            Some(ref _hand_type) => Ok(()),
        }
    }

    fn sort_hand(&mut self) -> Result<(), String> {
        let _ = self.set_hand_type()?;
        let poker_hand_type = self.poker_hand_type.clone().unwrap();
        let card_rank_histogram = self.card_rank_histogram();

        match poker_hand_type {
            PokerHandType::OnePair | PokerHandType::ThreeOfAKind | PokerHandType::FourOfAKind => {
                let priority_card_rank = card_rank_histogram[0].0;

                self.cards.sort_by(|a, b| {
                    if a.rank == priority_card_rank && b.rank == priority_card_rank {
                        Ordering::Equal
                    } else if a.rank == priority_card_rank && b.rank != priority_card_rank {
                        Ordering::Less
                    } else if a.rank != priority_card_rank && b.rank == priority_card_rank {
                        Ordering::Greater
                    } else {
                        a.cmp(&b)
                    }
                });

                Ok(())
            }
            PokerHandType::RoyalFlush
            | PokerHandType::StraightFlush
            | PokerHandType::Flush
            | PokerHandType::Straight
            | PokerHandType::HighCard => {
                self.cards.sort();

                if poker_hand_type == PokerHandType::StraightFlush
                    || poker_hand_type == PokerHandType::Straight
                {
                    // In the case where the straight is Ace, 5, 4 ,3, 2, 1, we need to list
                    // the left by 1 -> 5, 4, 3, 2, Ace
                    if self.cards[0].rank == 14 && self.cards[1].rank == 5 {
                        self.cards.rotate_left(1);
                    }
                }
                Ok(())
            }
            PokerHandType::FullHouse | PokerHandType::TwoPair => {
                let priority_1 = card_rank_histogram[0].0;
                let priority_2 = card_rank_histogram[1].0;

                self.cards.sort_by(|a, b| {
                    if a.rank == priority_1 && b.rank == priority_1 {
                        a.cmp(&b)
                    } else if a.rank == priority_1 && b.rank == priority_2 {
                        Ordering::Less
                    } else if a.rank == priority_2 && b.rank == priority_1 {
                        Ordering::Greater
                    } else if a.rank == priority_2 && b.rank == priority_2 {
                        a.cmp(&b)
                    } else if a.rank == priority_1 && b.rank != priority_1 && b.rank != priority_2 {
                        Ordering::Less
                    } else if a.rank == priority_2 && b.rank != priority_1 && b.rank != priority_2 {
                        Ordering::Less
                    } else if a.rank != priority_1 && a.rank != priority_2 && b.rank == priority_1 {
                        Ordering::Greater
                    } else if a.rank != priority_1 && a.rank != priority_2 && b.rank == priority_2 {
                        Ordering::Greater
                    } else {
                        a.cmp(&b)
                    }
                });

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poker_hands_cmp() {
        let cases = vec![
            (
                "Same hand different Suit -- RoyalFlush",
                PokerHand {
                    poker_hand_type: Some(PokerHandType::RoyalFlush),
                    cards: vec![
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(13, Suit::Club).unwrap(),
                        Card::new(12, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(10, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::RoyalFlush),
                    cards: vec![
                        Card::new(14, Suit::Heart).unwrap(),
                        Card::new(13, Suit::Heart).unwrap(),
                        Card::new(12, Suit::Heart).unwrap(),
                        Card::new(11, Suit::Heart).unwrap(),
                        Card::new(10, Suit::Heart).unwrap(),
                    ],
                },
                Ordering::Equal,
            ),
            (
                "PokerHandType gets compared first -- FourOfAKind > HighCard",
                PokerHand {
                    poker_hand_type: Some(PokerHandType::FourOfAKind),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::HighCard),
                    cards: vec![
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(8, Suit::Diamond).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                    ],
                },
                Ordering::Greater,
            ),
            (
                "Same PokerHandType, but the hands are not equal",
                PokerHand {
                    poker_hand_type: Some(PokerHandType::FourOfAKind),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::FourOfAKind),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(11, Suit::Diamond).unwrap(),
                    ],
                },
                Ordering::Less,
            ),
            (
                "Extra case for Straights when the Ace counts as 1",
                PokerHand {
                    poker_hand_type: Some(PokerHandType::Straight),
                    cards: vec![
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Club).unwrap(),
                        Card::new(3, Suit::Diamond).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::Straight),
                    cards: vec![
                        Card::new(6, Suit::Club).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Diamond).unwrap(),
                        Card::new(3, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                    ],
                },
                Ordering::Less,
            ),
        ];

        for (name, hand_1, hand_2, expected) in cases {
            assert_eq!(hand_1.cmp(&hand_2), expected, "Case: {name}");
        }
    }

    #[test]
    fn poker_hand_sort_hand() {
        let cases = vec![
            (
                "One Pair",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                        Card::new(14, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::OnePair),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(14, Suit::Spade).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Three of a Kind",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::ThreeOfAKind),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Four of a Kind",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::FourOfAKind),
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(2, Suit::Spade).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(10, Suit::Diamond).unwrap(),
                    ],
                },
            ),
            (
                "Royal Flush",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(12, Suit::Club).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(10, Suit::Club).unwrap(),
                        Card::new(13, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::RoyalFlush),
                    cards: vec![
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(13, Suit::Club).unwrap(),
                        Card::new(12, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(10, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Flush",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(13, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::Flush),
                    cards: vec![
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(13, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Straight Flush",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(7, Suit::Club).unwrap(),
                        Card::new(10, Suit::Club).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(8, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::StraightFlush),
                    cards: vec![
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(10, Suit::Club).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(8, Suit::Club).unwrap(),
                        Card::new(7, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Straight Flush -- Edge Case",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(3, Suit::Club).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::StraightFlush),
                    cards: vec![
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Club).unwrap(),
                        Card::new(3, Suit::Club).unwrap(),
                        Card::new(2, Suit::Club).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Straight",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(7, Suit::Club).unwrap(),
                        Card::new(10, Suit::Heart).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(8, Suit::Club).unwrap(),
                        Card::new(11, Suit::Spade).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::Straight),
                    cards: vec![
                        Card::new(11, Suit::Spade).unwrap(),
                        Card::new(10, Suit::Heart).unwrap(),
                        Card::new(9, Suit::Club).unwrap(),
                        Card::new(8, Suit::Club).unwrap(),
                        Card::new(7, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "Straight -- Edge Case",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(3, Suit::Diamond).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::Straight),
                    cards: vec![
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(4, Suit::Club).unwrap(),
                        Card::new(3, Suit::Diamond).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                    ],
                },
            ),
            (
                "High Card",
                PokerHand {
                    poker_hand_type: None,
                    cards: vec![
                        Card::new(2, Suit::Heart).unwrap(),
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(8, Suit::Diamond).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                    ],
                },
                PokerHand {
                    poker_hand_type: Some(PokerHandType::HighCard),
                    cards: vec![
                        Card::new(14, Suit::Club).unwrap(),
                        Card::new(11, Suit::Club).unwrap(),
                        Card::new(8, Suit::Diamond).unwrap(),
                        Card::new(5, Suit::Club).unwrap(),
                        Card::new(2, Suit::Heart).unwrap(),
                    ],
                },
            ),
        ];

        for (name, mut poker_hand, expected) in cases {
            poker_hand.sort_hand().unwrap();
            assert_eq!(poker_hand, expected, "Case {name} failed");
        }
    }
}
