use std::convert::TryFrom;
use std::io;

enum Token {
    Open,
    Close,
}

impl TryFrom<char> for Token {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '(' => Ok(Token::Open),
            ')' => Ok(Token::Close),
            _ => Err(()),
        }
    }
}

/// one takes in a list of up/down commands encoded as parenthesis
/// and produces (final floor * first floor to reach -1)
///
/// # Examples
/// ```
/// use picoprob::aoc::aoc_2015;
/// let input = ")))";
/// let output = aoc_2015::one(&input).unwrap();
/// assert_eq!(output, (-3, Some(1)));
/// ```
pub fn one(input: &str) -> Result<(i32, Option<usize>), io::Error> {
    // parse: &str -> Vec<Token>
    // count: Vec<Token> -> i32
    let output = input
        .chars()
        .map(|c| Token::try_from(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "picoprob: invalid data"))?
        .iter()
        .enumerate()
        .fold((0, None), |(floor, first_floor_neg), (i, cmd)| match cmd {
            Token::Open => (floor + 1, first_floor_neg),
            Token::Close => match first_floor_neg {
                Some(_) => (floor - 1, first_floor_neg),
                None => {
                    if floor == 0 {
                        (floor - 1, Some(i + 1)) // weird indexing
                    } else {
                        (floor - 1, None)
                    }
                }
            },
        });

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let inputs = vec!["(())", "()()"];
        inputs.iter().for_each(|&input| {
            let out = one(input).unwrap();
            assert_eq!(out.0, 0);
        });
    }

    #[test]
    fn test_two() {
        let inputs = vec!["(((", "(()(()(", "))((((("];
        inputs.iter().for_each(|&input| {
            let out = one(input).unwrap();
            assert_eq!(out.0, 3);
        });
    }

    #[test]
    fn test_three() {
        let inputs = vec!["())", "))("];
        inputs.iter().for_each(|&input| {
            let out = one(input).unwrap();
            assert_eq!(out.0, -1);
        });
    }

    #[test]
    fn test_four() {
        let inputs = vec![")))", ")())())"];
        inputs.iter().for_each(|&input| {
            let out = one(input).unwrap();
            assert_eq!(out.0, -3);
        });
    }

    #[test]
    fn test_five() {
        let input = ")";
        let output = one(input).unwrap();
        assert_eq!(output.1, Some(1))
    }

    #[test]
    fn test_six() {
        let input = "()())";
        let output = one(input).unwrap();
        assert_eq!(output.1, Some(5))
    }
}
