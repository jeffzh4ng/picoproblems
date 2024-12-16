#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "6
5 5
.....
.....
..#..
.....
.....
5 5
..#..
.###.
#####
.###.
..#..
5 6
......
......
.#....
###...
.#....
1 1
#
5 6
...#..
..###.
.#####
..###.
...#..
2 10
..........
...#......
";
        let output = d(&input);
        for o in output {
            println!("{:?} {:?}", o.0, o.1);
        }
    }
}

fn d(input: &str) -> Vec<(usize, usize)> {
    let t = input.lines().nth(0).unwrap().parse::<u32>().unwrap();
    let mut input = input.lines().skip(1);
    let mut output = Vec::new();

    for _ in 0..t {
        let split = input
            .by_ref()
            .next()
            .unwrap()
            .split(' ')
            .map(|tok| tok.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let (n, m) = (split[0], split[1]);
        let mut grid = vec![vec![]; n];
        for r in 0..n {
            let row = input.by_ref().next().unwrap().chars().collect::<Vec<_>>();
            grid[r] = row;
        }

        let (mut top, mut bot) = ((0, 0), (0, 0));
        for (r, row) in grid.iter().enumerate() {
            let mut top_found = false;
            for (c, _) in row.iter().enumerate() {
                if grid[r][c] == '#' {
                    top = (r, c);
                    top_found = true;
                    break;
                }
            }
            if top_found {
                break;
            }
        }

        for (r, row) in grid.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                if grid[r][c] == '#' {
                    bot = (r, c);
                    break;
                }
            }
        }

        // println!("{:?}, {:?}", top, bot);
        output.push((top.0 + ((bot.0 - top.0) / 2) + 1, top.1 + 1))
    }

    output
}

use std::io;
fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let output = d(&input);
    for o in output {
        println!("{:?} {:?}", o.0, o.1);
    }
}
