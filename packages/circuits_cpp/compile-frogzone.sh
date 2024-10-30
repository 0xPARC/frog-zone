#!/bin/sh

set -ex

project="frogzone"

if [ -z "${func}" ]; then
	funcs="apply_move
	get_cell
	get_five_cells
	get_cross_cells
	get_horizontal_cells
	get_vertical_cells"
else
	funcs="${func}"
fi

cp -r "/projects/${project}/" "transpiler/examples/prj_${project}"
mkdir -p "/projects/${project}/out"

for func in $funcs; do
	cat "transpiler/examples/prj_${project}/get_cell_no_check.cc" >> "transpiler/examples/prj_${project}/${func}.cc"
	bazel build --sandbox_debug --verbose_failures "//transpiler/examples/prj_${project}:${func}_rs_fhe_lib" || true
	# bazel test //transpiler/examples/prj_${project}:${project}_test
	cp "bazel-out/k8-opt/bin/transpiler/examples/prj_${project}/${func}_rs_fhe_lib.rs" \
		"/projects/${project}/out/"
done

chmod +w-x+X -R "/projects/${project}/out/"
chown 1000 -R "/projects/${project}/out/"

