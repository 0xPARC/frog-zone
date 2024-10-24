
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
    ((37, false, INV), &[Arg(1, 7)]),
    ((53, false, NOR2), &[Arg(1, 5), Arg(1, 6)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 2] = [
    ((52, false, NOR2), &[Arg(1, 3), Arg(1, 4)]),
    ((54, false, AND2), &[Tv(37), Tv(53)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 3] = [
    ((35, false, INV), &[Arg(1, 2)]),
    ((55, false, AND2), &[Tv(52), Tv(54)]),
    ((57, false, NOR2), &[Arg(1, 0), Arg(1, 1)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((36, false, INV), &[Arg(1, 1)]),
    ((56, false, AND2), &[Tv(35), Tv(55)]),
    ((58, false, AND2), &[Arg(1, 0), Arg(1, 1)]),
    ((4, false, AND2), &[Tv(55), Tv(57)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((59, false, XOR2), &[Arg(1, 0), Arg(1, 1)]),
    ((62, false, AND2), &[Arg(1, 0), Tv(36)]),
    ((3, false, AND2), &[Tv(56), Tv(58)]),
    ((5, false, AND2), &[Arg(1, 2), Tv(4)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 5] = [
    ((39, false, INV), &[Arg(0, 9)]),
    ((45, false, INV), &[Arg(0, 1)]),
    ((60, false, AND2), &[Tv(56), Tv(59)]),
    ((63, false, AND2), &[Tv(56), Tv(62)]),
    ((6, false, OR2), &[Tv(3), Tv(5)]),
];

static LEVEL_6: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((61, false, AND2), &[Arg(0, 8), Tv(60)]),
    ((65, false, XNOR2), &[Tv(39), Tv(63)]),
    ((7, false, AND2), &[Arg(0, 0), Tv(6)]),
    ((9, false, XNOR2), &[Tv(45), Tv(3)]),
];

static LEVEL_7: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((40, false, INV), &[Arg(0, 10)]),
    ((46, false, INV), &[Arg(0, 2)]),
    ((64, false, NAND2), &[Arg(0, 9), Tv(63)]),
    ((66, false, NAND2), &[Tv(61), Tv(65)]),
    ((8, false, NAND2), &[Arg(0, 1), Tv(3)]),
    ((10, false, NAND2), &[Tv(7), Tv(9)]),
];

static LEVEL_8: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((67, false, NAND2), &[Tv(64), Tv(66)]),
    ((69, false, XNOR2), &[Tv(40), Tv(63)]),
    ((11, false, NAND2), &[Tv(8), Tv(10)]),
    ((13, false, XNOR2), &[Tv(46), Tv(3)]),
];

static LEVEL_9: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((68, false, NAND2), &[Arg(0, 10), Tv(63)]),
    ((70, false, NAND2), &[Tv(67), Tv(69)]),
    ((12, false, NAND2), &[Arg(0, 2), Tv(3)]),
    ((14, false, NAND2), &[Tv(11), Tv(13)]),
];

static LEVEL_10: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((71, false, AND2), &[Tv(68), Tv(70)]),
    ((73, false, NAND2), &[Arg(0, 11), Tv(63)]),
    ((15, false, AND2), &[Tv(12), Tv(14)]),
    ((16, false, NAND2), &[Arg(0, 3), Tv(3)]),
];

static LEVEL_11: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((42, false, INV), &[Arg(0, 12)]),
    ((48, false, INV), &[Arg(0, 4)]),
    ((72, false, OR2), &[Arg(0, 11), Tv(63)]),
    ((77, false, NAND2), &[Tv(71), Tv(73)]),
    ((17, false, OR2), &[Arg(0, 3), Tv(3)]),
    ((21, false, NAND2), &[Tv(15), Tv(16)]),
];

static LEVEL_12: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((76, false, XNOR2), &[Tv(42), Tv(63)]),
    ((78, false, AND2), &[Tv(72), Tv(77)]),
    ((20, false, XNOR2), &[Tv(48), Tv(3)]),
    ((22, false, AND2), &[Tv(17), Tv(21)]),
];

static LEVEL_13: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((75, false, NAND2), &[Arg(0, 12), Tv(63)]),
    ((79, false, NAND2), &[Tv(76), Tv(78)]),
    ((19, false, NAND2), &[Arg(0, 4), Tv(3)]),
    ((23, false, NAND2), &[Tv(20), Tv(22)]),
];

static LEVEL_14: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((80, false, NAND2), &[Tv(75), Tv(79)]),
    ((81, false, OR2), &[Arg(0, 13), Tv(63)]),
    ((24, false, AND2), &[Tv(19), Tv(23)]),
    ((25, false, NAND2), &[Arg(0, 5), Tv(3)]),
];

static LEVEL_15: [((usize, bool, CellType), &[GateInput]); 6] = [
    ((43, false, INV), &[Arg(0, 14)]),
    ((50, false, INV), &[Arg(0, 6)]),
    ((82, false, NAND2), &[Arg(0, 13), Tv(63)]),
    ((87, false, NAND2), &[Tv(80), Tv(81)]),
    ((26, false, OR2), &[Arg(0, 5), Tv(3)]),
    ((30, false, NAND2), &[Tv(24), Tv(25)]),
];

static LEVEL_16: [((usize, bool, CellType), &[GateInput]); 5] = [
    ((84, false, AND2), &[Tv(43), Tv(63)]),
    ((85, false, OR2), &[Tv(43), Tv(63)]),
    ((88, false, AND2), &[Tv(82), Tv(87)]),
    ((29, false, XNOR2), &[Tv(50), Tv(3)]),
    ((31, false, AND2), &[Tv(26), Tv(30)]),
];

static LEVEL_17: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((41, false, INV), &[Arg(0, 11)]),
    ((47, false, INV), &[Arg(0, 3)]),
    ((49, false, INV), &[Arg(0, 5)]),
    ((51, false, INV), &[Arg(0, 7)]),
    ((0, false, NAND2), &[Tv(84), Tv(88)]),
    ((1, false, OR2), &[Tv(85), Tv(87)]),
    ((28, false, NAND2), &[Arg(0, 6), Tv(3)]),
    ((32, false, NAND2), &[Tv(29), Tv(31)]),
];

static LEVEL_18: [((usize, bool, CellType), &[GateInput]); 10] = [
    ((38, false, INV), &[Arg(0, 8)]),
    ((44, false, INV), &[Arg(0, 0)]),
    ((74, false, XNOR2), &[Tv(41), Tv(63)]),
    ((83, false, XNOR2), &[Arg(0, 13), Tv(63)]),
    ((86, false, XNOR2), &[Tv(43), Tv(63)]),
    ((2, false, AND2), &[Tv(0), Tv(1)]),
    ((18, false, XNOR2), &[Tv(47), Tv(3)]),
    ((27, false, XNOR2), &[Tv(49), Tv(3)]),
    ((33, false, AND2), &[Tv(28), Tv(32)]),
    ((34, false, XNOR2), &[Tv(51), Tv(3)]),
];

static LEVEL_19: [((usize, bool, CellType), &[GateInput]); 16] = [
    ((9, true, XOR2), &[Tv(61), Tv(65)]),
    ((10, true, XOR2), &[Tv(67), Tv(69)]),
    ((11, true, XNOR2), &[Tv(71), Tv(74)]),
    ((12, true, XOR2), &[Tv(76), Tv(78)]),
    ((13, true, XNOR2), &[Tv(80), Tv(83)]),
    ((14, true, XNOR2), &[Tv(86), Tv(88)]),
    ((15, true, XNOR2), &[Arg(0, 15), Tv(2)]),
    ((8, true, XNOR2), &[Tv(38), Tv(60)]),
    ((1, true, XOR2), &[Tv(7), Tv(9)]),
    ((2, true, XOR2), &[Tv(11), Tv(13)]),
    ((3, true, XNOR2), &[Tv(15), Tv(18)]),
    ((4, true, XOR2), &[Tv(20), Tv(22)]),
    ((5, true, XNOR2), &[Tv(24), Tv(27)]),
    ((6, true, XOR2), &[Tv(29), Tv(31)]),
    ((7, true, XNOR2), &[Tv(33), Tv(34)]),
    ((0, true, XNOR2), &[Tv(44), Tv(6)]),
];

static PRUNE_17: [usize; 3] = [
  84,
  85,
  87,
];

static PRUNE_5: [usize; 4] = [
  5,
  62,
  59,
  56,
];

static PRUNE_18: [usize; 11] = [
  43,
  47,
  51,
  41,
  3,
  0,
  28,
  63,
  32,
  1,
  49,
];

static PRUNE_6: [usize; 2] = [
  39,
  45,
];

static PRUNE_12: [usize; 6] = [
  77,
  48,
  17,
  72,
  42,
  21,
];

static PRUNE_19: [usize; 31] = [
  60,
  29,
  22,
  15,
  88,
  74,
  67,
  78,
  71,
  9,
  33,
  2,
  20,
  44,
  13,
  6,
  61,
  65,
  34,
  27,
  76,
  38,
  7,
  69,
  31,
  24,
  86,
  11,
  83,
  80,
  18,
];

static PRUNE_1: [usize; 2] = [
  53,
  37,
];

static PRUNE_8: [usize; 6] = [
  46,
  40,
  64,
  10,
  66,
  8,
];

static PRUNE_14: [usize; 4] = [
  19,
  23,
  75,
  79,
];

static PRUNE_2: [usize; 2] = [
  54,
  52,
];

static PRUNE_3: [usize; 3] = [
  57,
  55,
  35,
];

static PRUNE_15: [usize; 2] = [
  81,
  25,
];

static PRUNE_16: [usize; 4] = [
  26,
  50,
  82,
  30,
];

static PRUNE_4: [usize; 3] = [
  36,
  58,
  4,
];

static PRUNE_10: [usize; 4] = [
  70,
  12,
  68,
  14,
];

static PRUNE_11: [usize; 2] = [
  16,
  73,
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
    run_level(&mut temp_nodes, &LEVEL_8);
    prune(&mut temp_nodes, &PRUNE_8);
    run_level(&mut temp_nodes, &LEVEL_9);
    run_level(&mut temp_nodes, &LEVEL_10);
    prune(&mut temp_nodes, &PRUNE_10);
    run_level(&mut temp_nodes, &LEVEL_11);
    prune(&mut temp_nodes, &PRUNE_11);
    run_level(&mut temp_nodes, &LEVEL_12);
    prune(&mut temp_nodes, &PRUNE_12);
    run_level(&mut temp_nodes, &LEVEL_13);
    run_level(&mut temp_nodes, &LEVEL_14);
    prune(&mut temp_nodes, &PRUNE_14);
    run_level(&mut temp_nodes, &LEVEL_15);
    prune(&mut temp_nodes, &PRUNE_15);
    run_level(&mut temp_nodes, &LEVEL_16);
    prune(&mut temp_nodes, &PRUNE_16);
    run_level(&mut temp_nodes, &LEVEL_17);
    prune(&mut temp_nodes, &PRUNE_17);
    run_level(&mut temp_nodes, &LEVEL_18);
    prune(&mut temp_nodes, &PRUNE_18);
    run_level(&mut temp_nodes, &LEVEL_19);
    prune(&mut temp_nodes, &PRUNE_19);

    

    out.into_iter().map(|c| c.unwrap()).collect()
}

