
use rayon::prelude::*;
use std::collections::HashMap;

use phantom_zone_evaluator::boolean::{fhew::prelude::*, FheBool};

enum GateInput {
    Arg(usize, usize), // arg + index
    Output(usize), // reuse of output wire
    Tv(usize),  // temp value
    Cst(bool),  // constant
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


static LEVEL_0: [((usize, bool, CellType), &[GateInput]); 2] = [
    ((45, false, NOR2), &[Arg(1, 4), Arg(1, 5)]),
    ((46, false, NOR2), &[Arg(1, 6), Arg(1, 7)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 3] = [
    ((28, false, INV), &[Arg(1, 1)]),
    ((43, false, NOR2), &[Arg(1, 2), Arg(1, 3)]),
    ((47, false, AND2), &[Tv(45), Tv(46)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 5] = [
    ((26, false, INV), &[Arg(1, 0)]),
    ((44, false, AND2), &[Arg(1, 0), Tv(43)]),
    ((48, false, AND2), &[Arg(1, 1), Tv(47)]),
    ((52, false, AND2), &[Tv(28), Tv(47)]),
    ((53, false, NOR2), &[Arg(1, 0), Arg(1, 3)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((25, false, INV), &[Arg(0, 8)]),
    ((27, false, INV), &[Arg(0, 0)]),
    ((49, false, AND2), &[Tv(44), Tv(48)]),
    ((54, false, AND2), &[Arg(1, 2), Tv(53)]),
    ((0, false, AND2), &[Tv(44), Tv(52)]),
    ((3, false, AND2), &[Tv(26), Tv(43)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((29, false, INV), &[Arg(0, 9)]),
    ((36, false, INV), &[Arg(0, 1)]),
    ((50, false, AND2), &[Tv(27), Tv(49)]),
    ((51, false, XNOR2), &[Tv(27), Tv(49)]),
    ((55, false, AND2), &[Tv(52), Tv(54)]),
    ((1, false, AND2), &[Tv(25), Tv(0)]),
    ((2, false, XNOR2), &[Tv(25), Tv(0)]),
    ((4, false, AND2), &[Tv(48), Tv(3)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((30, false, INV), &[Arg(0, 10)]),
    ((37, false, INV), &[Arg(0, 2)]),
    ((56, false, AND2), &[Tv(51), Tv(55)]),
    ((57, false, AND2), &[Tv(36), Tv(50)]),
    ((58, false, XNOR2), &[Tv(36), Tv(50)]),
    ((5, false, AND2), &[Tv(2), Tv(4)]),
    ((6, false, AND2), &[Tv(29), Tv(1)]),
    ((7, false, XNOR2), &[Tv(29), Tv(1)]),
];

static LEVEL_6: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((31, false, INV), &[Arg(0, 11)]),
    ((38, false, INV), &[Arg(0, 3)]),
    ((59, false, AND2), &[Tv(56), Tv(58)]),
    ((60, false, AND2), &[Tv(37), Tv(57)]),
    ((61, false, XNOR2), &[Tv(37), Tv(57)]),
    ((8, false, AND2), &[Tv(5), Tv(7)]),
    ((9, false, AND2), &[Tv(30), Tv(6)]),
    ((10, false, XNOR2), &[Tv(30), Tv(6)]),
];

static LEVEL_7: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((32, false, INV), &[Arg(0, 12)]),
    ((39, false, INV), &[Arg(0, 4)]),
    ((62, false, AND2), &[Tv(59), Tv(61)]),
    ((63, false, AND2), &[Tv(38), Tv(60)]),
    ((64, false, XNOR2), &[Tv(38), Tv(60)]),
    ((11, false, AND2), &[Tv(8), Tv(10)]),
    ((12, false, AND2), &[Tv(31), Tv(9)]),
    ((13, false, XNOR2), &[Tv(31), Tv(9)]),
];

static LEVEL_8: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((33, false, INV), &[Arg(0, 13)]),
    ((40, false, INV), &[Arg(0, 5)]),
    ((65, false, AND2), &[Tv(62), Tv(64)]),
    ((66, false, AND2), &[Tv(39), Tv(63)]),
    ((67, false, XNOR2), &[Tv(39), Tv(63)]),
    ((14, false, AND2), &[Tv(11), Tv(13)]),
    ((15, false, AND2), &[Tv(32), Tv(12)]),
    ((16, false, XNOR2), &[Tv(32), Tv(12)]),
];

static LEVEL_9: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((34, false, INV), &[Arg(0, 14)]),
    ((41, false, INV), &[Arg(0, 6)]),
    ((68, false, AND2), &[Tv(65), Tv(67)]),
    ((69, false, AND2), &[Tv(40), Tv(66)]),
    ((70, false, XNOR2), &[Tv(40), Tv(66)]),
    ((17, false, AND2), &[Tv(14), Tv(16)]),
    ((18, false, AND2), &[Tv(33), Tv(15)]),
    ((19, false, XNOR2), &[Tv(33), Tv(15)]),
];

static LEVEL_10: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((35, false, INV), &[Arg(0, 15)]),
    ((42, false, INV), &[Arg(0, 7)]),
    ((71, false, AND2), &[Tv(68), Tv(70)]),
    ((72, false, AND2), &[Tv(41), Tv(69)]),
    ((73, false, XNOR2), &[Tv(41), Tv(69)]),
    ((20, false, AND2), &[Tv(17), Tv(19)]),
    ((21, false, AND2), &[Tv(34), Tv(18)]),
    ((22, false, XNOR2), &[Tv(34), Tv(18)]),
];

static LEVEL_11: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((74, false, NAND2), &[Tv(71), Tv(73)]),
    ((75, false, XNOR2), &[Tv(42), Tv(72)]),
    ((23, false, NAND2), &[Tv(20), Tv(22)]),
    ((24, false, XNOR2), &[Tv(35), Tv(21)]),
];

static LEVEL_12: [((usize, bool, CellType), &[GateInput]); 16] = [
    ((0, true, XOR2), &[Tv(51), Tv(55)]),
    ((1, true, XOR2), &[Tv(56), Tv(58)]),
    ((2, true, XOR2), &[Tv(59), Tv(61)]),
    ((3, true, XOR2), &[Tv(62), Tv(64)]),
    ((4, true, XOR2), &[Tv(65), Tv(67)]),
    ((5, true, XOR2), &[Tv(68), Tv(70)]),
    ((6, true, XOR2), &[Tv(71), Tv(73)]),
    ((7, true, XNOR2), &[Tv(74), Tv(75)]),
    ((8, true, XOR2), &[Tv(2), Tv(4)]),
    ((9, true, XOR2), &[Tv(5), Tv(7)]),
    ((10, true, XOR2), &[Tv(8), Tv(10)]),
    ((11, true, XOR2), &[Tv(11), Tv(13)]),
    ((12, true, XOR2), &[Tv(14), Tv(16)]),
    ((13, true, XOR2), &[Tv(17), Tv(19)]),
    ((14, true, XOR2), &[Tv(20), Tv(22)]),
    ((15, true, XNOR2), &[Tv(23), Tv(24)]),
];

static PRUNE_11: [usize; 4] = [
  72,
  21,
  35,
  42,
];

static PRUNE_5: [usize; 4] = [
  1,
  36,
  50,
  29,
];

static PRUNE_8: [usize; 4] = [
  63,
  32,
  12,
  39,
];

static PRUNE_2: [usize; 2] = [
  28,
  47,
];

static PRUNE_12: [usize; 32] = [
  24,
  55,
  62,
  7,
  14,
  58,
  65,
  10,
  17,
  73,
  56,
  8,
  4,
  59,
  11,
  67,
  5,
  74,
  19,
  70,
  22,
  61,
  68,
  13,
  75,
  20,
  51,
  64,
  2,
  71,
  16,
  23,
];

static PRUNE_9: [usize; 4] = [
  66,
  15,
  33,
  40,
];

static PRUNE_3: [usize; 4] = [
  43,
  26,
  53,
  44,
];

static PRUNE_6: [usize; 4] = [
  57,
  30,
  37,
  6,
];

static PRUNE_7: [usize; 4] = [
  31,
  38,
  60,
  9,
];

static PRUNE_1: [usize; 2] = [
  45,
  46,
];

static PRUNE_4: [usize; 8] = [
  0,
  27,
  3,
  48,
  49,
  25,
  52,
  54,
];

static PRUNE_10: [usize; 4] = [
  69,
  34,
  41,
  18,
];

fn prune<E: BoolEvaluator>(
    temp_nodes: &mut HashMap<usize, FheBool<E>>,
    temp_node_ids: &[usize],
) {
  for x in temp_node_ids {
    temp_nodes.remove(&x);
  }
}

pub fn entrypoint<E: BoolEvaluator>(dir: &Vec<FheBool<E>>, position: &Vec<FheBool<E>>) -> Vec<FheBool<E>> {
    let args: &[&Vec<FheBool<E>>] = &[position, dir];

    let mut temp_nodes = HashMap::new();
    let mut out = Vec::new();
    out.resize(16, None);

    let mut run_level = |
    temp_nodes: &mut HashMap<usize, FheBool<E>>,
    tasks: &[((usize, bool, CellType), &[GateInput])]
    | {
        let updates = tasks
            .into_par_iter()
            .map(|(k, task_args)| {
                let (id, is_output, celltype) = k;
                let task_args = task_args.into_iter()
                .map(|arg| match arg {
                    Cst(false) => todo!(),
                    Cst(true) => todo!(),
                    Arg(pos, ndx) => &args[*pos][*ndx],
                    Tv(ndx) => &temp_nodes[ndx],
                    Output(ndx) => &out[*ndx]
                                .as_ref()
                                .expect(&format!("Output node {ndx} not found")),
                }).collect::<Vec<_>>();

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
    run_level(&mut temp_nodes, &LEVEL_11);
    prune(&mut temp_nodes, &PRUNE_11);
    run_level(&mut temp_nodes, &LEVEL_12);
    prune(&mut temp_nodes, &PRUNE_12);

    

    out.into_iter().map(|c| c.unwrap()).collect()
}

