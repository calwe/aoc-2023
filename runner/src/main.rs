use std::fs::read_to_string;
use day_02::part1::process;

fn main() {
    let input = read_to_string("runner/inputs/day-02.txt").unwrap();
    let result = process(&input);
    println!("{result:?}");
}
