use std::fs;

use picoprob::aoc::aoc_2024;

fn main() {
    let input = fs::read_to_string("./src/aoc/data/2024_1").unwrap();
    let output = aoc_2024::one(&input);
    println!("{:?}", output);
}
