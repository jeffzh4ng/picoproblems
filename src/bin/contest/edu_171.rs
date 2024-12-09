// 7:45 -> start
// 7:50 -> understand problem
//         - two segments : AB >= K, CD >=K, AB | CD
//         - constraint: answer always exists
//         - complete search: search space
//         - 1 <= X,Y <= 1000. can probably perform complete search.

//         N, M, K
//         s = gen_segments: (gen all possible tuple points within NM cartesian plane)
//
//         s_pairs = s.flat_map(s.iter().map()...)
//         s_pairs.filter(|(ab, cd)| ab >= K && cd >= K && AB | CD)

// 8:00 -> idea 2
//         what if we look at calculating the property?
//         - is there any way we can decompose NxM problem -> (N-1)x(M-1)?
//         - what if i knew AB` CD` for (N-1)x(M-1)
//         - couldn't i just extend b.x += 1, b.y += m*1, d.x += 1, d.y += m*1

// 8:05 -> formalizing intuition. recurrence relation? trying to prove dependencies
//         - since we know for (N,M) there exists some (AB,CD) : (AB >= K, CD >=K, AB | CD)
//                then for (N-1,M-1) there exists some (AB`, CD`) : (AB` >= K, CD` >=K, AB` | CD`)

//       recurrence relations: topological order of dependencies (squares vs rectangles?)
//       1 <= N, M <= 1000
//       how is 1 <= K <= 1414? shouldn't K <= 1000? ah. because sqrt(1000^2 + 1000^2)=1414.

//       base case (N=1, M=1) ==> K=1
//       AB: (0,0)->(1,0), CD: (0,0)->(0,1)

//       (N=2, M=1) ==> K=1

//       8:21 (40minutes)
//       insight (dp on the property)
//                examples were red herring?
//                what matters is property: perpendicular
//                we can compress the entire search space into x-axis and y-axis.
//                can our subproblems just be the x-axis and the y-axis??
//                -> this would remove is_perpendicular check too, because
//                   all subproblems would be perpendicular by enforcing x-axis,y-axis invariant.
//                -> this feels right intuitively.
//                -> how could i prove this??

//       8:26 (trying to implement recursive/dp, combining subproblems)
//       -> can i just return the x-axis and y-axis itself??
//       -> since we're looking for two line segments that's *at least* K
//       -> russian understanding

//       8:40 blunder. when K >= |max(N,M)|
//                     then, can i just return a big cross?
//                     nope. cause then AB|CD does not always hold (very slanted X)

// 8:44 see problem tag is greedy/constructive
//      almost got the idea.
//      subproblem, maximize??
//      where do you start to ensure AB|CD?
//      if dp, 1. what is the base case here? 2. how do you combine subproblems (square/rectangles)

// fn foo(
//     N: usize,
//     M: usize,
//     K: usize, // assuming **K <= sqrt(N^2+M^2)** (K fits in the N-M cartesian plane)
// ) -> (
//     ((usize, usize), (usize, usize)), // AB
//     ((usize, usize), (usize, usize)), // CD
// ) {
//     if N == 1 && M == 1 {
//         (((0, 0), (1, 0)), ((0, 0), (0, 1)))
//     } else {
//     }
// }

fn a(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let toks = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .collect::<Vec<_>>();

        let (N, M, K) = (
            toks[0].parse::<usize>().unwrap(),
            toks[1].parse::<usize>().unwrap(),
            toks[2].parse::<usize>().unwrap(),
        );

        let min = min(N, M); // this was the single line i missed. why?
        output.push(format!("0 0 {min} {min}"));
        output.push(format!("0 {min} {min} 0"));
        // output.push(format!("0 {N} {M} 0"));
        // output.push(format!("0 0 {N} {M}"));
    }

    output
}

use std::{cmp::min, io};
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = a(&input);
    for o in output {
        println!("{o}");
    }
}
