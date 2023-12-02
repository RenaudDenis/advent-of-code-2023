use std::cmp::max;
use std::str::FromStr;
use anyhow::Result;
use utils::read_lines;

#[derive(Default, Debug)]
struct CubeSet(u32, u32, u32);

impl CubeSet {
    fn can_be_drawn_from(&self, other: &CubeSet) -> bool {
        self.0 <= other.0 && self.1 <= other.1 && self.2 <= other.2
    }

    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }
}

struct Game {
    index: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn parse(line: &str) -> Result<Game> {
        let split = line.find(':').unwrap();

        let index = u32::from_str(&line[5..split])?;

        let mut sets= Vec::new();
        for set_string in line[split + 2..].split(';') {
            let mut set = CubeSet::default();
            for cube_string in set_string.split(',') {
                if cube_string.ends_with("red") {
                    let number = &cube_string[..cube_string.len() - "red".len()].trim();
                    set.0 += u32::from_str(number)?;
                } else if cube_string.ends_with("green") {
                    let number = &cube_string[..cube_string.len() - "green".len()].trim();
                    set.1 += u32::from_str(number)?;
                } else if cube_string.ends_with("blue") {
                    let number = &cube_string[..cube_string.len() - "blue".len()].trim();
                    set.2 += u32::from_str(number)?;
                }
            }
            sets.push(set);
        }

        Ok(Game {
            index,
            sets,
        })
    }

    fn get_minimal_set(&self) -> CubeSet {
        let mut minimal = CubeSet::default();
        for set in &self.sets {
            minimal.0 = max(minimal.0, set.0);
            minimal.1 = max(minimal.1, set.1);
            minimal.2 = max(minimal.2, set.2);
        }
        minimal
    }
}

fn main() -> Result<()> {
    let bag = CubeSet(12, 13, 14);

    let mut sum1: u32 = 0;
    let mut sum2: u32 = 0;
    if let Ok(lines) = read_lines("./src/input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let game = Game::parse(&ip)?;

                // Part 1
                let possible = game.sets.iter().all(|set| set.can_be_drawn_from(&bag));
                if possible {
                    sum1 += game.index;
                }

                // Part 2
                let minimal = game.get_minimal_set();
                let power = minimal.power();
                sum2 += power;
                println!("{} -> possible: {}; minimal: {:?}; power: {} ", ip, possible, minimal, power);
            }
        }
    }

    println!("--------------------------------------");
    println!("Total indexes of possible games: {}", sum1);
    println!("Total power of minimal set for each game: {}", sum2);

    Ok(())
}

