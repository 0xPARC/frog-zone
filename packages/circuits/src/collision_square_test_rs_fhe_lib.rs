use rayon::prelude::*;
use std::collections::HashMap;

use phantom_zone_evaluator::boolean::{fhew::prelude::*, FheBool};

enum GateInput {
    Arg(usize, usize), // arg + index
    Output(usize),     // reuse of output wire
    Tv(usize),         // temp value
    Cst(bool),         // constant
}

use GateInput::*;

#[derive(PartialEq, Eq, Hash)]
enum CellType {
    AND2,
    NAND2,
    XOR2,
    XNOR2,
    OR2,
    NOR2,
    INV,
    // TODO: Add back MUX2
}

use CellType::*;

static LEVEL_0: [((usize, bool, CellType), &[GateInput]); 7] = [
    ((1, false, INV), &[Arg(0, 2)]),
    ((2, false, AND2), &[Arg(1, 0), Arg(1, 1)]),
    ((3, false, NOR2), &[Arg(1, 0), Arg(1, 1)]),
    ((4, false, NOR2), &[Arg(1, 4), Arg(1, 3)]),
    ((7, false, AND2), &[Arg(1, 4), Arg(1, 3)]),
    ((11, false, AND2), &[Arg(0, 0), Arg(0, 1)]),
    ((12, false, NOR2), &[Arg(0, 0), Arg(0, 1)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 7] = [
    ((0, false, INV), &[Arg(1, 2)]),
    ((5, false, AND2), &[Tv(3), Tv(4)]),
    ((8, false, AND2), &[Tv(2), Tv(7)]),
    ((13, false, NOR2), &[Arg(0, 4), Arg(0, 3)]),
    ((14, false, AND2), &[Tv(1), Tv(12)]),
    ((16, false, AND2), &[Arg(0, 4), Arg(0, 3)]),
    ((17, false, AND2), &[Arg(0, 2), Tv(11)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((6, false, NAND2), &[Tv(0), Tv(5)]),
    ((9, false, NAND2), &[Arg(1, 2), Tv(8)]),
    ((15, false, NAND2), &[Tv(13), Tv(14)]),
    ((18, false, NAND2), &[Tv(16), Tv(17)]),
    ((22, false, NOR2), &[Arg(0, 6), Arg(0, 7)]),
    ((23, false, NOR2), &[Arg(1, 6), Arg(1, 7)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((10, false, AND2), &[Tv(6), Tv(9)]),
    ((19, false, AND2), &[Tv(15), Tv(18)]),
    ((21, false, NOR2), &[Arg(1, 5), Arg(0, 5)]),
    ((24, false, AND2), &[Tv(22), Tv(23)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 2] = [
    ((20, false, NAND2), &[Tv(10), Tv(19)]),
    ((25, false, AND2), &[Tv(21), Tv(24)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 1] =
    [((0, true, NAND2), &[Tv(20), Tv(25)])];

static PRUNE_1: [usize; 7] = [12, 1, 7, 2, 3, 4, 11];

static PRUNE_5: [usize; 2] = [25, 20];

static PRUNE_3: [usize; 6] = [18, 6, 9, 15, 22, 23];

static PRUNE_4: [usize; 4] = [24, 19, 21, 10];

static PRUNE_2: [usize; 7] = [17, 5, 0, 13, 8, 14, 16];

fn prune<E: BoolEvaluator>(
    temp_nodes: &mut HashMap<usize, FheBool<E>>,
    temp_node_ids: &[usize],
) {
    for x in temp_node_ids {
        temp_nodes.remove(&x);
    }
}

pub fn entrypoint<E: BoolEvaluator>(
    x: &Vec<FheBool<E>>,
    y: &Vec<FheBool<E>>,
) -> FheBool<E> {
    let args: &[&Vec<FheBool<E>>] = &[x, y];

    let mut temp_nodes = HashMap::new();
    let mut out = Vec::new();
    out.resize(1, None);

    let mut run_level =
        |temp_nodes: &mut HashMap<usize, FheBool<E>>,
         tasks: &[((usize, bool, CellType), &[GateInput])]| {
            let updates = tasks
                .into_par_iter()
                .map(|(k, task_args)| {
                    let (id, is_output, celltype) = k;
                    let task_args = task_args
                        .into_iter()
                        .map(|arg| match arg {
                            Cst(false) => todo!(),
                            Cst(true) => todo!(),
                            Arg(pos, ndx) => &args[*pos][*ndx],
                            Tv(ndx) => &temp_nodes[ndx],
                            Output(ndx) => &out[*ndx]
                                .as_ref()
                                .expect(&format!("Output node {ndx} not found")),
                        })
                        .collect::<Vec<_>>();

                    let gate_func = |args: &[&FheBool<E>]| match celltype {
                        AND2 => args[0] & args[1],
                        NAND2 => args[0].bitnand(args[1]),
                        OR2 => args[0] | args[1],
                        NOR2 => args[0].bitnor(args[1]),
                        XOR2 => args[0] ^ args[1],
                        XNOR2 => args[0].bitxnor(args[1]),
                        INV => !args[0],
                    };

                    ((*id, *is_output), gate_func(&task_args))
                })
                .collect::<Vec<_>>();
            updates.into_iter().for_each(|(k, v)| {
                let (index, is_output) = k;
                if is_output {
                    out[index] = Some(v);
                } else {
                    temp_nodes.insert(index, v);
                }
            });
        };

    run_level(&mut temp_nodes, &LEVEL_0);
    run_level(&mut temp_nodes, &LEVEL_1);
    prune(&mut temp_nodes, &PRUNE_1);
    run_level(&mut temp_nodes, &LEVEL_2);
    prune(&mut temp_nodes, &PRUNE_2);
    run_level(&mut temp_nodes, &LEVEL_3);
    prune(&mut temp_nodes, &PRUNE_3);
    run_level(&mut temp_nodes, &LEVEL_4);
    prune(&mut temp_nodes, &PRUNE_4);
    run_level(&mut temp_nodes, &LEVEL_5);
    prune(&mut temp_nodes, &PRUNE_5);

    out[0].clone().unwrap()
}
