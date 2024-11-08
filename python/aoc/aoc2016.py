import unittest
import enum
import functools


class Dir(enum.Enum):
    North = 'N'
    East = 'E'
    South = 'S'
    West = 'W'


class Turn(enum.Enum):
    Left = 'L'
    Right = 'R'


type Magnitude = int

type Cmd = tuple[Turn, Magnitude]


def one(input: str) -> int:
    def parse(cmd: str) -> Cmd:
        dir, mag = cmd[0], cmd[1:]
        if dir == 'L':
            return (Turn.Left, int(mag))
        elif dir == 'R':
            return (Turn.Right, int(mag))
        else:
            raise ValueError(f"picoprobpy: invalid direction {dir}")

    def solve(acc: tuple[int, int, Dir], next: Cmd) -> tuple[int, int, Dir]:
        dist_ns, dist_ew, facing_dir = acc
        turning_dir, mag = next

        # 1. turn (update facing dir)
        match (facing_dir, turning_dir):
            case (Dir.North, Turn.Left):
                return (dist_ns, dist_ew-mag, Dir.West)
            case (Dir.North, Turn.Right):
                return (dist_ns, dist_ew+mag, Dir.East)
            case (Dir.East, Turn.Left):
                return (dist_ns+mag, dist_ew, Dir.North)
            case (Dir.East, Turn.Right):
                return (dist_ns-mag, dist_ew, Dir.South)
            case (Dir.South, Turn.Left):
                return (dist_ns, dist_ew+mag, Dir.East)
            case (Dir.South, Turn.Right):
                return (dist_ns, dist_ew-mag, Dir.West)
            case (Dir.West, Turn.Left):
                return (dist_ns-mag, dist_ew, Dir.South)
            case (Dir.West, Turn.Right):
                return (dist_ns+mag, dist_ew, Dir.North)

    # List[c1, c2, .... cn-1, cn] -> List[Cmd]
    # List[Cmd] -> int
    output = functools.reduce(
        solve, map(parse, list(input.split(', '))), (0, 0, Dir.North))

    return abs(output[0]) + abs(output[1])


class TestAOC2016(unittest.TestCase):
    def test(self):
        self.assertEqual(one("R2, L3"), 5)
        self.assertEqual(one("R2, R2, R2"), 2)
        self.assertEqual(one("R5, L5, R5, R3"), 12)


if __name__ == '__main__':
    input = open("../../rust/src/aoc/data/2016_1")
    output = one(input.read())
    print(output)
    unittest.main()
