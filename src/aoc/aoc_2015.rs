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

pub fn one(input: &str) -> Result<i32, io::Error> {
    // parse: &str -> Vec<Token>
    // count: Vec<Token> -> i32
    let output = input
        .chars()
        .map(|c| Token::try_from(c))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|foo| io::Error::new(io::ErrorKind::InvalidData, "picoprob: invalid data"))?
        .iter()
        .fold(0, |acc, next| match next {
            Token::Open => acc + 1,
            Token::Close => acc - 1,
        });

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let inps = vec!["(())", "()()"];
        inps.iter().for_each(|&inp| {
            let out = one(inp).unwrap();
            assert_eq!(out, 0);
        });
    }

    #[test]
    fn test_two() {
        let inps = vec!["(((", "(()(()(", "))((((("];
        inps.iter().for_each(|&inp| {
            let out = one(inp).unwrap();
            assert_eq!(out, 3);
        });
    }

    #[test]
    fn test_three() {
        let inps = vec!["())", "))("];
        inps.iter().for_each(|&inp| {
            let out = one(inp).unwrap();
            assert_eq!(out, -1);
        });
    }

    #[test]
    fn test_four() {
        let inps = vec![")))", ")())())"];
        inps.iter().for_each(|&inp| {
            let out = one(inp).unwrap();
            assert_eq!(out, -3);
        });
    }
}
