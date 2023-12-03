use anyhow::Result;

pub fn process(input: &str) -> Result<u64> {
    Ok(input.lines().map(|line| {
        let (game, results) = line.split_once(':').expect("Should be split by colon");
        let game_id = game.strip_prefix("Game ").expect("Begin with 'Game '")
            .parse::<u64>().expect("Game ID should be number");

        if results.split(';').find(|round| {
            round.trim().split(',').find(|pair| {
                let (count, color) = pair.trim().split_once(' ').expect("count color pair");
                let count: u64 = count.parse().expect("count should be numeric");

                match color {
                    "red" => count > 12,
                    "green" => count > 13,
                    "blue" => count > 14,
                    _ => panic!("invalid color")
                }
            }).is_some()
        }).is_none() {
            game_id
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
mod test {
    use crate::part1::process;

    #[test]
    fn aoc_test_input() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(process(input).unwrap(), 8)
    }
}