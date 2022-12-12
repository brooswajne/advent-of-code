import os

input_file_path = os.path.join(
    os.path.dirname(os.path.realpath(__file__)),
    "input",
)

characters_seen = {}
length_of_run = 0
largest_run = 0
with open(input_file_path, "r") as input_file:
    buffer = next(input_file)
    for index, char in enumerate(buffer):
        last_time_seen = characters_seen.get(char, -1)
        length_of_run = min(length_of_run + 1, index - last_time_seen)
        characters_seen[char] = index
        if length_of_run > largest_run:
            print(f"{length_of_run}: {index + 1}")
            largest_run = length_of_run
