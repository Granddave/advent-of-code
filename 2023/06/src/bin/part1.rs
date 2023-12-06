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
                .split_whitespace()
                .map(|n| n.parse::<i32>().expect("invalid value"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut num_records = vec![];
    for (ix, time) in lines.get(0).unwrap().iter().enumerate() {
        let record = lines.get(1).unwrap().get(ix).unwrap();
        let mut new_records = vec![];
        for hold_time in 0..*time {
            let distance = (time - hold_time) * hold_time;
            if distance > *record {
                new_records.push(distance);
            }
        }
        num_records.push(new_records.len());
    }

    Ok(num_records.iter().product::<usize>().to_string())
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
            "288",
        )];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
