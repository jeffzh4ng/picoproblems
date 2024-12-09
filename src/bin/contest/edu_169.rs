// 10:18 -> start

// 10:23
// - a point? or many points?

// 3 8
// -> yes (5)

// 5 6
// -> no (no int)

// 1 2 3 4 5 10
// -> no

// WA
// counterexample??

//

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_a() {
//         let input = "4
// 2
// 3 8
// 2
// 3 9
// 2
// 5 6
// 6
// 1 2 3 4 5 10
// ";
//         let output = a(&input);
//         for o in output {
//             println!("{o}");
//         }
//     }
// }

fn a(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let (N, n) = (
            input.by_ref().next().unwrap().parse::<usize>().unwrap(),
            input
                .by_ref()
                .next()
                .unwrap()
                .split(' ')
                .map(|tok| tok.parse::<u8>().unwrap())
                .collect::<Vec<_>>(),
        );

        // since the space is a line,
        // choosing a point that is
        // *different and closest*
        // to every other point is only
        // possible when N=2.

        // mapping a line to physical space,
        // it's impossible for me to draw such
        // a point with N=3. proof is trivial.

        // 10:35 blunder:
        // does not have to cut the space evenly

        if N > 2 {
            output.push("NO".to_string())
        } else if N == 2 {
            // if n[1] - n[0] > 1 && (n[1] - n[0]) % 2 != 0 {
            if n[1] - n[0] > 1 {
                output.push("YES".to_string())
            } else {
                output.push("NO".to_string())
            }
        } else {
            panic!() // n must be 2 due to input constraints 2 <= n <= 40
        }
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
