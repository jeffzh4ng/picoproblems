fn div3_984a(input: &str) -> Vec<String> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let n = input.by_ref().next().unwrap().parse::<usize>().unwrap();
        let A = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let mut perfect = true;
        for i in 1..n {
            if (A[i] - A[i - 1]).abs() == 5 || (A[i] - A[i - 1]).abs() == 7 {
                continue;
            } else {
                perfect = false;
            }
        }

        output.push(if perfect {
            "YES".to_owned()
        } else {
            "NO".to_owned()
        });
    }

    output
}
