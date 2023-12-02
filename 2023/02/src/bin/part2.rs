use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut power_sum = 0;

    for line in input.lines() {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let line_parts = line.split(":").skip(1);
        let mut draws: HashMap<&str, i32> = HashMap::new();
        for draw_texts in line_parts {
            // 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            for color in draw_texts.split(";") {
                // 3 blue, 4 red
                for color_part in color.trim().split(",") {
                    let mut color_part = color_part.split_whitespace();
                    let count = color_part.next().expect("number").parse::<i32>()?;
                    let key = color_part.next().expect("color");

                    // Make sure to have the highest count for each color
                    if let Some(prev_draw) = draws.get(key) {
                        if prev_draw < &count {
                            draws.insert(key, count);
                        }
                    } else {
                        draws.insert(key, count);
                    }
                }
            }
        }

        let power = draws.values().fold(1, |acc, x| acc * x);
        power_sum += power;
    }

    Ok(format!("{}", power_sum))
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
        let result = "2286".to_string();
        assert!(solve(input)? == result);

        Ok(())
    }
}
