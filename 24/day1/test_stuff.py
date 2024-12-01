import pytest
from main import diffs, parse, num_occurrences


@pytest.fixture
def input():
    return [
        "3  4",
        "4  3",
        "2  5",
        "1  3",
        "3  9",
        "3  3"
    ]


@pytest.fixture
def expected_list1():
    return [1, 2, 3, 3, 3, 4]


@pytest.fixture
def expected_list2():
    return [3, 3, 3, 4, 5, 9]


def test_parsing(input, expected_list1, expected_list2):
    result = [[], []]

    for line in input:
        lh, rh = parse(line)
        result[0].append(int(lh))
        result[1].append(int(rh))

    result[0].sort()
    result[1].sort()

    assert result == [expected_list1, expected_list2]


def test_diffs(expected_list1, expected_list2):
    expected_diffs = [2, 1, 0, 1, 2, 5]

    assert expected_diffs == diffs(expected_list1, expected_list2)


@pytest.mark.parametrize("num, expected", [
    (1, 0),
    (2, 0),
    (3, 3),
    (4, 1),
])
def test_occurrences(num, expected, expected_list2):
    assert expected == num_occurrences(num, expected_list2)
