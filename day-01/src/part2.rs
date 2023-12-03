const NUMS: [&'static str; 20] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
];

pub fn part2(input: &str) -> u64 {
    input.lines().map(|l| {
        let first = NUMS.iter()
            .map(|n| l.match_indices(n).next())
            .flatten()
            .min_by_key(|(i, _)| *i).unwrap().1;
        let first = replacenum(first);

        let last = NUMS.iter()
            .map(|n| l.rmatch_indices(n).next())
            .flatten()
            .max_by_key(|(i, _)| *i).unwrap().1;
        let last = replacenum(last);

        format!("{first}{last}").parse::<u64>().unwrap()
    }).sum()
}

fn replacenum<'a>(input: &str) -> String {
    input.replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9")
        .replace("zero", "0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_example() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part2(input), 281);
    }
}
