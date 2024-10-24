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

static LEVEL_0: [((usize, bool, CellType), &[GateInput]); 14] = [
    ((90, false, INV), &[Arg(1, 3)]),
    ((101, false, INV), &[Arg(1, 2)]),
    ((123, false, INV), &[Arg(0, 3)]),
    ((134, false, INV), &[Arg(0, 2)]),
    ((145, false, INV), &[Arg(0, 1)]),
    ((167, false, INV), &[Arg(0, 0)]),
    ((178, false, INV), &[Arg(1, 0)]),
    ((189, false, INV), &[Arg(1, 1)]),
    ((200, false, INV), &[Arg(0, 7)]),
    ((211, false, INV), &[Arg(1, 7)]),
    ((222, false, NOR2), &[Arg(1, 5), Arg(1, 6)]),
    ((255, false, NOR2), &[Arg(1, 0), Arg(1, 1)]),
    ((285, false, NOR2), &[Arg(0, 5), Arg(0, 6)]),
    ((337, false, NOR2), &[Arg(1, 3), Arg(1, 2)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 18] = [
    ((112, false, INV), &[Arg(1, 4)]),
    ((156, false, INV), &[Arg(0, 4)]),
    ((233, false, AND2), &[Tv(211), Tv(222)]),
    ((266, false, AND2), &[Arg(1, 3), Tv(101)]),
    ((284, false, AND2), &[Arg(0, 1), Tv(167)]),
    ((286, false, AND2), &[Tv(200), Tv(285)]),
    ((291, false, NOR2), &[Arg(0, 1), Arg(0, 0)]),
    ((299, false, AND2), &[Tv(90), Arg(1, 2)]),
    ((300, false, AND2), &[Arg(1, 0), Arg(1, 1)]),
    ((303, false, AND2), &[Arg(0, 3), Tv(134)]),
    ((307, false, AND2), &[Tv(145), Arg(0, 0)]),
    ((311, false, AND2), &[Arg(1, 3), Arg(1, 2)]),
    ((312, false, AND2), &[Tv(178), Arg(1, 1)]),
    ((315, false, AND2), &[Arg(0, 1), Arg(0, 0)]),
    ((316, false, AND2), &[Tv(123), Arg(0, 2)]),
    ((328, false, AND2), &[Arg(1, 0), Tv(189)]),
    ((2, false, NOR2), &[Arg(0, 3), Arg(0, 2)]),
    ((26, false, AND2), &[Tv(255), Tv(337)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 32] = [
    ((244, false, AND2), &[Tv(112), Tv(233)]),
    ((277, false, AND2), &[Tv(255), Tv(266)]),
    ((287, false, AND2), &[Tv(156), Tv(286)]),
    ((288, false, AND2), &[Arg(0, 3), Arg(0, 2)]),
    ((292, false, AND2), &[Arg(0, 4), Tv(286)]),
    ((298, false, AND2), &[Arg(1, 4), Tv(233)]),
    ((301, false, AND2), &[Tv(299), Tv(300)]),
    ((304, false, AND2), &[Tv(284), Tv(303)]),
    ((317, false, AND2), &[Tv(315), Tv(316)]),
    ((329, false, AND2), &[Tv(299), Tv(328)]),
    ((331, false, AND2), &[Tv(303), Tv(315)]),
    ((338, false, AND2), &[Tv(328), Tv(337)]),
    ((341, false, AND2), &[Tv(291), Tv(303)]),
    ((3, false, AND2), &[Tv(315), Tv(2)]),
    ((10, false, AND2), &[Tv(291), Tv(316)]),
    ((15, false, AND2), &[Tv(303), Tv(307)]),
    ((22, false, AND2), &[Tv(284), Tv(2)]),
    ((27, false, AND2), &[Tv(233), Tv(26)]),
    ((30, false, AND2), &[Tv(312), Tv(337)]),
    ((34, false, AND2), &[Tv(255), Tv(311)]),
    ((45, false, AND2), &[Tv(299), Tv(312)]),
    ((53, false, AND2), &[Tv(311), Tv(328)]),
    ((57, false, AND2), &[Tv(284), Tv(316)]),
    ((63, false, AND2), &[Tv(255), Tv(299)]),
    ((67, false, AND2), &[Tv(300), Tv(311)]),
    ((69, false, AND2), &[Tv(266), Tv(300)]),
    ((75, false, AND2), &[Tv(266), Tv(328)]),
    ((89, false, AND2), &[Tv(307), Tv(2)]),
    ((102, false, AND2), &[Tv(307), Tv(316)]),
    ((110, false, AND2), &[Tv(291), Tv(2)]),
    ((130, false, AND2), &[Tv(266), Tv(312)]),
    ((147, false, AND2), &[Tv(300), Tv(337)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 56] = [
    ((282, false, NAND2), &[Tv(244), Tv(277)]),
    ((289, false, AND2), &[Tv(287), Tv(288)]),
    ((293, false, AND2), &[Tv(288), Tv(292)]),
    ((302, false, AND2), &[Tv(298), Tv(301)]),
    ((305, false, AND2), &[Tv(287), Tv(304)]),
    ((306, false, NAND2), &[Tv(287), Tv(304)]),
    ((313, false, AND2), &[Tv(311), Tv(312)]),
    ((318, false, AND2), &[Tv(292), Tv(317)]),
    ((324, false, NAND2), &[Tv(292), Tv(304)]),
    ((339, false, AND2), &[Tv(298), Tv(338)]),
    ((340, false, NAND2), &[Tv(298), Tv(338)]),
    ((342, false, AND2), &[Tv(292), Tv(341)]),
    ((4, false, NAND2), &[Tv(287), Tv(3)]),
    ((11, false, AND2), &[Tv(286), Tv(10)]),
    ((18, false, AND2), &[Tv(287), Tv(341)]),
    ((23, false, AND2), &[Tv(292), Tv(22)]),
    ((24, false, NAND2), &[Tv(292), Tv(22)]),
    ((31, false, AND2), &[Tv(244), Tv(30)]),
    ((32, false, NAND2), &[Tv(244), Tv(30)]),
    ((35, false, AND2), &[Tv(244), Tv(34)]),
    ((36, false, AND2), &[Tv(287), Tv(15)]),
    ((46, false, AND2), &[Tv(244), Tv(45)]),
    ((54, false, AND2), &[Tv(244), Tv(53)]),
    ((55, false, AND2), &[Tv(298), Tv(45)]),
    ((64, false, AND2), &[Tv(244), Tv(63)]),
    ((65, false, NAND2), &[Tv(244), Tv(63)]),
    ((68, false, AND2), &[Tv(298), Tv(67)]),
    ((70, false, AND2), &[Tv(244), Tv(69)]),
    ((76, false, AND2), &[Tv(298), Tv(75)]),
    ((91, false, AND2), &[Tv(292), Tv(89)]),
    ((92, false, AND2), &[Tv(244), Tv(67)]),
    ((93, false, NAND2), &[Tv(244), Tv(67)]),
    ((94, false, NAND2), &[Tv(244), Tv(75)]),
    ((100, false, NAND2), &[Tv(244), Tv(301)]),
    ((103, false, AND2), &[Tv(292), Tv(102)]),
    ((104, false, NAND2), &[Tv(292), Tv(102)]),
    ((105, false, AND2), &[Tv(287), Tv(89)]),
    ((111, false, AND2), &[Tv(292), Tv(110)]),
    ((114, false, AND2), &[Tv(287), Tv(102)]),
    ((115, false, AND2), &[Tv(292), Tv(10)]),
    ((129, false, NAND2), &[Tv(287), Tv(110)]),
    ((131, false, NAND2), &[Tv(244), Tv(130)]),
    ((135, false, AND2), &[Tv(244), Tv(338)]),
    ((136, false, AND2), &[Tv(287), Tv(317)]),
    ((138, false, AND2), &[Tv(298), Tv(130)]),
    ((142, false, NAND2), &[Tv(298), Tv(69)]),
    ((148, false, AND2), &[Tv(244), Tv(147)]),
    ((154, false, AND2), &[Tv(244), Tv(329)]),
    ((164, false, AND2), &[Tv(298), Tv(147)]),
    ((169, false, AND2), &[Tv(287), Tv(331)]),
    ((182, false, AND2), &[Tv(298), Tv(53)]),
    ((188, false, AND2), &[Tv(287), Tv(22)]),
    ((190, false, NAND2), &[Tv(287), Tv(22)]),
    ((198, false, AND2), &[Tv(292), Tv(3)]),
    ((210, false, NAND2), &[Tv(287), Tv(57)]),
    ((231, false, NAND2), &[Tv(112), Tv(27)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 69] = [
    ((290, false, NAND2), &[Tv(284), Tv(289)]),
    ((294, false, AND2), &[Tv(291), Tv(293)]),
    ((308, false, NAND2), &[Tv(293), Tv(307)]),
    ((314, false, AND2), &[Tv(244), Tv(313)]),
    ((325, false, NAND2), &[Tv(289), Tv(307)]),
    ((330, false, AND2), &[Tv(298), Tv(329)]),
    ((332, false, AND2), &[Tv(292), Tv(331)]),
    ((0, false, AND2), &[Tv(298), Tv(313)]),
    ((1, false, AND2), &[Tv(293), Tv(315)]),
    ((5, false, INV), &[Tv(4)]),
    ((12, false, AND2), &[Tv(287), Tv(10)]),
    ((16, false, AND2), &[Tv(292), Tv(15)]),
    ((41, false, AND2), &[Tv(289), Tv(291)]),
    ((56, false, OR2), &[Tv(54), Tv(55)]),
    ((58, false, AND2), &[Tv(292), Tv(57)]),
    ((60, false, NAND2), &[Tv(298), Tv(30)]),
    ((71, false, OR2), &[Tv(68), Tv(70)]),
    ((77, false, INV), &[Tv(76)]),
    ((79, false, AND2), &[Tv(298), Tv(34)]),
    ((83, false, OR2), &[Tv(302), Tv(339)]),
    ((95, false, NAND2), &[Tv(93), Tv(94)]),
    ((98, false, OR2), &[Tv(54), Tv(76)]),
    ((106, false, NOR2), &[Tv(103), Tv(105)]),
    ((116, false, INV), &[Tv(115)]),
    ((117, false, OR2), &[Tv(114), Tv(115)]),
    ((121, false, AND2), &[Tv(289), Tv(315)]),
    ((128, false, NAND2), &[Tv(18), Tv(68)]),
    ((132, false, OR2), &[Tv(129), Tv(131)]),
    ((137, false, NAND2), &[Tv(135), Tv(136)]),
    ((139, false, NAND2), &[Tv(305), Tv(138)]),
    ((143, false, OR2), &[Tv(24), Tv(142)]),
    ((144, false, NAND2), &[Tv(31), Tv(111)]),
    ((149, false, NAND2), &[Tv(114), Tv(148)]),
    ((150, false, NAND2), &[Tv(302), Tv(36)]),
    ((155, false, NAND2), &[Tv(342), Tv(154)]),
    ((157, false, NAND2), &[Tv(103), Tv(135)]),
    ((159, false, NAND2), &[Tv(68), Tv(115)]),
    ((160, false, NAND2), &[Tv(23), Tv(54)]),
    ((163, false, NAND2), &[Tv(68), Tv(114)]),
    ((165, false, NAND2), &[Tv(318), Tv(164)]),
    ((168, false, NAND2), &[Tv(70), Tv(114)]),
    ((170, false, NAND2), &[Tv(46), Tv(169)]),
    ((175, false, NAND2), &[Tv(282), Tv(32)]),
    ((177, false, AND2), &[Arg(1, 4), Tv(27)]),
    ((183, false, OR2), &[Tv(92), Tv(182)]),
    ((187, false, NAND2), &[Tv(18), Tv(164)]),
    ((191, false, NAND2), &[Tv(154), Tv(188)]),
    ((193, false, NAND2), &[Tv(11), Tv(54)]),
    ((194, false, NAND2), &[Tv(342), Tv(182)]),
    ((197, false, NAND2), &[Tv(91), Tv(182)]),
    ((199, false, NAND2), &[Tv(35), Tv(198)]),
    ((202, false, NAND2), &[Tv(64), Tv(91)]),
    ((203, false, NAND2), &[Tv(305), Tv(148)]),
    ((216, false, OR2), &[Tv(46), Tv(92)]),
    ((221, false, AND2), &[Tv(324), Tv(190)]),
    ((229, false, NAND2), &[Tv(340), Tv(94)]),
    ((232, false, NAND2), &[Tv(131), Tv(231)]),
    ((237, false, NAND2), &[Tv(65), Tv(231)]),
    ((241, false, NAND2), &[Tv(142), Tv(231)]),
    ((243, false, OR2), &[Tv(64), Tv(76)]),
    ((247, false, NAND2), &[Tv(306), Tv(129)]),
    ((249, false, NAND2), &[Tv(24), Tv(210)]),
    ((254, false, OR2), &[Tv(55), Tv(138)]),
    ((257, false, OR2), &[Tv(35), Tv(154)]),
    ((260, false, AND2), &[Tv(94), Tv(100)]),
    ((265, false, NAND2), &[Tv(104), Tv(129)]),
    ((268, false, NAND2), &[Tv(282), Tv(231)]),
    ((271, false, OR2), &[Tv(148), Tv(154)]),
    ((273, false, NAND2), &[Tv(4), Tv(190)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 63] = [
    ((295, false, NAND2), &[Tv(291), Tv(293)]),
    ((333, false, INV), &[Tv(332)]),
    ((343, false, INV), &[Tv(342)]),
    ((13, false, AND2), &[Tv(277), Tv(298)]),
    ((17, false, NAND2), &[Tv(330), Tv(16)]),
    ((19, false, NAND2), &[Tv(0), Tv(18)]),
    ((25, false, NAND2), &[Tv(302), Tv(23)]),
    ((28, false, NAND2), &[Tv(342), Tv(27)]),
    ((33, false, NAND2), &[Tv(332), Tv(31)]),
    ((37, false, NAND2), &[Tv(35), Tv(36)]),
    ((42, false, INV), &[Tv(41)]),
    ((59, false, NAND2), &[Tv(56), Tv(58)]),
    ((61, false, OR2), &[Tv(308), Tv(60)]),
    ((66, false, NAND2), &[Tv(41), Tv(64)]),
    ((72, false, NAND2), &[Tv(332), Tv(71)]),
    ((78, false, NAND2), &[Tv(1), Tv(76)]),
    ((80, false, NAND2), &[Tv(294), Tv(79)]),
    ((82, false, NAND2), &[Tv(294), Tv(46)]),
    ((84, false, NAND2), &[Tv(16), Tv(83)]),
    ((88, false, OR2), &[Tv(290), Tv(65)]),
    ((96, false, NAND2), &[Tv(91), Tv(95)]),
    ((99, false, NAND2), &[Tv(332), Tv(98)]),
    ((107, false, OR2), &[Tv(100), Tv(106)]),
    ((113, false, NAND2), &[Tv(98), Tv(111)]),
    ((118, false, NAND2), &[Tv(92), Tv(117)]),
    ((120, false, OR2), &[Tv(325), Tv(77)]),
    ((122, false, NAND2), &[Tv(0), Tv(121)]),
    ((133, false, AND2), &[Tv(128), Tv(132)]),
    ((140, false, AND2), &[Tv(137), Tv(139)]),
    ((146, false, AND2), &[Tv(143), Tv(144)]),
    ((151, false, AND2), &[Tv(149), Tv(150)]),
    ((158, false, AND2), &[Tv(155), Tv(157)]),
    ((161, false, AND2), &[Tv(159), Tv(160)]),
    ((166, false, AND2), &[Tv(163), Tv(165)]),
    ((171, false, AND2), &[Tv(168), Tv(170)]),
    ((176, false, NAND2), &[Tv(12), Tv(175)]),
    ((179, false, NAND2), &[Tv(121), Tv(177)]),
    ((181, false, NAND2), &[Tv(41), Tv(164)]),
    ((184, false, NAND2), &[Tv(305), Tv(183)]),
    ((192, false, AND2), &[Tv(187), Tv(191)]),
    ((195, false, AND2), &[Tv(193), Tv(194)]),
    ((201, false, AND2), &[Tv(197), Tv(199)]),
    ((204, false, AND2), &[Tv(202), Tv(203)]),
    ((212, false, AND2), &[Tv(116), Tv(210)]),
    ((217, false, NAND2), &[Tv(57), Tv(216)]),
    ((218, false, NAND2), &[Tv(317), Tv(92)]),
    ((223, false, AND2), &[Tv(290), Tv(221)]),
    ((230, false, NAND2), &[Tv(136), Tv(229)]),
    ((234, false, NAND2), &[Tv(5), Tv(232)]),
    ((236, false, NAND2), &[Tv(1), Tv(54)]),
    ((238, false, NAND2), &[Tv(12), Tv(237)]),
    ((242, false, NAND2), &[Tv(305), Tv(241)]),
    ((245, false, NAND2), &[Tv(121), Tv(243)]),
    ((248, false, NAND2), &[Tv(177), Tv(247)]),
    ((250, false, NAND2), &[Tv(154), Tv(249)]),
    ((256, false, NAND2), &[Tv(332), Tv(254)]),
    ((258, false, NAND2), &[Tv(105), Tv(257)]),
    ((261, false, OR2), &[Tv(308), Tv(260)]),
    ((262, false, NAND2), &[Tv(339), Tv(121)]),
    ((267, false, NAND2), &[Tv(64), Tv(265)]),
    ((269, false, NAND2), &[Tv(111), Tv(268)]),
    ((272, false, NAND2), &[Tv(121), Tv(271)]),
    ((274, false, NAND2), &[Tv(314), Tv(273)]),
];

static LEVEL_6: [((usize, bool, CellType), &[GateInput]); 39] = [
    ((319, false, NAND2), &[Tv(292), Tv(317)]),
    ((326, false, NAND2), &[Tv(324), Tv(325)]),
    ((334, false, NAND2), &[Tv(325), Tv(333)]),
    ((344, false, NAND2), &[Tv(295), Tv(343)]),
    ((6, false, OR2), &[Tv(1), Tv(5)]),
    ((14, false, NAND2), &[Tv(12), Tv(13)]),
    ((20, false, AND2), &[Tv(17), Tv(19)]),
    ((29, false, AND2), &[Tv(25), Tv(28)]),
    ((38, false, AND2), &[Tv(33), Tv(37)]),
    ((43, false, NAND2), &[Tv(324), Tv(42)]),
    ((47, false, NAND2), &[Tv(290), Tv(343)]),
    ((62, false, AND2), &[Tv(59), Tv(61)]),
    ((73, false, AND2), &[Tv(66), Tv(72)]),
    ((81, false, AND2), &[Tv(78), Tv(80)]),
    ((85, false, AND2), &[Tv(82), Tv(84)]),
    ((97, false, AND2), &[Tv(88), Tv(96)]),
    ((108, false, AND2), &[Tv(99), Tv(107)]),
    ((119, false, AND2), &[Tv(113), Tv(118)]),
    ((124, false, AND2), &[Tv(120), Tv(122)]),
    ((141, false, AND2), &[Tv(133), Tv(140)]),
    ((152, false, AND2), &[Tv(146), Tv(151)]),
    ((162, false, AND2), &[Tv(158), Tv(161)]),
    ((172, false, AND2), &[Tv(166), Tv(171)]),
    ((180, false, AND2), &[Tv(176), Tv(179)]),
    ((185, false, AND2), &[Tv(181), Tv(184)]),
    ((196, false, AND2), &[Tv(192), Tv(195)]),
    ((205, false, AND2), &[Tv(201), Tv(204)]),
    ((213, false, AND2), &[Tv(4), Tv(212)]),
    ((219, false, NAND2), &[Tv(217), Tv(218)]),
    ((224, false, NAND2), &[Tv(308), Tv(223)]),
    ((225, false, AND2), &[Tv(298), Tv(63)]),
    ((235, false, AND2), &[Tv(230), Tv(234)]),
    ((239, false, AND2), &[Tv(236), Tv(238)]),
    ((246, false, AND2), &[Tv(242), Tv(245)]),
    ((251, false, AND2), &[Tv(248), Tv(250)]),
    ((259, false, AND2), &[Tv(256), Tv(258)]),
    ((263, false, AND2), &[Tv(261), Tv(262)]),
    ((270, false, AND2), &[Tv(267), Tv(269)]),
    ((275, false, AND2), &[Tv(272), Tv(274)]),
];

static LEVEL_7: [((usize, bool, CellType), &[GateInput]); 25] = [
    ((309, false, NAND2), &[Tv(306), Tv(308)]),
    ((320, false, NAND2), &[Tv(290), Tv(319)]),
    ((327, false, NAND2), &[Tv(314), Tv(326)]),
    ((335, false, NAND2), &[Tv(330), Tv(334)]),
    ((345, false, NAND2), &[Tv(339), Tv(344)]),
    ((7, false, NAND2), &[Tv(0), Tv(6)]),
    ((21, false, AND2), &[Tv(14), Tv(20)]),
    ((39, false, AND2), &[Tv(29), Tv(38)]),
    ((44, false, NAND2), &[Tv(35), Tv(43)]),
    ((48, false, NAND2), &[Tv(46), Tv(47)]),
    ((74, false, AND2), &[Tv(62), Tv(73)]),
    ((86, false, AND2), &[Tv(81), Tv(85)]),
    ((109, false, AND2), &[Tv(97), Tv(108)]),
    ((125, false, AND2), &[Tv(119), Tv(124)]),
    ((153, false, AND2), &[Tv(141), Tv(152)]),
    ((173, false, AND2), &[Tv(162), Tv(172)]),
    ((186, false, AND2), &[Tv(180), Tv(185)]),
    ((206, false, AND2), &[Tv(196), Tv(205)]),
    ((214, false, NAND2), &[Tv(308), Tv(213)]),
    ((220, false, NAND2), &[Tv(287), Tv(219)]),
    ((226, false, NAND2), &[Tv(224), Tv(225)]),
    ((240, false, AND2), &[Tv(235), Tv(239)]),
    ((252, false, AND2), &[Tv(246), Tv(251)]),
    ((264, false, AND2), &[Tv(259), Tv(263)]),
    ((276, false, AND2), &[Tv(270), Tv(275)]),
];

static LEVEL_8: [((usize, bool, CellType), &[GateInput]); 16] = [
    ((283, false, INV), &[Tv(282)]),
    ((296, false, NAND2), &[Tv(290), Tv(295)]),
    ((310, false, NAND2), &[Tv(302), Tv(309)]),
    ((321, false, NAND2), &[Tv(314), Tv(320)]),
    ((336, false, AND2), &[Tv(327), Tv(335)]),
    ((8, false, AND2), &[Tv(345), Tv(7)]),
    ((40, false, AND2), &[Tv(21), Tv(39)]),
    ((49, false, AND2), &[Tv(44), Tv(48)]),
    ((87, false, AND2), &[Tv(74), Tv(86)]),
    ((126, false, AND2), &[Tv(109), Tv(125)]),
    ((174, false, AND2), &[Tv(153), Tv(173)]),
    ((207, false, AND2), &[Tv(186), Tv(206)]),
    ((215, false, NAND2), &[Tv(148), Tv(214)]),
    ((227, false, AND2), &[Tv(220), Tv(226)]),
    ((253, false, AND2), &[Tv(240), Tv(252)]),
    ((278, false, AND2), &[Tv(264), Tv(276)]),
];

static LEVEL_9: [((usize, bool, CellType), &[GateInput]); 8] = [
    ((297, false, NAND2), &[Tv(283), Tv(296)]),
    ((322, false, AND2), &[Tv(310), Tv(321)]),
    ((9, false, AND2), &[Tv(336), Tv(8)]),
    ((50, false, AND2), &[Tv(40), Tv(49)]),
    ((127, false, AND2), &[Tv(87), Tv(126)]),
    ((208, false, AND2), &[Tv(174), Tv(207)]),
    ((228, false, AND2), &[Tv(215), Tv(227)]),
    ((279, false, AND2), &[Tv(253), Tv(278)]),
];

static LEVEL_10: [((usize, bool, CellType), &[GateInput]); 4] = [
    ((323, false, AND2), &[Tv(297), Tv(322)]),
    ((51, false, AND2), &[Tv(9), Tv(50)]),
    ((209, false, AND2), &[Tv(127), Tv(208)]),
    ((280, false, AND2), &[Tv(228), Tv(279)]),
];

static LEVEL_11: [((usize, bool, CellType), &[GateInput]); 2] = [
    ((52, false, AND2), &[Tv(323), Tv(51)]),
    ((281, false, AND2), &[Tv(209), Tv(280)]),
];

static LEVEL_12: [((usize, bool, CellType), &[GateInput]); 1] =
    [((0, true, AND2), &[Tv(52), Tv(281)])];

static PRUNE_12: [usize; 2] = [281, 52];

static PRUNE_4: [usize; 37] = [
    15, 331, 190, 94, 32, 10, 55, 129, 315, 11, 231, 169, 135, 34, 68, 198, 114, 142, 103, 289,
    244, 182, 131, 24, 188, 30, 340, 115, 70, 318, 329, 284, 307, 138, 104, 313, 93,
];

static PRUNE_1: [usize; 12] = [167, 145, 134, 123, 101, 90, 222, 200, 211, 189, 285, 178];

static PRUNE_7: [usize; 47] = [
    263, 246, 308, 43, 235, 196, 275, 213, 38, 224, 162, 326, 219, 185, 270, 225, 287, 152, 180,
    73, 259, 344, 124, 141, 62, 0, 85, 6, 29, 46, 108, 119, 339, 35, 97, 306, 334, 47, 239, 81,
    205, 251, 330, 14, 172, 319, 20,
];

static PRUNE_10: [usize; 8] = [297, 208, 50, 322, 228, 127, 279, 9];

static PRUNE_11: [usize; 4] = [280, 51, 209, 323];

static PRUNE_8: [usize; 31] = [
    252, 21, 314, 173, 320, 264, 44, 309, 39, 214, 276, 282, 220, 186, 327, 226, 153, 74, 125, 345,
    7, 86, 148, 109, 295, 335, 290, 48, 206, 302, 240,
];

static PRUNE_5: [usize; 84] = [
    60, 342, 77, 139, 229, 291, 150, 105, 128, 83, 111, 27, 100, 241, 117, 16, 202, 157, 95, 106,
    168, 247, 332, 163, 191, 293, 56, 197, 79, 175, 237, 265, 203, 23, 305, 243, 164, 254, 271,
    294, 91, 249, 170, 232, 260, 57, 277, 136, 221, 159, 187, 18, 41, 165, 58, 193, 210, 64, 137,
    199, 154, 92, 216, 177, 132, 194, 143, 98, 36, 160, 183, 121, 149, 273, 268, 144, 65, 155, 76,
    31, 257, 116, 54, 71,
];

static PRUNE_2: [usize; 12] = [26, 156, 337, 303, 299, 316, 300, 266, 328, 255, 233, 2];

static PRUNE_6: [usize; 72] = [
    325, 122, 184, 201, 88, 4, 66, 269, 218, 72, 258, 151, 179, 140, 78, 33, 343, 292, 230, 61, 84,
    146, 236, 298, 5, 107, 28, 242, 118, 158, 17, 96, 248, 192, 113, 333, 181, 12, 317, 238, 176,
    1, 63, 204, 80, 120, 272, 171, 250, 13, 261, 256, 19, 267, 245, 42, 262, 59, 324, 166, 25, 217,
    234, 195, 212, 274, 133, 161, 223, 82, 37, 99,
];

static PRUNE_9: [usize; 16] = [
    207, 49, 174, 253, 321, 310, 40, 215, 283, 227, 126, 278, 87, 8, 296, 336,
];

static PRUNE_3: [usize; 21] = [
    89, 286, 22, 112, 67, 338, 304, 45, 288, 147, 102, 130, 311, 69, 312, 75, 53, 301, 3, 341, 110,
];

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

    out[0].clone().unwrap()
}
