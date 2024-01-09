import re

test_input = """Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"""

def get_cube_data(data_string: str):
    data = {}
    # Split sets
    set_split = data_string.split(';')
    # print(set_split)
    for index, item in enumerate(set_split):
        curr_set = {}
        # Split colors
        color_splits = item.split(',')
        # Split on numbers
        for color in color_splits:
            # print(color)
            cube_split = color.strip().split(' ')
            # print(cube_split)
            curr_set[cube_split[1]] = cube_split[0]
        data[index] = curr_set

    return data

if __name__ == "__main__":
    with open("input.txt") as file:
        for index, line in enumerate(test_input.split('\n')):
        # for line in file:
            # print(line)
            game_split = line.split(':')
            # game_num = get_game_num(game_split[0])
            game_data = get_cube_data(game_split[1])
            print("Game Number: ", index + 1)
            print("Game Data: ", game_data)

