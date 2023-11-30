use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/1.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut sums = Vec::new();

    for chunk in input.split("\n\n") {
        let mut sum = 0;
        for line in chunk.lines() {
            // line is a i32
            sum += line.parse::<i32>()?;
        }
        sums.push(sum);
    }

    sums.sort();
    let num = sums.len();
    let top_three: i32 = sums[num - 3..].iter().sum();
    println!("{:?}", top_three);

    Ok(format!("{}", top_three))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let result = "45000".to_string();
        assert!(solve(input).unwrap() == result);

        Ok(())
    }
}
