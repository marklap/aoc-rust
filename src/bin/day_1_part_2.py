import sys

from pathlib import Path

INPUT_FILE_PATH = Path(__file__).parent / 'day_1_input.txt'
WORD_MAP = (
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
)

def find_digit_l2r(map, line):
    for i in range(len(line)):
        if line[i].isdigit():
            return int(line[i])
        for (w, d) in map:
            if line[i:].startswith(w):
                return d
    return None

def find_digit_r2l(map, line):
    for i in reversed(range(len(line))):
        if line[i].isdigit():
            return int(line[i])
        for (w, d) in map:
            if line[:i+1].endswith(w):
                return d
    return None

def main(args=None):
    total = 0
    with open(INPUT_FILE_PATH) as fp:
        for line in fp:
            line = line.strip()
            d1 = find_digit_l2r(WORD_MAP, line) or 0
            d2 = find_digit_r2l(WORD_MAP, line) or 0
            val = int(f'{d1}{d2}')
            print(f'{line} -> {val}')
            total += val
    print(total)

if __name__ == '__main__':
    sys.exit(main())
