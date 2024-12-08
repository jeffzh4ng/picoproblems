use std::collections::{HashMap, HashSet};

fn main() {}

//1:45 -> understand problem (spend the least to save the most)
//1:42 -> idea1: from greatest to least, add coins?
//1:35 -> grokking the constraints with the examples
//1:30 -> looked at first line editorial
//1:18 -> blunder: submit with debug statements
//1:17 -> solve a

// 5 4
// 4 1 2 3 2
// -> 0

// 5 10
// 4 1 2 3 2

// M:4,3,2 = (9), 2(11)
//

// -> 1
// -the reason why we don't add 6 to 4
// -is because we can just add 1 to 3  M takes 4,4,2
// - why don't we add 1 to 1? b/c then M takes 4,3,2,
// crux:
// 1. what's the order of traversal?? most -> least??
// sum=10
// for n in input.sort():
//   sum -= (n+1)

// -> obs: 1. we want to spend the least
//         2. and give *exactly* k editorial: consider the list of numbers right before overflow

// 2 10
// 1 1
// -> 8

// if input.sum() < k
//   return k-input.sum() // 10-2 = 8

// 3 8
// 3 3 3
// -> 2

fn a(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();

    let mut input = input.lines().skip(1);
    let mut output = Vec::new();
    for _ in 0..t {
        let foo = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();
        let (n, k) = (
            foo[0].parse::<i32>().unwrap(),
            foo[1].parse::<i32>().unwrap(),
        );

        let mut coins = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        coins.sort_by(|a, b| b.cmp(a));

        println!("{:?}", coins);
        let mut sum = 0;
        for c in coins {
            if sum + c <= k {
                sum += c;
            } else {
                break; // blunder: did not break
            }
        }

        output.push(k - sum)
    }

    output
}

// 1:16 -> understand the problem
// 1:14 -> have an idea of data (color counts)
// 1:07 -> if problem was just A, and B's moves were supplied,
//         i would have the idea of odd counts 1,3,5...
//      -> but i'm not sure how B should select. nvm...making progress (bfs?)
// 1:05 -> have an idea. going to start implementation.
// 0:47 -> brain starting to shut down when implementing b's choices...
// 0:39 -> brain is tapping out.

// need color count
// but how does alice optimally maximize? (that's the problem)

// idea 1:
// A chooses any color with odd count, starting from 1,3,5...
// so A can get bonus

// idea 2:
// A focusing on one color at a time
// prob: B can steal when there's 1 left
// would he though? his goal is to minimize his own score.
// does stealing from A help his goal?

// how does B select?
// B wants to avoid bonuses
// -> bfs (pick from each color) or picks color with largest count.
// -> latter seems easier to implement

fn b(input: &str) -> Vec<i32> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();

    let mut input = input.lines().skip(1);
    let mut output = Vec::new();
    for _ in 0..t {
        let n = input.by_ref().next().unwrap().parse::<i32>();
        let mut marbles = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .fold(HashMap::new(), |mut acc, n| {
                acc.entry(n).and_modify(|f| *f += 1).or_insert(1);
                acc
            });

        let (mut a_score, mut a_colors) = (0, HashSet::new());
        let (mut b_score, mut b_colors) = (0, HashSet::new());
        while marbles.values().any(|&count| count > 0) {
            // alice: colors with counts: 1,3,5..

            // want: filter keys with a predicate based on value
            // let foo = marbles.values().filter(|&&count| count % 2 == 0);
            let mut a_moves = marbles
                .iter()
                .filter(|(_k, &v)| v % 2 == 1) // A's optimal strat: go for odds
                .collect::<Vec<_>>();
            a_moves.sort_by(|(a, a_count), (b, b_count)| a_count.cmp(b_count));
            if a_moves.len() == 0 {
                // select even
            }

            // make the move
            let (&c, &cc) = a_moves[0];
            // marbles.entry(c).and_modify(|cc| *cc -= 1);
            a_colors.insert(c);
            a_score += 1;
            if cc == 1 && !b_colors.contains(&c) {
                a_score += 1; // bonus
            }

            let mut b_moves = marbles.iter().collect::<Vec<_>>();
            b_moves.sort_by(|(a, a_count), (b, b_count)| b_count.cmp(a_count));
            let (&c, &cc) = a_moves[0];
            b_colors.insert(c);
            b_score += 1;
        }

        output.push(a_score)
    }

    output
}

// 6:29-> start
// blunder:
// - 0/1 denotation vs point assigning (ith group->i-1 points)
// - blunder: score is simply the total value of all fishes that contestant caught

// 6:45-> understand problem. round based point assigning

// -> find a partiton of the input such that
//      each p_i is assigned s_i(p_i) (i-1 points for each 0/1)
//

// 6:47-> stepping through examples.
//     -> when is a partition not possible?

// 4 1
// 1001     (100)        (1)
// -> 2    A:0,B:0    A:0,B:1

// 4 1
// 1010
// ->

// 6:50 -> idea1: bruteforce
//         how do i iterate through all possible contiguous partitions?
//         not (n indices choose k) because partition must be contiguous

//         -> actually, i can choose arbitary indices. no matter how i slice and dice,
//            as long as i < input.len(), it's going to form a valid set of partitions.

//         -> 6:58: is it (n choose k?) and if it is, i don't know how to implement
//         -> i need to strengthen my combinatorics/dp

//         -> a partition not being possible was a red herring.
//         -> it's a partition such that **B-A == k** that is sometimes not possible.
//         -> i can just walk through all possbile partitions
//         -> 7:00 possible sol identified. is it going to TLE?
//         -> not sure. i'll just try implementing it for the first time
//            and see what happens. i need topic practice on this.

//         1 partition  -> trivial
//         2 partition  -> choose p-1 i's (1 i) to split
//         3 partitions -> choose p-1 i's (2 i) to split
//         4 partitions -> choose p-1 i's (3 i) to split

// 6:54 -> decompose the problem into functions
//      -> identify crux function and it's topic
//      -> which is partition(input: &Vec) and combinatorics

// fn partition(input: &Vec)-> {}

// changing how to calculate score (partitions -> suffixes)
// gives intuition into how to solve the problem.
// double counting, realizing maximizing suffixes = maximizing partitions
// favor former over latter, more intuitive.

// -> suffix sum
// -> prefix sum (very simple dp) 1->+1 0->-1
//    (combinators: double counting)
//               idea -> look at the sum a different way
// https://en.wikipedia.org/wiki/Double_counting_(proof_technique)

// ->
// fn assign_score(input: &Vec<u8>, partition_index: usize) -> {}

fn c(input: &str) -> () {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "4
5 4
4 1 2 3 2
5 10
4 1 2 3 2
2 10
1 1
3 8
3 3 3
";
        let output = a(input);
        assert_eq!(output, vec![0, 1, 8, 2]);
    }

    #[test]
    fn test_b() {
        let input = "3
5
1 3 1 3 4
3
1 2 3
4
4 4 4 4
";
        let output = b(input);
        assert_eq!(output, vec![4, 4, 1]);
    }
}
