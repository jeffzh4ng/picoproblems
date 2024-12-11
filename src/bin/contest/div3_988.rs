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

    #[test]
    fn test_b() {
        let input = "5
3
1 1 2
11
3 3 4 5 6 7 8 9 9 10 11
8
8 4 8 3 8 2 8 1
6
2 1 4 5 3 3
8
1 2 6 3 8 5 5 3
";
        let output = b(&input);
        for (x, y) in output {
            println!("{x} {y}");
        }
    }
}

fn b(input: &str) -> Vec<(usize, usize)> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let N = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let n = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        // _, _, N-2 grid
        //       (_, _)
        //       (_, _)
        //       (_, _)

        // idea 1: for i in 0..1-(N-2)/2:
        //           if (N-2)%i==0: output.push((_, _))
        //         panic!() // guaranteed a solution. just scrambled.

        // dividing odd is fine, division is truncating
        // 7/2=3 -> 7%3

        let mut factors = Vec::new();
        for i in 1..=(((N - 2) / 2).max(1)) {
            if (N - 2) % i == 0 {
                factors.push((i, (N - 2) / i));
            }
        }

        for (x, y) in factors {
            if n.contains(&x) && n.contains(&y) {
                output.push((x, y));
                break;
            }
        }
    }

    output
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
    let output = b(&input);
    for (x, y) in output {
        println!("{x} {y}");
    }
}
