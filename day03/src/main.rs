use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::path::Path;
use utils::read_lines;
use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: i16,
    y: i16,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct InclusionZone {
    upper_left: Coordinate,
    lower_right: Coordinate,
}

impl InclusionZone {
    fn contains(&self, coordinate: &Coordinate) -> bool {
        coordinate.x >= self.upper_left.x && coordinate.x <= self.lower_right.x
            && coordinate.y >= self.upper_left.y && coordinate.y <= self.lower_right.y
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Number {
    value: u32,
    inclusion_zone: InclusionZone,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Symbol {
    coordinate: Coordinate,
    value: char,
}

struct Gear {
    coordinate: Coordinate,
    numbers: Vec<Number>,
}

impl Gear {
    fn ratio(&self) -> u32 {
        self.numbers.iter().fold(1, |acc, n| acc * n.value)
    }
}

impl Debug for Gear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:?}, {}, {}, ratio={}]", self.coordinate, self.numbers.get(0).unwrap().value, self.numbers.get(1).unwrap().value, self.ratio())
    }
}

struct Analysis {
    surrounded_numbers: HashSet<Number>,
    gears: Vec<Gear>,
}

struct Schematic {
    numbers: HashSet<Number>,
    symbols: HashSet<Symbol>,
}

impl Schematic {
    fn load<P>(file_path: P) -> Result<Schematic> where
        P: AsRef<Path> {

        let mut numbers = HashSet::new();
        let mut symbols = HashSet::new();

        let mut current_y = 0;
        if let Ok(lines) = read_lines(file_path) {
            for line in lines {
                if let Ok(mut ip) = line {
                    ip.push('.');
                    let mut in_number = false;
                    let mut current_number: u32 = 0;
                    let mut current_number_startx: i16 = 0;
                    for (current_x, c) in ip.chars().enumerate() {
                        if c.is_numeric() {
                            if !in_number {
                                // New number
                                current_number_startx = current_x as i16;
                                current_number = 0;
                            }
                            in_number = true;
                            current_number = current_number * 10 + c.to_digit(10).unwrap() as u32;
                        } else {
                            if in_number {
                                // Number ended
                                in_number = false;
                                numbers.insert(Number {
                                    value: current_number,
                                    inclusion_zone: InclusionZone {
                                        upper_left: Coordinate {
                                            x: current_number_startx - 1,
                                            y: current_y - 1,
                                        },
                                        lower_right: Coordinate {
                                            x: current_x as i16,
                                            y: current_y + 1,
                                        },
                                    },
                                });
                            }
                            if c != '.' {
                                // Not a number, not a dot, that would be a symbol
                                symbols.insert(Symbol {
                                    coordinate: Coordinate {
                                        x: current_x as i16,
                                        y: current_y,
                                    },
                                    value: c,
                                });
                            }
                        }
                    }
                    current_y += 1;
                }
            }
        }

        Ok(Schematic {
            numbers,
            symbols,
        })
    }

    fn analyze(&self) -> Analysis {
        let mut surrounded_numbers = HashSet::new();
        let mut gears = Vec::new();
        for symbol in &self.symbols {
            let mut gear_numbers = Vec::new();
            for number in &self.numbers {
                if number.inclusion_zone.contains(&symbol.coordinate) {
                    surrounded_numbers.insert(number.clone());
                    gear_numbers.push(number.clone());
                }
            }
            if symbol.value == '*' {
                // This might be a gear
                if gear_numbers.len() == 2 {
                    gears.push(Gear {
                        coordinate: symbol.coordinate.clone(),
                        numbers: gear_numbers,
                    });
                }
            }
        }
        Analysis {
            surrounded_numbers,
            gears
        }
    }
}

fn main() -> Result<()>{
    let schematic = Schematic::load("./src/input.txt")?;

    let analysis = schematic.analyze();

    let surrounded_numbers = analysis.surrounded_numbers;
    let mut surrounded_values: Vec<u32> = surrounded_numbers.iter()
        .map(|number| number.value)
        .collect();
    surrounded_values.sort();

    println!("Surrounded numbers: {:?}", surrounded_values);

    let excluded_numbers = &schematic.numbers - &surrounded_numbers;
    let mut excluded_values: Vec<u32> = excluded_numbers.iter()
        .map(|number| number.value)
        .collect();
    excluded_values.sort();

    println!("Excluded numbers: {:?}", excluded_values);

    let sum1: u32 = surrounded_values.iter().sum();
    println!("Sum of numbers surrounded by symbols: {}", sum1);

    println!("Gears: {:?}", analysis.gears);

    let gear_ratio_sum: u32 = analysis.gears.iter().map(|gear| gear.ratio()).sum();
    println!("Sum of Gear Ratios: {}", gear_ratio_sum);

    Ok(())
}