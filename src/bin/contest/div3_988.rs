#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "5
1
1
2
2 2
2
1 2
4
1 2 3 1
6
1 2 3 1 2 3
";
        let output = a(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn a(input: &str) -> Vec<u32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let n = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let parsed_input = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| (tok.parse::<u32>().unwrap() - 1) as usize)
            .collect::<Vec<_>>();

        let pair_count = parsed_input
            .iter()
            .fold(vec![0; n], |mut p, &n| {
                // char counts
                p[n] += 1;
                p
            })
            .iter()
            .fold(0, |p, n| p + (n / 2)); // pair count (score)

        output.push(pair_count)
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
