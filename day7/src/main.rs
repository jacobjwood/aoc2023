use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //let contents = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";

    let hand_wagers = contents.lines().map(|l| l.split(' ').collect::<Vec<&str>>()).map(|t| (t[0], t[1].parse::<usize>().unwrap())).collect::<Vec<(&str, usize)>>();

    println!("{:?}", hand_wagers);

    let mut hand_vec: Vec<ParsedHand> = Vec::new();

    for (hand, wager) in hand_wagers {
        hand_vec.push(hand_type(hand, wager));
    }

    hand_vec.sort_by(|a, b| a.cmp(&b));

    println!("{:?}", hand_vec);

    let part1: usize = hand_vec.iter().enumerate().map(|(i, h)| h.wager * (i + 1)).sum();
    println!("Part 1: {}", part1);


}

fn hand_type(hand: &str, wager: usize) -> ParsedHand {

    let mut handmap: HashMap<char, usize> = HashMap::new();
    
    for card in hand.chars() {

        match handmap.get(&card) {
            Some(count) => handmap.insert(card, count+1),
            None => handmap.insert(card, 1),
        };

    }

    let mut card_counts = handmap.iter().map(|(_, v)| *v).collect::<Vec<usize>>();
    card_counts.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", card_counts);

    let hand_type = match card_counts[..] {
        [5] => ("five of a kind", 7),
        [4, 1] => ("four of a kind", 6),
        [3, 2] => ("full house", 5),
        [3, 1, 1] => ("three of a kind", 4),
        [2, 2, 1] => ("two pair", 3),
        [2, 1, 1, 1] => ("one pair", 2),
        [1, 1, 1, 1, 1] => ("high card", 1),
        _ => ("jeff", 1), // should never get here
    };

    let value_map = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('1', 1),
    ]);

    let card_values = hand.chars().map(|h| *value_map.get(&h).unwrap()).collect::<Vec<usize>>();

    println!("{:?} {:?}", hand_type, card_values);

    let parsed_hand = ParsedHand{ hand: hand, wager: wager, hand_type: hand_type.0, value: hand_type.1, card_values: card_values };

    println!("{:?}", parsed_hand);

    parsed_hand


}

#[derive(Debug, Eq, PartialEq)]
struct ParsedHand<'a> {
    hand: &'a str,
    wager: usize,
    hand_type: &'a str, 
    value: usize,
    card_values: Vec<usize>
}

impl<'a> Ord for ParsedHand<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        
        match self.value.cmp(&other.value) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (v1, v2) in std::iter::zip(&self.card_values, &other.card_values) {
                    if v1 != v2 {
                        return v1.cmp(&v2);
                    }
                }
                Ordering::Greater // should never reach here, but for loop gets pissy with me
            }
        }
    }
}

impl<'a> PartialOrd for ParsedHand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        Some(self.cmp(&other))
    }

}