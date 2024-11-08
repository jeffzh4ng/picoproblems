use std::fs;

use picoprob::aoc::aoc_2015;

fn main() {
    let input = fs::read_to_string("./src/aoc/data/2015_1").unwrap();
    let output = aoc_2015::one(&input).unwrap();
    println!("{:?}", output);
}
