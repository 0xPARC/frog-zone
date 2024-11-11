#!/bin/sh

# This script will compile all the functions using gcc and run a very basic
# test.  This is useful to make sure the cpp source compiles correctly and for
# testing functionality.

set -ex

g++ apply_move.cc \
    apply_move_monster.cc \
    apply_move_flying.cc \
    get_cell_no_check.cc \
    get_cell.cc \
    get_five_cells.cc \
    get_cross_cells.cc \
    get_horizontal_cells.cc \
    get_vertical_cells.cc \
    main.cc -o main
./main
