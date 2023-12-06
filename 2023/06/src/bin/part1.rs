use std::collections::HashSet;

use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let tests = vec![(
            "Time:      7  15   30
Distance:  9  40  200",
            "288",
        )];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
