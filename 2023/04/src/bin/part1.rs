use std::collections::HashSet;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let (winning, numbers) = line
            .split(":")
            .nth(1)
            .expect("numbers")
            .split_once("|")
            .map(|(w, g)| {
                (
                    w.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<HashSet<i32>>(),
                    g.split_whitespace()
                        .map(|x| x.parse::<i32>().unwrap())
                        .collect::<HashSet<i32>>(),
                )
            })
            .expect("winning and guesses");
        sum += match numbers.intersection(&winning).count() as i32 {
            0 => 0,
            1 => 1,
            n => 2u32.pow(n as u32 - 1),
        };
    }

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let tests = vec![(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
            "13",
        )];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
