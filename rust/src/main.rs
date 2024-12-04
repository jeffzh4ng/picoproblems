use std::fs;

use picoprob::aoc::aoc_2024::three;

fn main() {
    let input = fs::read_to_string("./src/aoc/data/2024_3").unwrap();
    let output = three(&input);
    println!("{output}");
}
