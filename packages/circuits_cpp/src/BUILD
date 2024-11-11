load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//transpiler/rust:fhe_rs.bzl", "fhe_rust_library")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

licenses(["notice"])

fhe_rust_library(
    name = "apply_move_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "apply_move.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "apply_move_monster_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "apply_move_monster.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "apply_move_flying_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "apply_move_flying.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "get_cell_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "get_cell.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "get_five_cells_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "get_five_cells.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "get_cross_cells_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "get_cross_cells.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "get_horizontal_cells_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "get_horizontal_cells.cc",
    hdrs = ["frogzone.h"],
)

fhe_rust_library(
    name = "get_vertical_cells_rs_fhe_lib",
    lut_size = 0,
    loop_unroll_rlimit = 256,
    src = "get_vertical_cells.cc",
    hdrs = ["frogzone.h"],
)

rust_library(
    name = "frogzone_rs_lib",
    srcs = ["frogzone_rs_lib.rs"],
    deps = [
        ":frogzone_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

rust_binary(
    name = "frogzone_rs_main",
    srcs = ["frogzone_rs_lib.rs"],
    deps = [
        ":frogzone_rs_fhe_lib",
        "@crate_index//:phantom-zone",
    ],
)

exports_files(
    [
        "frogzone.h",
        "frogzone.cc",
    ],
)
