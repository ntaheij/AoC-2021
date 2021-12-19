import sys
import itertools

test = """\
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""

DEBUG = 'debug' in sys.argv

if 'test' in sys.argv:
    data = test.splitlines()
else:
    data = open('input.txt').readlines()

# Convert a string to our mixed list structure.

def convert(ln):
    return [int(s) if s.isdigit() else s for s in ln.rstrip()]

# Convert back to a string for printing.

def out(lst):
    return ''.join(str(t) for t in lst)

# Add two structures.

def add( a, b ):
    if not a: return b
    return ['['] + a + [','] + b + [']']

# Is this entry a number?

def isdigit(k):
    return isinstance(k,int)

# Explode the pair starting at n.

def explode( a, n ):
    if DEBUG:
        print("explode at", n, a[n:n+5])
    # Explode the pair starting at n.
    left = a[n+1]
    right = a[n+3]
    # Search backwards for a digit.
    for nn in range(n-1, -1, -1):
        if isdigit(a[nn]):
            if DEBUG:
                print("l adding",left,"to",a[nn])
            a[nn] += left
            break
    # Search forwards for a digit.
    for nn in range(n+5, len(a)):
        if isdigit(a[nn]):
            if DEBUG:
                print("r adding",right,"to",a[nn])
            a[nn] += right
            break
    # Remove the pair.
    a = a[:n] + [0] + a[n+5:]
    return a

def testexplode():
    testdata = [
        ("[[[[[9,8],1],2],3],4]", 4),
        ("[7,[6,[5,[4,[3,2]]]]]", 12),
        ("[[6,[5,[4,[3,2]]]],1]", 10),
        ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", 10),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", 24)
    ]

    for a,b in testdata:
        t = convert(a)
        print(a, "==>", out(explode(t,b)))

# Split the number at n.

def split( a, n ):
    val = a[n]
    if DEBUG:
        print("splitting", val, "at", n )
    return a[:n] + ["[", (val)//2, ",", (val+1)//2, "]"] + a[n+1:]

# Take all actions.  Not sure it's documented, but we must do all
# explodes before we do a split.

def actions( a ):
    changed = True
    while changed:
        changed = False

        # Find an item of depth 4.

        depth = 0
        for i,c in enumerate(a):
            if c == ']':
                depth -= 1
            elif c == '[':
                depth += 1
                if depth == 5:
                    a = explode( a, i )
                    changed = True
                    break
        if changed:
            continue

        # Find a number bigger than 9.

        for i,c in enumerate(a):
            if isdigit(c) and c >= 10:
                a = split( a, i )
                changed = True
                break
    return a

def testactions():
    s1 = convert("[[[[4,3],4],4],[7,[[8,4],9]]]")
    s2 = convert("[1,1]")
    s3 = add(s1,s2)
    print(out(s3))
    s3 = actions( s3 )
    print(out(s3))

# Return the magnitude of a structure.

def magnitude(a):
    while len(a) > 1:
        for i in range(len(a)):
            if isdigit(a[i]) and isdigit(a[i+2]):
                a = a[:i-1] + [a[i]*3 + a[i+2]*2] + a[i+4:]
                break
    return a[0]

def testmagnitude():
    testmag = [
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ]
    for a,b in testmag:
        chk = magnitude(convert(a))
        print(a, chk, b )
        assert chk == b

def testing():
    testexplode()
    testactions()
    testmagnitude()

# Sum all of the shellfish and print the magnitude.

def part1(data):
    base = []
    for expr in data:
        base = add( base, expr )
        base = actions( base )
    return magnitude(base)

# Find the best combo.
# I think we have to do this exhaustively.

def part2(data):
    maxval = 0
    for a in data:
        for b in data:
            if a == b:
                continue
            val = magnitude( actions( add( a, b ) ) )
            if val > maxval:
                print(val)
                maxval = val
    return maxval

data = [convert(d) for d in data]
print( "Part 1:", part1(data) )
print( "Part 2:", part2(data) )


