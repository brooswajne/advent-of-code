import os

input_file_path = os.path.join(
    os.path.dirname(os.path.realpath(__file__)),
    "input",
)

stacks: list[list[str]] = []
CRATE_SIZE = 3
STACK_WIDTH = CRATE_SIZE + 1

with open(input_file_path, "r") as input_file:
    # parse drawing
    for line in input_file:
        if line.strip()[0] != "[": break
        print(line.strip("\n"))
        for offset in range(0, len(line), STACK_WIDTH):
            stack = int(offset / STACK_WIDTH)
            crate = line[offset : offset + CRATE_SIZE]
            contents = crate[1]
            if contents == " ": continue
            while len(stacks) <= stack: stacks.append([])
            stacks[stack].append(contents)

    for stack in stacks: stack.reverse()
    print("Initial:")
    for stack in stacks: print(stack)

    for line in input_file:
        if not line.startswith("move"): continue
        _move, amount, _from, source, _to, target = line.split(" ")
        amount = int(amount)
        source = stacks[int(source) - 1]
        target = stacks[int(target) - 1]
        crates = source[-amount:]
        del source[-amount:]
        target.extend(crates)
    
    print("Done:")
    for stack in stacks: print(stack)
    answer = "".join([ stack[-1] for stack in stacks ])
    print("Answer:", answer)
