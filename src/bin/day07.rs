use core::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let input_lines: Vec<String> = read_to_string("input/day07.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    fn hand<T: From<u8> + RemoveJokers<T> + Ord + Clone>(cards_str: &str, bid: usize) -> Hand<T> {
        let cards = cards_str.bytes().map(|b| b.into()).collect::<Vec<_>>();
        let class = classify(&cards);
        Hand { class, cards, bid }
    }

    let (hands1, hands2): (Vec<Hand<Card1>>, Vec<Hand<Card2>>) = input_lines
        .into_iter()
        .map(|line| {
            let (cards_str, bid_str) = line.split_once(' ').unwrap();
            let bid = str::parse(bid_str).unwrap();
            (hand(cards_str, bid), hand(cards_str, bid))
        })
        .unzip();
    println!("part 1: {}", score(hands1));
    println!("part 2: {}", score(hands2));
}

fn score<T: Ord>(mut hands: Vec<Hand<T>>) -> usize {
    hands.sort();
    let mut result = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        result += (i + 1) * hand.bid;
    }
    result
}

#[derive(Eq, Ord)]
struct Hand<T: PartialOrd> {
    class: HandClass,
    cards: Vec<T>,
    bid: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum HandClass {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

trait RemoveJokers<T> {
    fn remove_jokers(cards: &mut Vec<T>) -> usize;
}

impl RemoveJokers<Self> for Card1 {
    fn remove_jokers(_cards: &mut Vec<Card1>) -> usize {
        0
    }
}

impl RemoveJokers<Self> for Card2 {
    fn remove_jokers(cards: &mut Vec<Card2>) -> usize {
        let mut joker_count = 0;
        while !cards.is_empty() && cards[0] == Card2::J {
            joker_count += 1;
            cards.remove(0);
        }
        joker_count
    }
}

fn classify<T: Ord + Clone + RemoveJokers<T>>(cards: &[T]) -> HandClass {
    let mut cards = cards.to_vec();
    cards.sort();
    let joker_count = T::remove_jokers(&mut cards);
    if joker_count >= 4 {
        return HandClass::FiveOfKind;
    }

    let mut runs = Vec::<usize>::new();
    let mut run_start = 0;
    for i in 1..=cards.len() {
        if i == cards.len() || cards[i] != cards[run_start] {
            let run_length = i - run_start;
            if run_length > 1 {
                runs.push(run_length);
            }
            run_start = i;
        }
    }
    runs.sort();

    use HandClass::*;
    let upgrade_path_1 = [HighCard, Pair, ThreeOfKind, FourOfKind, FiveOfKind];
    let upgrade_path_2 = [TwoPair, FullHouse];

    match runs.len() {
        0 => upgrade_path_1[joker_count],
        1 => upgrade_path_1[runs[0] - 1 + joker_count],
        2 => upgrade_path_2[runs[1] - 2 + joker_count],
        _ => panic!("{runs:?}"),
    }
}

impl<T: PartialOrd> PartialEq for Hand<T> {
    fn eq(&self, other: &Self) -> bool {
        self.class == other.class && self.cards == other.cards
    }
}

impl<T: PartialOrd> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.class.partial_cmp(&other.class) {
            Some(Ordering::Equal) => self.cards.partial_cmp(&other.cards),
            ord => ord,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Card1 {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
enum Card2 {
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    Q,
    K,
    A,
}

impl From<u8> for Card1 {
    fn from(value: u8) -> Self {
        match value {
            b'2' => Self::_2,
            b'3' => Self::_3,
            b'4' => Self::_4,
            b'5' => Self::_5,
            b'6' => Self::_6,
            b'7' => Self::_7,
            b'8' => Self::_8,
            b'9' => Self::_9,
            b'T' => Self::T,
            b'J' => Self::J,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => panic!("{value}"),
        }
    }
}

impl From<u8> for Card2 {
    fn from(value: u8) -> Self {
        match value {
            b'J' => Self::J,
            b'2' => Self::_2,
            b'3' => Self::_3,
            b'4' => Self::_4,
            b'5' => Self::_5,
            b'6' => Self::_6,
            b'7' => Self::_7,
            b'8' => Self::_8,
            b'9' => Self::_9,
            b'T' => Self::T,
            b'Q' => Self::Q,
            b'K' => Self::K,
            b'A' => Self::A,
            _ => panic!("{value}"),
        }
    }
}
