#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        let input = "3
5
3 5 2 1 3
2
abfda
afbfa
2
1 2
3
ab
abc
aa
4
5 -3 5 -3
5
aaaa
bcbc
aba
cbcb
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
        let T = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let t = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let S = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let ss = input.by_ref().take(S).collect::<Vec<_>>();

        for s in ss {
            if s.len() != t.len() {
                output.push("NO".to_string());
                continue;
            }

            // 1-1 check
            let mut map = HashMap::new();
            let mut one_to_one = true;
            for (i, c) in s.chars().enumerate() {
                if !map.contains_key(&c) && !map.values().any(|&x| x == t[i]) {
                    map.insert(c, t[i]);
                } else {
                    if map.get(&c).is_some_and(|&x| x != t[i])
                        || (map.iter().any(|(&k, &v)| v == t[i] && k != c))
                    {
                        output.push("NO".to_string());
                        one_to_one = false;
                        break;
                    }
                }
            }

            if one_to_one {
                output.push("YES".to_owned());
            }
        }
    }

    output
}

use std::collections::HashMap;
use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = c(&input);
    for o in output {
        println!("{o}");
    }
}
