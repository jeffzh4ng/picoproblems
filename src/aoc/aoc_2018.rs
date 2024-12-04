use std::collections::HashSet;

fn one(input: &str) -> (i32, i32) {
    let input = input
        .lines()
        .map(|l| {
            let (sign, dig) = (&l.chars().nth(0).unwrap(), &l[1..]);
            let n = dig.parse::<i32>().unwrap();

            match sign {
                '+' => n,
                '-' => -n,
                _ => panic!(),
            }
        })
        .collect::<Vec<_>>();

    let sum = input.iter().fold(0, |p, n| p + n);

    let mut seen = HashSet::new();
    let (mut freq, mut i) = (0, 0);

    // [1, -2, 3, 1]
    while !seen.contains(&freq) {
        seen.insert(freq); // check
        freq += input[i]; // insert
        i = if i + 1 == input.len() { 0 } else { i + 1 }; // step
    }

    (sum, freq)
}

#[cfg(test)]
mod tests_one {
    use std::fs;

    use super::*;

    #[test]
    fn part_one() {
        let input = "+1
-2
+3
+1
";
        let (output, _) = one(input);
        assert_eq!(output, 3);

        let input_two = "+1
+1
+1
";
        let (output_two, _) = one(input_two);
        assert_eq!(output_two, 3);

        let input_three = "+1
+1
-2
";
        let (output_three, _) = one(input_three);
        assert_eq!(output_three, 0);

        let input_four = "-1
-2
-3
";
        let (output_four, _) = one(input_four);
        assert_eq!(output_four, -6);

        let input_five = fs::read_to_string("./src/aoc/data/2018_1").unwrap();
        let (output_five, _) = one(&input_five);
        println!("{:?}", output_five);
    }

    #[test]
    fn part_two() {
        let input = "+1
-2
+3
+1
";
        let (_, output) = one(input);
        assert_eq!(output, 2);

        let input = "+1
-1
";
        let (_, output) = one(input);
        assert_eq!(output, 0);

        let input = "+3
+3
+4
-2
-4
";
        let (_, output) = one(input);
        assert_eq!(output, 10);

        let input = "-6
+3
+8
+5
-6
";
        let (_, output) = one(input);
        assert_eq!(output, 5);

        let input = "+7
+7
-2
-7
-4
";
        let (_, output) = one(input);
        assert_eq!(output, 14);

        let input = fs::read_to_string("./src/aoc/data/2018_1").unwrap();
        let (_, output) = one(&input);
        println!("{:?}", output);
    }
}
