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
    let output = a(&input);
    for o in output {
        println!("{o}");
    }
}
