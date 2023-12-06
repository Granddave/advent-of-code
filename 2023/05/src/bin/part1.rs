use anyhow::{Ok, Result};

fn main() -> Result<()> {
    let input = include_str!("../../in.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

#[derive(Debug)]
struct Range {
    dst_start: i64,
    src_start: i64,
    src_len: i64,
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        value >= self.src_start && value < self.src_start + self.src_len
    }
}

fn solve(input: &str) -> Result<String> {
    let mut almanac_parts = input.split("\n\n");

    let mut seeds = almanac_parts
        .next()
        .expect("no seeds")
        .split(':')
        .nth(1)
        .expect("no seeds")
        .split_whitespace()
        .map(|n| n.parse::<i64>().expect("invalid seed"))
        .collect::<Vec<i64>>();

    let mappings = almanac_parts
        .map(|part| {
            part.lines()
                .skip(1)
                .map(|line| {
                    let values = line
                        .split_whitespace()
                        .map(|n| n.parse::<i64>().expect("invalid value"))
                        .collect::<Vec<i64>>();
                    Range {
                        dst_start: values[0],
                        src_start: values[1],
                        src_len: values[2],
                    }
                })
                .collect::<Vec<Range>>()
        })
        .collect::<Vec<_>>();

    let min_value = seeds
        .iter_mut()
        .map(|seed| {
            for ranges in &mappings {
                if let Some(range) = ranges.iter().filter(|range| range.contains(*seed)).next() {
                    *seed += range.dst_start - range.src_start
                }
            }
            *seed
        })
        .min()
        .unwrap();

    Ok(min_value.to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let tests = vec![(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
",
            "35",
        )];

        for (input, expected) in tests {
            assert!(solve(input)? == expected);
        }

        Ok(())
    }
}
