use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap},
    io,
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

pub fn two(input: &str) -> (usize, usize) {
    fn safe_window(is_asc: bool, a: &i128, b: &i128) -> bool {
        let direction_ok = (is_asc && a < b) || (!is_asc && a > b);
        let difference_ok = 1 <= (b - a).abs() && (b - a).abs() <= 3;

        direction_ok && difference_ok
    }

    fn safe<I: Iterator<Item = i128>>(i: I) -> (bool, bool) {
        let vals = i.collect::<Vec<_>>();

        let asc_from_front = match vals.iter().take(2).collect::<Vec<_>>().as_slice() {
            &[a, b] => a < b,
            _ => panic!(),
        };
        let safe = vals
            .iter()
            .try_fold(None, |prev: Option<&i128>, next| match prev {
                Some(p) => {
                    if safe_window(asc_from_front, p, next) {
                        Ok(Some(next))
                    } else {
                        Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "picoprob: invalid sequence",
                        ))
                    }
                }
                None => Ok(Some(next)),
            })
            .is_ok();

        let asc_from_back = match vals.iter().rev().take(2).collect::<Vec<_>>().as_slice() {
            &[a, b] => b < a, // b: n-2, a:n-1
            _ => panic!(),
        };

        let safe_dampened = if safe {
            true
        } else {
            fn is_safe_dampened(is_asc: bool, vals: &[i128]) -> bool {
                let n = vals.len();
                let mut safe = true;

                for i in 0..(n - 1) {
                    let (p, next) = (vals[i], vals[i + 1]);
                    if safe_window(is_asc, &p, &next) {
                        continue; // OK
                    } else {
                        // VIOLATION at i
                        // check whether (removal of i ==> safely dampened)

                        // remove i
                        let mut safely_dampened = true;
                        for j in 0..(n - 1) {
                            if j == i {
                                // check to see if j-1 and j+1 are good.
                                if j == 0
                                    || (j > 0 && safe_window(is_asc, &vals[j - 1], &vals[j + 1]))
                                {
                                    continue;
                                } else {
                                    // removal of VIOLATION DOES NOT DAMPEN
                                    safely_dampened = false;
                                    break;
                                }
                            }

                            let (p, next) = (vals[j], vals[j + 1]);
                            if safe_window(is_asc, &p, &next) {
                                continue;
                            } else {
                                // ANOTHER VIOLATION, CANNOT DAMPEN
                                safely_dampened = false;
                                break;
                            };
                        }

                        if !safely_dampened {
                            safe = false;
                        } else {
                            println!("list {:?} was safely dampened at i: {:?}", vals, i);
                        }
                        break;
                    }
                }

                safe
            }

            // check if list is safe based on direction from 0->1 OR check if list is safe based on direction from (n-2)->(n-1)
            is_safe_dampened(asc_from_front, &vals) || is_safe_dampened(asc_from_back, &vals)
        };

        (safe, safe_dampened)
    }

    let (safe, safe_dampened): (Vec<bool>, Vec<bool>) = input
        .lines()
        .map(|l| l.split(' ').map(|c| c.parse::<i128>().unwrap()))
        .map(|i| safe(i))
        .unzip(); // allocate for readability

    let safe_count = safe.iter().filter(|safe| **safe).collect::<Vec<_>>().len();
    let safe_dampened_count = safe_dampened
        .iter()
        .filter(|safe| **safe)
        .collect::<Vec<_>>()
        .len();

    (safe_count, safe_dampened_count)
}

#[cfg(test)]
mod tests_two {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let (output, _) = two(input);
        assert_eq!(output, 2);

        let input_two = fs::read_to_string("./src/aoc/data/2024_2").unwrap();
        let (output_two, _) = two(&input_two);
        assert_eq!(output_two, 218);
    }

    #[test]
    fn part_two() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        let (_, output) = two(input);
        println!("{:?}", output);
        assert_eq!(output, 4);

        let input_two = "1 10 9 7";
        let (_, output_two) = two(&input_two);
        assert_eq!(output_two, 1);

        let input_three = fs::read_to_string("./src/aoc/data/2024_2").unwrap();
        let (_, output_three) = two(&input_three);
        println!("{output_three}");
        // assert_eq!(output_three, 218);

        let input_four = "1 3 2 4 5";
        let (_, output_four) = two(&input_four);
        assert_eq!(output_four, 1);
    }
}
