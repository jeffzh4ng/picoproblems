#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "5
1 2
1 5
2 2
10 20
1 1000000000
";
        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn c(input: &str) -> Vec<usize> {
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
        let (lower, upper) = (split[0], split[1]);
        let mut A = vec![lower];
        let mut delta = 1;

        while A[A.len() - 1] + delta <= upper {
            A.push(A[A.len() - 1] + delta);
            delta += 1;
        }

        output.push(A.len());
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
