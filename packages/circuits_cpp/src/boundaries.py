#!/usr/bin/env python3

import random

def print_boundaries(boundaries):
    print('const int boundaries[] = {')
    num=len(boundaries)
    for i in range(num):
        (x, y) = boundaries[i]
        if i != 0:
            print(', ', end='')
        print(f'{x}, {y}', end='')
        if i % 8 == 7:
            print()
    print('};')
    print(f'const int len = {num};')

random.seed(42)
width=32
num = 128
boundaries = []
# for i in range(num):
#     x = random.randrange(0, width)
#     y = random.randrange(0, width)
#     boundaries.append((x, y))
for i in range(width):
    boundaries.append((0, i))
    boundaries.append((width-1, i))
    boundaries.append((i, 0))
    boundaries.append((i, width-1))

print_boundaries(boundaries)

