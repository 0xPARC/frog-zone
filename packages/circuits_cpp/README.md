# Circuits cpp

C++ source code of the frogzone circuits.

# Compiling the circuits

To compile the circuits you need to use docker.  The entire process is
automated with the `run-docker.sh` script which will create a docker container
using the `ed255/phantom-zone` image and then run the `compile-frogzone.sh`
script inside of it.

Compile all circuits:
```
SUDO=1 ./run-docker.sh
```

Compile a single circuit:
```
SUDO=1 ./run-docker.sh get_cell
```

If you don't need `sudo` to run docker you can remove the `SUDO=1` part from
the commands.

# Files

Circuit function definitions.  The circuit entrypoint is the function that has the same name as the file:
- apply_move.cc
- get_cell.cc
- get_cross_cells.cc
- get_five_cells.cc
- get_horizontal_cells.cc
- get_vertical_cells.cc

Quick script to generate boundaries for testing:
- boundaries.py

Bazel build file.  It has an entry for each circuit:
- BUILD

Header file.  Contains all the structs, classes and circuit function declarations:
- frogzone.h

Common functions.  This file contains functions that are used by several circuit functions.  In the compilation process it's concatenated to each circuit function file.
- get_cell_no_check.cc

A simple main function used for testing purposes:
- main.cc

A script to compile the cpp source with gcc and run the main function.  This is
for testing before compiling the circuits:
- test_compile.sh

The circuit compilation process will put the results here as rust files:
- out
