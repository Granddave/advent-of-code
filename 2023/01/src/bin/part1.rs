use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/1.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "";
        let result = "".to_string();
        assert!(solve(input).unwrap() == result);

        Ok(())
    }
}
