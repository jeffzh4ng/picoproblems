#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "3
2
4
6
";
        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn a(input: &str) -> Vec<usize> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let n = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        output.push(n - 1)
    }

    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = c(&input);
    for o in output {
        println!("{o}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "5
qwq
ppppp
pppwwwqqq
wqpqwpqwwqp
pqpqpqpq
";
        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn b(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let o = input
            .by_ref()
            .next()
            .unwrap()
            .chars()
            .rev()
            .map(|c| match c {
                'p' => 'q',
                'q' => 'p',
                'w' => 'w',
                _ => panic!(),
            })
            .collect::<String>();

        output.push(o)
    }

    output
}

fn div4_944D(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let s = input.by_ref().next().unwrap();
        let mut s_sorted = s.chars().collect::<Vec<_>>();
        s_sorted.sort();
        let s_sorted = s_sorted.iter().collect::<String>();

        if s == s_sorted {
            output.push(1)
        } else if s.chars().rev().collect::<String>() == s_sorted {
            output.push(2)
        } else {
            // contains substring '01'
            let s = s.chars().collect::<Vec<_>>();
            let mut count = 1;

            for i in 1..s.len() {
                if s[i] != s[i - 1] {
                    count += 1;
                }
            }

            output.push(count - 1)
        }
    }

    output
}

fn div4_898D(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let split = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, k) = (split[0], split[1]);
        let s = input.by_ref().next().unwrap().chars().collect::<Vec<_>>();

        let mut i = 0;
        let mut o = 0;
        while i < s.len() {
            if s[i] == 'B' {
                o += 1;
                i += k;
            } else {
                i += 1;
            }
        }
        output.push(o)
    }

    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = c(&input);
    for o in output {
        println!("{o}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "5
10 5 5 10
3 6 1 1
15 14 12 4
1 1 1 1
420 6 9 69
";
        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn div4_886d(input: &str) -> Vec<usize> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let split = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (N, k) = (split[0], split[1]);
        let mut ns = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        ns.sort();

        let (mut max_valid_subarray, mut valid_subarray) = (0, 1);
        for i in 1..ns.len() {
            if ns[i] - ns[i - 1] <= k {
                valid_subarray += 1;
            } else {
                max_valid_subarray = max_valid_subarray.max(valid_subarray);
                valid_subarray = 1;
            }
        }
        max_valid_subarray = max_valid_subarray.max(valid_subarray);

        output.push(ns.len() - max_valid_subarray);
    }

    output
}

fn c(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let split = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (m, mut first, mut second, mut fill) = (split[0], split[1], split[2], split[3]);

        // constrains first, second <= m, so we can flatten later
        if first > m {
            first = m;
        }
        if second > m {
            second = m;
        }

        let mut remaining_seats = 2 * m;
        // println!(
        //     "{:?}, {:?}, {:?}, {:?}, {:?}",
        //     m, remaining_seats, first, second, fill
        // );
        remaining_seats -= first; // simple greedy
        remaining_seats -= second; // simple greedy
        if fill > remaining_seats {
            fill = remaining_seats;
        }
        if remaining_seats > 0 {
            remaining_seats -= fill;
        }

        output.push(2 * m - remaining_seats)
    }

    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = c(&input);
    for o in output {
        println!("{o}");
    }
}
