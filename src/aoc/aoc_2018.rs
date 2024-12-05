use std::collections::{HashMap, HashSet};

fn one(input: &str) -> (i32, i32) {
    let input = input
        .lines()
        .map(|l| {
            let (sign, dig) = (&l.chars().nth(0).unwrap(), &l[1..]);
            let n = dig.parse::<i32>().unwrap();

            match sign {
                '+' => n,
                '-' => -n,
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    let sum = input.iter().fold(0, |p, n| p + n);

    let mut seen = HashSet::new();
    let (mut freq, mut i) = (0, 0);

    // [1, -2, 3, 1]
    while !seen.contains(&freq) {
        seen.insert(freq); // check
        freq += input[i]; // insert
        i = if i + 1 == input.len() { 0 } else { i + 1 }; // step
    }

    (sum, freq)
}

#[cfg(test)]
mod tests_one {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "+1
-2
+3
+1
";
        let (output, _) = one(input);
        assert_eq!(output, 3);

        let input_two = "+1
+1
+1
";
        let (output_two, _) = one(input_two);
        assert_eq!(output_two, 3);

        let input_three = "+1
+1
-2
";
        let (output_three, _) = one(input_three);
        assert_eq!(output_three, 0);

        let input_four = "-1
-2
-3
";
        let (output_four, _) = one(input_four);
        assert_eq!(output_four, -6);

        let input_five = fs::read_to_string("./src/aoc/data/2018_1").unwrap();
        let (output_five, _) = one(&input_five);
        println!("{:?}", output_five);
    }

    #[test]
    fn part_two() {
        let input = "+1
-2
+3
+1
";
        let (_, output) = one(input);
        assert_eq!(output, 2);

        let input = "+1
-1
";
        let (_, output) = one(input);
        assert_eq!(output, 0);

        let input = "+3
+3
+4
-2
-4
";
        let (_, output) = one(input);
        assert_eq!(output, 10);

        let input = "-6
+3
+8
+5
-6
";
        let (_, output) = one(input);
        assert_eq!(output, 5);

        let input = "+7
+7
-2
-7
-4
";
        let (_, output) = one(input);
        assert_eq!(output, 14);

        let input = fs::read_to_string("./src/aoc/data/2018_1").unwrap();
        let (_, output) = one(&input);
        println!("{:?}", output);
    }
}

fn two(input: &str) -> (i32, String) {
    fn bar<I: Iterator<Item = char>>(chars: I) -> (i32, i32) {
        let counts = chars.fold(HashMap::new(), |mut p, n| {
            p.entry(n).and_modify(|c| *c += 1).or_insert(1);
            p
        });

        let has_double = counts.iter().any(|(_, c)| *c == 2);
        let has_triplet = counts.iter().any(|(_, c)| *c == 3);

        let (x, y) = (
            if has_double { 1 } else { 0 },
            if has_triplet { 1 } else { 0 },
        );

        (x, y)
    }

    let (x, y) = input
        .lines()
        .map(|l| l.chars())
        .map(|chars| bar(chars))
        .fold((0, 0), |p, n| (p.0 + n.0, p.1 + n.1));

    let common_pair = input
        .lines()
        .enumerate()
        .flat_map(|(i, line_one)| {
            input
                .lines()
                .skip(i + 1)
                .map(move |line_two| (line_one, line_two))
        })
        .map(|(s, t)| {
            let s_counts = s.chars().fold(HashMap::new(), |mut p, n| {
                p.entry(n).and_modify(|c| *c += 1).or_insert(1);
                p
            });

            let t_counts = t.chars().fold(HashMap::new(), |mut p, n| {
                p.entry(n).and_modify(|c| *c += 1).or_insert(1);
                p
            });

            ((s, s_counts), (t, t_counts))
        })
        .filter_map(|((s, s_counts), (t, t_counts))| {
            let mut common = true;
            let mut violation = None;

            for k in s_counts.keys() {
                match (s_counts.get(k), t_counts.get(k)) {
                    (None, None) => continue,
                    (None, Some(_)) | (Some(_), None) => {
                        if violation.is_some() {
                            common = false;
                            break;
                        } else {
                            violation = Some(*k)
                        }
                    }
                    (Some(v1), Some(v2)) => {
                        if v1 != v2 {
                            if violation.is_some() {
                                common = false;
                                break;
                            } else {
                                violation = Some(*k)
                            }
                        }
                    }
                }
            }

            if common && violation.is_some() {
                Some((s, t, violation.unwrap()))
            } else {
                None
            }
        })
        // .take(1)
        .collect::<Vec<_>>();

    println!("{:?}", common_pair);

    let pair = common_pair.get(0);
    let common_string = match pair {
        Some((s, _, violation)) => s.chars().filter(|c| c != violation).collect::<String>(),
        None => panic!(), // input should contain one common pair
    };

    (x * y, common_string)
}

#[cfg(test)]
mod tests_two {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";
        let (output, _) = two(input);
        assert_eq!(output, 12);

        let input = fs::read_to_string("./src/aoc/data/2018_2").unwrap();
        let (output, _) = two(&input);
        assert_eq!(output, 4980);
    }

    #[test]
    fn part_two() {
        let input = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

        let (_, output) = two(input);
        assert_eq!(output, "fgij".to_string());

        // let input = fs::read_to_string("./src/aoc/data/2018_2").unwrap();
        // let (_, output) = two(&input);
        // println!("{:?}", output);
    }
}
