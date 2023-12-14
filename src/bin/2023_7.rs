use adventofcode::day;
use core::cmp::Ordering;
use std::collections::BTreeMap;

mod part1 {
    use super::HandType;
    use core::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum CardType {
        A,
        K,
        Q,
        J,
        T,
        N9,
        N8,
        N7,
        N6,
        N5,
        N4,
        N3,
        N2,
    }

    impl PartialOrd for CardType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for CardType {
        fn cmp(&self, other: &Self) -> Ordering {
            (*self as usize).cmp(&(*other as usize)).reverse()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Hand([CardType; 5]);

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type().cmp(&other.hand_type()) {
                Ordering::Equal => {}
                other => return other,
            }

            self.0.cmp(&other.0)
        }
    }

    impl Hand {
        fn hand_type(&self) -> HandType {
            use HandType::*;

            let mut map = HashMap::new();

            for c in self.0 {
                *map.entry(c).or_insert(0usize) += 1;
            }

            let mut counts = map.into_values().collect::<Vec<_>>();
            counts.sort_unstable();
            match counts[..] {
                [5] => FiveOfAKind,
                [1, 4] => FourOfAKind,
                [2, 3] => FullHouse,
                [1, 1, 3] => ThreeOfAKind,
                [1, 2, 2] => TwoPair,
                [1, 1, 1, 2] => OnePair,
                [1, 1, 1, 1, 1] => HighCard,
                _ => unreachable!(),
            }
        }

        pub fn from_str(str: &str) -> Self {
            use CardType::*;

            assert!(str.len() == 5);

            let mut hand = [A; 5];

            for (&byte, card) in str.as_bytes().iter().zip(hand.iter_mut()) {
                *card = match byte {
                    b'A' => A,
                    b'K' => K,
                    b'Q' => Q,
                    b'J' => J,
                    b'T' => T,
                    b'2' => N2,
                    b'3' => N3,
                    b'4' => N4,
                    b'5' => N5,
                    b'6' => N6,
                    b'7' => N7,
                    b'8' => N8,
                    b'9' => N9,
                    _ => unreachable!(),
                };
            }

            Self(hand)
        }
    }
}

mod part2 {
    use super::HandType;
    use core::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    enum CardType {
        A,
        K,
        Q,
        T,
        N9,
        N8,
        N7,
        N6,
        N5,
        N4,
        N3,
        N2,
        J,
    }

    impl PartialOrd for CardType {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for CardType {
        fn cmp(&self, other: &Self) -> Ordering {
            (*self as usize).cmp(&(*other as usize)).reverse()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Hand([CardType; 5]);

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            match self.hand_type().cmp(&other.hand_type()) {
                Ordering::Equal => {}
                other => return other,
            }

            self.0.cmp(&other.0)
        }
    }

    impl Hand {
        fn hand_type(&self) -> HandType {
            use HandType::*;

            let mut map = HashMap::new();
            let mut j = 0;

            for c in self.0 {
                if c == CardType::J {
                    j += 1;
                } else {
                    *map.entry(c).or_insert(0usize) += 1;
                }
            }

            let mut counts = map.into_values().collect::<Vec<_>>();
            counts.sort_unstable();
            if j == 5 {
                counts.push(5);
            } else if j > 0 {
                *counts.last_mut().unwrap() += j;
            }
            match counts[..] {
                [5] => FiveOfAKind,
                [1, 4] => FourOfAKind,
                [2, 3] => FullHouse,
                [1, 1, 3] => ThreeOfAKind,
                [1, 2, 2] => TwoPair,
                [1, 1, 1, 2] => OnePair,
                [1, 1, 1, 1, 1] => HighCard,
                _ => unreachable!(),
            }
        }

        pub fn from_str(str: &str) -> Self {
            use CardType::*;

            assert!(str.len() == 5);

            let mut hand = [A; 5];

            for (&byte, card) in str.as_bytes().iter().zip(hand.iter_mut()) {
                *card = match byte {
                    b'A' => A,
                    b'K' => K,
                    b'Q' => Q,
                    b'J' => J,
                    b'T' => T,
                    b'2' => N2,
                    b'3' => N3,
                    b'4' => N4,
                    b'5' => N5,
                    b'6' => N6,
                    b'7' => N7,
                    b'8' => N8,
                    b'9' => N9,
                    _ => unreachable!(),
                };
            }

            Self(hand)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self as usize).cmp(&(*other as usize)).reverse()
    }
}

fn solve(input: &str) -> (usize, usize) {
    let map = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = part1::Hand::from_str(split.next().unwrap());
            let bid = split.next().unwrap().parse::<usize>().unwrap();

            (hand, bid)
        })
        .collect::<BTreeMap<_, _>>();
    let answer_1 = map
        .into_values()
        .enumerate()
        .map(|(i, bid)| bid * (i + 1))
        .sum::<usize>();

    let map = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let hand = part2::Hand::from_str(split.next().unwrap());
            let bid = split.next().unwrap().parse::<usize>().unwrap();

            (hand, bid)
        })
        .collect::<BTreeMap<_, _>>();
    let answer_2 = map
        .into_values()
        .enumerate()
        .map(|(i, bid)| bid * (i + 1))
        .sum::<usize>();

    (answer_1, answer_2)
}

day!(2023 7, 252052080, 252898370);
