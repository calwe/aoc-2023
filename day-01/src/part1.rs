use std::iter::zip;
use anyhow::{Result, anyhow};

pub fn part1(input: &str) -> u64 {
    let first = input.lines()
        .map(|l| l.chars()
            .filter(|c| c.is_numeric())
            .next().unwrap());
    let last = input.lines()
        .map(|l| l.chars()
            .filter(|c| c.is_numeric())
            .last().unwrap());

    zip(first, last).map(|(a, b)| [a,b].iter().collect::<String>().parse::<u64>().unwrap()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(input), 142);
    }
}