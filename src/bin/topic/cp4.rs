use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// EXHAUSTIVE SEARCH
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

mod test_precalc {}
// kattis_cardtrick2
// kattis_foolingaround
// kattis_sgcoin

mod test_mathsim {}
// kattis_growlinggears
// kattis_trollhunt
// kattis_videospeedup
// kattis_crackingrsa
// kattis_falling
// kattis_thanosthehero

mod test_iterative {}
// double loop
// ------------------
// kattis_pet
// kattis_blackfriday
// kattis_closestsums
// kattis_golumbrulers

// triple loop
// ------------------
// kattis_cudoviste
// kattis_npuzzle
// kattis_set
// kattis_mathhomework
// kattis_patuljci
// kattis_safehouses
// kattis_calculatingdartscores
// kattis_lektira
// kattis_tautology

#[cfg(test)]
mod test_permutation_combinations {
    fn test_kattis_dancerecital() {}
    fn test_kattis_dreamer() {}
    fn test_kattis_veci() {}
    fn kattis_geppetto() {}
    fn kattis_squaredeal() {}
    fn kattis_zagrade() {}
}

// fn kattis_dancerecital(input: &str) -> u32 {
//     let N = input.lines().next().unwrap().parse::<usize>().unwrap();
//     let n = input.lines().take(N).collect::<Vec<_>>();

//     fn gen_perms(input: &[&str]) -> Vec<Vec<String>> {
//         match input {
//             [] => vec![vec![]],
//             input => {
//                 let output = Vec::new();

//                 for (i, n) in input.iter().enumerate() {}

//                 let foo = gen_perms(r);

//                 output
//             }
//         }
//     }

//     fn calc_cost(input: &Vec<String>) -> u32 {
//         todo!()
//     }

//     let min_cost = gen_perms(&n) // pay O(N!)
//         .iter()
//         .map(|order| calc_cost(order)) // pay O(N)
//         .fold(u32::MIN, |p, n| if p < n { p } else { n }); // pay O(N)

//     min_cost
// }

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    // let output = kattis_dancerecital(&input);
    // println!("{output}");
}

mod test_josephus {}
// kattis_eenymeeny
// kattis_musicalchairs
// kattis_toys

mod test_backtracking {}
// kattis_goodmorning
// kattis_natjecanje
// kattis_paintings
// kattis_dobra
// kattis_fruitbaskets
// kattis_pagelayout

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// GREEDY
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
fn main() {}

mod test_classical {}
// kattis_classrooms
// kattis_froshweek2
// kattis_squarepegs
// kattis_avoidland
// kattis_color
// kattis_fishmongers
// kattis_grass
// kattis_inflation
// kattis_intervalcover
// kattis_loowater
// kattis_messages

mod test_nonclassical {}
// kattis_ants
// kattis_bank
// kattis_marblestree
// kattis_applesack
// kattis_driver
// kattis_haybales
// kattis_horrorfilmnight
// kattis_pripreme
// kattis_simplicity
// kattis_skocimis
// kattis_teacherevaluation
// kattis_duds
// kattis_stockbroker
// kattis_virus
// kattis_cardtraining
// kattis_logland
// kattis_playground
// kattis_wordspin

// see:
// - prims/kruskals (greedy on graphs for MST)
// - dijkstras (greedy on graphs for SSSP)

mod test_sorting {}
// kattis_icpcteamselection
// kattis_minimumscalar
// kattis_shopaholic
// kattis_acm2
// kattis_aprizenoonecanwin
// kattis_akcija
// kattis_fallingapart
// kattis_fridge
// kattis_gettowork
// kattis_pikemaneasy
// kattis_planetaris
// kattis_plantingtrees
// kattis_redistribution
// kattis_standings
// kattis_textmessaing
// kattis_woodcutting
// kattis_airconditioned
// kattis_birds
// kattis_delivery
// kattis_andrewant
// kattis_ceremony
// kattis_dasort
// kattis_fairdivison
// kattis_help
// kattis_intergalacticbidding
// kattis_trip2007
// kattis_wffnproof

mod test_priorityqueue {}
// kattis_ballotboxes
// kattis_canvas
// kattis_vegetables
// kattis_workstations
// kattis_convoy
// kattis_entertainmentbox
// kattis_simplification

fn main() {}

mod test_rangesum {}
// kattis_commercials
// kattis_prozor
// kattis_sellingspatulas
// kattis_alicedigital
// kattis_foldedmap
// kattis_purplerain
// kattis_shortsell

mod test_lis {}
// kattis_increasingsubsequence
// kattis_nesteddolls
// kattis_trainsorting
// kattis_alphabet
// kattis_longincsubseq
// kattis_commercials
// kattis_manhattanmornings
// kattis_studentsko

mod test_01knapsack {}
// kattis_knapsack
// kattis_orders
// kattis_presidentialelections
// kattis_muzicari
// kattis_ninepacks

mod test_cc {}
// kattis_bagoftiles
// kattis_canonical
// kattis_exactchange

mod test_tsp {}
// kattis_beepers
// kattis_bustour
// kattis_cycleeasy
// kattis_errands
// kattis_maximizingyourpay
// kattis_pokemongogo
// kattis_race

mod test_dp1 {}
// kattis_nikola
// kattis_spiderman
// kattis_ticketpricing
// kattis_keyboardconcert
// kattis_permutatindescent
// kattis_weightofwords
// kattis_worldclouds

mod test_dp2 {}
// kattis_kutevi
// kattis_tight
// kattis_walrusweights
// kattis_debugging
// kattis_driving
// kattis_watersheds

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// EFFICIENT SEARCH (DP)
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {}

mod test_rangesum {}
// kattis_commercials
// kattis_prozor
// kattis_sellingspatulas
// kattis_alicedigital
// kattis_foldedmap
// kattis_purplerain
// kattis_shortsell

mod test_lis {}
// kattis_increasingsubsequence
// kattis_nesteddolls
// kattis_trainsorting
// kattis_alphabet
// kattis_longincsubseq
// kattis_commercials
// kattis_manhattanmornings
// kattis_studentsko

mod test_01knapsack {}
// kattis_knapsack
// kattis_orders
// kattis_presidentialelections
// kattis_muzicari
// kattis_ninepacks

mod test_cc {}
// kattis_bagoftiles
// kattis_canonical
// kattis_exactchange

mod test_tsp {}
// kattis_beepers
// kattis_bustour
// kattis_cycleeasy
// kattis_errands
// kattis_maximizingyourpay
// kattis_pokemongogo
// kattis_race

mod test_dp1 {}
// kattis_nikola
// kattis_spiderman
// kattis_ticketpricing
// kattis_keyboardconcert
// kattis_permutatindescent
// kattis_weightofwords
// kattis_worldclouds

mod test_dp2 {}
// kattis_kutevi
// kattis_tight
// kattis_walrusweights
// kattis_debugging
// kattis_driving
// kattis_watersheds

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// DIVIDE AND CONQUER
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() {}

mod test_binarysearch {}
// kattis_firefly
// kattis_outofsorts
// kattis_roompainting
// kattis_synchronizinglists

mod test_ternarysearch {}
// kattis_a1paper
// kattis_ceiling
// kattis_goingtoseed
// kattis_cantor
// kattis_euclideantsp
// kattis_jewelrybox
// kattis_qanat
// kattis_reconnaissance
// kattis_sretan
// kattis_sylvester
// kattis_tricktreat
// kattis_zipline

mod test_bisection {}
// kattis_carefulascent
// kattis_freeweights
// kattis_monk
// kattis_suspensionbridges
// kattis_expeditiouscubing
// kattis_financialplanning
// kattis_hindex
// kattis_htoo
// kattis_rainfall2
// kattis_slalom2
// kattis_smallschedule
// kattis_speed
// kattis_svada
// kattis_taxing

// 1. traverals:
// - cc/scc/ff
// - aps/bridges

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
/// GRAPHS
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

    #[test]
    fn test_reachableroads() {
        let input = "2
5
3
0 1
1 2
3 4
2
1
0 1
";
        let output = kattis_reachableroads(input);
        assert_eq!(output, vec![1, 0]);
    }
}

fn kattis_reachableroads(input: &str) -> Vec<u32> {
    let n = input.lines().nth(0).unwrap().parse::<u32>().unwrap();

    let mut input = input.lines().skip(1);
    let mut output = Vec::new();
    for _ in 0..n {
        let (V, E) = (
            input.by_ref().next().unwrap().parse::<usize>().unwrap(),
            input.by_ref().next().unwrap().parse::<usize>().unwrap(),
        );

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
            .fold(vec![vec![]; V], |mut al, (v, w)| {
                al[v].push(w);
                al[w].push(v);
                al
            });

        fn dfs(al: &Vec<Vec<usize>>, v: usize, seen: &mut Vec<bool>) -> () {
            seen[v] = true; // mark
            for &w in &al[v] {
                if !seen[w] {
                    // recurse on neighbors
                    dfs(al, w, seen)
                }
            }
        }

        let mut seen = vec![false; V];
        let ccs = (0..V).fold(0, |p, v| {
            if !seen[v] {
                dfs(&al, v, &mut seen); // visit connected component
                p + 1
            } else {
                p
            }
        });

        output.push(ccs - 1)
    }

    output
}

// use std::io;
// fn main() {
//     let input = io::read_to_string(io::stdin()).unwrap();
//     let output = kattis_reachableroads(&input);
//     for o in output {
//         println!("{o}");
//     }
// }

// fn kattis_terraces(input: &str) -> () {}

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

fn main() {}
