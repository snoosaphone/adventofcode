def parse(line):
    return line.split('  ')


def diffs(list1, list2):
    return [abs(a - b) for a, b in zip(list1, list2)]


def part1(lh_list, rh_list):
    lh_list.sort()
    rh_list.sort()

    diff_list = diffs(lh_list, rh_list)

    print(sum(diff_list))


def num_occurrences(num, list):
    return len([ii for ii, x in enumerate(list) if x == num])


def sim(lh_list, rh_list):
    sum = 0

    for x in lh_list:
        sum += x * num_occurrences(x, rh_list)

    return sum


def part2(lh_list, rh_list):
    score = sim(lh_list, rh_list)
    print(score)
    pass


def parse_file(file_handle):
    lh_list = []
    rh_list = []

    for line in file_handle.readlines():
        lh, rh = parse(line.strip())
        lh_list.append(int(lh))
        rh_list.append(int(rh))

    return lh_list, rh_list


if __name__ == '__main__':
    with open('./input.txt') as file_handle:
        lh_list, rh_list = parse_file(file_handle)

        part1(lh_list, rh_list)
        part2(lh_list, rh_list)
