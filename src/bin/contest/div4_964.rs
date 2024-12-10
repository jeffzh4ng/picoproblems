#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "8
77
21
40
34
19
84
10
99
";
        let output = a(&input);
        for o in output {
            println!("{o}");
        }
    }

    #[test]
    fn test_b() {
        let input = "5
3 8 2 6
1 1 1 1
10 10 2 2
1 1 10 10
3 8 7 2
";
        let output = b(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn b(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let parsed_input = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let (a1, a2, b1, b2) = (
            parsed_input[0],
            parsed_input[1],
            parsed_input[2],
            parsed_input[3],
        );

        let mut count = 0;
        // picks a1 first
        if a1 > b1 && a2 >= b2 || a1 >= b1 && a2 > b2 {
            count += 1;
        }
        if a1 > b2 && a2 >= b1 || a1 >= b2 && a2 > b1 {
            count += 1;
        }

        // picks a2 first
        if a2 > b1 && a1 >= b2 || a2 >= b1 && a1 > b2 {
            count += 1;
        }
        if a2 > b2 && a1 >= b1 || a2 >= b2 && a1 > b1 {
            count += 1;
        }

        output.push(count)
    }

    output
}

fn a(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let n = input
            .by_ref()
            .next()
            .unwrap()
            .chars()
            .fold(0, |p, c| p + c.to_string().parse::<i32>().unwrap());

        output.push(n)
    }

    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = b(&input);
    for o in output {
        println!("{o}");
    }
}
