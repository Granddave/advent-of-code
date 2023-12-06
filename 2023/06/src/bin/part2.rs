use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let lines = input
        .lines()
        .map(|line| {
            line.split(':')
                .skip(1)
                .next()
                .unwrap()
                .replace(" ", "")
                .trim()
                .parse::<i64>()
                .expect("invalid value")
        })
        .collect::<Vec<_>>();

    let time = lines.get(0).unwrap();
    let record = lines.get(1).unwrap();
    let mut new_records = 0;
    for hold_time in 0..*time {
        let distance = (time - hold_time) * hold_time;
        if distance > *record {
            new_records += 1;
        }
    }

    Ok(new_records.to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let tests = vec![(
            "Time:      7  15   30
Distance:  9  40  200",
            "71503",
        )];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
