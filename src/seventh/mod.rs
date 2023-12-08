use std::usize;

use crate::{utils::get_non_empty_lines, Part, Runner};

const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

const CARDS_TWO: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<usize>,
    value: usize,
}

impl Hand {
    pub fn parse(value: &str, cards: [char; 13]) -> Result<Self, &'static str> {
        let mut cards_u = vec![];
        let value: usize = match cards[0] {
            'J' => {
                let mut counts: Vec<usize> = vec![0; 12];
                let mut jockers_count: usize = 0;
                for c in value.chars() {
                    match cards.iter().position(|&x| x == c) {
                        Some(card) => {
                            cards_u.push(card);
                            if card != 0 {
                                counts[card - 1] += 1;
                            } else {
                                jockers_count += 1;
                            }
                        }
                        None => return Err("Invalid card"),
                    }
                }

                counts.sort();
                counts.reverse();
                counts[0] += jockers_count;
                counts.iter().map(|x| x * x).sum()
            }
            _ => {
                let mut counts: Vec<usize> = vec![0; 13];
                for c in value.chars() {
                    match cards.iter().position(|&x| x == c) {
                        Some(card) => {
                            cards_u.push(card);
                            counts[card] += 1;
                        }
                        None => return Err("Invalid card"),
                    }
                }

                counts.iter().map(|x| x * x).sum()
            }
        };

        Ok(Hand {
            cards: cards_u,
            value,
        })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value == other.value {
            self.cards.cmp(&other.cards)
        } else {
            self.value.cmp(&other.value)
        }
    }
}

#[derive(Debug)]
struct Game {
    hand: Hand,
    bid: usize,
}

impl Game {
    fn parse(s: &str, cards: [char; 13]) -> Result<Self, &'static str> {
        if let Some((hand, bid)) = s.split_once(' ') {
            return Ok(Game {
                hand: Hand::parse(hand, cards).unwrap(),
                bid: bid.parse().unwrap(),
            });
        }
        Err("Invalid line")
    }
}

fn result(mut games: Vec<Game>) -> usize {
    games.sort_by(|a, b| a.hand.partial_cmp(&b.hand).unwrap());

    games
        .iter()
        .enumerate()
        .fold(0, |acc, (index, game)| acc + game.bid * (index + 1))
}

pub fn run(runner: &Runner) {
    let mut games: Vec<Game> = vec![];
    let cards = match runner.part {
        Part::One => CARDS,
        Part::Two => CARDS_TWO,
    };

    for line in get_non_empty_lines(&runner.path) {
        games.push(Game::parse(line.as_str(), cards).unwrap());
    }

    println!("{}", result(games));
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_hand() {
        let hand = Hand::parse("32T3K", CARDS).unwrap();
        assert_eq!(hand.cards, vec![1, 0, 8, 1, 11]);
        assert_eq!(hand.value, 7);

        let hand = Hand::parse("T55J5", CARDS).unwrap();
        assert_eq!(hand.cards, vec![8, 3, 3, 9, 3]);
        assert_eq!(hand.value, 11);

        let hand = Hand::parse("T55J5", CARDS_TWO).unwrap();
        assert_eq!(hand.cards, vec![9, 4, 4, 0, 4]);
        assert_eq!(hand.value, 17);

        let hand = Hand::parse("KK677", CARDS).unwrap();
        assert_eq!(hand.cards, vec![11, 11, 4, 5, 5]);
        assert_eq!(hand.value, 9);

        let hand = Hand::parse("KTJJT", CARDS).unwrap();
        assert_eq!(hand.cards, vec![11, 8, 9, 9, 8]);
        assert_eq!(hand.value, 9);

        let hand = Hand::parse("KTJJT", CARDS_TWO).unwrap();
        assert_eq!(hand.cards, vec![11, 9, 0, 0, 9]);
        assert_eq!(hand.value, 17);

        let hand = Hand::parse("QQQJA", CARDS).unwrap();
        assert_eq!(hand.cards, vec![10, 10, 10, 9, 12]);
        assert_eq!(hand.value, 11);

        let hand = Hand::parse("QQQJA", CARDS_TWO).unwrap();
        assert_eq!(hand.cards, vec![10, 10, 10, 0, 12]);
        assert_eq!(hand.value, 17);
    }

    #[test]
    fn sort() {
        let mut vec = vec![
            Hand::parse("32T3K", CARDS).unwrap(),
            Hand::parse("T55J5", CARDS).unwrap(),
            Hand::parse("KK677", CARDS).unwrap(),
            Hand::parse("KTJJT", CARDS).unwrap(),
            Hand::parse("QQQJA", CARDS).unwrap(),
        ];

        vec.sort();

        assert_eq!(
            vec,
            vec![
                Hand::parse("32T3K", CARDS).unwrap(),
                Hand::parse("KTJJT", CARDS).unwrap(),
                Hand::parse("KK677", CARDS).unwrap(),
                Hand::parse("T55J5", CARDS).unwrap(),
                Hand::parse("QQQJA", CARDS).unwrap(),
            ]
        );
    }

    #[test]
    fn test_game() {
        let game = Game::parse("32T3K 765", CARDS).unwrap();
        assert_eq!(game.hand.cards, vec![1, 0, 8, 1, 11]);
        assert_eq!(game.hand.value, 7);
        assert_eq!(game.bid, 765);

        let game2 = Game::parse("T55J5 684", CARDS).unwrap();
        assert_eq!(game2.hand.cards, vec![8, 3, 3, 9, 3]);
        assert_eq!(game2.hand.value, 11);
        assert_eq!(game2.bid, 684);

        let game3 = Game::parse("KK677 28", CARDS).unwrap();
        assert_eq!(game3.hand.cards, vec![11, 11, 4, 5, 5]);
        assert_eq!(game3.hand.value, 9);
        assert_eq!(game3.bid, 28);

        let game4 = Game::parse("KTJJT 220", CARDS).unwrap();
        assert_eq!(game4.hand.cards, vec![11, 8, 9, 9, 8]);
        assert_eq!(game4.hand.value, 9);
        assert_eq!(game4.bid, 220);

        let game5 = Game::parse("QQQJA 483", CARDS).unwrap();
        assert_eq!(game5.hand.cards, vec![10, 10, 10, 9, 12]);
        assert_eq!(game5.hand.value, 11);
        assert_eq!(game5.bid, 483);
    }

    #[test]
    fn part_one() {
        let games = vec![
            Game::parse("32T3K 765", CARDS).unwrap(),
            Game::parse("T55J5 684", CARDS).unwrap(),
            Game::parse("KK677 28", CARDS).unwrap(),
            Game::parse("KTJJT 220", CARDS).unwrap(),
            Game::parse("QQQJA 483", CARDS).unwrap(),
        ];

        assert_eq!(result(games), 6440);
    }
}
