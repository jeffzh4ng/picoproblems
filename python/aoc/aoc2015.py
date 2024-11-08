import unittest
import typing
import functools
import enum


class Token(enum.Enum):
    Open = '('
    Close = ')'


def one(input: str) -> tuple[int, typing.Optional[int]]:
    def parse_char(c: str) -> Token:
        if c == '(':
            return Token.Open
        elif c == ')':
            return Token.Close
        else:
            raise ValueError(f"picoprobpy: invalid char {c}")

    def solve(acc: tuple[int, typing.Optional[int]], next: tuple[int, Token]) -> tuple[int, typing.Optional[int]]:
        floor, first_floor_neg = acc
        i, token = next
        match token:
            case Token.Open:
                return (floor + 1, first_floor_neg)
            case Token.Close:
                if first_floor_neg == None and floor == 0:
                    return (floor - 1, i+1)
                else:
                    return (floor - 1, first_floor_neg)

    output = functools.reduce(
        solve,  # 2. solve: (int * int) * Token -> int
        enumerate(map(parse_char, list(input))),  # 1. parse_char: str -> Token
        (0, None),
    )

    return output


class TestAOC2015(unittest.TestCase):
    def test(self):
        self.assertEqual(one("(())")[0], 0)
        self.assertEqual(one("()()")[0], 0)
        self.assertEqual(one("(((")[0], 3)
        self.assertEqual(one("(()(()(")[0], 3)
        self.assertEqual(one("))(((((")[0], 3)
        self.assertEqual(one("())")[0], -1)
        self.assertEqual(one("))(")[0], -1)
        self.assertEqual(one(")))")[0], -3)
        self.assertEqual(one(")())())")[0], -3)
        self.assertEqual(one(")")[1], 1)
        self.assertEqual(one("()())")[1], 5)


if __name__ == '__main__':
    input = open("../../rust/src/aoc/data/2015_1")
    output = one(input.read())
    print(output)
    unittest.main()
