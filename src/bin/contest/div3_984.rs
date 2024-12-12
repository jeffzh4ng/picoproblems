#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "4
100
4
1 1
2 0
2 0
3 1
1100000
3
6 1
7 1
4 1
111010
4
1 1
5 0
4 1
5 0
0100
4
3 1
1 1
2 0
2 1
";
        let output = c(&input);
        for o in output {
            println!("{o}");
        }
    }
}

fn c(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let (mut s, q) = (
            input
                .by_ref()
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
            input.by_ref().next().unwrap().parse::<usize>().unwrap(),
        );

        let queries = input
            .by_ref()
            .take(q)
            .map(|l| {
                let foo = l.split(' ').collect::<Vec<_>>();
                (
                    foo[0].parse::<usize>().unwrap() - 1,
                    foo[1].parse::<u8>().unwrap(),
                )
            })
            .collect::<Vec<_>>();

        for q in queries {
            let (i, v) = q;
            s[i] = v;

            let contains = s.windows(4).any(|window| window == &[1, 1, 0, 0]);
            if contains {
                output.push("YES".to_string())
            } else {
                output.push("NO".to_string())
            }
        }
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
