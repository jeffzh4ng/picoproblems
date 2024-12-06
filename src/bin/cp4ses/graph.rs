// 1. traverals
// - connected components, strongly connected components
// - articulation points, bridges

use std::io;

// rust: .take()/.skip() for filter, .nth() for selection
fn kattis_wheresmyinternet(input: &str) -> Option<Vec<usize>> {
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

    fn dfs(al: &Vec<Vec<usize>>, v: usize, seen: &mut [bool], path: &mut Vec<usize>) -> () {
        let ((), ()) = (path.push(v), seen[v] = true); // visit and mark
        for &w in &al[v] {
            if !seen[w] {
                dfs(al, w, seen, path); // recurse
            }
        }
    }

    let (ccs, _) = (1..V).fold(
        (Vec::new(), vec![false; V + 1]),
        |(mut ccs, mut seen), v| {
            if !seen[v] {
                let mut cc = Vec::new();
                dfs(&al, v, &mut seen, &mut cc);
                ccs.push(cc); // update
            };

            (ccs, seen)
        },
    );

    if ccs.len() == 1 {
        None
    } else {
        let ccs_flattened = ccs
            .iter()
            .skip(1)
            .flat_map(|cc| cc.iter().copied())
            .collect::<Vec<_>>();

        Some(ccs_flattened)
    }
}

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = kattis_wheresmyinternet(&input);
    match output {
        Some(cc_flattened) => cc_flattened.iter().for_each(|v| println!("{v}")),
        None => println!("Connected"),
    }
}

// fn kattis_dominoes2(input: &str) -> () {}
// fn kattis_reachableroads(input: &str) -> () {}
// fn kattis_terraces(input: &str) -> () {}
// fn kattis_cartrouble(input: &str) -> () {}
// fn kattis_daceydice(input: &str) -> () {}
// fn kattis_foldingcube(input: &str) -> () {}
// fn kattis_moneymatters(input: &str) -> () {}
// fn kattis_pearwise(input: &str) -> () {}
// fn kattis_securitybadge(input: &str) -> () {}
// fn cses_buildingroads(input: &str) -> () {}

#[cfg(test)]
mod tests_traverals {
    use super::*;

    #[test]
    fn test() {
        let input = "6 4
1 2
2 3
3 4
5 6
";
        let output = kattis_wheresmyinternet(input).unwrap();
        assert_eq!(output, vec![5, 6]);

        let input = "2 1
2 1
";
        let output = kattis_wheresmyinternet(input);
        assert_eq!(output, None);

        let input = "4 3
2 3
4 2
3 4
";
        let output = kattis_wheresmyinternet(input).unwrap();
        assert_eq!(output, Vec::from([2, 3, 4]));
    }
}

// - flood fill easy
// - flood fill hard
// - toposort
// - bipartite cycle check
// - articular point/bridge
// - strongly connected components
// - ad hoc graph traversals

// 2. mst
// 3. sssp
// 4. apsp
// 5. network flows
// 6. graph matching
// 7. np-hard/np-complete
