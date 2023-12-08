use itertools::Itertools;
use nom::{
    character::complete::{anychar, digit1, multispace0, multispace1},
    combinator::map_res,
    multi::{count, many1},
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let hands = many1(delimited(multispace0, parse_line, multispace0))(&input).unwrap();

    let mut hands: Vec<_> = hands.1.clone();
    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    let sum: usize = hands
        .iter()
        .enumerate()
        .map(|(i, (_h, bid))| bid * (i + 1))
        .sum();

    println!("Pt1 - Sum of all bids multiplied by ranking: {}", sum);

    for (hand, _bid) in hands.iter_mut() {
        for card in hand.cards.iter_mut() {
            if card.0 == 'J' {
                card.0 = 'X';
            }
        }
    }
    hands.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let sum: usize = hands
        .iter()
        .enumerate()
        .map(|(i, (_h, bid))| bid * (i + 1))
        .sum();

    println!("Pt2 - Sum of all bids multiplied by ranking: {}", sum);
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card(char);

impl TryFrom<char> for Card {
    type Error = ();
    fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        match c {
            '2'..='9' | 'T' | 'J' | 'Q' | 'K' | 'A' | 'X' => Ok(Card(c)),
            _ => Err(()),
        }
    }
}

impl Card {
    fn val(&self) -> u8 {
        match self.0 {
            '2'..='9' => self.0 as u8 - '0' as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.val().partial_cmp(&other.val())
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.val().cmp(&other.val())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: [Card; 5],
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, rhs: &Hand) -> Option<std::cmp::Ordering> {
        match self.hand_type().cmp(&rhs.hand_type()) {
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Equal => {
                // compare lexicographically the unsorted hand
                Some(self.cards.iter().cmp(rhs.cards.iter()))
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.partial_cmp(rhs).unwrap()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard = 0,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut cards: Vec<_> = self.cards.iter().map(|c| c.0).collect();
        cards.sort();
        let mut cards: Vec<_> = cards.iter().dedup_with_count().collect();
        cards.sort_by_key(|(amount, _c)| *amount);
        if let Some((index, (joker_amt, _j))) =
            cards.iter().copied().find_position(|(_amt, c)| **c == 'X')
        {
            if cards.len() == 1 {
                // jokers only, nothing to do
            } else {
                cards.remove(index);
                cards.last_mut().unwrap().0 += joker_amt;
            }
        }

        cards
            .iter()
            .fold(HandType::HighCard, |acc, (amount, _card)| match amount {
                5 => HandType::Five,
                4 => HandType::Four,
                3 => {
                    if acc == HandType::Pair {
                        HandType::FullHouse
                    } else {
                        HandType::Three
                    }
                }
                2 => {
                    if acc == HandType::Pair {
                        HandType::TwoPair
                    } else {
                        HandType::Pair
                    }
                }
                _ => acc,
            })
    }
}

fn parse_card(input: &str) -> IResult<&str, Card> {
    anychar(input).map(|(rest, c)| (rest, c.try_into().unwrap()))
}

fn parse_hand(input: &str) -> IResult<&str, Hand> {
    map_res(count(parse_card, 5), |v| {
        v.try_into().map(|arr: [Card; 5]| Hand { cards: arr })
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, (Hand, usize)> {
    separated_pair(
        parse_hand,
        multispace1,
        map_res(digit1, str::parse::<usize>),
    )(input)
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn hand_type() {
        assert_eq!(
            Hand {
                cards: [Card('5'), Card('5'), Card('5'), Card('5'), Card('5')],
            }
            .hand_type(),
            HandType::Five
        );

        assert_eq!(
            Hand {
                cards: [Card('4'), Card('4'), Card('4'), Card('4'), Card('1')],
            }
            .hand_type(),
            HandType::Four
        );
        assert_eq!(
            Hand {
                cards: [Card('K'), Card('K'), Card('K'), Card('Q'), Card('Q')],
            }
            .hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand {
                cards: [Card('3'), Card('3'), Card('3'), Card('Q'), Card('J')],
            }
            .hand_type(),
            HandType::Three
        );
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('A'), Card('K'), Card('K'), Card('J')],
            }
            .hand_type(),
            HandType::TwoPair
        );
        assert_eq!(
            Hand {
                cards: [Card('2'), Card('2'), Card('K'), Card('Q'), Card('J')],
            }
            .hand_type(),
            HandType::Pair
        );
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('2'), Card('3'), Card('4'), Card('5')],
            }
            .hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn joker_hand_type() {
        assert_eq!(
            Hand {
                cards: [Card('5'), Card('5'), Card('5'), Card('5'), Card('5')],
            }
            .hand_type(),
            HandType::Five
        );

        assert_eq!(
            Hand {
                cards: [Card('4'), Card('4'), Card('4'), Card('4'), Card('X')],
            }
            .hand_type(),
            HandType::Five
        );
        assert_eq!(
            Hand {
                cards: [Card('K'), Card('K'), Card('K'), Card('X'), Card('Q')],
            }
            .hand_type(),
            HandType::Four
        );
        assert_eq!(
            Hand {
                cards: [Card('3'), Card('3'), Card('Q'), Card('Q'), Card('X')],
            }
            .hand_type(),
            HandType::FullHouse
        );
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('A'), Card('K'), Card('Q'), Card('X')],
            }
            .hand_type(),
            HandType::Three
        );
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('2'), Card('K'), Card('Q'), Card('X')],
            }
            .hand_type(),
            HandType::Pair
        );
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('2'), Card('3'), Card('4'), Card('5')],
            }
            .hand_type(),
            HandType::HighCard
        );
    }

    #[test]
    fn compare_hand_type() {
        assert!(HandType::HighCard < HandType::Pair);
        assert!(HandType::Pair < HandType::TwoPair);
        assert!(HandType::TwoPair < HandType::Three);
        assert!(HandType::Three < HandType::FullHouse);
        assert!(HandType::FullHouse < HandType::Four);
        assert!(HandType::Four < HandType::Five);
    }

    #[test]
    fn compare_hands() {
        assert_eq!(
            Hand {
                cards: [Card('A'), Card('2'), Card('3'), Card('4'), Card('5')],
            }
            .cmp(&Hand {
                cards: [Card('A'), Card('2'), Card('2'), Card('4'), Card('5')],
            }),
            std::cmp::Ordering::Less
        );

        assert_eq!(
            Hand {
                cards: [Card('2'), Card('2'), Card('3'), Card('4'), Card('5')],
            }
            .cmp(&Hand {
                cards: [Card('A'), Card('2'), Card('2'), Card('4'), Card('5')],
            }),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_parse_hands() {
        let example = "32T3K 765
                            T55J5 684
                            KK677 28
                            KTJJT 220
                            QQQJA 483";
        assert_eq!(
            parse_line("32T3K 765"),
            Ok((
                "",
                (
                    Hand {
                        cards: [Card('3'), Card('2'), Card('T'), Card('3'), Card('K')],
                    },
                    765
                )
            ))
        );

        assert_eq!(
            many1(delimited(multispace0, parse_line, multispace0))("32T3K 765"),
            Ok((
                "",
                vec![(
                    Hand {
                        cards: [Card('3'), Card('2'), Card('T'), Card('3'), Card('K')],
                    },
                    765
                )]
            ))
        );
        assert_eq!(
            many1(delimited(multispace0, parse_line, multispace0))(example),
            Ok((
                "",
                vec![
                    (
                        Hand {
                            cards: [Card('3'), Card('2'), Card('T'), Card('3'), Card('K')],
                        },
                        765
                    ),
                    (
                        Hand {
                            cards: [Card('T'), Card('5'), Card('5'), Card('J'), Card('5')],
                        },
                        684
                    ),
                    (
                        Hand {
                            cards: [Card('K'), Card('K'), Card('6'), Card('7'), Card('7')],
                        },
                        28
                    ),
                    (
                        Hand {
                            cards: [Card('K'), Card('T'), Card('J'), Card('J'), Card('T')],
                        },
                        220
                    ),
                    (
                        Hand {
                            cards: [Card('Q'), Card('Q'), Card('Q'), Card('J'), Card('A')],
                        },
                        483
                    )
                ]
            ))
        );
    }

    #[test]
    fn ranking_hands() {
        let example = "32T3K 765
                            T55J5 684
                            KK677 28
                            KTJJT 220
                            QQQJA 483";
        let hands = many1(delimited(multispace0, parse_line, multispace0))(example).unwrap();

        let handtypes: Vec<_> = hands.1.iter().map(|(h, _bid)| h.hand_type()).collect();
        assert_eq!(
            handtypes,
            vec![
                HandType::Pair,
                HandType::Three,
                HandType::TwoPair,
                HandType::TwoPair,
                HandType::Three,
            ]
        );
    }
}
