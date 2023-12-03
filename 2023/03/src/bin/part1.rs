use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn is_symbol(ch: char) -> bool {
    !ch.is_numeric() && ch != '.'
}

fn solve(input: &str) -> Result<String> {
    let mut symbols: Vec<(usize, usize)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if is_symbol(ch) {
                symbols.push((x, y))
            }
        }
    }

    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = "4361".to_string();
        assert!(solve(input)? == result);

        Ok(())
    }
}
