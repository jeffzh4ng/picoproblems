use core::str;
use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    io, iter,
};

fn one(input: &str) -> (i128, i128) {
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

fn two(input: &str) -> (usize, usize) {
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
        // println!("{:?}", output);
        assert_eq!(output, 4);

        let input_two = "1 10 9 7";
        let (_, output_two) = two(&input_two);
        assert_eq!(output_two, 1);

        let input_three = fs::read_to_string("./src/aoc/data/2024_2").unwrap();
        let (_, output_three) = two(&input_three);
        println!("{output_three}");
        // assert_eq!(output_three, 218);

        // let input_four = "1 3 2 4 5";
        // let (_, output_four) = two(&input_four);
        // assert_eq!(output_four, 1);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TT {
    Int,
    Mul,
    LeftParen,
    RightParen,
    Comma,
    Illegal,
    Enable,
    Disable,
}

#[derive(Debug, PartialEq)]
struct Token {
    lexeme: Option<String>,
    typ: TT,
}

fn three(input: &str) -> (i128, i128) {
    // mul(X, Y)
    // X: 1-3 digits
    // Y: 1-3 digits

    fn lex_op(input: &[char]) -> Option<(Token, usize)> {
        if (input[0..3].iter().collect::<String>()).as_str() == "mul" {
            Some((
                Token {
                    lexeme: None,
                    typ: TT::Mul,
                },
                3,
            ))
        } else if (input[0..4].iter().collect::<String>()).as_str() == "do()" {
            Some((
                Token {
                    lexeme: None,
                    typ: TT::Enable,
                },
                4,
            ))
        } else if (input[0..7].iter().collect::<String>()).as_str() == "don't()" {
            Some((
                Token {
                    lexeme: None,
                    typ: TT::Disable,
                },
                7,
            ))
        } else {
            None
        }
    }

    fn lex_int(input: &[char]) -> Option<(Token, usize)> {
        let i = input.iter().take_while(|c| c.is_numeric()).count();
        if !(1 <= i && i <= 3) {
            None
        } else {
            let n = input[..i].iter().collect::<String>();

            Some((
                Token {
                    lexeme: Some(n),
                    typ: TT::Int,
                },
                i,
            ))
        }
    }

    fn lex(input: &[char]) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut pos = 0;

        while pos < input.len() {
            match input[pos] {
                '0'..='9' => {
                    if let Some((token, consumed)) = lex_int(&input[pos..]) {
                        tokens.push(token);
                        pos += consumed;
                    } else {
                        tokens.push(Token {
                            lexeme: None,
                            typ: TT::Illegal,
                        });
                        pos += 1;
                    }
                }
                'm' | 'd' => {
                    if let Some((token, consumed)) = lex_op(&input[pos..]) {
                        tokens.push(token);
                        pos += consumed;
                    } else {
                        tokens.push(Token {
                            lexeme: None,
                            typ: TT::Illegal,
                        });
                        pos += 1;
                    }
                }
                '(' => {
                    tokens.push(Token {
                        lexeme: None,
                        typ: TT::LeftParen,
                    });
                    pos += 1;
                }
                ')' => {
                    tokens.push(Token {
                        lexeme: None,
                        typ: TT::RightParen,
                    });
                    pos += 1;
                }
                ',' => {
                    tokens.push(Token {
                        lexeme: None,
                        typ: TT::Comma,
                    });
                    pos += 1;
                }
                _ => {
                    tokens.push(Token {
                        lexeme: None,
                        typ: TT::Illegal,
                    });
                    pos += 1;
                }
            }
        }

        tokens
    }

    fn parse(input: &[Token]) -> (Vec<(i128, i128)>, Vec<(i128, i128)>) {
        let mut output = Vec::new();
        let mut output_toggled = Vec::new();
        let mut i = 0;
        let mut enabled = true;

        const WIN: [&TT; 6] = [
            &TT::Mul,
            &TT::LeftParen,
            &TT::Int,
            &TT::Comma,
            &TT::Int,
            &TT::RightParen,
        ];

        while i + 5 < input.len() {
            // mul(X,Y)
            let sliding_win: [&TT; 6] = std::array::from_fn(|n| &input[i + n].typ);
            if sliding_win == WIN {
                let (x, y) = (
                    input[i + 2]
                        .lexeme
                        .as_ref()
                        .unwrap()
                        .parse::<i128>()
                        .unwrap(),
                    input[i + 4]
                        .lexeme
                        .as_ref()
                        .unwrap()
                        .parse::<i128>()
                        .unwrap(),
                );
                output.push((x, y));
                if enabled {
                    output_toggled.push((x, y));
                }
                i += 6;
            } else {
                match input[i].typ {
                    TT::Enable => enabled = true,
                    TT::Disable => enabled = false,
                    _ => (),
                }
                i += 1;
            }
        }

        (output, output_toggled)
    }

    let tokens = lex(&input.chars().collect::<Vec<_>>());
    let mul_ops = parse(&tokens);

    let output = mul_ops
        .0
        .iter()
        .fold(0, |prev, next| prev + (next.0 * next.1));

    let output_toggled = mul_ops
        .1
        .iter()
        .fold(0, |prev, next| prev + (next.0 * next.1));

    (output, output_toggled)
}

#[cfg(test)]
mod tests_three {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let (output, _) = three(input);
        assert_eq!(output, 161);

        let input_two = fs::read_to_string("./src/aoc/data/2024_3").unwrap();
        let (output_two, _) = three(&input_two);
        assert_eq!(output_two, 175700056);
    }

    #[test]
    fn part_two() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let (_, output) = three(input);
        assert_eq!(output, 48);

        let input_two = fs::read_to_string("./src/aoc/data/2024_3").unwrap();
        let (_, output_two) = three(&input_two);
        println!("{output_two}")
        // assert_eq!(output_two, 175700056);
    }
}

fn four(input: &str) -> (usize, usize) {
    fn file_hit(
        haystack: &Vec<Vec<char>>,
        needle: &[char],
        (r, c): (usize, usize),
        file: &[(i32, i32)],
    ) -> bool {
        let mut file_hit = true;
        for (i, (r_offset, c_offset)) in file.iter().enumerate() {
            let out_of_bounds = (r as i32 + r_offset) < 0 // no usize cast. panics in debug. wraps in release.
                || (r as i32 + r_offset) as usize >= haystack.len()
                || (c as i32 + c_offset) < 0 // no usize cast. panics in debug. wraps in release.
                || (c as i32 + c_offset) as usize >= haystack[0].len();

            if !out_of_bounds {
                let (new_r, new_c) = (
                    (r as i32 + r_offset) as usize,
                    (c as i32 + c_offset) as usize,
                );

                if haystack[new_r][new_c] != needle[i] {
                    file_hit = false;
                    break;
                }
            } else {
                file_hit = false;
                break;
            }
        }

        file_hit
    }

    fn hits(haystack: &Vec<Vec<char>>, (r, c): (usize, usize)) -> (usize, usize) {
        // only searches starting from X to avoid double counting
        const INDEX_OFFSETS_ONE: [[(i32, i32); 4]; 8] = [
            [(0, 0), (1, 0), (2, 0), (3, 0)],       // north
            [(0, 0), (-1, 0), (-2, 0), (-3, 0)],    // south
            [(0, 0), (0, 1), (0, 2), (0, 3)],       // east
            [(0, 0), (0, -1), (0, -2), (0, -3)],    // west
            [(0, 0), (1, 1), (2, 2), (3, 3)],       // north east
            [(0, 0), (-1, 1), (-2, 2), (-3, 3)],    // south east
            [(0, 0), (1, -1), (2, -2), (3, -3)],    // north west
            [(0, 0), (-1, -1), (-2, -2), (-3, -3)], // south west
        ];
        const INDEX_OFFSETS_TWO: [[(i32, i32); 5]; 6] = [
            // |permutations(MSMS) = permutations(MM)| = 4*3
            // combinations = (4*3)choose2 --> M_sally, M_bob are considered same.
            // M: top left
            // M M           M S       M S
            // S S           M S       S M
            [(0, 0), (1, -1), (1, 1), (-1, -1), (-1, 1)],
            [(0, 0), (1, -1), (-1, -1), (1, 1), (-1, 1)],
            [(0, 0), (1, -1), (-1, 1), (1, 1), (-1, -1)],
            // M: top right
            // M M (covered)  S M       S M
            // S S            M S       S M
            [(0, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)],
            [(0, 0), (1, 1), (-1, 1), (1, -1), (-1, -1)],
            // M: bottom right
            // M S (covered)  S M (covered)   S S
            // S M            S M             M M
            [(0, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)],
            // M: bottom left
            // M S (covered)    S M (covered)   S S (covered)
            // M S              M S             M M
        ];
        let needle_one = "XMAS".chars().collect::<Vec<_>>();
        let needle_two = "AMMSS".chars().collect::<Vec<_>>();

        // one
        let mut output_one = 0;
        for file in INDEX_OFFSETS_ONE {
            if file_hit(haystack, &needle_one, (r, c), &file) {
                output_one += 1;
            }
        }

        // two
        let mut output_two = 0;
        for file in INDEX_OFFSETS_TWO {
            if file_hit(haystack, &needle_two, (r, c), &file) {
                output_two += 1;
                break;
            }
        }

        (output_one, output_two)
    }

    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (n, m) = (grid.len(), grid[0].len());
    let count = (0..n)
        .flat_map(|r| (0..m).map(move |c| (r, c)))
        .map(|(r, c)| hits(&grid, (r, c)))
        .fold((0, 0), |p, n| (p.0 + n.0, p.1 + n.1));

    count
}

#[cfg(test)]
mod tests_four {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let (output, _) = four(input);
        assert_eq!(output, 18);

        let input_two = fs::read_to_string("./src/aoc/data/2024_4").unwrap();
        let (output_two, _) = four(&input_two);
        assert_eq!(output_two, 2536);
    }

    #[test]
    fn part_two() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
        let (_, output) = four(input);
        assert_eq!(output, 9);

        let input_two = fs::read_to_string("./src/aoc/data/2024_4").unwrap();
        let (_, output_two) = four(&input_two);
        println!("{:?}", output_two);
        assert_eq!(output_two, 1940);
    }
}

// fn five(input: &str) -> i32 {
//     let input = input.split("\n\n").collect::<Vec<_>>();
//     let (order, update) = (input[0], input[1]);
//     let graph = order
//         .lines()
//         .map(|l| {
//             let bar = l.split('|').collect::<Vec<_>>();
//             (
//                 bar[0].parse::<i32>().unwrap(),
//                 bar[1].parse::<i32>().unwrap(),
//             )
//         })
//         .fold(HashMap::new(), |mut p, (u, v)| {
//             p.entry(u)
//                 .and_modify(|edges: &mut HashSet<i32>| {
//                     edges.insert(v);
//                 })
//                 .or_insert(HashSet::from([v]));
//             p
//         });

//     // blunder 1: recover total order with grand_children.len()
//     // blunder 2: recover total order with grand_children.contains()
//     // idea 3: partial order with toposort

//     let sum = update
//         .lines()
//         .map(|l| l.split(',').map(|tok| tok.parse::<i32>().unwrap()))
//         .map(|update| {
//             let update = update.collect::<Vec<_>>();
//             let valid = update.iter().try_fold(None, |p, v| match p {
//                 Some(u) => {
//                     if total_order.get(u).or(Some(&-1)) < total_order.get(v).or(Some(&-1)) {
//                         Err(io::Error::new(
//                             io::ErrorKind::InvalidData,
//                             "picoprob: invalid data",
//                         ))
//                     } else {
//                         Ok(Some(v))
//                     }
//                 }
//                 None => Ok(Some(v)),
//             });

//             (valid.is_ok(), update)
//         })
//         // .inspect(|x| println!("{:?}", x))
//         .filter_map(|(valid, update)| if valid { Some(update) } else { None })
//         .map(|update| update[update.len() / 2])
//         // .inspect(|x| println!("{:?}", x))
//         .sum();

//     sum
// }

// #[cfg(test)]
// mod tests_five {
//     use std::fs;

//     use super::*;

//     #[test]
//     fn part_one() {
//         let input = "47|53
// 97|13
// 97|61
// 97|47
// 75|29
// 61|13
// 75|53
// 29|13
// 97|29
// 53|29
// 61|53
// 97|53
// 61|29
// 47|13
// 75|47
// 97|75
// 47|61
// 75|61
// 47|29
// 75|13
// 53|13

// 75,47,61,53,29
// 97,61,53,29,13
// 75,29,13
// 75,97,47,61,53
// 61,13,29
// 97,13,75,29,47
// ";
//         let output = five(input);
//         assert_eq!(output, 143);

//         let input = fs::read_to_string("./src/aoc/data/2024_5").unwrap();
//         let output = five(&input);
//         println!("{:?}", output); // 7643 too high
//     }
// }
fn main() {}
