// 9:38 start

// action1 - append
// action2 - copy (1..N)

// in: s, t --> out: min(time)
// - max(time) = len(s) + len(t)
// - overlap? substring?

// notes:
// -> level of constraints: substring -> prefix
// -> max bounding was right thinking. i can find
//    the answer with calculation. perhaps to give
//    better idea of the goal.
//    time |S|+|T| -|copy|+1

// smaller, bigger
// if bigger.contains(smaller):
//   return bigger.len() + 1

// GARAGE
// ------
// GARAGEFORSALE ->13+1

// ABCDE
// -
// AABCD -> 10

// TRAINING
//  ------- stronger constraint: prefix. not substring
// DRAINING -> 16
// bigger.contains_prefix(smaller)

// 9:52 WA
// -> whe does prefix fail to find *min* time??
// -> counterexample??
// -> found it, but by exploring via representation

// XXX***
// ---
// XXX***

// i:3
// S=6 ==> s_time = 3
// T=6 ==> t_time = 3

// expected: 10
// actual: 12

// ------- stronger constraint: bigger.contains_prefix(all_of_smallers_prefixes)
// -> how to evaluate all_of_smallers_prefixes?
// -> generate all upfront
// -> pay less by sliding window both s and t

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_a() {
//         let input = "4
// GARAGE
// GARAGEFORSALE
// ABCDE
// AABCD
// TRAINING
// DRAINING
// JEFABC
// JEFDEF
// ";
//         let output = a(&input);
//         for o in output {
//             println!("{o}");
//         }
//     }
// }

fn a(input: &str) -> Vec<usize> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let (s, t) = (
            input.by_ref().next().unwrap(),
            input.by_ref().next().unwrap(),
        );

        // s: smaller, t: bigger
        let (s, t) = if s.len() > t.len() { (t, s) } else { (s, t) };
        let (s, S, t, T) = (
            s.chars().collect::<Vec<_>>(),
            s.len(),
            t.chars().collect::<Vec<_>>(),
            t.len(),
        );

        let mut i = 0; // common_prefix_len
        while i < S && s[i] == t[i] {
            // only check s, since s < t
            i += 1;
        }

        let s_time = S - i;
        let t_time = T - i;
        let i = if i > 0 { i + 1 } else { i }; // copy time if common_prefix exists
        output.push(s_time + t_time + i);
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
