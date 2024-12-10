#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "3
1 2
3 10
5 5
";
        let output = a(&input);
        for o in output {
            println!("{o}");
        }
    }

    #[test]
    fn test_b() {
        let input = "3
4
#...
.#..
..#.
...#
2
.#..
.#..
1
...#
";
        let output = b(&input);
        for o in output {
            println!("{o}");
        }
    }

    #[test]
    fn test_c() {
        let input = "3
9 11 3
0 10 8
1000000 100000 10
";

        // why is the optimal amount for tc3 199,999
        // and not 110,000 (1,000,000/10  + 100,000/10)?

        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn c(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let parsed_input = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (X, Y, d) = (parsed_input[0], parsed_input[1], parsed_input[2]);
        let (mut x, mut y) = (0, 0);

        let mut moves = 0;
        while x < X || y < Y {
            x += d;
            moves += 1;

            if x < X || y < Y {
                y += d;
                moves += 1;
            }
        }

        output.push(moves)
    }

    output
}

fn b(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let N = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let o = input
            .by_ref()
            .take(N)
            .map(|l| l.chars().enumerate().find(|(_, c)| *c == '#').unwrap())
            .map(|(i, _)| format!("{} ", i + 1))
            .collect::<String>();

        let o = o.chars().rev().collect::<String>();
        output.push(o)
    }

    output
}

fn a(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let toks = (input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>());

        let (a, b) = (toks[0], toks[1]);

        let mut min = i32::MAX;
        for c in a..=b {
            if (c - a) + (b - c) < min {
                min = (c - a) + (b - c)
            }
        }
        output.push(min)
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
