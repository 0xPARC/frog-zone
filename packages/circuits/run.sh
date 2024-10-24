#!/bin/sh

set -e

echo "Cpu:"
lscpu  | grep "Model name"

# benches="Collision1
# Collision2
# Move1
# Move2
# Getview10"

# benches="Getview5
# Getview10
# Getview15
# Getview20"

# benches="Getviewx"
# benches="Getview20Partial1
# Getview20Partial5"

benches="FZApplyMove
FZGetCell
FZGetCrossCells
FZGetFiveCells
FZGetHorizontalCells
FZGetVerticalCells"

for bench in $benches; do
    echo "# Benchmark $bench"
    # RAYON_NUM_THREADS=1
    BENCH=$bench RUSTFLAGS='-C target-cpu=native -Awarnings' cargo run --quiet --release
done
