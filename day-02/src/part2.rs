use anyhow::Result;

pub fn process(input: &str) -> Result<u64> {
    Ok(input.lines().map(|line| {
        let (_, results) = line.split_once(':').expect("Should be split by colon");

        let mut rmax = 0;
        let mut gmax = 0;
        let mut bmax = 0;
        for round in results.split(';') {
            for pair in round.trim().split(',') {
                let (count, color) = pair.trim().split_once(' ').expect("count color pair");
                let count: u64 = count.parse().expect("count should be numeric");

                match color {
                    "red" => if count > rmax {
                        rmax = count;
                    },
                    "green" => if count > gmax {
                        gmax = count
                    },
                    "blue" => if count > bmax {
                        bmax = count
                    },
                    _ => panic!("invalid color")
                }
            }
        }
        rmax * gmax * bmax
    }).sum())
}

#[cfg(test)]
mod test {
    use crate::part2::process;

    #[test]
    fn aoc_test_input() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(process(input).unwrap(), 2286)
    }
}
