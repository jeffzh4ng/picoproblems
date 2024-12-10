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
    let output = a(&input);
    for o in output {
        println!("{o}");
    }
}
