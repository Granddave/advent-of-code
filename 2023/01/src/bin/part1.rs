use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/1.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let first_number = line.chars().find(|c| c.is_digit(10)).expect("first");
        let last_number = line.chars().rev().find(|c| c.is_digit(10)).expect("last");
        let combined_number = format!("{}{}", first_number, last_number);
        sum += combined_number.parse::<i32>()?;
    }

    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = "142".to_string();
        assert!(solve(input).unwrap() == result);

        Ok(())
    }
}
