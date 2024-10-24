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

static LEVEL_0: [((usize, bool, CellType), &[GateInput]); 12] = [
    ((14, false, INV), &[Arg(0, 29)]),
    ((20, false, NOR2), &[Arg(0, 21), Arg(0, 22)]),
    ((21, false, NOR2), &[Arg(0, 16), Arg(0, 19)]),
    ((23, false, NOR2), &[Arg(0, 25), Arg(0, 26)]),
    ((26, false, NOR2), &[Arg(0, 20), Arg(0, 23)]),
    ((27, false, NOR2), &[Arg(0, 17), Arg(0, 18)]),
    ((29, false, NOR2), &[Arg(0, 28), Arg(0, 30)]),
    ((30, false, NOR2), &[Arg(0, 24), Arg(0, 27)]),
    ((34, false, NOR2), &[Arg(0, 12), Arg(0, 13)]),
    ((35, false, NOR2), &[Arg(0, 14), Arg(0, 15)]),
    ((37, false, NOR2), &[Arg(0, 8), Arg(0, 9)]),
    ((38, false, NOR2), &[Arg(0, 10), Arg(0, 11)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 10] = [
    ((15, false, INV), &[Arg(0, 2)]),
    ((22, false, AND2), &[Tv(20), Tv(21)]),
    ((24, false, AND2), &[Tv(14), Tv(23)]),
    ((28, false, AND2), &[Tv(26), Tv(27)]),
    ((31, false, AND2), &[Tv(29), Tv(30)]),
    ((36, false, AND2), &[Tv(34), Tv(35)]),
    ((39, false, AND2), &[Tv(37), Tv(38)]),
    ((41, false, NOR2), &[Arg(0, 4), Arg(0, 5)]),
    ((42, false, NOR2), &[Arg(0, 6), Arg(0, 7)]),
    ((44, false, OR2), &[Arg(0, 2), Arg(0, 1)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((16, false, INV), &[Arg(0, 0)]),
    ((17, false, INV), &[Arg(0, 31)]),
    ((25, false, AND2), &[Tv(22), Tv(24)]),
    ((32, false, AND2), &[Tv(28), Tv(31)]),
    ((40, false, AND2), &[Tv(36), Tv(39)]),
    ((43, false, AND2), &[Tv(41), Tv(42)]),
    ((45, false, NAND2), &[Arg(0, 3), Tv(44)]),
    ((52, false, AND2), &[Tv(15), Arg(0, 3)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 7] = [
    ((18, false, INV), &[Arg(0, 3)]),
    ((33, false, AND2), &[Tv(25), Tv(32)]),
    ((46, false, AND2), &[Tv(43), Tv(45)]),
    ((49, false, AND2), &[Tv(17), Tv(40)]),
    ((51, false, AND2), &[Arg(0, 1), Tv(43)]),
    ((53, false, AND2), &[Tv(16), Tv(52)]),
    ((61, false, NOR2), &[Arg(0, 0), Arg(0, 1)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((47, false, AND2), &[Tv(40), Tv(46)]),
    ((50, false, AND2), &[Tv(33), Tv(49)]),
    ((54, false, AND2), &[Tv(51), Tv(53)]),
    ((62, false, AND2), &[Tv(43), Tv(61)]),
    ((65, false, NOR2), &[Arg(0, 2), Arg(0, 3)]),
    ((70, false, AND2), &[Arg(0, 2), Tv(18)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 5] = [
    ((48, false, NAND2), &[Tv(33), Tv(47)]),
    ((55, false, NAND2), &[Tv(50), Tv(54)]),
    ((66, false, AND2), &[Tv(43), Tv(65)]),
    ((68, false, XOR2), &[Arg(0, 0), Arg(0, 1)]),
    ((71, false, AND2), &[Tv(62), Tv(70)]),
];

static LEVEL_6: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((56, false, AND2), &[Tv(50), Tv(52)]),
    ((59, false, NAND2), &[Tv(48), Tv(55)]),
    ((67, false, AND2), &[Arg(0, 0), Arg(0, 1)]),
    ((69, false, NAND2), &[Tv(66), Tv(68)]),
    ((72, false, NOR2), &[Tv(54), Tv(71)]),
    ((2, false, AND2), &[Arg(0, 0), Tv(43)]),
];

static LEVEL_7: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((19, false, INV), &[Arg(0, 1)]),
    ((57, false, AND2), &[Tv(16), Tv(51)]),
    ((60, false, AND2), &[Tv(17), Tv(59)]),
    ((63, false, NAND2), &[Tv(56), Tv(62)]),
    ((0, false, NAND2), &[Tv(69), Tv(72)]),
    ((3, false, AND2), &[Tv(50), Tv(70)]),
    ((6, false, AND2), &[Tv(56), Tv(2)]),
    ((9, false, NAND2), &[Tv(66), Tv(67)]),
];

static LEVEL_8: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((58, false, NAND2), &[Tv(56), Tv(57)]),
    ((64, false, AND2), &[Tv(60), Tv(63)]),
    ((1, false, NAND2), &[Tv(50), Tv(0)]),
    ((4, false, NAND2), &[Tv(2), Tv(3)]),
    ((7, false, NAND2), &[Tv(19), Tv(6)]),
    ((10, false, NAND2), &[Tv(72), Tv(9)]),
];

static LEVEL_9: [((usize, bool, CellType), &[GateInput]); 5] = [
    ((5, false, AND2), &[Tv(1), Tv(4)]),
    ((8, false, AND2), &[Tv(60), Tv(7)]),
    ((11, false, NAND2), &[Tv(50), Tv(10)]),
    ((12, false, AND2), &[Tv(58), Tv(64)]),
    ((13, false, NAND2), &[Tv(51), Tv(3)]),
];

static LEVEL_10: [((usize, bool, CellType), &[GateInput]); 7] = [
    ((31, true, INV), &[Tv(60)]),
    ((0, true, NAND2), &[Tv(64), Tv(5)]),
    ((1, true, NAND2), &[Tv(8), Tv(11)]),
    ((4, true, INV), &[Tv(12)]),
    ((2, true, NAND2), &[Tv(4), Tv(12)]),
    ((3, true, NAND2), &[Tv(60), Tv(13)]),
    ((5, true, NAND2), &[Tv(58), Tv(8)]),
];

static PRUNE_2: [usize; 10] = [36, 39, 15, 22, 28, 42, 24, 31, 41, 44];

static PRUNE_5: [usize; 3] = [65, 33, 47];

static PRUNE_6: [usize; 7] = [43, 52, 55, 48, 68, 71, 54];

static PRUNE_9: [usize; 6] = [50, 1, 7, 3, 10, 51];

static PRUNE_1: [usize; 12] = [26, 29, 21, 35, 38, 14, 27, 34, 30, 37, 20, 23];

static PRUNE_7: [usize; 8] = [67, 70, 59, 66, 62, 69, 17, 16];

static PRUNE_10: [usize; 9] = [5, 12, 60, 8, 4, 11, 58, 13, 64];

static PRUNE_4: [usize; 6] = [46, 53, 18, 49, 61, 40];

static PRUNE_8: [usize; 9] = [19, 57, 56, 63, 0, 72, 6, 2, 9];

static PRUNE_3: [usize; 3] = [25, 32, 45];

fn prune<E: BoolEvaluator>(
    temp_nodes: &mut HashMap<usize, FheBool<E>>,
    temp_node_ids: &[usize],
) {
    for x in temp_node_ids {
        temp_nodes.remove(&x);
    }
}

pub fn fibonacci_number<E: BoolEvaluator>(n: &Vec<FheBool<E>>) -> Vec<FheBool<E>> {
    let args: &[&Vec<FheBool<E>>] = &[n];

    let mut temp_nodes = HashMap::new();
    let mut out = Vec::new();
    out.resize(32, None);

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
    run_level(&mut temp_nodes, &LEVEL_6);
    prune(&mut temp_nodes, &PRUNE_6);
    run_level(&mut temp_nodes, &LEVEL_7);
    prune(&mut temp_nodes, &PRUNE_7);
    run_level(&mut temp_nodes, &LEVEL_8);
    prune(&mut temp_nodes, &PRUNE_8);
    run_level(&mut temp_nodes, &LEVEL_9);
    prune(&mut temp_nodes, &PRUNE_9);
    run_level(&mut temp_nodes, &LEVEL_10);
    prune(&mut temp_nodes, &PRUNE_10);

    out[10] = out[31].clone();
    out[11] = out[31].clone();
    out[12] = out[31].clone();
    out[13] = out[31].clone();
    out[14] = out[31].clone();
    out[15] = out[31].clone();
    out[16] = out[31].clone();
    out[17] = out[31].clone();
    out[18] = out[31].clone();
    out[19] = out[31].clone();
    out[20] = out[31].clone();
    out[21] = out[31].clone();
    out[22] = out[31].clone();
    out[23] = out[31].clone();
    out[24] = out[31].clone();
    out[25] = out[31].clone();
    out[26] = out[31].clone();
    out[27] = out[31].clone();
    out[28] = out[31].clone();
    out[29] = out[31].clone();
    out[30] = out[31].clone();
    out[6] = out[31].clone();
    out[7] = out[31].clone();
    out[8] = out[31].clone();
    out[9] = out[31].clone();

    out.into_iter().map(|c| c.unwrap()).collect()
}
