use std::collections::HashSet;
use std::collections::HashMap;

struct Card {
    winner_numbers: HashSet<usize>,
    have: Vec<usize>,
}

fn parse_card(card: String) -> Card {
    let mut winner_numbers: HashSet<usize> = HashSet::new();
    let mut have: Vec<usize> = Vec::new();
    let mut values = card.split(':').nth(1).unwrap().split('|');
    winner_numbers.extend(values.next().unwrap().trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()));
    have.extend(values.next().unwrap().trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()));
    Card {
        winner_numbers,
        have,
    }
}

fn calculate_points(card: &Card) -> usize {
    let mut points = 0;
    for own_number in &card.have {
        if card.winner_numbers.contains(own_number){
            if points == 0 {
                points += 1;

            } else {
                points *= 2;
            }
        }

    }
    points
}

fn process_scratch_card(id: usize, card: &Card, cards: &mut HashMap<usize, usize>, max_card_num: usize) {
   let extra_cards: usize = card.have.clone().into_iter().filter(|x| card.winner_numbers.contains(x)).collect::<Vec<usize>>().len();
   let start = id+1;
   let end = id + extra_cards;
   if !cards.contains_key(&id){
        cards.insert(id, 1);
   }
   let ncards = *cards.get(&id).unwrap();
   for n in start..=end {
        if n > max_card_num {
            break;
        }
        if !cards.contains_key(&n){
            cards.insert(n, 1);
        }
        cards.insert(n, cards.get(&n).unwrap() + ncards);
    }
}

fn part1(input: String)  -> String {
    let cards = input.lines().map(|x| parse_card(x.to_string()));
    let points = cards.fold(0, |acc, e| acc + calculate_points(&e));
    points.to_string()
}

fn part2(input: String) -> String {
    let mut card_count: HashMap<usize,usize> = HashMap::new();
    let cards: Vec<Card> = input.lines().map(|x| parse_card(x.to_string())).collect();
    let max_card_num = cards.len();
    for (i, c) in cards.iter().enumerate() {
        process_scratch_card(i+1, &c, &mut card_count, max_card_num);
    }
    let total: usize = card_count.values().sum();
    total.to_string()
}

pub fn solve(input: String) -> (String, String) {
    (part1(input.clone()), part2(input.clone()))
}
