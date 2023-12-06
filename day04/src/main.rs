use std::collections::HashSet;
use anyhow::Result;
use regex::Regex;
use utils::read_lines;
use std::str::FromStr;
use once_cell::sync::Lazy;

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"Card \s*(?<card_num>\d+): (?<winning_row>.*) \| (?<draw_row>.*)").expect("Failed to compile regex")
});

static NUMS_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\d+").expect("Failed to compile regex")
});

fn find_numerics(card_row: &str) -> Result<Vec<u64>> {
    Ok(NUMS_REGEX.find_iter(card_row)
        .filter_map(|digits| u64::from_str(digits.as_str()).ok())
        .collect())
}

#[derive(Default, Debug)]
struct Card {
    num: u32,
    winning_nums: Vec<u64>,
    draw_nums: Vec<u64>,
    matching_nums: Vec<u64>,
    points: u64,
}

impl Card {
    fn parse(line: String) -> Result<Card> {
        let mut card = Card::default();
        if let Some(result) = LINE_REGEX.captures(&line) {
            card.num = u32::from_str(&result["card_num"])?;
            card.winning_nums = find_numerics(&result["winning_row"])?;
            card.draw_nums = find_numerics(&result["draw_row"])?;

            card.matching_nums = card.winning_nums.iter().collect::<HashSet<_>>()
                .intersection(&card.draw_nums.iter().collect::<HashSet<_>>())
                .map(|&&num| num)
                .collect();

            card.points = match card.matching_nums.len() {
                0 => 0,
                n => 1 << (n -1),
            };

            println!("Card: {:?}", card);
        }

        Ok(card)
    }
}

fn process_deck_part2(cards: Vec<Card>) -> u64 {
    let mut num = 0;
    let mut stack: Vec<&Card> = cards.iter().rev().collect();
    while let Some(card) = stack.pop() {
        let copies_won = card.matching_nums.len();
        for i in 0..copies_won {
            if let Some(copy_card) = cards.get(i + card.num as usize) {
                stack.push(copy_card)
            }
        }
        num +=1;
    }
    num
}

fn main() -> Result<()> {
    let mut cards = Vec::new();
    let mut total_points = 0;

    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let card = Card::parse(ip)?;
                total_points += &card.points;
                cards.push(card);
            }
        }
    }

    println!("Total Points: {}", total_points);

    // Part 2
    let total_cards_processed = process_deck_part2(cards);
    println!("Total Cards Processed for Part 2: {}", total_cards_processed);


    Ok(())
}
