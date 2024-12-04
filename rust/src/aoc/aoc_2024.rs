use core::str;
use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap},
    io, iter,
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
        // println!("{:?}", output);
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

pub fn three(input: &str) -> (i128, i128) {
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
