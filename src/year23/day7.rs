use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum PokerHand {
    HighCard,     // 5 different cards
    OnePair,      // 2 equal cards, 3 different (4)
    TwoPair,      // 2 pairs of equal cards, 1 different (3)
    ThreeOfAKind, // 3 equal cards, 2 different (3)
    FullHouse,    // 3 equal cards, 2 equal (2)
    FourOfAKind,  // 4 equal cards, 1 different (2)
    FiveOfAKind,  // 5 equal cards
}

fn check_hand(cards: &str) -> PokerHand {
    let mut count = HashMap::new();

    for c in cards.chars() {
        *count.entry(c).or_insert(0) += 1;
    }

    let len = count.len();
    let has_three: Vec<&usize> = count.values().filter(|&x| *x == 3).collect();
    if len == 5 {
        return PokerHand::HighCard;
    } else if len == 1 {
        return PokerHand::FiveOfAKind;
    } else if len == 4 {
        return PokerHand::OnePair;
    } else if len == 3 {
        if has_three.is_empty() {
            return PokerHand::TwoPair;
        }
        return PokerHand::ThreeOfAKind;
    } else {
        if has_three.is_empty() {
            return PokerHand::FourOfAKind;
        }
        return PokerHand::FullHouse;
    }
}

fn parse_char(c: char) -> u32 {
    //println!("{}", c);
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => c.to_digit(10).unwrap(),
    }
}

fn compare_cards(cards1: &str, cards2: &str) -> Ordering {
    let hand1 = check_hand(cards1);
    let hand2 = check_hand(cards2);
    if hand1 < hand2 {
        return Ordering::Less;
    } else if hand1 > hand2 {
        return Ordering::Greater;
    }

    for (a, b) in cards1.chars().zip(cards2.chars()) {
        if parse_char(a) < parse_char(b) {
            return Ordering::Less;
        } else if parse_char(a) > parse_char(b) {
            return Ordering::Greater;
        }
    }

    Ordering::Equal
}

fn part1(input: String) -> String {
    let parsed_hands: Vec<Vec<&str>> = input
        .lines()
        .map(|x| x.trim().split(' ').collect())
        .collect();
    let mut hands = Vec::new();
    for hand in parsed_hands {
        for cards_bid in hand.chunks(2) {
            let cards = cards_bid[0];
            let bid = cards_bid[1]
                .parse::<usize>()
                .expect("Bid should be a number");
            hands.push(Hand { cards, bid });
        }
    }
    hands.sort_by(|a, b| compare_cards(a.cards, b.cards));
    let mut total = 0;
    let mut rank = 1;
    for hand in hands {
        total += hand.bid * rank;
        rank += 1;
    }
    total.to_string()
}

fn part2(_input: String) -> String {
    "Not implemented".to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
