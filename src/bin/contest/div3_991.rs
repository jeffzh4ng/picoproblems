#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "5
3 1
a
b
c
2 9
alpha
beta
4 12
hello
world
and
codeforces
3 2
ab
c
d
3 2
abc
ab
a
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
        let toks = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (n, mut m) = (toks[0], toks[1] as i32);
        let words = input.by_ref().take(n).collect::<Vec<_>>();

        let (mut count, mut i) = (0, 0);
        while i < words.len() && m - words[i].len() as i32 >= 0 {
            m -= words[i].len() as i32;
            count += 1;
            i += 1;
        }
        output.push(count)
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
