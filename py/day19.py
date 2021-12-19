#!/usr/bin/python3
import sys
import itertools
import re
import ast
from collections import defaultdict, Counter, deque

infile = sys.argv[1] if len(sys.argv)>1 else '19.in'
data = open(infile).read().strip()

scanners = data.split('\n\n')
B = []
for scan in scanners:
  beacons = []
  for line in scan.split('\n'):
    line = line.strip()
    if line.startswith('--'):
      continue
    x,y,z = [int(v) for v in line.split(',')]
    beacons.append((x,y,z))
  B.append(beacons)

def adjust(point, d):
  assert 0<=d<48

  ret = [point[0], point[1], point[2]]
  for i,p in enumerate(list(itertools.permutations([0, 1, 2]))):
    if d//8 == i:
      ret = [ret[p[0]], ret[p[1]], ret[p[2]]]
  
  if d%2==1:
    ret[0] *= -1
  if (d//2)%2==1:
    ret[1] *= -1
  if (d//4)%2==1:
    ret[2] *= -1
  return ret

N = len(B)
FINAL = set(B[0])
P = [None for _ in range(N)]
P[0] = (0,0,0)

GOOD = set([0])
BAD = set([x for x in range(1,N)])

B_ADJ = {}
for i in range(N):
  for d in range(48):
    B_ADJ[(i,d)] = [adjust(p, d) for p in B[i]]

while BAD:
  found = None
  for b in BAD:
    if found:
      continue
    for g in [0]:
      g_scan = [tuple([p[0]+P[g][0], p[1]+P[g][1], p[2]+P[g][2]]) for p in FINAL] #B[g]]
      g_set = set(g_scan)
      for b_dir in range(48):
        b_scan = B_ADJ[(b, b_dir)]
        VOTE = defaultdict(int)
        for bi in range(len(B[b])):
          for gi in range(len(g_scan)):
            dx = -b_scan[bi][0] + g_scan[gi][0]
            dy = -b_scan[bi][1] + g_scan[gi][1]
            dz = -b_scan[bi][2] + g_scan[gi][2]
            VOTE[(dx,dy,dz)] += 1
        for (dx,dy,dz), val in VOTE.items():
          if val >= 12:
              P[b] = (dx, dy, dz)
              #print(f'FOUND {b} via {g} at (x={dx} y={dy} z={dz})')
              for p in b_scan:
                FINAL.add(tuple([p[0] + dx, p[1]+dy, p[2]+dz]))
              found = b
  #print(BAD, found)
  assert found
  BAD.remove(found)
  GOOD.add(found)
print(len(FINAL))

p2_ans = 0
for p1 in P:
  for p2 in P:
    dist = abs(p1[0]-p2[0]) + abs(p1[1]-p2[1]) + abs(p1[2]-p2[2])
    if dist > p2_ans:
      p2_ans = dist
print(p2_ans)

