#!/bin/sh

set -ex

FUNCS="apply_move
get_cell
get_cross_cells
get_five_cells
get_horizontal_cells
get_vertical_cells"

for func in $FUNCS; do
    cp ~/git/fully-homomorphic-encryption/projects/frogzone/out/${func}_rs_fhe_lib.rs src/frogzone_${func}_rs_fhe_lib.rs 
done
