use anyhow::Result;

fn main() -> Result<()> {
    let input = include_str!("../../input/1.txt");
    let output = solve(input)?;
    println!("{}", output);

    Ok(())
}

fn solve(input: &str) -> Result<String> {
    let mut sum = 0;

    let number_words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in input.lines().filter(|l| !l.is_empty()) {
        let first_number = {
            let digit = line.chars().find(|c| c.is_digit(10));
            let digit_pos = match digit {
                Some(digit) => line.chars().position(|c| c == digit),
                None => None,
            };

            let mut first_word = String::new();
            let word_pos = {
                let mut first_word_pos = usize::MAX;
                for w in &number_words {
                    let word_pos = line.find(w);
                    if let Some(word_pos) = word_pos {
                        if word_pos < first_word_pos {
                            first_word_pos = word_pos;
                            first_word = w.to_string()
                        }
                    }
                }

                if first_word_pos != usize::MAX {
                    Some(first_word_pos)
                } else {
                    None
                }
            };

            if word_pos.is_none() || digit_pos.is_some() && digit_pos.unwrap() < word_pos.unwrap() {
                digit.unwrap()
            } else {
                number_words
                    .iter()
                    .position(|w| w == &first_word)
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
            }
        };

        let last_number = {
            let digit = line.chars().rev().find(|c| c.is_digit(10));
            let digit_pos = match digit {
                Some(digit) => {
                    Some(line.len() - line.chars().rev().position(|c| c == digit).unwrap())
                }
                None => None,
            };

            let mut last_word = String::new();
            let word_pos = {
                let mut last_word_pos = None;
                for w in &number_words {
                    let word_pos = line.rfind(w);
                    if let Some(word_pos) = word_pos {
                        if last_word_pos.is_none() || word_pos > last_word_pos.unwrap() {
                            last_word_pos = Some(word_pos);
                            last_word = w.to_string()
                        }
                    }
                }

                last_word_pos
            };

            if word_pos.is_none() || digit_pos.is_some() && digit_pos.unwrap() > word_pos.unwrap() {
                digit.unwrap()
            } else {
                number_words
                    .iter()
                    .position(|w| w == &last_word)
                    .unwrap()
                    .to_string()
                    .chars()
                    .next()
                    .unwrap()
            }
        };
        let combined_number = format!("{}{}", first_number, last_number);
        // eprintln!("{}: {:?}", line, combined_number);
        sum += combined_number.parse::<i32>().expect("combine parse")
    }

    Ok(format!("{}", sum))
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
kvhfqspcpsxndjqlonesixthree24kdmqvone
17ninenineninegczplbj81";
        let result = "303".to_string();
        assert!(solve(input).unwrap() == result);

        Ok(())
    }
}
