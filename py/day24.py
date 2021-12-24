#!/usr/bin/python3
import sys
import heapq
import itertools
import re
import ast
import numpy as np
import math
import functools
from collections import defaultdict, Counter, deque

inf = sys.argv[1] if len(sys.argv) > 1 else '24.in'
ll = [x for x in open(inf).read().strip().split('\n')]

AX = []
DZ = []
AY = []
for lineno, line in enumerate(ll):
	if "add x " in line and "add x z" not in line:
		AX.append(int(line.split()[2]))
	if "div z " in line:
		DZ.append(int(line.split()[2]))
	if "add y " in line and lineno%18 == 15:
		AY.append(int(line.split()[2]))
print("Extracted from input", AX, DZ, AY)

if len(AX) != 14 or len(DZ) != 14 or len(AY) != 14:
	raise Exception("couldn't understand your input")

def run(ch, z, w):
	x = AX[ch] + (z % 26)
	z = z // DZ[ch]
	if x != w:
		z *= 26
		z += w + AY[ch]
	return z

Zbudget = [26**len([x for x in range(len(DZ)) if DZ[x]==26 and x >= i]) for i in range(len(DZ))]
print("Threshold for giving up due to Z being too high, at each stage has been calculated as", Zbudget)
CANDIDATES = list(range(1, 10))
@functools.lru_cache(maxsize=None)
def search(ch, zsofar):
	if ch == 14:
		if zsofar == 0:
			return [""]
		return []
	if zsofar > Zbudget[ch]:
		return []
	xwillbe = AX[ch] + zsofar % 26
	wopts = CANDIDATES
	if xwillbe in range(1, 10):
		wopts = [xwillbe]
	ret = []
	for w in wopts:
		znext = run(ch, zsofar, w)
		nxt = search(ch + 1, znext)
		for x in nxt:
			ret.append(str(w) + x)
	return ret

solns = search(0, 0)
solns = [int(x) for x in solns]
print("num solutions", len(solns))
print(max(solns), min(solns))