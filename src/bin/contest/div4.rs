use std::collections::HashMap;

fn div4_849d(input: &str) -> Vec<usize> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let n = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let s = input.by_ref().next().unwrap();
        let mut ccr = s.chars().fold(HashMap::new(), |mut p, n| {
            p.entry(n).and_modify(|e| *e += 1).or_insert(1);
            p
        });

        let mut ccl = HashMap::new();
        let mut max = 0;
        for c in s.chars() {
            ccl.entry(c).and_modify(|e| *e += 1).or_insert(1);
            ccr.entry(c).and_modify(|e| *e -= 1);
            if *ccr.get(&c).unwrap() == 0 {
                ccr.remove(&c);
            }
            let score = ccl.keys().len() + ccr.keys().len();
            max = max.max(score);
        }
        output.push(max)
    }

    output
}
