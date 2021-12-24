#!/usr/bin/python3
import sys
import heapq
import itertools
import re
import ast
import numpy as np
import math
from collections import defaultdict, Counter, deque
from functools import lru_cache

def parse_input(i):
	out = [[], [], [], []]
	for idx, line in enumerate(i):
		if idx == 2 or idx == 3:
			row = [DICT[c] for c in line.split("#") if c != "" and c != "  " and c != "\n"]
			for idx in range(4):
				out[idx].append(row[idx])
	return out

COMPLETED = (tuple([0] * 11), (1, 10, 100, 1000), (1, 10, 100, 1000))
COMPLETED_2 = (tuple([0] * 11), (1, 10, 100, 1000), (1, 10, 100, 1000), (1, 10, 100, 1000), (1, 10, 100, 1000))
LOGS = {1: 0, 10: 1, 100: 2, 1000: 3}
REV_LOGS = {0: 1, 1: 10, 2: 100, 3: 1000}
ROOM_ENTRANCES = {1: 2, 10: 4, 100: 6, 1000: 8}
ROOM_ENTRANCES_2 = {0: 2, 1: 4, 2: 6, 3: 8}
DICT = {"A": 1, "B": 10, "C": 100, "D": 1000}

@lru_cache
def validate_room(to_move, idx, pos):
	"""Validates a to-room move and calculates the destination and new position after it, if valid"""
	room = [pos[1][LOGS[to_move]], pos[2][LOGS[to_move]]]
	if (room[0] == 0 or room[0] == to_move) and (room[1] == 0 or room[1] == to_move):
		# check that there's no obstruction
		# ensure that between idx and goal, all are False
		room_height = 1
		dist = 1 + abs(idx - ROOM_ENTRANCES[to_move])
		if room[1] == 0:
			dist += 1
			room_height = 2
		dist *= to_move
		if idx > ROOM_ENTRANCES[to_move]:
			b = all([x == 0 for x in pos[0][ROOM_ENTRANCES[to_move]:idx]])
			new_pos = list([list(p) for p in pos])
			new_pos[0][idx] = 0
			new_pos[room_height][LOGS[to_move]] = to_move
			return b, dist, tuple([tuple(p) for p in new_pos])
		else:
			b = all([x == 0 for x in pos[0][idx + 1:ROOM_ENTRANCES[to_move]]])
			new_pos = list([list(p) for p in pos])
			new_pos[0][idx] = 0
			new_pos[room_height][LOGS[to_move]] = to_move
			return b, dist, tuple([tuple(p) for p in new_pos])
	return False, 0, []

@lru_cache
def validate_hallway(to_move, idx, pos, dest, lower):
	"""Validates a to-hallway move and calculates the destination and new position after it, if valid"""
	# only restriction: must not be obstructions
	rm_height = 2 if lower else 1
	dist = rm_height + abs(dest - ROOM_ENTRANCES_2[idx])
	dist *= to_move
	rng = pos[0][min(ROOM_ENTRANCES_2[idx], dest):max(ROOM_ENTRANCES_2[idx] + 1, dest + 1)]
	b = all([x == 0 for x in rng])
	if b:
		new_pos = list([list(p) for p in pos])
		new_pos[rm_height][idx] = 0
		new_pos[0][dest] = to_move
		return b, dist, tuple([tuple(p) for p in new_pos])
	return False, 0, []
	
@lru_cache
def valid_moves(pos):
	"""Gets all the valid moves and the energy costs of each from the current position, excluding certain always-suboptimal moves"""
	# gives a list of all valid positions and their associated costs from the current pos
	# start: get all valid from-hallway moves
	moves = []
	for idx, thing in enumerate(pos[0]):
		if thing > 0:
			# can we move this into its room?
			# check two things: room is valid for move, no obstructions
			v, d, np = validate_room(thing, idx, tuple(pos))
			if v:
				return [[np, d]]
	for idx, thing in enumerate(pos[1]):
		if thing == 0:
			pass
		# check if we can move this to a hallway pos
		else:
			for i in range(11):
				# all possible hallway positions
				if i in [2, 4, 6, 8] or pos[0][i] or (idx == LOGS[thing] and pos[2][idx] == thing):
					pass
				else:
					v, d, np = validate_hallway(thing, idx, pos, i, False)
					if v:
						moves.append([np, d])
	for idx, thing in enumerate(pos[2]):
		if pos[1][idx] or thing == 0:
			pass
		else:
			# check if we can move this to a hallway pos
			for i in range(11):
				# all possible hallway positions
				if i in [2, 4, 6, 8] or pos[0][i] or idx == LOGS[thing]:
					pass
				else:
					v, d, np = validate_hallway(thing, idx, pos, i, True)
					if v:
						moves.append([np, d])
	return moves
				
@lru_cache
def enumerate_positions(pos):
	"""Calculates the minimum possible cost from this position to the complete position"""
	# calc all legal moves from pos, sum across them
	if pos == COMPLETED:
		return 0
	else:
		moves = valid_moves(pos)
		if len(moves) == 0:
			return math.inf
		else:
			return min([enumerate_positions(p[0]) + p[1] for p in moves])
			
			
			
			
			
			
			
			
			
			
			
@lru_cache
def validate_room2(to_move, idx, pos):
	"""Validates a to-room move and calculates the destination and new position after it, if valid"""
	room = [pos[1][LOGS[to_move]], pos[2][LOGS[to_move]], pos[3][LOGS[to_move]], pos[4][LOGS[to_move]]]
	if (room[0] == 0 or room[0] == to_move) and (room[1] == 0 or room[1] == to_move) and (room[2] == 0 or room[2] == to_move) and (room[3] == 0 or room[3] == to_move):
		# check that there's no obstruction
		# ensure that between idx and goal, all are False
		room_height = 1
		dist = 1 + abs(idx - ROOM_ENTRANCES[to_move])
		if room[1] == 0:
			dist += 1
			room_height = 2
		if room[2] == 0:
			dist += 1
			room_height = 3
		if room[3] == 0:
			dist += 1
			room_height = 4
		dist *= to_move
		if idx > ROOM_ENTRANCES[to_move]:
			b = all([x == 0 for x in pos[0][ROOM_ENTRANCES[to_move]:idx]])
			new_pos = list([list(p) for p in pos])
			new_pos[0][idx] = 0
			new_pos[room_height][LOGS[to_move]] = to_move
			return b, dist, tuple([tuple(p) for p in new_pos])
		else:
			b = all([x == 0 for x in pos[0][idx + 1:ROOM_ENTRANCES[to_move]]])
			new_pos = list([list(p) for p in pos])
			new_pos[0][idx] = 0
			new_pos[room_height][LOGS[to_move]] = to_move
			return b, dist, tuple([tuple(p) for p in new_pos])
	return False, 0, []

@lru_cache
def validate_hallway2(to_move, idx, pos, dest, rh):
	"""Validates a to-hallway move and calculates the destination and new position after it, if valid"""
	# only restriction: must not be obstructions
	dist = rh + abs(dest - ROOM_ENTRANCES_2[idx])
	dist *= to_move
	rng = pos[0][min(ROOM_ENTRANCES_2[idx], dest):max(ROOM_ENTRANCES_2[idx] + 1, dest + 1)]
	b = all([x == 0 for x in rng])
	if b:
		new_pos = list([list(p) for p in pos])
		new_pos[rh][idx] = 0
		new_pos[0][dest] = to_move
		return b, dist, tuple([tuple(p) for p in new_pos])
	return False, 0, []
	
@lru_cache
def valid_moves2(pos):
	"""Gets all the valid moves and the energy costs of each from the current position, excluding certain always-suboptimal moves"""
	# gives a list of all valid positions and their associated costs from the current pos
	# start: get all valid from-hallway moves
	moves = []
	for idx, thing in enumerate(pos[0]):
		if thing > 0:
			# can we move this into its room?
			# check two things: room is valid for move, no obstructions
			v, d, np = validate_room2(thing, idx, tuple(pos))
			if v:
				return [[np, d]] # always optimal if possible
	for rh in range(1, 5):
		for idx, thing in enumerate(pos[rh]):
			if thing == 0 or any([pos[i][idx] for i in range(1, rh)]):
				pass
			# check if we can move this to a hallway pos
			else:
				for i in range(11):
					# all possible hallway positions
					if i in [2, 4, 6, 8] or pos[0][i] or (idx == LOGS[thing] and all(pos[h][idx] == thing for h in range(rh + 1, 5))):
						pass
					else:
						v, d, np = validate_hallway2(thing, idx, pos, i, rh)
						if v:
							moves.append([np, d])
	return moves
				
@lru_cache
def enumerate_positions2(pos):
	"""Calculates the minimum possible cost from this position to the complete position"""
	# calc all legal moves from pos, sum across them
	if pos == COMPLETED_2:
		return 0
	else:
		moves = valid_moves2(pos)
		if len(moves) == 0:
			return math.inf
		else:
			return min([enumerate_positions2(p[0]) + p[1] for p in moves])
			

def part1(i):
	position = [[0] * 11, [i[0][0], i[1][0], i[2][0], i[3][0]], [i[0][1], i[1][1], i[2][1], i[3][1]]]
	position = [tuple(p) for p in position]
	return enumerate_positions(tuple(position))

def part2(i):
	position = [[0] * 11, [i[0][0], i[1][0], i[2][0], i[3][0]], [1000, 100, 10, 1], [1000, 10, 1, 100], [i[0][1], i[1][1], i[2][1], i[3][1]]]
	position = [tuple(p) for p in position]
	return enumerate_positions2(tuple(position))

def main():
	with open(sys.argv[1], "r") as f:
		i = parse_input(f.readlines())
	print(part1(i))
	print(part2(i))

if __name__ == '__main__':
		main()