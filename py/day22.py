#!/usr/bin/python3
import sys
import heapq
import itertools
import re
import ast
import numpy as np
from collections import defaultdict, Counter, deque

infile = sys.argv[1] if len(sys.argv)>1 else '22.in'
handle = open(infile, 'r')

def part_1():
  handle = open('input.txt','r')
  data = np.zeros((101,101,101))
  for line in handle:
    items = line.split(' ')
    if items[0] == 'on':
      value = 1
    else:
      value = 0
    ranges = [x[2:] for x in items[1].split(',')]
    x_start,x_end = [int(x)+50 for x in ranges[0].split("..")]
    y_start,y_end = [int(x)+50 for x in ranges[1].split("..")]
    z_start,z_end = [int(x)+50 for x in ranges[2].split("..")]
    data[x_start:x_end+1,y_start:y_end+1,z_start:z_end+1] = value
  
  print(np.sum(data))

def part_2():
  handle = open('input.txt','r')
  data = []
  for line in handle:
    items = line.split(' ')
    if items[0] == 'on':
      value = 1
    else:
      value = 0
    ranges = [x[2:] for x in items[1].split(',')]
    x_start,x_end = [int(x) for x in ranges[0].split("..")]
    y_start,y_end = [int(x) for x in ranges[1].split("..")]
    z_start,z_end = [int(x) for x in ranges[2].split("..")]
    x_end+=1
    y_end+=1
    z_end+=1
    cube = [x_start,x_end,y_start,y_end,z_start,z_end,value]
    new_items = []
    for i in range(len(data)):
      item = data[i]

      x_overlap = x_end > item[0] and x_start < item[1]
      y_overlap = y_end > item[2] and y_start < item[3]
      z_overlap = z_end > item[4] and z_start < item[5]
      if x_overlap and y_overlap and z_overlap:

        if item[0] < x_start:
          new_item = [item[0],x_start,item[2],item[3],item[4],item[5],item[6]]
          item[0] = x_start
          new_items.append(new_item)
        if item[1] > x_end:
          new_item = [x_end,item[1],item[2],item[3],item[4],item[5],item[6]]
          item[1] = x_end
          new_items.append(new_item)
        if item[2] < y_start:
          new_item = [item[0],item[1],item[2],y_start,item[4],item[5],item[6]]
          item[2] = y_start
          new_items.append(new_item)        
        if item[3] > y_end:
          new_item = [item[0],item[1],y_end,item[3],item[4],item[5],item[6]]
          item[3] = y_end
          new_items.append(new_item) 
        if item[4] < z_start:
          new_item = [item[0],item[1],item[2],item[3],item[4],z_start,item[6]]
          item[4] = z_start
          new_items.append(new_item)        
        if item[5] > z_end:
          new_item = [item[0],item[1],item[2],item[3],z_end,item[5],item[6]]
          item[5] = z_end
          new_items.append(new_item)
      else:
        new_items.append(item)
    
    new_items.append(cube)
    data = new_items

  total = 0
  for i in range(len(data)):
    item = data[i]
    if item[6]==1:
      total+=(item[1]-item[0])*(item[3]-item[2])*(item[5]-item[4])

  print(total)

part_1()
part_2()