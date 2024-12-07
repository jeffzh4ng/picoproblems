// 1. traverals:
// - cc/scc/ff
// - aps/bridges

// rust: .take()/.skip() for filter, .nth() for selection
// blunder: (1..=V).fold(|v|) is not sequential??
fn kattis_wheresmyinternet(input: &str) -> Vec<usize> {
    let (V, _) = input
        .lines()
        .take(1)
        .map(|l| {
            let split = l.split_ascii_whitespace().collect::<Vec<_>>();
            (
                split[0].parse::<usize>().unwrap(),
                split[1].parse::<usize>().unwrap(),
            )
        })
        .nth(0)
        .unwrap();

    let al = input
        .lines()
        .skip(1)
        .map(|l| {
            let split = l.split_ascii_whitespace().collect::<Vec<_>>();
            (
                split[0].parse::<usize>().unwrap(),
                split[1].parse::<usize>().unwrap(),
            )
        })
        // V+1 because nodes start at 1
        .fold(vec![vec![]; V + 1], |mut p, (v, w)| {
            p[v].push(w); // undirected
            p[w].push(v); // undirected
            p
        });

    fn dfs(al: &Vec<Vec<usize>>, v: usize, seen: &mut [bool]) -> () {
        seen[v] = true; // mark, no visit
        for &w in &al[v] {
            if !seen[w] {
                dfs(al, w, seen); // recurse
            }
        }
    }

    let mut seen = vec![false; V + 1];
    dfs(&al, 1, &mut seen);
    let not_seen = (1..=V).filter(|&v| !seen[v]).collect::<Vec<_>>();
    not_seen
}

fn kattis_dominoes2(input: &str) -> Vec<u32> {
    let n = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut output = Vec::new();
    let mut input = input.lines().skip(1);

    for _ in 0..n {
        let foo = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (V, E, R) = (foo[0], foo[1], foo[2]);

        let al = input
            .by_ref()
            .take(E)
            .map(|l| {
                let split = l.split(' ').collect::<Vec<_>>();

                (
                    split[0].parse::<usize>().unwrap(),
                    split[1].parse::<usize>().unwrap(),
                )
            })
            // .inspect(|x| println!("{:?}", x))
            .fold(vec![vec![]; V + 1], |mut p, (v, w)| {
                // blunder: directed
                p[v].push(w);
                p
            });

        let r = input
            .by_ref()
            .take(R)
            .map(|l| l.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        fn dfs(al: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>) -> () {
            seen[v] = true; // mark
            for &w in &al[v] {
                // if not visited, recurse
                if !seen[w] {
                    dfs(al, w, seen);
                }
            }
        }

        let mut seen = vec![false; V + 1];
        for v in r {
            dfs(&al, v, &mut seen);
        }

        let knocked_over = seen.iter().fold(0, |p, &v| if v { p + 1 } else { p });
        output.push(knocked_over);
    }
    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = kattis_dominoes2(&input);
    for o in output {
        println!("{o}");
    }
}

#[cfg(test)]
mod tests_traverals {
    use super::*;

    #[test]
    fn test_wheresmyinternet() {
        let input = "6 4
1 2
2 3
3 4
5 6
";
        let output = kattis_wheresmyinternet(input);
        assert_eq!(output, vec![5, 6]);

        let input = "2 1
2 1
";
        let output = kattis_wheresmyinternet(input);
        assert_eq!(output, vec![]);

        let input = "4 3
2 3
4 2
3 4
";
        let output = kattis_wheresmyinternet(input);
        assert_eq!(output, Vec::from([2, 3, 4]));

        let input = "4 3
2 3
4 2
3 4
";
        let output = kattis_wheresmyinternet(input);
        assert_eq!(output, Vec::from([2, 3, 4]));
    }

    #[test]
    fn test_dominoes2() {
        let input = "1
3 2 1
1 2
2 3
2
";
        let output = kattis_dominoes2(input);
        assert_eq!(output, vec![2]);

        let input = "2
3 2 1
1 2
2 3
2
3 2 1
1 2
2 3
2
";
        let output = kattis_dominoes2(input);
        assert_eq!(output, vec![2, 2]);

        let input = "1
3 2 2
1 2
2 3
2
3
";
        let output = kattis_dominoes2(input);
        assert_eq!(output, vec![2]);

        let input = "1
3 1 1
1 2
1
";
        let output = kattis_dominoes2(input);
        assert_eq!(output, vec![2]);

        let input = "1
3 1 0
1 2
";
        let output = kattis_dominoes2(input);
        assert_eq!(output, vec![0]);
    }
}

// fn kattis_reachableroads(input: &str) -> () {}
// fn kattis_terraces(input: &str) -> () {}
// fn kattis_cartrouble(input: &str) -> () {}
// fn kattis_daceydice(input: &str) -> () {}
// fn kattis_foldingcube(input: &str) -> () {}
// fn kattis_moneymatters(input: &str) -> () {}
// fn kattis_pearwise(input: &str) -> () {}
// fn kattis_securitybadge(input: &str) -> () {}
// fn cses_buildingroads(input: &str) -> () {}

// - toposort
// - bipartite cycle check
// - ad hoc graph traversals

mod tests_mst {}
mod tests_sssp {}
mod tests_apsp {}

// --adv
// network flows
// graph matching
// np-hard/np-complete
