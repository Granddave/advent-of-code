use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn is_symbol(ch: char) -> bool {
    if !ch.is_numeric() && ch != '.' && ch != '\n' {
        // println!("{}", ch);
        return true;
    }
    false
}

fn is_part_number(line_ix: i32, start_ix: i32, end_ix: i32, symbols: &Vec<(i32, i32)>) -> bool {
    for &(x, y) in symbols {
        let around_columns = x >= start_ix - 1 && x <= end_ix + 1;
        let around_line = y >= line_ix - 1 && y <= line_ix + 1;
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
            let is_last_char = ix == line.len() - 1;
            if ch.is_numeric() {
                if start_ix.is_none() {
                    start_ix = Some(ix);
                }
                end_ix = Some(ix)
            }
            if !ch.is_numeric() && end_ix.is_some() || is_last_char && end_ix.is_some() {
                if is_part_number(
                    line_ix as i32,
                    start_ix.unwrap() as i32,
                    end_ix.unwrap() as i32,
                    &symbols,
                ) {
                    sum += line
                        .get(start_ix.unwrap()..end_ix.unwrap() + 1)
                        .expect("get slice")
                        .parse::<i32>()
                        .expect("parse");
                }

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
..........%
..........3
...........",
                "3".to_string(),
            ),
        ];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
