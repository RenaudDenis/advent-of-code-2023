use anyhow::Result;

use utils::read_lines;

const DIGIT_STRINGS: [&str; 18] = [
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn main() -> Result<()> {
    let mut sum1: usize = 0;
    let mut sum2: usize = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let code1 = to_code_simple(&ip);
                let code2 = to_code_advanced(&ip);
                println!("{} -> {} -> {}", ip, code1, code2);
                sum1 += code1;
                sum2 += code2;
            }
        }
    }

    println!("-------------");
    println!("Total Part 1: {}", sum1);
    println!("Total Part 2: {}", sum2);

    Ok(())
}

fn to_code_simple(word: &str) -> usize {
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    for c in word.chars() {
        if c.is_numeric() {
            let current = c.to_digit(10).unwrap() as usize;
            if first.is_none() {
                first = Some(current);
            }
            last = Some(current);
        }
    }
    first.unwrap_or(0) * 10 + last.unwrap_or(0)
}

fn to_code_advanced(word: &str) -> usize {
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    for i in 0..word.len() {
        if let Some(index) = matches(&word[i..]) {
            let digit = if index > 8 {
                index - 8
            } else {
                index + 1
            };
            if first.is_none() {
                first = Some(digit);
            }
            last = Some(digit);
        }
    }
    first.unwrap_or(0) * 10 + last.unwrap_or(0)
}

fn matches(slice: &str) -> Option<usize> {
    for i in 0..DIGIT_STRINGS.len() {
        if slice.starts_with(DIGIT_STRINGS[i]) {
            return Some(i)
        }
    }
    None
}

