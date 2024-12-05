use std::collections::HashSet;

fn one(input: &str) -> (i32, i32) {
    let set = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let n = input
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .find(|n| set.contains(&(2020 - n)))
        .unwrap(); // input should contain valid pair

    let m = input
        .lines()
        .flat_map(|i| {
            input
                .lines()
                .map(move |j| (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap()))
        })
        .find(|(x, y)| set.contains(&(2020 - x - y)))
        .unwrap();

    (n * (2020 - n), m.0 * m.1 * (2020 - m.0 - m.1))
}

#[cfg(test)]
mod tests_one {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "1721
979
366
299
675
1456
";
        let (output, _) = one(input);
        assert_eq!(output, 514579);

        let input = fs::read_to_string("./src/aoc/data/2020_1").unwrap();
        let (output, _) = one(&input);
        assert_eq!(output, 918339);
    }

    #[test]
    fn part_two() {
        let input = "1721
979
366
299
675
1456
";
        let (_, output) = one(input);
        assert_eq!(output, 241861950);

        let input = fs::read_to_string("./src/aoc/data/2020_1").unwrap();
        let output = one(&input);
        println!("{:?}", output);
    }
}

fn two(input: &str) -> usize {
    let foo = input.lines().map(|l| {
        let bar = l.split_ascii_whitespace().collect::<Vec<_>>();
        let (range, char, str) = (bar[0], bar[1], bar[2]);
        // let (min, max) = (range)
    });

    todo!()
}

#[cfg(test)]
mod tests_two {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc
";
        let output = two(input);
        assert_eq!(output, 2);
    }
}
fn main() {}
