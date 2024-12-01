import sys, re, math

dirs, data = open(sys.argv[1]).read().strip().split("\n\n")
data = [re.findall("\\w+", line) for line in data.splitlines()]
G = {n: (dstL, dstR) for n, dstL, dstR in data}

def solve(node, dirs):
    steps = 0
    while node[-1] != "Z":
        dir = dirs[steps % len(dirs)]
        node = G[node][0 if dir == "L" else 1]
        steps += 1
    return steps

print(solve("AAA", dirs))
print(math.lcm(*[solve(n, dirs) for n in G.keys() if n[-1] == "A"]))