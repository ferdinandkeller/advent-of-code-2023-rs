use std::{collections::HashMap, error::Error};

/// We get the input as a string, parse each line, extract the game id and parse the set of games in a Vec<Vec<(&str, u64)>>. We flatten that, keep the maximum of each color, and keep only the game ids that satisfy the condition of max_red<=12, max_green<=13 and max_blue<=14. The result printed is the total sum of the game ids that satisfy the condition.
pub fn solve_a(input: &str) -> Result<(), Box<dyn Error>> {
    let total = input
        .lines()
        .map(|l| {
            let (game_data, sets) = l.split_once(": ").unwrap();
            let game_id = game_data.split_once(" ").unwrap().1.parse::<u64>().unwrap();
            let games = sets
                .split("; ")
                .map(|s| {
                    s.split(", ")
                        .map(|cubes| {
                            let (number, color) = cubes.split_once(" ").unwrap();
                            (color, number.parse::<u64>().unwrap())
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .fold(HashMap::new(), |mut acc, (color, number)| {
                    let entry = acc.entry(color).or_insert(0);
                    if *entry < number {
                        *entry = number;
                    }
                    acc
                });
            (game_id, games)
        })
        .filter(|(_, cubes)| {
            *cubes.get("red").unwrap_or(&0) <= 12
                && *cubes.get("green").unwrap_or(&0) <= 13
                && *cubes.get("blue").unwrap_or(&0) <= 14
        })
        .map(|(id, _)| id)
        .sum::<u64>();

    println!("{:?}", total);

    Ok(())
}

pub fn solve_b(input: &str) -> Result<(), Box<dyn Error>> {
    let total = input
        .lines()
        .map(|l| {
            let (game_data, sets) = l.split_once(": ").unwrap();
            let game_id = game_data.split_once(" ").unwrap().1.parse::<u64>().unwrap();
            let games = sets
                .split("; ")
                .map(|s| {
                    s.split(", ")
                        .map(|cubes| {
                            let (number, color) = cubes.split_once(" ").unwrap();
                            (color, number.parse::<u64>().unwrap())
                        })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .fold(HashMap::new(), |mut acc, (color, number)| {
                    let entry = acc.entry(color).or_insert(0);
                    if *entry < number {
                        *entry = number;
                    }
                    acc
                });
            (game_id, games)
        })
        .map(|(_, cubes)| {
            let red = *cubes.get("red").unwrap_or(&0);
            let green = *cubes.get("green").unwrap_or(&0);
            let blue = *cubes.get("blue").unwrap_or(&0);
            red * green * blue
        })
        .sum::<u64>();

    println!("{:?}", total);

    Ok(())
}
