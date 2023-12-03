use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn is_symbol(ch: char) -> bool {
    !ch.is_numeric() && ch != '.' && ch != '\n'
}

fn is_part_number(line_ix: i32, start_ix: i32, end_ix: i32, symbols: &Vec<(i32, i32)>) -> bool {
    // eprintln!("--");
    for symbol in symbols {
        let around_columns = symbol.0 >= start_ix - 1 && symbol.0 <= end_ix + 1;
        let around_line = symbol.1 >= line_ix - 1 && symbol.1 <= line_ix + 1;
        // eprintln!("x: {}, y: {}", symbol.0, symbol.1);
        // eprintln!("l: {}, s: {}, e:{}", line_ix, start_ix, end_ix);
        // eprintln!("{}, {}", around_columns, around_line);
        if around_columns && around_line {
            return true;
        }
    }

    false
}

fn solve(input: &str) -> Result<String> {
    let mut symbols: Vec<(i32, i32)> = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if is_symbol(ch) {
                symbols.push((x as i32, y as i32))
            }
        }
    }

    let mut sum = 0;
    for (line_ix, line) in input.lines().enumerate() {
        let mut start_ix: Option<usize> = None;
        let mut end_ix: Option<usize> = None;
        for (ix, ch) in line.chars().enumerate() {
            if ch.is_numeric() {
                if start_ix.is_none() {
                    start_ix = Some(ix);
                }
                end_ix = Some(ix)
            } else if !ch.is_numeric() && end_ix.is_some() {
                // Check for symbols
                if is_part_number(
                    line_ix as i32,
                    start_ix.unwrap() as i32,
                    end_ix.unwrap() as i32,
                    &symbols,
                ) {
                    // eprintln!("{}", line[start_ix.unwrap()..end_ix.unwrap()].to_string());
                    sum += line[start_ix.unwrap()..end_ix.unwrap() + 1].parse::<i32>()?;
                }

                // Reset start and end
                start_ix = None;
                end_ix = None;
            }
        }
    }

    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let tests = vec![
            (
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
                "4361".to_string(),
            ),
            (
                "...........
...........
...%.......
123........",
                "123".to_string(),
            ),
        ];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
