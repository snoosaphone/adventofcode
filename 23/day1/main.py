import re

test_input = """1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"""

test_input2 = """two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"""

written_out = [('one', 1), ('two', 2), ('three', 3), ('four', 4), ('five', 5), ('six', 6), ('seven', 7), ('eight', 8), ('nine', 9)]

if __name__ == "__main__":
    values = []

    with open("input.txt") as file:
        for line in file:
        # for line in test_input2.split('\n'):
            print(line)
            digits = []
            for index, char in enumerate(line):
                if char.isdigit():
                    digits.append(char)
                else:
                    for num in written_out:
                        if line[index:].startswith(num[0]):
                            digits.append(num[1])

            # digits = re.findall(r'\d', line.strip())
            print(digits)
            first = digits[0]
            last = digits[-1]
            final = str(first) + str(last)
            values.append(final)

        print(values)
        total = 0
        for val in values:
            total += int(val)
        print(total)
