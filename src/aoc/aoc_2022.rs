fn one(input: &str) -> (i32, i32) {
    // let _input = Vec::new();

    // &str.split("\n\n"). alternatively Vec.split(|l| l.is_empty()).map(|chunk|)
    let chunks = input
        .split("\n\n")
        .map(|c| {
            c.lines()
                .map(|l| l.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cal_per_chunk = chunks
        .iter()
        .map(|c| c.iter().fold(0, |p, n| p + n))
        .collect::<Vec<_>>();
    cal_per_chunk.sort();
    let (x, y, z) = (
        cal_per_chunk[cal_per_chunk.len() - 1],
        cal_per_chunk[cal_per_chunk.len() - 2],
        cal_per_chunk[cal_per_chunk.len() - 3],
    );

    (*cal_per_chunk.iter().max().unwrap(), x + y + z)
}

#[cfg(test)]
mod tests_one {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let (output, _) = one(input);
        assert_eq!(output, 24000);

        let input_two = fs::read_to_string("./src/aoc/data/2022_1").unwrap();
        let (output_two, _) = one(&input_two);
        println!("{output_two}");
    }

    #[test]
    fn part_two() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let (_, output) = one(input);
        assert_eq!(output, 45000);

        let input_two = fs::read_to_string("./src/aoc/data/2022_1").unwrap();
        let (_, output_two) = one(&input_two);
        println!("{output_two}");
    }
}
