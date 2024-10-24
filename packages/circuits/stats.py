#!/usr/bin/env python3

from os import listdir
from os.path import isfile, join

def scan(src_file):
    lines = []
    with open(src_file) as file:
        lines = [line.rstrip() for line in file]
    level_size = []
    size = 0
    in_level = False
    for line in lines:
        if line.startswith('static LEVEL_'):
            in_level = True
            continue
        if in_level:
            size += 1
            if line == '':
                in_level = False
                level_size.append(size)
                size = 0
    return level_size

src_files = [f for f in listdir('src') if isfile(join('src', f))]
src_files = [f for f in src_files if 'rs_fhe_lib.rs' in f]
src_files = sorted(src_files)

for src_file in src_files:
    print(f'{src_file}')
    level_size = scan(join('src', src_file))
    print(f'- gates: {sum([size for size in level_size])}')
    print(f'- levels: {len(level_size)}')
    print(f'- levels_size: {level_size}')
    print()
