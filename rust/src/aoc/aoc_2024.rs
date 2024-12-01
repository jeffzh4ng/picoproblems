use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

pub fn one(input: &str) -> (i128, i128) {
    let (lh, rh, lv, rhm) = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|s| s.parse::<i128>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold(
            (
                BinaryHeap::new(),
                BinaryHeap::new(),
                Vec::new(),
                HashMap::new(),
            ),
            |(mut lh, mut rh, mut lv, mut rhm), v| {
                let (l, r) = (v[0], v[1]);

                // one
                lh.push(Reverse(l));
                rh.push(Reverse(r));

                // two
                lv.push(l);
                rhm.entry(r).and_modify(|e| *e += 1).or_insert(1);
                (lh, rh, lv, rhm)
            },
        );

    let sum = lh
        .into_sorted_vec()
        .into_iter()
        .zip(rh.into_sorted_vec())
        .rev()
        .map(|(Reverse(l), Reverse(r))| (l - r).abs())
        .sum();

    let sim = lv.iter().fold(0, |acc, l| {
        let occurences = rhm.get(l).unwrap_or(&0);
        acc + (l * occurences)
    });

    (sum, sim)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
        let (output, _) = one(&input);
        assert_eq!(output, 11);

        let input_two = fs::read_to_string("./src/aoc/data/2024_1").unwrap();
        let (output_two, _) = one(&input_two);
        assert_eq!(output_two, 1222801);
    }

    #[test]
    fn test_part_two() {
        let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
        let (_, output) = one(&input);
        assert_eq!(output, 31);

        let input_two = fs::read_to_string("./src/aoc/data/2024_1").unwrap();
        let (_, output_two) = one(&input_two);
        assert_eq!(output_two, 22545250);
    }
}
