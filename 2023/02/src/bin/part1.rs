use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut possible_games: Vec<i32> = vec![];

    let actual_bag = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    for (line_ix, line) in input.lines().enumerate() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let line_parts = line.split(":").skip(1);
        let game_number = line_ix as i32 + 1;
        let mut possible = true;
        for draw_texts in line_parts {
            // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            for color in draw_texts.split(";") {
                // 3 blue, 4 red
                for color_part in color.trim().split(",") {
                    let mut color_part = color_part.split_whitespace();
                    let count = color_part.next().expect("key").parse::<i32>()?;
                    let key = color_part.next().expect("color");

                    if let Some(true_value) = actual_bag.get(key) {
                        if count > *true_value {
                            eprintln!("{}: {} > {}", key, count, true_value);
                            possible = false;
                        }
                    }
                }
            }
        }

        if possible {
            possible_games.push(game_number);
        }
    }

    Ok(format!("{}", possible_games.iter().sum::<i32>()))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        // Game 1, 2 and 5 are possible
        let result = "8".to_string();
        assert!(solve(input)? == result);

        Ok(())
    }
}
