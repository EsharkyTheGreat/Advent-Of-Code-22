with open("input.txt","r") as f:
    print(max([sum([int(y) for y in x.split("\n")]) for x in f.read().split("\n\n")]))