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
    ((2440, false, INV), &[Arg(1, 3)]),
    ((2444, false, INV), &[Arg(2, 3)]),
    ((198, false, NOR2), &[Arg(1, 2), Arg(1, 1)]),
    ((205, false, NOR2), &[Arg(2, 1), Arg(2, 2)]),
    ((524, false, XNOR2), &[Arg(2, 0), Arg(0, 49)]),
    ((525, false, XOR2), &[Arg(2, 1), Arg(0, 50)]),
    ((527, false, XOR2), &[Arg(1, 1), Arg(0, 42)]),
    ((528, false, XNOR2), &[Arg(1, 0), Arg(0, 41)]),
    ((571, false, XNOR2), &[Arg(2, 0), Arg(0, 8)]),
    ((572, false, XOR2), &[Arg(2, 1), Arg(0, 9)]),
    ((573, false, XOR2), &[Arg(1, 1), Arg(0, 1)]),
    ((574, false, XNOR2), &[Arg(1, 0), Arg(0, 0)]),
];

static LEVEL_1: [((usize, bool, CellType), &[GateInput]); 19] = [
    ((2441, false, INV), &[Arg(1, 4)]),
    ((2445, false, INV), &[Arg(2, 4)]),
    ((93, false, INV), &[Arg(0, 2)]),
    ((99, false, INV), &[Arg(0, 10)]),
    ((100, false, INV), &[Arg(0, 11)]),
    ((109, false, INV), &[Arg(0, 51)]),
    ((199, false, AND2), &[Tv(2440), Tv(198)]),
    ((206, false, AND2), &[Tv(2444), Tv(205)]),
    ((248, false, XOR2), &[Arg(1, 2), Arg(1, 1)]),
    ((250, false, XOR2), &[Arg(2, 1), Arg(2, 2)]),
    ((256, false, XNOR2), &[Tv(2444), Tv(205)]),
    ((526, false, AND2), &[Tv(524), Tv(525)]),
    ((529, false, AND2), &[Tv(527), Tv(528)]),
    ((575, false, AND2), &[Tv(572), Tv(574)]),
    ((576, false, AND2), &[Tv(571), Tv(573)]),
    ((614, false, XNOR2), &[Arg(2, 0), Arg(0, 90)]),
    ((615, false, XOR2), &[Arg(2, 1), Arg(0, 91)]),
    ((617, false, XOR2), &[Arg(1, 1), Arg(0, 83)]),
    ((618, false, XNOR2), &[Arg(1, 0), Arg(0, 82)]),
];

static LEVEL_2: [((usize, bool, CellType), &[GateInput]); 20] = [
    ((2442, false, INV), &[Arg(1, 5)]),
    ((2446, false, INV), &[Arg(2, 5)]),
    ((94, false, INV), &[Arg(0, 3)]),
    ((104, false, INV), &[Arg(0, 43)]),
    ((105, false, INV), &[Arg(0, 44)]),
    ((110, false, INV), &[Arg(0, 52)]),
    ((114, false, INV), &[Arg(0, 84)]),
    ((119, false, INV), &[Arg(0, 92)]),
    ((200, false, AND2), &[Tv(2441), Tv(199)]),
    ((207, false, AND2), &[Tv(2445), Tv(206)]),
    ((229, false, XNOR2), &[Arg(1, 3), Tv(198)]),
    ((255, false, XNOR2), &[Arg(2, 3), Tv(205)]),
    ((523, false, XNOR2), &[Tv(109), Tv(250)]),
    ((530, false, AND2), &[Tv(526), Tv(529)]),
    ((565, false, XNOR2), &[Tv(93), Tv(248)]),
    ((566, false, XNOR2), &[Tv(99), Tv(250)]),
    ((570, false, NAND2), &[Tv(100), Tv(256)]),
    ((577, false, AND2), &[Tv(575), Tv(576)]),
    ((616, false, AND2), &[Tv(614), Tv(615)]),
    ((619, false, AND2), &[Tv(617), Tv(618)]),
];

static LEVEL_3: [((usize, bool, CellType), &[GateInput]); 34] = [
    ((2447, false, INV), &[Arg(2, 6)]),
    ((95, false, INV), &[Arg(0, 4)]),
    ((101, false, INV), &[Arg(0, 12)]),
    ((106, false, INV), &[Arg(0, 45)]),
    ((111, false, INV), &[Arg(0, 53)]),
    ((115, false, INV), &[Arg(0, 85)]),
    ((120, false, INV), &[Arg(0, 93)]),
    ((123, false, INV), &[Arg(0, 125)]),
    ((128, false, INV), &[Arg(0, 133)]),
    ((201, false, AND2), &[Tv(2442), Tv(200)]),
    ((208, false, AND2), &[Tv(2446), Tv(207)]),
    ((223, false, XNOR2), &[Arg(2, 5), Tv(207)]),
    ((240, false, XNOR2), &[Arg(1, 4), Tv(199)]),
    ((246, false, XNOR2), &[Arg(2, 4), Tv(206)]),
    ((522, false, XNOR2), &[Tv(104), Tv(248)]),
    ((531, false, AND2), &[Tv(523), Tv(530)]),
    ((533, false, XNOR2), &[Tv(105), Tv(229)]),
    ((534, false, XNOR2), &[Tv(110), Tv(255)]),
    ((564, false, NAND2), &[Arg(0, 11), Tv(255)]),
    ((567, false, AND2), &[Tv(565), Tv(566)]),
    ((569, false, XNOR2), &[Tv(94), Tv(229)]),
    ((578, false, AND2), &[Tv(570), Tv(577)]),
    ((613, false, NAND2), &[Arg(0, 93), Tv(255)]),
    ((620, false, AND2), &[Tv(616), Tv(619)]),
    ((622, false, XNOR2), &[Tv(114), Tv(248)]),
    ((623, false, XNOR2), &[Tv(119), Tv(250)]),
    ((637, false, XNOR2), &[Arg(2, 0), Arg(0, 131)]),
    ((638, false, XOR2), &[Arg(2, 1), Arg(0, 132)]),
    ((639, false, XOR2), &[Arg(1, 1), Arg(0, 124)]),
    ((640, false, XNOR2), &[Arg(1, 0), Arg(0, 123)]),
    ((691, false, XOR2), &[Arg(2, 1), Arg(0, 173)]),
    ((692, false, XNOR2), &[Arg(2, 0), Arg(0, 172)]),
    ((694, false, XNOR2), &[Arg(1, 0), Arg(0, 164)]),
    ((695, false, XOR2), &[Arg(1, 1), Arg(0, 165)]),
];

static LEVEL_4: [((usize, bool, CellType), &[GateInput]); 36] = [
    ((2443, false, INV), &[Arg(1, 6)]),
    ((96, false, INV), &[Arg(0, 5)]),
    ((107, false, INV), &[Arg(0, 46)]),
    ((112, false, INV), &[Arg(0, 54)]),
    ((124, false, INV), &[Arg(0, 126)]),
    ((129, false, INV), &[Arg(0, 134)]),
    ((133, false, INV), &[Arg(0, 166)]),
    ((209, false, AND2), &[Tv(2447), Tv(208)]),
    ((219, false, XNOR2), &[Arg(1, 6), Tv(201)]),
    ((227, false, XNOR2), &[Arg(1, 5), Tv(200)]),
    ((532, false, AND2), &[Tv(522), Tv(531)]),
    ((535, false, AND2), &[Tv(533), Tv(534)]),
    ((537, false, XNOR2), &[Tv(111), Tv(246)]),
    ((538, false, XNOR2), &[Tv(106), Tv(240)]),
    ((556, false, XNOR2), &[Tv(95), Tv(240)]),
    ((557, false, OR2), &[Arg(0, 13), Tv(223)]),
    ((559, false, NAND2), &[Arg(0, 13), Tv(223)]),
    ((560, false, XNOR2), &[Tv(101), Tv(246)]),
    ((568, false, AND2), &[Tv(564), Tv(567)]),
    ((579, false, AND2), &[Tv(569), Tv(578)]),
    ((599, false, OR2), &[Arg(0, 94), Tv(246)]),
    ((600, false, OR2), &[Arg(0, 86), Tv(240)]),
    ((602, false, NAND2), &[Arg(0, 94), Tv(246)]),
    ((603, false, NAND2), &[Arg(0, 86), Tv(240)]),
    ((610, false, NAND2), &[Tv(120), Tv(256)]),
    ((611, false, XNOR2), &[Tv(115), Tv(229)]),
    ((621, false, AND2), &[Tv(613), Tv(620)]),
    ((624, false, AND2), &[Tv(622), Tv(623)]),
    ((641, false, AND2), &[Tv(637), Tv(639)]),
    ((642, false, AND2), &[Tv(638), Tv(640)]),
    ((651, false, XNOR2), &[Tv(123), Tv(248)]),
    ((652, false, XNOR2), &[Tv(128), Tv(250)]),
    ((693, false, AND2), &[Tv(691), Tv(692)]),
    ((696, false, AND2), &[Tv(694), Tv(695)]),
    ((780, false, XNOR2), &[Arg(1, 0), Arg(0, 246)]),
    ((781, false, XOR2), &[Arg(2, 1), Arg(0, 255)]),
];

static LEVEL_5: [((usize, bool, CellType), &[GateInput]); 46] = [
    ((20, false, INV), &[Arg(1, 0)]),
    ((27, false, INV), &[Arg(2, 0)]),
    ((97, false, INV), &[Arg(0, 6)]),
    ((102, false, INV), &[Arg(0, 14)]),
    ((113, false, INV), &[Arg(0, 55)]),
    ((116, false, INV), &[Arg(0, 87)]),
    ((121, false, INV), &[Arg(0, 95)]),
    ((125, false, INV), &[Arg(0, 127)]),
    ((134, false, INV), &[Arg(0, 167)]),
    ((138, false, INV), &[Arg(0, 174)]),
    ((139, false, INV), &[Arg(0, 175)]),
    ((144, false, INV), &[Arg(0, 207)]),
    ((147, false, INV), &[Arg(0, 215)]),
    ((172, false, INV), &[Arg(0, 328)]),
    ((177, false, INV), &[Arg(0, 336)]),
    ((202, false, AND2), &[Tv(2443), Tv(201)]),
    ((210, false, XNOR2), &[Arg(2, 7), Tv(209)]),
    ((217, false, XNOR2), &[Arg(2, 6), Tv(208)]),
    ((536, false, AND2), &[Tv(532), Tv(535)]),
    ((539, false, AND2), &[Tv(537), Tv(538)]),
    ((547, false, XNOR2), &[Tv(107), Tv(227)]),
    ((548, false, OR2), &[Arg(0, 47), Tv(219)]),
    ((550, false, NAND2), &[Arg(0, 47), Tv(219)]),
    ((551, false, XNOR2), &[Tv(112), Tv(223)]),
    ((558, false, AND2), &[Tv(556), Tv(557)]),
    ((561, false, AND2), &[Tv(559), Tv(560)]),
    ((563, false, XNOR2), &[Tv(96), Tv(227)]),
    ((580, false, AND2), &[Tv(568), Tv(579)]),
    ((601, false, AND2), &[Tv(599), Tv(600)]),
    ((604, false, AND2), &[Tv(602), Tv(603)]),
    ((612, false, AND2), &[Tv(610), Tv(611)]),
    ((625, false, AND2), &[Tv(621), Tv(624)]),
    ((643, false, AND2), &[Tv(641), Tv(642)]),
    ((644, false, XNOR2), &[Tv(124), Tv(229)]),
    ((650, false, OR2), &[Arg(0, 135), Tv(246)]),
    ((653, false, AND2), &[Tv(651), Tv(652)]),
    ((655, false, XNOR2), &[Tv(129), Tv(255)]),
    ((656, false, NAND2), &[Arg(0, 135), Tv(246)]),
    ((690, false, XNOR2), &[Tv(133), Tv(248)]),
    ((697, false, AND2), &[Tv(693), Tv(696)]),
    ((727, false, XNOR2), &[Arg(2, 0), Arg(0, 213)]),
    ((728, false, XOR2), &[Arg(2, 1), Arg(0, 214)]),
    ((729, false, XOR2), &[Arg(1, 1), Arg(0, 206)]),
    ((730, false, XNOR2), &[Arg(1, 0), Arg(0, 205)]),
    ((783, false, XOR2), &[Arg(1, 1), Arg(0, 247)]),
    ((784, false, AND2), &[Tv(780), Tv(781)]),
];

static LEVEL_6: [((usize, bool, CellType), &[GateInput]); 57] = [
    ((98, false, INV), &[Arg(0, 7)]),
    ((103, false, INV), &[Arg(0, 15)]),
    ((108, false, INV), &[Arg(0, 48)]),
    ((117, false, INV), &[Arg(0, 88)]),
    ((122, false, INV), &[Arg(0, 96)]),
    ((126, false, INV), &[Arg(0, 128)]),
    ((130, false, INV), &[Arg(0, 136)]),
    ((140, false, INV), &[Arg(0, 176)]),
    ((145, false, INV), &[Arg(0, 208)]),
    ((148, false, INV), &[Arg(0, 216)]),
    ((150, false, INV), &[Arg(0, 248)]),
    ((154, false, INV), &[Arg(0, 256)]),
    ((155, false, INV), &[Arg(0, 257)]),
    ((186, false, INV), &[Arg(0, 377)]),
    ((203, false, XNOR2), &[Arg(1, 7), Tv(202)]),
    ((521, false, OR2), &[Arg(0, 56), Tv(210)]),
    ((540, false, AND2), &[Tv(536), Tv(539)]),
    ((542, false, NAND2), &[Arg(0, 56), Tv(210)]),
    ((543, false, XNOR2), &[Tv(113), Tv(217)]),
    ((549, false, AND2), &[Tv(547), Tv(548)]),
    ((552, false, AND2), &[Tv(550), Tv(551)]),
    ((562, false, AND2), &[Tv(558), Tv(561)]),
    ((581, false, AND2), &[Tv(563), Tv(580)]),
    ((583, false, XNOR2), &[Tv(102), Tv(217)]),
    ((584, false, XNOR2), &[Tv(97), Tv(219)]),
    ((598, false, XNOR2), &[Tv(121), Tv(223)]),
    ((605, false, AND2), &[Tv(601), Tv(604)]),
    ((626, false, AND2), &[Tv(612), Tv(625)]),
    ((627, false, XNOR2), &[Tv(116), Tv(227)]),
    ((636, false, XNOR2), &[Tv(125), Tv(240)]),
    ((645, false, AND2), &[Tv(643), Tv(644)]),
    ((654, false, AND2), &[Tv(650), Tv(653)]),
    ((657, false, AND2), &[Tv(655), Tv(656)]),
    ((684, false, XNOR2), &[Tv(139), Tv(255)]),
    ((685, false, OR2), &[Arg(0, 168), Tv(240)]),
    ((689, false, XNOR2), &[Tv(138), Tv(250)]),
    ((698, false, AND2), &[Tv(690), Tv(697)]),
    ((700, false, NAND2), &[Arg(0, 168), Tv(240)]),
    ((701, false, XNOR2), &[Tv(134), Tv(229)]),
    ((731, false, AND2), &[Tv(727), Tv(729)]),
    ((732, false, AND2), &[Tv(728), Tv(730)]),
    ((743, false, XNOR2), &[Tv(144), Tv(248)]),
    ((744, false, XNOR2), &[Tv(147), Tv(250)]),
    ((782, false, XNOR2), &[Arg(2, 0), Arg(0, 254)]),
    ((785, false, AND2), &[Tv(783), Tv(784)]),
    ((810, false, XNOR2), &[Arg(1, 0), Arg(0, 287)]),
    ((815, false, XOR2), &[Arg(2, 1), Arg(0, 296)]),
    ((816, false, XNOR2), &[Arg(2, 0), Arg(0, 295)]),
    ((819, false, XOR2), &[Arg(1, 1), Arg(0, 288)]),
    ((869, false, NAND2), &[Tv(20), Arg(0, 328)]),
    ((870, false, NAND2), &[Arg(2, 0), Tv(177)]),
    ((872, false, NAND2), &[Tv(27), Arg(0, 336)]),
    ((873, false, OR2), &[Arg(2, 1), Arg(0, 337)]),
    ((876, false, OR2), &[Arg(1, 1), Arg(0, 329)]),
    ((877, false, NAND2), &[Arg(2, 1), Arg(0, 337)]),
    ((879, false, NAND2), &[Arg(1, 1), Arg(0, 329)]),
    ((880, false, NAND2), &[Arg(1, 0), Tv(172)]),
];

static LEVEL_7: [((usize, bool, CellType), &[GateInput]); 56] = [
    ((118, false, INV), &[Arg(0, 89)]),
    ((131, false, INV), &[Arg(0, 137)]),
    ((135, false, INV), &[Arg(0, 169)]),
    ((141, false, INV), &[Arg(0, 177)]),
    ((146, false, INV), &[Arg(0, 209)]),
    ((165, false, INV), &[Arg(0, 298)]),
    ((541, false, AND2), &[Tv(521), Tv(540)]),
    ((544, false, AND2), &[Tv(542), Tv(543)]),
    ((546, false, XNOR2), &[Tv(108), Tv(203)]),
    ((553, false, AND2), &[Tv(549), Tv(552)]),
    ((582, false, AND2), &[Tv(562), Tv(581)]),
    ((585, false, AND2), &[Tv(583), Tv(584)]),
    ((587, false, XNOR2), &[Tv(103), Tv(210)]),
    ((588, false, XNOR2), &[Tv(98), Tv(203)]),
    ((593, false, NAND2), &[Arg(0, 97), Tv(210)]),
    ((594, false, XNOR2), &[Tv(117), Tv(219)]),
    ((606, false, AND2), &[Tv(598), Tv(605)]),
    ((607, false, XNOR2), &[Tv(122), Tv(217)]),
    ((609, false, OR2), &[Arg(0, 97), Tv(210)]),
    ((628, false, AND2), &[Tv(626), Tv(627)]),
    ((646, false, AND2), &[Tv(636), Tv(645)]),
    ((647, false, XNOR2), &[Tv(126), Tv(227)]),
    ((649, false, OR2), &[Arg(0, 129), Tv(219)]),
    ((658, false, AND2), &[Tv(654), Tv(657)]),
    ((664, false, NAND2), &[Arg(0, 129), Tv(219)]),
    ((665, false, XNOR2), &[Tv(130), Tv(223)]),
    ((686, false, AND2), &[Tv(684), Tv(685)]),
    ((687, false, XNOR2), &[Tv(140), Tv(246)]),
    ((699, false, AND2), &[Tv(689), Tv(698)]),
    ((702, false, AND2), &[Tv(700), Tv(701)]),
    ((733, false, AND2), &[Tv(731), Tv(732)]),
    ((734, false, XNOR2), &[Tv(145), Tv(229)]),
    ((742, false, NAND2), &[Arg(0, 217), Tv(246)]),
    ((745, false, AND2), &[Tv(743), Tv(744)]),
    ((747, false, OR2), &[Arg(0, 217), Tv(246)]),
    ((748, false, XNOR2), &[Tv(148), Tv(255)]),
    ((776, false, NAND2), &[Tv(155), Tv(256)]),
    ((777, false, NAND2), &[Arg(0, 249), Tv(229)]),
    ((786, false, AND2), &[Tv(782), Tv(785)]),
    ((787, false, XNOR2), &[Tv(154), Tv(250)]),
    ((789, false, NAND2), &[Arg(0, 257), Tv(255)]),
    ((790, false, XNOR2), &[Tv(150), Tv(248)]),
    ((806, false, NAND2), &[Arg(0, 290), Tv(229)]),
    ((809, false, OR2), &[Arg(0, 290), Tv(229)]),
    ((817, false, AND2), &[Tv(815), Tv(816)]),
    ((820, false, AND2), &[Tv(810), Tv(819)]),
    ((871, false, AND2), &[Tv(869), Tv(870)]),
    ((874, false, AND2), &[Tv(872), Tv(873)]),
    ((878, false, AND2), &[Tv(876), Tv(877)]),
    ((881, false, AND2), &[Tv(879), Tv(880)]),
    ((911, false, NAND2), &[Arg(2, 1), Arg(0, 378)]),
    ((912, false, NAND2), &[Arg(1, 1), Arg(0, 370)]),
    ((916, false, OR2), &[Arg(1, 1), Arg(0, 370)]),
    ((917, false, NAND2), &[Arg(2, 0), Tv(186)]),
    ((919, false, NAND2), &[Tv(27), Arg(0, 377)]),
    ((920, false, OR2), &[Arg(2, 1), Arg(0, 378)]),
];

static LEVEL_8: [((usize, bool, CellType), &[GateInput]); 48] = [
    ((127, false, INV), &[Arg(0, 130)]),
    ((132, false, INV), &[Arg(0, 138)]),
    ((136, false, INV), &[Arg(0, 170)]),
    ((142, false, INV), &[Arg(0, 178)]),
    ((149, false, INV), &[Arg(0, 218)]),
    ((151, false, INV), &[Arg(0, 250)]),
    ((156, false, INV), &[Arg(0, 258)]),
    ((159, false, INV), &[Arg(0, 289)]),
    ((160, false, INV), &[Arg(0, 291)]),
    ((168, false, INV), &[Arg(0, 330)]),
    ((173, false, INV), &[Arg(0, 338)]),
    ((174, false, INV), &[Arg(0, 339)]),
    ((545, false, AND2), &[Tv(541), Tv(544)]),
    ((554, false, AND2), &[Tv(546), Tv(553)]),
    ((586, false, AND2), &[Tv(582), Tv(585)]),
    ((589, false, AND2), &[Tv(587), Tv(588)]),
    ((595, false, AND2), &[Tv(593), Tv(594)]),
    ((596, false, XNOR2), &[Tv(118), Tv(203)]),
    ((608, false, AND2), &[Tv(606), Tv(607)]),
    ((629, false, AND2), &[Tv(609), Tv(628)]),
    ((648, false, AND2), &[Tv(646), Tv(647)]),
    ((659, false, AND2), &[Tv(649), Tv(658)]),
    ((666, false, AND2), &[Tv(664), Tv(665)]),
    ((667, false, XNOR2), &[Tv(131), Tv(217)]),
    ((681, false, XNOR2), &[Tv(135), Tv(227)]),
    ((682, false, XNOR2), &[Tv(141), Tv(223)]),
    ((688, false, AND2), &[Tv(686), Tv(687)]),
    ((703, false, AND2), &[Tv(699), Tv(702)]),
    ((726, false, NAND2), &[Arg(0, 210), Tv(227)]),
    ((735, false, AND2), &[Tv(733), Tv(734)]),
    ((737, false, XNOR2), &[Tv(146), Tv(240)]),
    ((738, false, OR2), &[Arg(0, 210), Tv(227)]),
    ((746, false, AND2), &[Tv(742), Tv(745)]),
    ((749, false, AND2), &[Tv(747), Tv(748)]),
    ((775, false, OR2), &[Arg(0, 249), Tv(229)]),
    ((778, false, AND2), &[Tv(776), Tv(777)]),
    ((788, false, AND2), &[Tv(786), Tv(787)]),
    ((791, false, AND2), &[Tv(789), Tv(790)]),
    ((804, false, OR2), &[Arg(0, 299), Tv(246)]),
    ((805, false, XNOR2), &[Tv(165), Tv(255)]),
    ((818, false, AND2), &[Tv(806), Tv(817)]),
    ((821, false, AND2), &[Tv(809), Tv(820)]),
    ((875, false, AND2), &[Tv(871), Tv(874)]),
    ((882, false, AND2), &[Tv(878), Tv(881)]),
    ((913, false, AND2), &[Tv(911), Tv(912)]),
    ((914, false, XNOR2), &[Arg(1, 0), Arg(0, 369)]),
    ((918, false, AND2), &[Tv(916), Tv(917)]),
    ((921, false, AND2), &[Tv(919), Tv(920)]),
];

static LEVEL_9: [((usize, bool, CellType), &[GateInput]); 43] = [
    ((137, false, INV), &[Arg(0, 171)]),
    ((143, false, INV), &[Arg(0, 179)]),
    ((152, false, INV), &[Arg(0, 251)]),
    ((157, false, INV), &[Arg(0, 259)]),
    ((164, false, INV), &[Arg(0, 297)]),
    ((178, false, INV), &[Arg(0, 371)]),
    ((182, false, INV), &[Arg(0, 379)]),
    ((555, false, AND2), &[Tv(545), Tv(554)]),
    ((590, false, NAND2), &[Tv(586), Tv(589)]),
    ((597, false, AND2), &[Tv(595), Tv(596)]),
    ((630, false, AND2), &[Tv(608), Tv(629)]),
    ((660, false, AND2), &[Tv(648), Tv(659)]),
    ((661, false, XNOR2), &[Tv(127), Tv(203)]),
    ((663, false, XNOR2), &[Tv(132), Tv(210)]),
    ((668, false, AND2), &[Tv(666), Tv(667)]),
    ((678, false, XNOR2), &[Tv(142), Tv(217)]),
    ((679, false, XNOR2), &[Tv(136), Tv(219)]),
    ((683, false, AND2), &[Tv(681), Tv(682)]),
    ((704, false, AND2), &[Tv(688), Tv(703)]),
    ((719, false, NAND2), &[Arg(0, 219), Tv(217)]),
    ((720, false, OR2), &[Arg(0, 211), Tv(219)]),
    ((722, false, NAND2), &[Arg(0, 211), Tv(219)]),
    ((723, false, XNOR2), &[Tv(149), Tv(223)]),
    ((736, false, AND2), &[Tv(726), Tv(735)]),
    ((739, false, AND2), &[Tv(737), Tv(738)]),
    ((741, false, OR2), &[Arg(0, 219), Tv(217)]),
    ((750, false, AND2), &[Tv(746), Tv(749)]),
    ((772, false, XNOR2), &[Tv(156), Tv(246)]),
    ((773, false, XNOR2), &[Tv(151), Tv(240)]),
    ((779, false, AND2), &[Tv(775), Tv(778)]),
    ((792, false, AND2), &[Tv(788), Tv(791)]),
    ((808, false, XNOR2), &[Tv(159), Tv(248)]),
    ((812, false, AND2), &[Tv(804), Tv(805)]),
    ((813, false, XNOR2), &[Tv(160), Tv(240)]),
    ((822, false, AND2), &[Tv(818), Tv(821)]),
    ((861, false, NAND2), &[Arg(0, 339), Tv(255)]),
    ((862, false, OR2), &[Arg(0, 331), Tv(229)]),
    ((865, false, XNOR2), &[Tv(173), Tv(250)]),
    ((866, false, XNOR2), &[Tv(168), Tv(248)]),
    ((868, false, NAND2), &[Tv(174), Tv(256)]),
    ((883, false, AND2), &[Tv(875), Tv(882)]),
    ((915, false, AND2), &[Tv(913), Tv(914)]),
    ((922, false, AND2), &[Tv(918), Tv(921)]),
];

static LEVEL_10: [((usize, bool, CellType), &[GateInput]); 92] = [
    ((2448, false, INV), &[Arg(0, 98)]),
    ((2449, false, INV), &[Arg(0, 99)]),
    ((2450, false, INV), &[Arg(0, 100)]),
    ((2451, false, INV), &[Arg(0, 101)]),
    ((2452, false, INV), &[Arg(0, 102)]),
    ((2453, false, INV), &[Arg(0, 103)]),
    ((2454, false, INV), &[Arg(0, 104)]),
    ((2455, false, INV), &[Arg(0, 105)]),
    ((2496, false, INV), &[Arg(0, 106)]),
    ((2497, false, INV), &[Arg(0, 107)]),
    ((2498, false, INV), &[Arg(0, 108)]),
    ((2499, false, INV), &[Arg(0, 109)]),
    ((2500, false, INV), &[Arg(0, 110)]),
    ((2501, false, INV), &[Arg(0, 111)]),
    ((2502, false, INV), &[Arg(0, 112)]),
    ((2503, false, INV), &[Arg(0, 113)]),
    ((2544, false, INV), &[Arg(0, 114)]),
    ((2545, false, INV), &[Arg(0, 115)]),
    ((2546, false, INV), &[Arg(0, 116)]),
    ((2547, false, INV), &[Arg(0, 117)]),
    ((2548, false, INV), &[Arg(0, 118)]),
    ((2549, false, INV), &[Arg(0, 119)]),
    ((2550, false, INV), &[Arg(0, 120)]),
    ((2551, false, INV), &[Arg(0, 121)]),
    ((14, false, INV), &[Arg(0, 122)]),
    ((153, false, INV), &[Arg(0, 252)]),
    ((158, false, INV), &[Arg(0, 260)]),
    ((161, false, INV), &[Arg(0, 292)]),
    ((166, false, INV), &[Arg(0, 300)]),
    ((183, false, INV), &[Arg(0, 380)]),
    ((591, false, NOR2), &[Tv(555), Tv(590)]),
    ((631, false, AND2), &[Tv(597), Tv(630)]),
    ((632, false, NAND2), &[Tv(597), Tv(630)]),
    ((633, false, NAND2), &[Arg(0, 57), Tv(555)]),
    ((662, false, AND2), &[Tv(660), Tv(661)]),
    ((669, false, AND2), &[Tv(663), Tv(668)]),
    ((675, false, XNOR2), &[Tv(137), Tv(203)]),
    ((676, false, XNOR2), &[Tv(143), Tv(210)]),
    ((680, false, AND2), &[Tv(678), Tv(679)]),
    ((705, false, AND2), &[Tv(683), Tv(704)]),
    ((712, false, OR2), &[Arg(0, 212), Tv(203)]),
    ((713, false, OR2), &[Arg(0, 220), Tv(210)]),
    ((715, false, NAND2), &[Arg(0, 220), Tv(210)]),
    ((716, false, NAND2), &[Arg(0, 212), Tv(203)]),
    ((721, false, AND2), &[Tv(719), Tv(720)]),
    ((724, false, AND2), &[Tv(722), Tv(723)]),
    ((740, false, AND2), &[Tv(736), Tv(739)]),
    ((751, false, AND2), &[Tv(741), Tv(750)]),
    ((769, false, XNOR2), &[Tv(157), Tv(223)]),
    ((770, false, XNOR2), &[Tv(152), Tv(227)]),
    ((774, false, AND2), &[Tv(772), Tv(773)]),
    ((793, false, AND2), &[Tv(779), Tv(792)]),
    ((803, false, NAND2), &[Arg(0, 299), Tv(246)]),
    ((807, false, XNOR2), &[Tv(164), Tv(250)]),
    ((814, false, AND2), &[Tv(812), Tv(813)]),
    ((823, false, AND2), &[Tv(808), Tv(822)]),
    ((849, false, NAND2), &[Arg(0, 332), Tv(240)]),
    ((850, false, OR2), &[Arg(0, 340), Tv(246)]),
    ((852, false, NAND2), &[Arg(0, 340), Tv(246)]),
    ((853, false, OR2), &[Arg(0, 332), Tv(240)]),
    ((860, false, NAND2), &[Arg(0, 331), Tv(229)]),
    ((863, false, AND2), &[Tv(861), Tv(862)]),
    ((867, false, AND2), &[Tv(865), Tv(866)]),
    ((884, false, AND2), &[Tv(868), Tv(883)]),
    ((907, false, XNOR2), &[Tv(182), Tv(250)]),
    ((908, false, XNOR2), &[Tv(178), Tv(248)]),
    ((910, false, OR2), &[Arg(0, 372), Tv(229)]),
    ((923, false, AND2), &[Tv(915), Tv(922)]),
    ((1103, false, NAND2), &[Arg(0, 58), Tv(555)]),
    ((1159, false, NAND2), &[Arg(0, 59), Tv(555)]),
    ((1215, false, NAND2), &[Arg(0, 60), Tv(555)]),
    ((1271, false, NAND2), &[Arg(0, 61), Tv(555)]),
    ((1327, false, NAND2), &[Arg(0, 62), Tv(555)]),
    ((1383, false, NAND2), &[Arg(0, 63), Tv(555)]),
    ((1439, false, NAND2), &[Arg(0, 64), Tv(555)]),
    ((1495, false, NAND2), &[Arg(0, 65), Tv(555)]),
    ((1551, false, NAND2), &[Arg(0, 66), Tv(555)]),
    ((1607, false, NAND2), &[Arg(0, 67), Tv(555)]),
    ((1663, false, NAND2), &[Arg(0, 68), Tv(555)]),
    ((1719, false, NAND2), &[Arg(0, 69), Tv(555)]),
    ((1775, false, NAND2), &[Arg(0, 70), Tv(555)]),
    ((1831, false, NAND2), &[Arg(0, 71), Tv(555)]),
    ((1887, false, NAND2), &[Arg(0, 72), Tv(555)]),
    ((1943, false, NAND2), &[Arg(0, 73), Tv(555)]),
    ((1999, false, NAND2), &[Arg(0, 74), Tv(555)]),
    ((2055, false, NAND2), &[Arg(0, 75), Tv(555)]),
    ((2111, false, NAND2), &[Arg(0, 76), Tv(555)]),
    ((2167, false, NAND2), &[Arg(0, 77), Tv(555)]),
    ((2223, false, NAND2), &[Arg(0, 78), Tv(555)]),
    ((2279, false, NAND2), &[Arg(0, 79), Tv(555)]),
    ((2335, false, NAND2), &[Arg(0, 80), Tv(555)]),
    ((2391, false, NAND2), &[Arg(0, 81), Tv(555)]),
];

static LEVEL_11: [((usize, bool, CellType), &[GateInput]); 115] = [
    ((162, false, INV), &[Arg(0, 293)]),
    ((167, false, INV), &[Arg(0, 301)]),
    ((169, false, INV), &[Arg(0, 333)]),
    ((175, false, INV), &[Arg(0, 341)]),
    ((505, false, XNOR2), &[Arg(1, 0), Arg(0, 410)]),
    ((506, false, XOR2), &[Arg(2, 1), Arg(0, 419)]),
    ((507, false, XOR2), &[Arg(1, 1), Arg(0, 411)]),
    ((509, false, XNOR2), &[Arg(2, 0), Arg(0, 418)]),
    ((592, false, NAND2), &[Arg(0, 16), Tv(591)]),
    ((634, false, AND2), &[Tv(632), Tv(633)]),
    ((670, false, AND2), &[Tv(662), Tv(669)]),
    ((671, false, NAND2), &[Tv(662), Tv(669)]),
    ((672, false, NAND2), &[Tv(2448), Tv(631)]),
    ((677, false, AND2), &[Tv(675), Tv(676)]),
    ((706, false, AND2), &[Tv(680), Tv(705)]),
    ((714, false, AND2), &[Tv(712), Tv(713)]),
    ((717, false, AND2), &[Tv(715), Tv(716)]),
    ((725, false, AND2), &[Tv(721), Tv(724)]),
    ((752, false, AND2), &[Tv(740), Tv(751)]),
    ((759, false, OR2), &[Arg(0, 253), Tv(203)]),
    ((760, false, OR2), &[Arg(0, 261), Tv(210)]),
    ((762, false, NAND2), &[Arg(0, 261), Tv(210)]),
    ((763, false, NAND2), &[Arg(0, 253), Tv(203)]),
    ((766, false, XNOR2), &[Tv(153), Tv(219)]),
    ((767, false, XNOR2), &[Tv(158), Tv(217)]),
    ((771, false, AND2), &[Tv(769), Tv(770)]),
    ((794, false, AND2), &[Tv(774), Tv(793)]),
    ((811, false, AND2), &[Tv(803), Tv(807)]),
    ((824, false, AND2), &[Tv(814), Tv(823)]),
    ((832, false, XNOR2), &[Tv(161), Tv(227)]),
    ((833, false, XNOR2), &[Tv(166), Tv(223)]),
    ((851, false, AND2), &[Tv(849), Tv(850)]),
    ((854, false, AND2), &[Tv(852), Tv(853)]),
    ((864, false, AND2), &[Tv(860), Tv(863)]),
    ((885, false, AND2), &[Tv(867), Tv(884)]),
    ((897, false, NAND2), &[Arg(0, 381), Tv(246)]),
    ((898, false, NAND2), &[Arg(0, 373), Tv(240)]),
    ((900, false, OR2), &[Arg(0, 381), Tv(246)]),
    ((901, false, OR2), &[Arg(0, 373), Tv(240)]),
    ((904, false, NAND2), &[Arg(0, 372), Tv(229)]),
    ((905, false, XNOR2), &[Tv(183), Tv(255)]),
    ((909, false, AND2), &[Tv(907), Tv(908)]),
    ((924, false, AND2), &[Tv(910), Tv(923)]),
    ((1102, false, NAND2), &[Arg(0, 17), Tv(591)]),
    ((1104, false, AND2), &[Tv(632), Tv(1103)]),
    ((1106, false, NAND2), &[Tv(2449), Tv(631)]),
    ((1158, false, NAND2), &[Arg(0, 18), Tv(591)]),
    ((1160, false, AND2), &[Tv(632), Tv(1159)]),
    ((1162, false, NAND2), &[Tv(2450), Tv(631)]),
    ((1214, false, NAND2), &[Arg(0, 19), Tv(591)]),
    ((1216, false, AND2), &[Tv(632), Tv(1215)]),
    ((1218, false, NAND2), &[Tv(2451), Tv(631)]),
    ((1270, false, NAND2), &[Arg(0, 20), Tv(591)]),
    ((1272, false, AND2), &[Tv(632), Tv(1271)]),
    ((1274, false, NAND2), &[Tv(2452), Tv(631)]),
    ((1326, false, NAND2), &[Arg(0, 21), Tv(591)]),
    ((1328, false, AND2), &[Tv(632), Tv(1327)]),
    ((1330, false, NAND2), &[Tv(2453), Tv(631)]),
    ((1382, false, NAND2), &[Arg(0, 22), Tv(591)]),
    ((1384, false, AND2), &[Tv(632), Tv(1383)]),
    ((1386, false, NAND2), &[Tv(2454), Tv(631)]),
    ((1438, false, NAND2), &[Arg(0, 23), Tv(591)]),
    ((1440, false, AND2), &[Tv(632), Tv(1439)]),
    ((1442, false, NAND2), &[Tv(2455), Tv(631)]),
    ((1494, false, NAND2), &[Arg(0, 24), Tv(591)]),
    ((1496, false, AND2), &[Tv(632), Tv(1495)]),
    ((1498, false, NAND2), &[Tv(2496), Tv(631)]),
    ((1550, false, NAND2), &[Arg(0, 25), Tv(591)]),
    ((1552, false, AND2), &[Tv(632), Tv(1551)]),
    ((1554, false, NAND2), &[Tv(2497), Tv(631)]),
    ((1606, false, NAND2), &[Arg(0, 26), Tv(591)]),
    ((1608, false, AND2), &[Tv(632), Tv(1607)]),
    ((1610, false, NAND2), &[Tv(2498), Tv(631)]),
    ((1662, false, NAND2), &[Arg(0, 27), Tv(591)]),
    ((1664, false, AND2), &[Tv(632), Tv(1663)]),
    ((1666, false, NAND2), &[Tv(2499), Tv(631)]),
    ((1718, false, NAND2), &[Arg(0, 28), Tv(591)]),
    ((1720, false, AND2), &[Tv(632), Tv(1719)]),
    ((1722, false, NAND2), &[Tv(2500), Tv(631)]),
    ((1774, false, NAND2), &[Arg(0, 29), Tv(591)]),
    ((1776, false, AND2), &[Tv(632), Tv(1775)]),
    ((1778, false, NAND2), &[Tv(2501), Tv(631)]),
    ((1830, false, NAND2), &[Arg(0, 30), Tv(591)]),
    ((1832, false, AND2), &[Tv(632), Tv(1831)]),
    ((1834, false, NAND2), &[Tv(2502), Tv(631)]),
    ((1886, false, NAND2), &[Arg(0, 31), Tv(591)]),
    ((1888, false, AND2), &[Tv(632), Tv(1887)]),
    ((1890, false, NAND2), &[Tv(2503), Tv(631)]),
    ((1942, false, NAND2), &[Arg(0, 32), Tv(591)]),
    ((1944, false, AND2), &[Tv(632), Tv(1943)]),
    ((1946, false, NAND2), &[Tv(2544), Tv(631)]),
    ((1998, false, NAND2), &[Arg(0, 33), Tv(591)]),
    ((2000, false, AND2), &[Tv(632), Tv(1999)]),
    ((2002, false, NAND2), &[Tv(2545), Tv(631)]),
    ((2054, false, NAND2), &[Arg(0, 34), Tv(591)]),
    ((2056, false, AND2), &[Tv(632), Tv(2055)]),
    ((2058, false, NAND2), &[Tv(2546), Tv(631)]),
    ((2110, false, NAND2), &[Arg(0, 35), Tv(591)]),
    ((2112, false, AND2), &[Tv(632), Tv(2111)]),
    ((2114, false, NAND2), &[Tv(2547), Tv(631)]),
    ((2166, false, NAND2), &[Arg(0, 36), Tv(591)]),
    ((2168, false, AND2), &[Tv(632), Tv(2167)]),
    ((2170, false, NAND2), &[Tv(2548), Tv(631)]),
    ((2222, false, NAND2), &[Arg(0, 37), Tv(591)]),
    ((2224, false, AND2), &[Tv(632), Tv(2223)]),
    ((2226, false, NAND2), &[Tv(2549), Tv(631)]),
    ((2278, false, NAND2), &[Arg(0, 38), Tv(591)]),
    ((2280, false, AND2), &[Tv(632), Tv(2279)]),
    ((2282, false, NAND2), &[Tv(2550), Tv(631)]),
    ((2334, false, NAND2), &[Arg(0, 39), Tv(591)]),
    ((2336, false, AND2), &[Tv(632), Tv(2335)]),
    ((2338, false, NAND2), &[Tv(2551), Tv(631)]),
    ((2390, false, NAND2), &[Arg(0, 40), Tv(591)]),
    ((2392, false, AND2), &[Tv(632), Tv(2391)]),
    ((2394, false, NAND2), &[Tv(14), Tv(631)]),
];

static LEVEL_12: [((usize, bool, CellType), &[GateInput]); 132] = [
    ((2456, false, INV), &[Arg(0, 180)]),
    ((2457, false, INV), &[Arg(0, 181)]),
    ((2458, false, INV), &[Arg(0, 182)]),
    ((2459, false, INV), &[Arg(0, 183)]),
    ((2460, false, INV), &[Arg(0, 184)]),
    ((2461, false, INV), &[Arg(0, 185)]),
    ((2462, false, INV), &[Arg(0, 186)]),
    ((2463, false, INV), &[Arg(0, 187)]),
    ((2504, false, INV), &[Arg(0, 188)]),
    ((2505, false, INV), &[Arg(0, 189)]),
    ((2506, false, INV), &[Arg(0, 190)]),
    ((2507, false, INV), &[Arg(0, 191)]),
    ((2508, false, INV), &[Arg(0, 192)]),
    ((2509, false, INV), &[Arg(0, 193)]),
    ((2510, false, INV), &[Arg(0, 194)]),
    ((2511, false, INV), &[Arg(0, 195)]),
    ((2552, false, INV), &[Arg(0, 196)]),
    ((2553, false, INV), &[Arg(0, 197)]),
    ((2554, false, INV), &[Arg(0, 198)]),
    ((2555, false, INV), &[Arg(0, 199)]),
    ((2556, false, INV), &[Arg(0, 200)]),
    ((2557, false, INV), &[Arg(0, 201)]),
    ((2558, false, INV), &[Arg(0, 202)]),
    ((2559, false, INV), &[Arg(0, 203)]),
    ((15, false, INV), &[Arg(0, 204)]),
    ((163, false, INV), &[Arg(0, 294)]),
    ((170, false, INV), &[Arg(0, 334)]),
    ((176, false, INV), &[Arg(0, 342)]),
    ((179, false, INV), &[Arg(0, 374)]),
    ((184, false, INV), &[Arg(0, 382)]),
    ((187, false, INV), &[Arg(0, 412)]),
    ((189, false, INV), &[Arg(0, 420)]),
    ((190, false, INV), &[Arg(0, 421)]),
    ((508, false, AND2), &[Tv(506), Tv(507)]),
    ((510, false, AND2), &[Tv(505), Tv(509)]),
    ((635, false, NAND2), &[Tv(592), Tv(634)]),
    ((673, false, AND2), &[Tv(671), Tv(672)]),
    ((707, false, AND2), &[Tv(677), Tv(706)]),
    ((708, false, NAND2), &[Tv(677), Tv(706)]),
    ((709, false, NAND2), &[Arg(0, 139), Tv(670)]),
    ((718, false, AND2), &[Tv(714), Tv(717)]),
    ((753, false, AND2), &[Tv(725), Tv(752)]),
    ((761, false, AND2), &[Tv(759), Tv(760)]),
    ((764, false, AND2), &[Tv(762), Tv(763)]),
    ((768, false, AND2), &[Tv(766), Tv(767)]),
    ((795, false, AND2), &[Tv(771), Tv(794)]),
    ((802, false, NAND2), &[Arg(0, 302), Tv(210)]),
    ((825, false, AND2), &[Tv(811), Tv(824)]),
    ((827, false, XNOR2), &[Tv(167), Tv(217)]),
    ((828, false, OR2), &[Arg(0, 302), Tv(210)]),
    ((834, false, AND2), &[Tv(832), Tv(833)]),
    ((835, false, XNOR2), &[Tv(162), Tv(219)]),
    ((848, false, XNOR2), &[Tv(169), Tv(227)]),
    ((855, false, AND2), &[Tv(851), Tv(854)]),
    ((886, false, AND2), &[Tv(864), Tv(885)]),
    ((887, false, XNOR2), &[Tv(175), Tv(223)]),
    ((899, false, AND2), &[Tv(897), Tv(898)]),
    ((902, false, AND2), &[Tv(900), Tv(901)]),
    ((906, false, AND2), &[Tv(904), Tv(905)]),
    ((925, false, AND2), &[Tv(909), Tv(924)]),
    ((1105, false, NAND2), &[Tv(1102), Tv(1104)]),
    ((1107, false, AND2), &[Tv(671), Tv(1106)]),
    ((1109, false, NAND2), &[Arg(0, 140), Tv(670)]),
    ((1161, false, NAND2), &[Tv(1158), Tv(1160)]),
    ((1163, false, AND2), &[Tv(671), Tv(1162)]),
    ((1165, false, NAND2), &[Arg(0, 141), Tv(670)]),
    ((1217, false, NAND2), &[Tv(1214), Tv(1216)]),
    ((1219, false, AND2), &[Tv(671), Tv(1218)]),
    ((1221, false, NAND2), &[Arg(0, 142), Tv(670)]),
    ((1273, false, NAND2), &[Tv(1270), Tv(1272)]),
    ((1275, false, AND2), &[Tv(671), Tv(1274)]),
    ((1277, false, NAND2), &[Arg(0, 143), Tv(670)]),
    ((1329, false, NAND2), &[Tv(1326), Tv(1328)]),
    ((1331, false, AND2), &[Tv(671), Tv(1330)]),
    ((1333, false, NAND2), &[Arg(0, 144), Tv(670)]),
    ((1385, false, NAND2), &[Tv(1382), Tv(1384)]),
    ((1387, false, AND2), &[Tv(671), Tv(1386)]),
    ((1389, false, NAND2), &[Arg(0, 145), Tv(670)]),
    ((1441, false, NAND2), &[Tv(1438), Tv(1440)]),
    ((1443, false, AND2), &[Tv(671), Tv(1442)]),
    ((1445, false, NAND2), &[Arg(0, 146), Tv(670)]),
    ((1497, false, NAND2), &[Tv(1494), Tv(1496)]),
    ((1499, false, AND2), &[Tv(671), Tv(1498)]),
    ((1501, false, NAND2), &[Arg(0, 147), Tv(670)]),
    ((1553, false, NAND2), &[Tv(1550), Tv(1552)]),
    ((1555, false, AND2), &[Tv(671), Tv(1554)]),
    ((1557, false, NAND2), &[Arg(0, 148), Tv(670)]),
    ((1609, false, NAND2), &[Tv(1606), Tv(1608)]),
    ((1611, false, AND2), &[Tv(671), Tv(1610)]),
    ((1613, false, NAND2), &[Arg(0, 149), Tv(670)]),
    ((1665, false, NAND2), &[Tv(1662), Tv(1664)]),
    ((1667, false, AND2), &[Tv(671), Tv(1666)]),
    ((1669, false, NAND2), &[Arg(0, 150), Tv(670)]),
    ((1721, false, NAND2), &[Tv(1718), Tv(1720)]),
    ((1723, false, AND2), &[Tv(671), Tv(1722)]),
    ((1725, false, NAND2), &[Arg(0, 151), Tv(670)]),
    ((1777, false, NAND2), &[Tv(1774), Tv(1776)]),
    ((1779, false, AND2), &[Tv(671), Tv(1778)]),
    ((1781, false, NAND2), &[Arg(0, 152), Tv(670)]),
    ((1833, false, NAND2), &[Tv(1830), Tv(1832)]),
    ((1835, false, AND2), &[Tv(671), Tv(1834)]),
    ((1837, false, NAND2), &[Arg(0, 153), Tv(670)]),
    ((1889, false, NAND2), &[Tv(1886), Tv(1888)]),
    ((1891, false, AND2), &[Tv(671), Tv(1890)]),
    ((1893, false, NAND2), &[Arg(0, 154), Tv(670)]),
    ((1945, false, NAND2), &[Tv(1942), Tv(1944)]),
    ((1947, false, AND2), &[Tv(671), Tv(1946)]),
    ((1949, false, NAND2), &[Arg(0, 155), Tv(670)]),
    ((2001, false, NAND2), &[Tv(1998), Tv(2000)]),
    ((2003, false, AND2), &[Tv(671), Tv(2002)]),
    ((2005, false, NAND2), &[Arg(0, 156), Tv(670)]),
    ((2057, false, NAND2), &[Tv(2054), Tv(2056)]),
    ((2059, false, AND2), &[Tv(671), Tv(2058)]),
    ((2061, false, NAND2), &[Arg(0, 157), Tv(670)]),
    ((2113, false, NAND2), &[Tv(2110), Tv(2112)]),
    ((2115, false, AND2), &[Tv(671), Tv(2114)]),
    ((2117, false, NAND2), &[Arg(0, 158), Tv(670)]),
    ((2169, false, NAND2), &[Tv(2166), Tv(2168)]),
    ((2171, false, AND2), &[Tv(671), Tv(2170)]),
    ((2173, false, NAND2), &[Arg(0, 159), Tv(670)]),
    ((2225, false, NAND2), &[Tv(2222), Tv(2224)]),
    ((2227, false, AND2), &[Tv(671), Tv(2226)]),
    ((2229, false, NAND2), &[Arg(0, 160), Tv(670)]),
    ((2281, false, NAND2), &[Tv(2278), Tv(2280)]),
    ((2283, false, AND2), &[Tv(671), Tv(2282)]),
    ((2285, false, NAND2), &[Arg(0, 161), Tv(670)]),
    ((2337, false, NAND2), &[Tv(2334), Tv(2336)]),
    ((2339, false, AND2), &[Tv(671), Tv(2338)]),
    ((2341, false, NAND2), &[Arg(0, 162), Tv(670)]),
    ((2393, false, NAND2), &[Tv(2390), Tv(2392)]),
    ((2395, false, AND2), &[Tv(671), Tv(2394)]),
    ((2397, false, NAND2), &[Arg(0, 163), Tv(670)]),
];

static LEVEL_13: [((usize, bool, CellType), &[GateInput]); 102] = [
    ((171, false, INV), &[Arg(0, 335)]),
    ((180, false, INV), &[Arg(0, 375)]),
    ((185, false, INV), &[Arg(0, 383)]),
    ((497, false, NAND2), &[Arg(0, 421), Tv(255)]),
    ((498, false, OR2), &[Arg(0, 413), Tv(229)]),
    ((501, false, XNOR2), &[Tv(187), Tv(248)]),
    ((502, false, XNOR2), &[Tv(189), Tv(250)]),
    ((504, false, NAND2), &[Tv(190), Tv(256)]),
    ((511, false, AND2), &[Tv(508), Tv(510)]),
    ((674, false, NAND2), &[Tv(635), Tv(673)]),
    ((710, false, AND2), &[Tv(708), Tv(709)]),
    ((754, false, AND2), &[Tv(718), Tv(753)]),
    ((755, false, NAND2), &[Tv(718), Tv(753)]),
    ((756, false, NAND2), &[Tv(2456), Tv(707)]),
    ((765, false, AND2), &[Tv(761), Tv(764)]),
    ((796, false, AND2), &[Tv(768), Tv(795)]),
    ((826, false, AND2), &[Tv(802), Tv(825)]),
    ((829, false, AND2), &[Tv(827), Tv(828)]),
    ((831, false, XNOR2), &[Tv(163), Tv(203)]),
    ((836, false, AND2), &[Tv(834), Tv(835)]),
    ((843, false, NAND2), &[Arg(0, 343), Tv(210)]),
    ((844, false, XNOR2), &[Tv(170), Tv(219)]),
    ((856, false, AND2), &[Tv(848), Tv(855)]),
    ((857, false, XNOR2), &[Tv(176), Tv(217)]),
    ((859, false, OR2), &[Arg(0, 343), Tv(210)]),
    ((888, false, AND2), &[Tv(886), Tv(887)]),
    ((903, false, AND2), &[Tv(899), Tv(902)]),
    ((926, false, AND2), &[Tv(906), Tv(925)]),
    ((934, false, XNOR2), &[Tv(179), Tv(227)]),
    ((935, false, XNOR2), &[Tv(184), Tv(223)]),
    ((1108, false, NAND2), &[Tv(1105), Tv(1107)]),
    ((1110, false, AND2), &[Tv(708), Tv(1109)]),
    ((1112, false, NAND2), &[Tv(2457), Tv(707)]),
    ((1164, false, NAND2), &[Tv(1161), Tv(1163)]),
    ((1166, false, AND2), &[Tv(708), Tv(1165)]),
    ((1168, false, NAND2), &[Tv(2458), Tv(707)]),
    ((1220, false, NAND2), &[Tv(1217), Tv(1219)]),
    ((1222, false, AND2), &[Tv(708), Tv(1221)]),
    ((1224, false, NAND2), &[Tv(2459), Tv(707)]),
    ((1276, false, NAND2), &[Tv(1273), Tv(1275)]),
    ((1278, false, AND2), &[Tv(708), Tv(1277)]),
    ((1280, false, NAND2), &[Tv(2460), Tv(707)]),
    ((1332, false, NAND2), &[Tv(1329), Tv(1331)]),
    ((1334, false, AND2), &[Tv(708), Tv(1333)]),
    ((1336, false, NAND2), &[Tv(2461), Tv(707)]),
    ((1388, false, NAND2), &[Tv(1385), Tv(1387)]),
    ((1390, false, AND2), &[Tv(708), Tv(1389)]),
    ((1392, false, NAND2), &[Tv(2462), Tv(707)]),
    ((1444, false, NAND2), &[Tv(1441), Tv(1443)]),
    ((1446, false, AND2), &[Tv(708), Tv(1445)]),
    ((1448, false, NAND2), &[Tv(2463), Tv(707)]),
    ((1500, false, NAND2), &[Tv(1497), Tv(1499)]),
    ((1502, false, AND2), &[Tv(708), Tv(1501)]),
    ((1504, false, NAND2), &[Tv(2504), Tv(707)]),
    ((1556, false, NAND2), &[Tv(1553), Tv(1555)]),
    ((1558, false, AND2), &[Tv(708), Tv(1557)]),
    ((1560, false, NAND2), &[Tv(2505), Tv(707)]),
    ((1612, false, NAND2), &[Tv(1609), Tv(1611)]),
    ((1614, false, AND2), &[Tv(708), Tv(1613)]),
    ((1616, false, NAND2), &[Tv(2506), Tv(707)]),
    ((1668, false, NAND2), &[Tv(1665), Tv(1667)]),
    ((1670, false, AND2), &[Tv(708), Tv(1669)]),
    ((1672, false, NAND2), &[Tv(2507), Tv(707)]),
    ((1724, false, NAND2), &[Tv(1721), Tv(1723)]),
    ((1726, false, AND2), &[Tv(708), Tv(1725)]),
    ((1728, false, NAND2), &[Tv(2508), Tv(707)]),
    ((1780, false, NAND2), &[Tv(1777), Tv(1779)]),
    ((1782, false, AND2), &[Tv(708), Tv(1781)]),
    ((1784, false, NAND2), &[Tv(2509), Tv(707)]),
    ((1836, false, NAND2), &[Tv(1833), Tv(1835)]),
    ((1838, false, AND2), &[Tv(708), Tv(1837)]),
    ((1840, false, NAND2), &[Tv(2510), Tv(707)]),
    ((1892, false, NAND2), &[Tv(1889), Tv(1891)]),
    ((1894, false, AND2), &[Tv(708), Tv(1893)]),
    ((1896, false, NAND2), &[Tv(2511), Tv(707)]),
    ((1948, false, NAND2), &[Tv(1945), Tv(1947)]),
    ((1950, false, AND2), &[Tv(708), Tv(1949)]),
    ((1952, false, NAND2), &[Tv(2552), Tv(707)]),
    ((2004, false, NAND2), &[Tv(2001), Tv(2003)]),
    ((2006, false, AND2), &[Tv(708), Tv(2005)]),
    ((2008, false, NAND2), &[Tv(2553), Tv(707)]),
    ((2060, false, NAND2), &[Tv(2057), Tv(2059)]),
    ((2062, false, AND2), &[Tv(708), Tv(2061)]),
    ((2064, false, NAND2), &[Tv(2554), Tv(707)]),
    ((2116, false, NAND2), &[Tv(2113), Tv(2115)]),
    ((2118, false, AND2), &[Tv(708), Tv(2117)]),
    ((2120, false, NAND2), &[Tv(2555), Tv(707)]),
    ((2172, false, NAND2), &[Tv(2169), Tv(2171)]),
    ((2174, false, AND2), &[Tv(708), Tv(2173)]),
    ((2176, false, NAND2), &[Tv(2556), Tv(707)]),
    ((2228, false, NAND2), &[Tv(2225), Tv(2227)]),
    ((2230, false, AND2), &[Tv(708), Tv(2229)]),
    ((2232, false, NAND2), &[Tv(2557), Tv(707)]),
    ((2284, false, NAND2), &[Tv(2281), Tv(2283)]),
    ((2286, false, AND2), &[Tv(708), Tv(2285)]),
    ((2288, false, NAND2), &[Tv(2558), Tv(707)]),
    ((2340, false, NAND2), &[Tv(2337), Tv(2339)]),
    ((2342, false, AND2), &[Tv(708), Tv(2341)]),
    ((2344, false, NAND2), &[Tv(2559), Tv(707)]),
    ((2396, false, NAND2), &[Tv(2393), Tv(2395)]),
    ((2398, false, AND2), &[Tv(708), Tv(2397)]),
    ((2400, false, NAND2), &[Tv(15), Tv(707)]),
];

static LEVEL_14: [((usize, bool, CellType), &[GateInput]); 129] = [
    ((2464, false, INV), &[Arg(0, 262)]),
    ((2465, false, INV), &[Arg(0, 263)]),
    ((2466, false, INV), &[Arg(0, 264)]),
    ((2467, false, INV), &[Arg(0, 265)]),
    ((2468, false, INV), &[Arg(0, 266)]),
    ((2469, false, INV), &[Arg(0, 267)]),
    ((2470, false, INV), &[Arg(0, 268)]),
    ((2471, false, INV), &[Arg(0, 269)]),
    ((2512, false, INV), &[Arg(0, 270)]),
    ((2513, false, INV), &[Arg(0, 271)]),
    ((2514, false, INV), &[Arg(0, 272)]),
    ((2515, false, INV), &[Arg(0, 273)]),
    ((2516, false, INV), &[Arg(0, 274)]),
    ((2517, false, INV), &[Arg(0, 275)]),
    ((2518, false, INV), &[Arg(0, 276)]),
    ((2519, false, INV), &[Arg(0, 277)]),
    ((2560, false, INV), &[Arg(0, 278)]),
    ((2561, false, INV), &[Arg(0, 279)]),
    ((2562, false, INV), &[Arg(0, 280)]),
    ((2563, false, INV), &[Arg(0, 281)]),
    ((2564, false, INV), &[Arg(0, 282)]),
    ((2565, false, INV), &[Arg(0, 283)]),
    ((2566, false, INV), &[Arg(0, 284)]),
    ((2567, false, INV), &[Arg(0, 285)]),
    ((16, false, INV), &[Arg(0, 286)]),
    ((21, false, INV), &[Arg(0, 461)]),
    ((181, false, INV), &[Arg(0, 376)]),
    ((192, false, INV), &[Arg(0, 453)]),
    ((446, false, XNOR2), &[Arg(1, 0), Arg(0, 451)]),
    ((447, false, XOR2), &[Arg(1, 1), Arg(0, 452)]),
    ((448, false, XNOR2), &[Arg(2, 0), Arg(0, 459)]),
    ((450, false, XOR2), &[Arg(2, 1), Arg(0, 460)]),
    ((489, false, NAND2), &[Arg(0, 414), Tv(240)]),
    ((490, false, OR2), &[Arg(0, 422), Tv(246)]),
    ((492, false, OR2), &[Arg(0, 414), Tv(240)]),
    ((493, false, NAND2), &[Arg(0, 422), Tv(246)]),
    ((496, false, NAND2), &[Arg(0, 413), Tv(229)]),
    ((499, false, AND2), &[Tv(497), Tv(498)]),
    ((503, false, AND2), &[Tv(501), Tv(502)]),
    ((512, false, AND2), &[Tv(504), Tv(511)]),
    ((711, false, NAND2), &[Tv(674), Tv(710)]),
    ((757, false, AND2), &[Tv(755), Tv(756)]),
    ((797, false, AND2), &[Tv(765), Tv(796)]),
    ((798, false, NAND2), &[Tv(765), Tv(796)]),
    ((799, false, NAND2), &[Arg(0, 221), Tv(754)]),
    ((830, false, AND2), &[Tv(826), Tv(829)]),
    ((837, false, AND2), &[Tv(831), Tv(836)]),
    ((845, false, AND2), &[Tv(843), Tv(844)]),
    ((846, false, XNOR2), &[Tv(171), Tv(203)]),
    ((858, false, AND2), &[Tv(856), Tv(857)]),
    ((889, false, AND2), &[Tv(859), Tv(888)]),
    ((896, false, NAND2), &[Arg(0, 384), Tv(210)]),
    ((927, false, AND2), &[Tv(903), Tv(926)]),
    ((929, false, XNOR2), &[Tv(185), Tv(217)]),
    ((930, false, OR2), &[Arg(0, 384), Tv(210)]),
    ((936, false, AND2), &[Tv(934), Tv(935)]),
    ((937, false, XNOR2), &[Tv(180), Tv(219)]),
    ((1111, false, NAND2), &[Tv(1108), Tv(1110)]),
    ((1113, false, AND2), &[Tv(755), Tv(1112)]),
    ((1115, false, NAND2), &[Arg(0, 222), Tv(754)]),
    ((1167, false, NAND2), &[Tv(1164), Tv(1166)]),
    ((1169, false, AND2), &[Tv(755), Tv(1168)]),
    ((1171, false, NAND2), &[Arg(0, 223), Tv(754)]),
    ((1223, false, NAND2), &[Tv(1220), Tv(1222)]),
    ((1225, false, AND2), &[Tv(755), Tv(1224)]),
    ((1227, false, NAND2), &[Arg(0, 224), Tv(754)]),
    ((1279, false, NAND2), &[Tv(1276), Tv(1278)]),
    ((1281, false, AND2), &[Tv(755), Tv(1280)]),
    ((1283, false, NAND2), &[Arg(0, 225), Tv(754)]),
    ((1335, false, NAND2), &[Tv(1332), Tv(1334)]),
    ((1337, false, AND2), &[Tv(755), Tv(1336)]),
    ((1339, false, NAND2), &[Arg(0, 226), Tv(754)]),
    ((1391, false, NAND2), &[Tv(1388), Tv(1390)]),
    ((1393, false, AND2), &[Tv(755), Tv(1392)]),
    ((1395, false, NAND2), &[Arg(0, 227), Tv(754)]),
    ((1447, false, NAND2), &[Tv(1444), Tv(1446)]),
    ((1449, false, AND2), &[Tv(755), Tv(1448)]),
    ((1451, false, NAND2), &[Arg(0, 228), Tv(754)]),
    ((1503, false, NAND2), &[Tv(1500), Tv(1502)]),
    ((1505, false, AND2), &[Tv(755), Tv(1504)]),
    ((1507, false, NAND2), &[Arg(0, 229), Tv(754)]),
    ((1559, false, NAND2), &[Tv(1556), Tv(1558)]),
    ((1561, false, AND2), &[Tv(755), Tv(1560)]),
    ((1563, false, NAND2), &[Arg(0, 230), Tv(754)]),
    ((1615, false, NAND2), &[Tv(1612), Tv(1614)]),
    ((1617, false, AND2), &[Tv(755), Tv(1616)]),
    ((1619, false, NAND2), &[Arg(0, 231), Tv(754)]),
    ((1671, false, NAND2), &[Tv(1668), Tv(1670)]),
    ((1673, false, AND2), &[Tv(755), Tv(1672)]),
    ((1675, false, NAND2), &[Arg(0, 232), Tv(754)]),
    ((1727, false, NAND2), &[Tv(1724), Tv(1726)]),
    ((1729, false, AND2), &[Tv(755), Tv(1728)]),
    ((1731, false, NAND2), &[Arg(0, 233), Tv(754)]),
    ((1783, false, NAND2), &[Tv(1780), Tv(1782)]),
    ((1785, false, AND2), &[Tv(755), Tv(1784)]),
    ((1787, false, NAND2), &[Arg(0, 234), Tv(754)]),
    ((1839, false, NAND2), &[Tv(1836), Tv(1838)]),
    ((1841, false, AND2), &[Tv(755), Tv(1840)]),
    ((1843, false, NAND2), &[Arg(0, 235), Tv(754)]),
    ((1895, false, NAND2), &[Tv(1892), Tv(1894)]),
    ((1897, false, AND2), &[Tv(755), Tv(1896)]),
    ((1899, false, NAND2), &[Arg(0, 236), Tv(754)]),
    ((1951, false, NAND2), &[Tv(1948), Tv(1950)]),
    ((1953, false, AND2), &[Tv(755), Tv(1952)]),
    ((1955, false, NAND2), &[Arg(0, 237), Tv(754)]),
    ((2007, false, NAND2), &[Tv(2004), Tv(2006)]),
    ((2009, false, AND2), &[Tv(755), Tv(2008)]),
    ((2011, false, NAND2), &[Arg(0, 238), Tv(754)]),
    ((2063, false, NAND2), &[Tv(2060), Tv(2062)]),
    ((2065, false, AND2), &[Tv(755), Tv(2064)]),
    ((2067, false, NAND2), &[Arg(0, 239), Tv(754)]),
    ((2119, false, NAND2), &[Tv(2116), Tv(2118)]),
    ((2121, false, AND2), &[Tv(755), Tv(2120)]),
    ((2123, false, NAND2), &[Arg(0, 240), Tv(754)]),
    ((2175, false, NAND2), &[Tv(2172), Tv(2174)]),
    ((2177, false, AND2), &[Tv(755), Tv(2176)]),
    ((2179, false, NAND2), &[Arg(0, 241), Tv(754)]),
    ((2231, false, NAND2), &[Tv(2228), Tv(2230)]),
    ((2233, false, AND2), &[Tv(755), Tv(2232)]),
    ((2235, false, NAND2), &[Arg(0, 242), Tv(754)]),
    ((2287, false, NAND2), &[Tv(2284), Tv(2286)]),
    ((2289, false, AND2), &[Tv(755), Tv(2288)]),
    ((2291, false, NAND2), &[Arg(0, 243), Tv(754)]),
    ((2343, false, NAND2), &[Tv(2340), Tv(2342)]),
    ((2345, false, AND2), &[Tv(755), Tv(2344)]),
    ((2347, false, NAND2), &[Arg(0, 244), Tv(754)]),
    ((2399, false, NAND2), &[Tv(2396), Tv(2398)]),
    ((2401, false, AND2), &[Tv(755), Tv(2400)]),
    ((2403, false, NAND2), &[Arg(0, 245), Tv(754)]),
];

static LEVEL_15: [((usize, bool, CellType), &[GateInput]); 95] = [
    ((22, false, INV), &[Arg(0, 462)]),
    ((188, false, INV), &[Arg(0, 415)]),
    ((191, false, INV), &[Arg(0, 423)]),
    ((193, false, INV), &[Arg(0, 454)]),
    ((449, false, AND2), &[Tv(447), Tv(448)]),
    ((451, false, AND2), &[Tv(446), Tv(450)]),
    ((453, false, XNOR2), &[Tv(21), Tv(250)]),
    ((454, false, XNOR2), &[Tv(192), Tv(248)]),
    ((491, false, AND2), &[Tv(489), Tv(490)]),
    ((494, false, AND2), &[Tv(492), Tv(493)]),
    ((500, false, AND2), &[Tv(496), Tv(499)]),
    ((513, false, AND2), &[Tv(503), Tv(512)]),
    ((758, false, NAND2), &[Tv(711), Tv(757)]),
    ((800, false, AND2), &[Tv(798), Tv(799)]),
    ((838, false, AND2), &[Tv(830), Tv(837)]),
    ((839, false, NAND2), &[Tv(830), Tv(837)]),
    ((840, false, NAND2), &[Tv(2464), Tv(797)]),
    ((847, false, AND2), &[Tv(845), Tv(846)]),
    ((890, false, AND2), &[Tv(858), Tv(889)]),
    ((928, false, AND2), &[Tv(896), Tv(927)]),
    ((931, false, AND2), &[Tv(929), Tv(930)]),
    ((933, false, XNOR2), &[Tv(181), Tv(203)]),
    ((938, false, AND2), &[Tv(936), Tv(937)]),
    ((1114, false, NAND2), &[Tv(1111), Tv(1113)]),
    ((1116, false, AND2), &[Tv(798), Tv(1115)]),
    ((1118, false, NAND2), &[Tv(2465), Tv(797)]),
    ((1170, false, NAND2), &[Tv(1167), Tv(1169)]),
    ((1172, false, AND2), &[Tv(798), Tv(1171)]),
    ((1174, false, NAND2), &[Tv(2466), Tv(797)]),
    ((1226, false, NAND2), &[Tv(1223), Tv(1225)]),
    ((1228, false, AND2), &[Tv(798), Tv(1227)]),
    ((1230, false, NAND2), &[Tv(2467), Tv(797)]),
    ((1282, false, NAND2), &[Tv(1279), Tv(1281)]),
    ((1284, false, AND2), &[Tv(798), Tv(1283)]),
    ((1286, false, NAND2), &[Tv(2468), Tv(797)]),
    ((1338, false, NAND2), &[Tv(1335), Tv(1337)]),
    ((1340, false, AND2), &[Tv(798), Tv(1339)]),
    ((1342, false, NAND2), &[Tv(2469), Tv(797)]),
    ((1394, false, NAND2), &[Tv(1391), Tv(1393)]),
    ((1396, false, AND2), &[Tv(798), Tv(1395)]),
    ((1398, false, NAND2), &[Tv(2470), Tv(797)]),
    ((1450, false, NAND2), &[Tv(1447), Tv(1449)]),
    ((1452, false, AND2), &[Tv(798), Tv(1451)]),
    ((1454, false, NAND2), &[Tv(2471), Tv(797)]),
    ((1506, false, NAND2), &[Tv(1503), Tv(1505)]),
    ((1508, false, AND2), &[Tv(798), Tv(1507)]),
    ((1510, false, NAND2), &[Tv(2512), Tv(797)]),
    ((1562, false, NAND2), &[Tv(1559), Tv(1561)]),
    ((1564, false, AND2), &[Tv(798), Tv(1563)]),
    ((1566, false, NAND2), &[Tv(2513), Tv(797)]),
    ((1618, false, NAND2), &[Tv(1615), Tv(1617)]),
    ((1620, false, AND2), &[Tv(798), Tv(1619)]),
    ((1622, false, NAND2), &[Tv(2514), Tv(797)]),
    ((1674, false, NAND2), &[Tv(1671), Tv(1673)]),
    ((1676, false, AND2), &[Tv(798), Tv(1675)]),
    ((1678, false, NAND2), &[Tv(2515), Tv(797)]),
    ((1730, false, NAND2), &[Tv(1727), Tv(1729)]),
    ((1732, false, AND2), &[Tv(798), Tv(1731)]),
    ((1734, false, NAND2), &[Tv(2516), Tv(797)]),
    ((1786, false, NAND2), &[Tv(1783), Tv(1785)]),
    ((1788, false, AND2), &[Tv(798), Tv(1787)]),
    ((1790, false, NAND2), &[Tv(2517), Tv(797)]),
    ((1842, false, NAND2), &[Tv(1839), Tv(1841)]),
    ((1844, false, AND2), &[Tv(798), Tv(1843)]),
    ((1846, false, NAND2), &[Tv(2518), Tv(797)]),
    ((1898, false, NAND2), &[Tv(1895), Tv(1897)]),
    ((1900, false, AND2), &[Tv(798), Tv(1899)]),
    ((1902, false, NAND2), &[Tv(2519), Tv(797)]),
    ((1954, false, NAND2), &[Tv(1951), Tv(1953)]),
    ((1956, false, AND2), &[Tv(798), Tv(1955)]),
    ((1958, false, NAND2), &[Tv(2560), Tv(797)]),
    ((2010, false, NAND2), &[Tv(2007), Tv(2009)]),
    ((2012, false, AND2), &[Tv(798), Tv(2011)]),
    ((2014, false, NAND2), &[Tv(2561), Tv(797)]),
    ((2066, false, NAND2), &[Tv(2063), Tv(2065)]),
    ((2068, false, AND2), &[Tv(798), Tv(2067)]),
    ((2070, false, NAND2), &[Tv(2562), Tv(797)]),
    ((2122, false, NAND2), &[Tv(2119), Tv(2121)]),
    ((2124, false, AND2), &[Tv(798), Tv(2123)]),
    ((2126, false, NAND2), &[Tv(2563), Tv(797)]),
    ((2178, false, NAND2), &[Tv(2175), Tv(2177)]),
    ((2180, false, AND2), &[Tv(798), Tv(2179)]),
    ((2182, false, NAND2), &[Tv(2564), Tv(797)]),
    ((2234, false, NAND2), &[Tv(2231), Tv(2233)]),
    ((2236, false, AND2), &[Tv(798), Tv(2235)]),
    ((2238, false, NAND2), &[Tv(2565), Tv(797)]),
    ((2290, false, NAND2), &[Tv(2287), Tv(2289)]),
    ((2292, false, AND2), &[Tv(798), Tv(2291)]),
    ((2294, false, NAND2), &[Tv(2566), Tv(797)]),
    ((2346, false, NAND2), &[Tv(2343), Tv(2345)]),
    ((2348, false, AND2), &[Tv(798), Tv(2347)]),
    ((2350, false, NAND2), &[Tv(2567), Tv(797)]),
    ((2402, false, NAND2), &[Tv(2399), Tv(2401)]),
    ((2404, false, AND2), &[Tv(798), Tv(2403)]),
    ((2406, false, NAND2), &[Tv(16), Tv(797)]),
];

static LEVEL_16: [((usize, bool, CellType), &[GateInput]); 124] = [
    ((2472, false, INV), &[Arg(0, 344)]),
    ((2473, false, INV), &[Arg(0, 345)]),
    ((2474, false, INV), &[Arg(0, 346)]),
    ((2475, false, INV), &[Arg(0, 347)]),
    ((2476, false, INV), &[Arg(0, 348)]),
    ((2477, false, INV), &[Arg(0, 349)]),
    ((2478, false, INV), &[Arg(0, 350)]),
    ((2479, false, INV), &[Arg(0, 351)]),
    ((2520, false, INV), &[Arg(0, 352)]),
    ((2521, false, INV), &[Arg(0, 353)]),
    ((2522, false, INV), &[Arg(0, 354)]),
    ((2523, false, INV), &[Arg(0, 355)]),
    ((2524, false, INV), &[Arg(0, 356)]),
    ((2525, false, INV), &[Arg(0, 357)]),
    ((2526, false, INV), &[Arg(0, 358)]),
    ((2527, false, INV), &[Arg(0, 359)]),
    ((2568, false, INV), &[Arg(0, 360)]),
    ((2569, false, INV), &[Arg(0, 361)]),
    ((2570, false, INV), &[Arg(0, 362)]),
    ((2571, false, INV), &[Arg(0, 363)]),
    ((2572, false, INV), &[Arg(0, 364)]),
    ((2573, false, INV), &[Arg(0, 365)]),
    ((2574, false, INV), &[Arg(0, 366)]),
    ((2575, false, INV), &[Arg(0, 367)]),
    ((17, false, INV), &[Arg(0, 368)]),
    ((23, false, INV), &[Arg(0, 463)]),
    ((28, false, INV), &[Arg(0, 494)]),
    ((31, false, INV), &[Arg(0, 502)]),
    ((194, false, INV), &[Arg(0, 455)]),
    ((409, false, XNOR2), &[Arg(2, 0), Arg(0, 500)]),
    ((410, false, XNOR2), &[Arg(1, 0), Arg(0, 492)]),
    ((411, false, XOR2), &[Arg(1, 1), Arg(0, 493)]),
    ((412, false, XOR2), &[Arg(2, 1), Arg(0, 501)]),
    ((452, false, AND2), &[Tv(449), Tv(451)]),
    ((455, false, AND2), &[Tv(453), Tv(454)]),
    ((462, false, XNOR2), &[Tv(22), Tv(255)]),
    ((463, false, XNOR2), &[Tv(193), Tv(229)]),
    ((479, false, OR2), &[Arg(0, 424), Tv(217)]),
    ((480, false, OR2), &[Arg(0, 416), Tv(219)]),
    ((482, false, NAND2), &[Arg(0, 416), Tv(219)]),
    ((483, false, NAND2), &[Arg(0, 424), Tv(217)]),
    ((486, false, XNOR2), &[Tv(191), Tv(223)]),
    ((487, false, XNOR2), &[Tv(188), Tv(227)]),
    ((495, false, AND2), &[Tv(491), Tv(494)]),
    ((514, false, AND2), &[Tv(500), Tv(513)]),
    ((801, false, NAND2), &[Tv(758), Tv(800)]),
    ((841, false, AND2), &[Tv(839), Tv(840)]),
    ((891, false, AND2), &[Tv(847), Tv(890)]),
    ((892, false, NAND2), &[Tv(847), Tv(890)]),
    ((893, false, NAND2), &[Arg(0, 303), Tv(838)]),
    ((932, false, AND2), &[Tv(928), Tv(931)]),
    ((939, false, AND2), &[Tv(933), Tv(938)]),
    ((1117, false, NAND2), &[Tv(1114), Tv(1116)]),
    ((1119, false, AND2), &[Tv(839), Tv(1118)]),
    ((1121, false, NAND2), &[Arg(0, 304), Tv(838)]),
    ((1173, false, NAND2), &[Tv(1170), Tv(1172)]),
    ((1175, false, AND2), &[Tv(839), Tv(1174)]),
    ((1177, false, NAND2), &[Arg(0, 305), Tv(838)]),
    ((1229, false, NAND2), &[Tv(1226), Tv(1228)]),
    ((1231, false, AND2), &[Tv(839), Tv(1230)]),
    ((1233, false, NAND2), &[Arg(0, 306), Tv(838)]),
    ((1285, false, NAND2), &[Tv(1282), Tv(1284)]),
    ((1287, false, AND2), &[Tv(839), Tv(1286)]),
    ((1289, false, NAND2), &[Arg(0, 307), Tv(838)]),
    ((1341, false, NAND2), &[Tv(1338), Tv(1340)]),
    ((1343, false, AND2), &[Tv(839), Tv(1342)]),
    ((1345, false, NAND2), &[Arg(0, 308), Tv(838)]),
    ((1397, false, NAND2), &[Tv(1394), Tv(1396)]),
    ((1399, false, AND2), &[Tv(839), Tv(1398)]),
    ((1401, false, NAND2), &[Arg(0, 309), Tv(838)]),
    ((1453, false, NAND2), &[Tv(1450), Tv(1452)]),
    ((1455, false, AND2), &[Tv(839), Tv(1454)]),
    ((1457, false, NAND2), &[Arg(0, 310), Tv(838)]),
    ((1509, false, NAND2), &[Tv(1506), Tv(1508)]),
    ((1511, false, AND2), &[Tv(839), Tv(1510)]),
    ((1513, false, NAND2), &[Arg(0, 311), Tv(838)]),
    ((1565, false, NAND2), &[Tv(1562), Tv(1564)]),
    ((1567, false, AND2), &[Tv(839), Tv(1566)]),
    ((1569, false, NAND2), &[Arg(0, 312), Tv(838)]),
    ((1621, false, NAND2), &[Tv(1618), Tv(1620)]),
    ((1623, false, AND2), &[Tv(839), Tv(1622)]),
    ((1625, false, NAND2), &[Arg(0, 313), Tv(838)]),
    ((1677, false, NAND2), &[Tv(1674), Tv(1676)]),
    ((1679, false, AND2), &[Tv(839), Tv(1678)]),
    ((1681, false, NAND2), &[Arg(0, 314), Tv(838)]),
    ((1733, false, NAND2), &[Tv(1730), Tv(1732)]),
    ((1735, false, AND2), &[Tv(839), Tv(1734)]),
    ((1737, false, NAND2), &[Arg(0, 315), Tv(838)]),
    ((1789, false, NAND2), &[Tv(1786), Tv(1788)]),
    ((1791, false, AND2), &[Tv(839), Tv(1790)]),
    ((1793, false, NAND2), &[Arg(0, 316), Tv(838)]),
    ((1845, false, NAND2), &[Tv(1842), Tv(1844)]),
    ((1847, false, AND2), &[Tv(839), Tv(1846)]),
    ((1849, false, NAND2), &[Arg(0, 317), Tv(838)]),
    ((1901, false, NAND2), &[Tv(1898), Tv(1900)]),
    ((1903, false, AND2), &[Tv(839), Tv(1902)]),
    ((1905, false, NAND2), &[Arg(0, 318), Tv(838)]),
    ((1957, false, NAND2), &[Tv(1954), Tv(1956)]),
    ((1959, false, AND2), &[Tv(839), Tv(1958)]),
    ((1961, false, NAND2), &[Arg(0, 319), Tv(838)]),
    ((2013, false, NAND2), &[Tv(2010), Tv(2012)]),
    ((2015, false, AND2), &[Tv(839), Tv(2014)]),
    ((2017, false, NAND2), &[Arg(0, 320), Tv(838)]),
    ((2069, false, NAND2), &[Tv(2066), Tv(2068)]),
    ((2071, false, AND2), &[Tv(839), Tv(2070)]),
    ((2073, false, NAND2), &[Arg(0, 321), Tv(838)]),
    ((2125, false, NAND2), &[Tv(2122), Tv(2124)]),
    ((2127, false, AND2), &[Tv(839), Tv(2126)]),
    ((2129, false, NAND2), &[Arg(0, 322), Tv(838)]),
    ((2181, false, NAND2), &[Tv(2178), Tv(2180)]),
    ((2183, false, AND2), &[Tv(839), Tv(2182)]),
    ((2185, false, NAND2), &[Arg(0, 323), Tv(838)]),
    ((2237, false, NAND2), &[Tv(2234), Tv(2236)]),
    ((2239, false, AND2), &[Tv(839), Tv(2238)]),
    ((2241, false, NAND2), &[Arg(0, 324), Tv(838)]),
    ((2293, false, NAND2), &[Tv(2290), Tv(2292)]),
    ((2295, false, AND2), &[Tv(839), Tv(2294)]),
    ((2297, false, NAND2), &[Arg(0, 325), Tv(838)]),
    ((2349, false, NAND2), &[Tv(2346), Tv(2348)]),
    ((2351, false, AND2), &[Tv(839), Tv(2350)]),
    ((2353, false, NAND2), &[Arg(0, 326), Tv(838)]),
    ((2405, false, NAND2), &[Tv(2402), Tv(2404)]),
    ((2407, false, AND2), &[Tv(839), Tv(2406)]),
    ((2409, false, NAND2), &[Arg(0, 327), Tv(838)]),
];

static LEVEL_17: [((usize, bool, CellType), &[GateInput]); 100] = [
    ((24, false, INV), &[Arg(0, 464)]),
    ((29, false, INV), &[Arg(0, 495)]),
    ((32, false, INV), &[Arg(0, 503)]),
    ((195, false, INV), &[Arg(0, 456)]),
    ((368, false, XNOR2), &[Arg(2, 0), Arg(0, 541)]),
    ((369, false, XOR2), &[Arg(2, 1), Arg(0, 542)]),
    ((371, false, XNOR2), &[Arg(1, 0), Arg(0, 533)]),
    ((372, false, XOR2), &[Arg(1, 1), Arg(0, 534)]),
    ((413, false, AND2), &[Tv(409), Tv(411)]),
    ((414, false, AND2), &[Tv(410), Tv(412)]),
    ((425, false, XNOR2), &[Tv(28), Tv(248)]),
    ((426, false, XNOR2), &[Tv(31), Tv(250)]),
    ((445, false, XNOR2), &[Tv(23), Tv(246)]),
    ((456, false, AND2), &[Tv(452), Tv(455)]),
    ((461, false, XNOR2), &[Tv(194), Tv(240)]),
    ((464, false, AND2), &[Tv(462), Tv(463)]),
    ((472, false, NAND2), &[Arg(0, 417), Tv(203)]),
    ((473, false, NAND2), &[Arg(0, 425), Tv(210)]),
    ((475, false, OR2), &[Arg(0, 417), Tv(203)]),
    ((476, false, OR2), &[Arg(0, 425), Tv(210)]),
    ((481, false, AND2), &[Tv(479), Tv(480)]),
    ((484, false, AND2), &[Tv(482), Tv(483)]),
    ((488, false, AND2), &[Tv(486), Tv(487)]),
    ((515, false, AND2), &[Tv(495), Tv(514)]),
    ((842, false, NAND2), &[Tv(801), Tv(841)]),
    ((894, false, AND2), &[Tv(892), Tv(893)]),
    ((941, false, NAND2), &[Tv(932), Tv(939)]),
    ((942, false, NAND2), &[Tv(2472), Tv(891)]),
    ((1120, false, NAND2), &[Tv(1117), Tv(1119)]),
    ((1122, false, AND2), &[Tv(892), Tv(1121)]),
    ((1124, false, NAND2), &[Tv(2473), Tv(891)]),
    ((1176, false, NAND2), &[Tv(1173), Tv(1175)]),
    ((1178, false, AND2), &[Tv(892), Tv(1177)]),
    ((1180, false, NAND2), &[Tv(2474), Tv(891)]),
    ((1232, false, NAND2), &[Tv(1229), Tv(1231)]),
    ((1234, false, AND2), &[Tv(892), Tv(1233)]),
    ((1236, false, NAND2), &[Tv(2475), Tv(891)]),
    ((1288, false, NAND2), &[Tv(1285), Tv(1287)]),
    ((1290, false, AND2), &[Tv(892), Tv(1289)]),
    ((1292, false, NAND2), &[Tv(2476), Tv(891)]),
    ((1344, false, NAND2), &[Tv(1341), Tv(1343)]),
    ((1346, false, AND2), &[Tv(892), Tv(1345)]),
    ((1348, false, NAND2), &[Tv(2477), Tv(891)]),
    ((1400, false, NAND2), &[Tv(1397), Tv(1399)]),
    ((1402, false, AND2), &[Tv(892), Tv(1401)]),
    ((1404, false, NAND2), &[Tv(2478), Tv(891)]),
    ((1456, false, NAND2), &[Tv(1453), Tv(1455)]),
    ((1458, false, AND2), &[Tv(892), Tv(1457)]),
    ((1460, false, NAND2), &[Tv(2479), Tv(891)]),
    ((1512, false, NAND2), &[Tv(1509), Tv(1511)]),
    ((1514, false, AND2), &[Tv(892), Tv(1513)]),
    ((1516, false, NAND2), &[Tv(2520), Tv(891)]),
    ((1568, false, NAND2), &[Tv(1565), Tv(1567)]),
    ((1570, false, AND2), &[Tv(892), Tv(1569)]),
    ((1572, false, NAND2), &[Tv(2521), Tv(891)]),
    ((1624, false, NAND2), &[Tv(1621), Tv(1623)]),
    ((1626, false, AND2), &[Tv(892), Tv(1625)]),
    ((1628, false, NAND2), &[Tv(2522), Tv(891)]),
    ((1680, false, NAND2), &[Tv(1677), Tv(1679)]),
    ((1682, false, AND2), &[Tv(892), Tv(1681)]),
    ((1684, false, NAND2), &[Tv(2523), Tv(891)]),
    ((1736, false, NAND2), &[Tv(1733), Tv(1735)]),
    ((1738, false, AND2), &[Tv(892), Tv(1737)]),
    ((1740, false, NAND2), &[Tv(2524), Tv(891)]),
    ((1792, false, NAND2), &[Tv(1789), Tv(1791)]),
    ((1794, false, AND2), &[Tv(892), Tv(1793)]),
    ((1796, false, NAND2), &[Tv(2525), Tv(891)]),
    ((1848, false, NAND2), &[Tv(1845), Tv(1847)]),
    ((1850, false, AND2), &[Tv(892), Tv(1849)]),
    ((1852, false, NAND2), &[Tv(2526), Tv(891)]),
    ((1904, false, NAND2), &[Tv(1901), Tv(1903)]),
    ((1906, false, AND2), &[Tv(892), Tv(1905)]),
    ((1908, false, NAND2), &[Tv(2527), Tv(891)]),
    ((1960, false, NAND2), &[Tv(1957), Tv(1959)]),
    ((1962, false, AND2), &[Tv(892), Tv(1961)]),
    ((1964, false, NAND2), &[Tv(2568), Tv(891)]),
    ((2016, false, NAND2), &[Tv(2013), Tv(2015)]),
    ((2018, false, AND2), &[Tv(892), Tv(2017)]),
    ((2020, false, NAND2), &[Tv(2569), Tv(891)]),
    ((2072, false, NAND2), &[Tv(2069), Tv(2071)]),
    ((2074, false, AND2), &[Tv(892), Tv(2073)]),
    ((2076, false, NAND2), &[Tv(2570), Tv(891)]),
    ((2128, false, NAND2), &[Tv(2125), Tv(2127)]),
    ((2130, false, AND2), &[Tv(892), Tv(2129)]),
    ((2132, false, NAND2), &[Tv(2571), Tv(891)]),
    ((2184, false, NAND2), &[Tv(2181), Tv(2183)]),
    ((2186, false, AND2), &[Tv(892), Tv(2185)]),
    ((2188, false, NAND2), &[Tv(2572), Tv(891)]),
    ((2240, false, NAND2), &[Tv(2237), Tv(2239)]),
    ((2242, false, AND2), &[Tv(892), Tv(2241)]),
    ((2244, false, NAND2), &[Tv(2573), Tv(891)]),
    ((2296, false, NAND2), &[Tv(2293), Tv(2295)]),
    ((2298, false, AND2), &[Tv(892), Tv(2297)]),
    ((2300, false, NAND2), &[Tv(2574), Tv(891)]),
    ((2352, false, NAND2), &[Tv(2349), Tv(2351)]),
    ((2354, false, AND2), &[Tv(892), Tv(2353)]),
    ((2356, false, NAND2), &[Tv(2575), Tv(891)]),
    ((2408, false, NAND2), &[Tv(2405), Tv(2407)]),
    ((2410, false, AND2), &[Tv(892), Tv(2409)]),
    ((2412, false, NAND2), &[Tv(17), Tv(891)]),
];

static LEVEL_18: [((usize, bool, CellType), &[GateInput]); 75] = [
    ((25, false, INV), &[Arg(0, 465)]),
    ((30, false, INV), &[Arg(0, 496)]),
    ((36, false, INV), &[Arg(0, 544)]),
    ((196, false, INV), &[Arg(0, 457)]),
    ((360, false, NAND2), &[Arg(0, 535), Tv(248)]),
    ((361, false, NAND2), &[Arg(0, 543), Tv(250)]),
    ((363, false, OR2), &[Arg(0, 535), Tv(248)]),
    ((364, false, OR2), &[Arg(0, 543), Tv(250)]),
    ((370, false, AND2), &[Tv(368), Tv(369)]),
    ((373, false, AND2), &[Tv(371), Tv(372)]),
    ((415, false, AND2), &[Tv(413), Tv(414)]),
    ((416, false, XNOR2), &[Tv(29), Tv(229)]),
    ((424, false, NAND2), &[Arg(0, 504), Tv(246)]),
    ((427, false, AND2), &[Tv(425), Tv(426)]),
    ((429, false, OR2), &[Arg(0, 504), Tv(246)]),
    ((430, false, XNOR2), &[Tv(32), Tv(255)]),
    ((457, false, AND2), &[Tv(445), Tv(456)]),
    ((458, false, XNOR2), &[Tv(24), Tv(223)]),
    ((460, false, XNOR2), &[Tv(195), Tv(227)]),
    ((465, false, AND2), &[Tv(461), Tv(464)]),
    ((474, false, AND2), &[Tv(472), Tv(473)]),
    ((477, false, AND2), &[Tv(475), Tv(476)]),
    ((485, false, AND2), &[Tv(481), Tv(484)]),
    ((516, false, AND2), &[Tv(488), Tv(515)]),
    ((895, false, NAND2), &[Tv(842), Tv(894)]),
    ((940, false, AND2), &[Tv(932), Tv(939)]),
    ((943, false, AND2), &[Tv(941), Tv(942)]),
    ((1123, false, NAND2), &[Tv(1120), Tv(1122)]),
    ((1125, false, AND2), &[Tv(941), Tv(1124)]),
    ((1179, false, NAND2), &[Tv(1176), Tv(1178)]),
    ((1181, false, AND2), &[Tv(941), Tv(1180)]),
    ((1235, false, NAND2), &[Tv(1232), Tv(1234)]),
    ((1237, false, AND2), &[Tv(941), Tv(1236)]),
    ((1291, false, NAND2), &[Tv(1288), Tv(1290)]),
    ((1293, false, AND2), &[Tv(941), Tv(1292)]),
    ((1347, false, NAND2), &[Tv(1344), Tv(1346)]),
    ((1349, false, AND2), &[Tv(941), Tv(1348)]),
    ((1403, false, NAND2), &[Tv(1400), Tv(1402)]),
    ((1405, false, AND2), &[Tv(941), Tv(1404)]),
    ((1459, false, NAND2), &[Tv(1456), Tv(1458)]),
    ((1461, false, AND2), &[Tv(941), Tv(1460)]),
    ((1515, false, NAND2), &[Tv(1512), Tv(1514)]),
    ((1517, false, AND2), &[Tv(941), Tv(1516)]),
    ((1571, false, NAND2), &[Tv(1568), Tv(1570)]),
    ((1573, false, AND2), &[Tv(941), Tv(1572)]),
    ((1627, false, NAND2), &[Tv(1624), Tv(1626)]),
    ((1629, false, AND2), &[Tv(941), Tv(1628)]),
    ((1683, false, NAND2), &[Tv(1680), Tv(1682)]),
    ((1685, false, AND2), &[Tv(941), Tv(1684)]),
    ((1739, false, NAND2), &[Tv(1736), Tv(1738)]),
    ((1741, false, AND2), &[Tv(941), Tv(1740)]),
    ((1795, false, NAND2), &[Tv(1792), Tv(1794)]),
    ((1797, false, AND2), &[Tv(941), Tv(1796)]),
    ((1851, false, NAND2), &[Tv(1848), Tv(1850)]),
    ((1853, false, AND2), &[Tv(941), Tv(1852)]),
    ((1907, false, NAND2), &[Tv(1904), Tv(1906)]),
    ((1909, false, AND2), &[Tv(941), Tv(1908)]),
    ((1963, false, NAND2), &[Tv(1960), Tv(1962)]),
    ((1965, false, AND2), &[Tv(941), Tv(1964)]),
    ((2019, false, NAND2), &[Tv(2016), Tv(2018)]),
    ((2021, false, AND2), &[Tv(941), Tv(2020)]),
    ((2075, false, NAND2), &[Tv(2072), Tv(2074)]),
    ((2077, false, AND2), &[Tv(941), Tv(2076)]),
    ((2131, false, NAND2), &[Tv(2128), Tv(2130)]),
    ((2133, false, AND2), &[Tv(941), Tv(2132)]),
    ((2187, false, NAND2), &[Tv(2184), Tv(2186)]),
    ((2189, false, AND2), &[Tv(941), Tv(2188)]),
    ((2243, false, NAND2), &[Tv(2240), Tv(2242)]),
    ((2245, false, AND2), &[Tv(941), Tv(2244)]),
    ((2299, false, NAND2), &[Tv(2296), Tv(2298)]),
    ((2301, false, AND2), &[Tv(941), Tv(2300)]),
    ((2355, false, NAND2), &[Tv(2352), Tv(2354)]),
    ((2357, false, AND2), &[Tv(941), Tv(2356)]),
    ((2411, false, NAND2), &[Tv(2408), Tv(2410)]),
    ((2413, false, AND2), &[Tv(941), Tv(2412)]),
];

static LEVEL_19: [((usize, bool, CellType), &[GateInput]); 77] = [
    ((26, false, INV), &[Arg(0, 466)]),
    ((33, false, INV), &[Arg(0, 505)]),
    ((62, false, INV), &[Arg(0, 656)]),
    ((68, false, INV), &[Arg(0, 664)]),
    ((197, false, INV), &[Arg(0, 458)]),
    ((331, false, XNOR2), &[Arg(2, 0), Arg(0, 582)]),
    ((332, false, XOR2), &[Arg(2, 1), Arg(0, 583)]),
    ((334, false, XNOR2), &[Arg(1, 0), Arg(0, 574)]),
    ((335, false, XOR2), &[Arg(1, 1), Arg(0, 575)]),
    ((356, false, NAND2), &[Arg(0, 544), Tv(255)]),
    ((357, false, OR2), &[Arg(0, 536), Tv(229)]),
    ((362, false, AND2), &[Tv(360), Tv(361)]),
    ((365, false, AND2), &[Tv(363), Tv(364)]),
    ((367, false, NAND2), &[Tv(36), Tv(256)]),
    ((374, false, AND2), &[Tv(370), Tv(373)]),
    ((408, false, NAND2), &[Arg(0, 497), Tv(227)]),
    ((417, false, AND2), &[Tv(415), Tv(416)]),
    ((419, false, XNOR2), &[Tv(30), Tv(240)]),
    ((420, false, OR2), &[Arg(0, 497), Tv(227)]),
    ((428, false, AND2), &[Tv(424), Tv(427)]),
    ((431, false, AND2), &[Tv(429), Tv(430)]),
    ((440, false, XNOR2), &[Tv(25), Tv(217)]),
    ((441, false, XNOR2), &[Tv(196), Tv(219)]),
    ((459, false, AND2), &[Tv(457), Tv(458)]),
    ((466, false, AND2), &[Tv(460), Tv(465)]),
    ((478, false, AND2), &[Tv(474), Tv(477)]),
    ((517, false, AND2), &[Tv(485), Tv(516)]),
    ((944, false, NAND2), &[Tv(895), Tv(943)]),
    ((945, false, NAND2), &[Arg(0, 385), Tv(940)]),
    ((1126, false, NAND2), &[Tv(1123), Tv(1125)]),
    ((1127, false, NAND2), &[Arg(0, 386), Tv(940)]),
    ((1182, false, NAND2), &[Tv(1179), Tv(1181)]),
    ((1183, false, NAND2), &[Arg(0, 387), Tv(940)]),
    ((1238, false, NAND2), &[Tv(1235), Tv(1237)]),
    ((1239, false, NAND2), &[Arg(0, 388), Tv(940)]),
    ((1294, false, NAND2), &[Tv(1291), Tv(1293)]),
    ((1295, false, NAND2), &[Arg(0, 389), Tv(940)]),
    ((1350, false, NAND2), &[Tv(1347), Tv(1349)]),
    ((1351, false, NAND2), &[Arg(0, 390), Tv(940)]),
    ((1406, false, NAND2), &[Tv(1403), Tv(1405)]),
    ((1407, false, NAND2), &[Arg(0, 391), Tv(940)]),
    ((1462, false, NAND2), &[Tv(1459), Tv(1461)]),
    ((1463, false, NAND2), &[Arg(0, 392), Tv(940)]),
    ((1518, false, NAND2), &[Tv(1515), Tv(1517)]),
    ((1519, false, NAND2), &[Arg(0, 393), Tv(940)]),
    ((1574, false, NAND2), &[Tv(1571), Tv(1573)]),
    ((1575, false, NAND2), &[Arg(0, 394), Tv(940)]),
    ((1630, false, NAND2), &[Tv(1627), Tv(1629)]),
    ((1631, false, NAND2), &[Arg(0, 395), Tv(940)]),
    ((1686, false, NAND2), &[Tv(1683), Tv(1685)]),
    ((1687, false, NAND2), &[Arg(0, 396), Tv(940)]),
    ((1742, false, NAND2), &[Tv(1739), Tv(1741)]),
    ((1743, false, NAND2), &[Arg(0, 397), Tv(940)]),
    ((1798, false, NAND2), &[Tv(1795), Tv(1797)]),
    ((1799, false, NAND2), &[Arg(0, 398), Tv(940)]),
    ((1854, false, NAND2), &[Tv(1851), Tv(1853)]),
    ((1855, false, NAND2), &[Arg(0, 399), Tv(940)]),
    ((1910, false, NAND2), &[Tv(1907), Tv(1909)]),
    ((1911, false, NAND2), &[Arg(0, 400), Tv(940)]),
    ((1966, false, NAND2), &[Tv(1963), Tv(1965)]),
    ((1967, false, NAND2), &[Arg(0, 401), Tv(940)]),
    ((2022, false, NAND2), &[Tv(2019), Tv(2021)]),
    ((2023, false, NAND2), &[Arg(0, 402), Tv(940)]),
    ((2078, false, NAND2), &[Tv(2075), Tv(2077)]),
    ((2079, false, NAND2), &[Arg(0, 403), Tv(940)]),
    ((2134, false, NAND2), &[Tv(2131), Tv(2133)]),
    ((2135, false, NAND2), &[Arg(0, 404), Tv(940)]),
    ((2190, false, NAND2), &[Tv(2187), Tv(2189)]),
    ((2191, false, NAND2), &[Arg(0, 405), Tv(940)]),
    ((2246, false, NAND2), &[Tv(2243), Tv(2245)]),
    ((2247, false, NAND2), &[Arg(0, 406), Tv(940)]),
    ((2302, false, NAND2), &[Tv(2299), Tv(2301)]),
    ((2303, false, NAND2), &[Arg(0, 407), Tv(940)]),
    ((2358, false, NAND2), &[Tv(2355), Tv(2357)]),
    ((2359, false, NAND2), &[Arg(0, 408), Tv(940)]),
    ((2414, false, NAND2), &[Tv(2411), Tv(2413)]),
    ((2415, false, NAND2), &[Arg(0, 409), Tv(940)]),
];

static LEVEL_20: [((usize, bool, CellType), &[GateInput]); 64] = [
    ((40, false, INV), &[Arg(0, 576)]),
    ((44, false, INV), &[Arg(0, 584)]),
    ((45, false, INV), &[Arg(0, 585)]),
    ((291, false, XNOR2), &[Arg(1, 0), Arg(0, 615)]),
    ((292, false, XOR2), &[Arg(1, 1), Arg(0, 616)]),
    ((293, false, XNOR2), &[Arg(2, 0), Arg(0, 623)]),
    ((295, false, XOR2), &[Arg(2, 1), Arg(0, 624)]),
    ((333, false, AND2), &[Tv(331), Tv(332)]),
    ((336, false, AND2), &[Tv(334), Tv(335)]),
    ((348, false, OR2), &[Arg(0, 537), Tv(240)]),
    ((349, false, OR2), &[Arg(0, 545), Tv(246)]),
    ((351, false, NAND2), &[Arg(0, 537), Tv(240)]),
    ((352, false, NAND2), &[Arg(0, 545), Tv(246)]),
    ((355, false, NAND2), &[Arg(0, 536), Tv(229)]),
    ((358, false, AND2), &[Tv(356), Tv(357)]),
    ((366, false, AND2), &[Tv(362), Tv(365)]),
    ((375, false, AND2), &[Tv(367), Tv(374)]),
    ((401, false, NAND2), &[Arg(0, 506), Tv(217)]),
    ((402, false, OR2), &[Arg(0, 498), Tv(219)]),
    ((404, false, NAND2), &[Arg(0, 498), Tv(219)]),
    ((405, false, XNOR2), &[Tv(33), Tv(223)]),
    ((418, false, AND2), &[Tv(408), Tv(417)]),
    ((421, false, AND2), &[Tv(419), Tv(420)]),
    ((423, false, OR2), &[Arg(0, 506), Tv(217)]),
    ((432, false, AND2), &[Tv(428), Tv(431)]),
    ((439, false, XNOR2), &[Tv(26), Tv(210)]),
    ((442, false, AND2), &[Tv(440), Tv(441)]),
    ((444, false, XNOR2), &[Tv(197), Tv(203)]),
    ((467, false, AND2), &[Tv(459), Tv(466)]),
    ((518, false, AND2), &[Tv(478), Tv(517)]),
    ((519, false, NAND2), &[Tv(478), Tv(517)]),
    ((946, false, NAND2), &[Tv(944), Tv(945)]),
    ((983, false, NAND2), &[Tv(20), Arg(0, 656)]),
    ((984, false, NAND2), &[Arg(2, 0), Tv(68)]),
    ((986, false, NAND2), &[Tv(27), Arg(0, 664)]),
    ((987, false, OR2), &[Arg(2, 1), Arg(0, 665)]),
    ((990, false, OR2), &[Arg(1, 1), Arg(0, 657)]),
    ((991, false, NAND2), &[Arg(2, 1), Arg(0, 665)]),
    ((993, false, NAND2), &[Arg(1, 1), Arg(0, 657)]),
    ((994, false, NAND2), &[Arg(1, 0), Tv(62)]),
    ((1128, false, NAND2), &[Tv(1126), Tv(1127)]),
    ((1184, false, NAND2), &[Tv(1182), Tv(1183)]),
    ((1240, false, NAND2), &[Tv(1238), Tv(1239)]),
    ((1296, false, NAND2), &[Tv(1294), Tv(1295)]),
    ((1352, false, NAND2), &[Tv(1350), Tv(1351)]),
    ((1408, false, NAND2), &[Tv(1406), Tv(1407)]),
    ((1464, false, NAND2), &[Tv(1462), Tv(1463)]),
    ((1520, false, NAND2), &[Tv(1518), Tv(1519)]),
    ((1576, false, NAND2), &[Tv(1574), Tv(1575)]),
    ((1632, false, NAND2), &[Tv(1630), Tv(1631)]),
    ((1688, false, NAND2), &[Tv(1686), Tv(1687)]),
    ((1744, false, NAND2), &[Tv(1742), Tv(1743)]),
    ((1800, false, NAND2), &[Tv(1798), Tv(1799)]),
    ((1856, false, NAND2), &[Tv(1854), Tv(1855)]),
    ((1912, false, NAND2), &[Tv(1910), Tv(1911)]),
    ((1968, false, NAND2), &[Tv(1966), Tv(1967)]),
    ((2024, false, NAND2), &[Tv(2022), Tv(2023)]),
    ((2080, false, NAND2), &[Tv(2078), Tv(2079)]),
    ((2136, false, NAND2), &[Tv(2134), Tv(2135)]),
    ((2192, false, NAND2), &[Tv(2190), Tv(2191)]),
    ((2248, false, NAND2), &[Tv(2246), Tv(2247)]),
    ((2304, false, NAND2), &[Tv(2302), Tv(2303)]),
    ((2360, false, NAND2), &[Tv(2358), Tv(2359)]),
    ((2416, false, NAND2), &[Tv(2414), Tv(2415)]),
];

static LEVEL_21: [((usize, bool, CellType), &[GateInput]); 81] = [
    ((34, false, INV), &[Arg(0, 538)]),
    ((37, false, INV), &[Arg(0, 546)]),
    ((49, false, INV), &[Arg(0, 617)]),
    ((53, false, INV), &[Arg(0, 625)]),
    ((54, false, INV), &[Arg(0, 626)]),
    ((294, false, AND2), &[Tv(292), Tv(293)]),
    ((296, false, AND2), &[Tv(291), Tv(295)]),
    ((323, false, OR2), &[Arg(0, 577), Tv(229)]),
    ((324, false, NAND2), &[Tv(45), Tv(256)]),
    ((327, false, XNOR2), &[Tv(44), Tv(250)]),
    ((328, false, XNOR2), &[Tv(40), Tv(248)]),
    ((330, false, NAND2), &[Arg(0, 577), Tv(229)]),
    ((337, false, AND2), &[Tv(333), Tv(336)]),
    ((350, false, AND2), &[Tv(348), Tv(349)]),
    ((353, false, AND2), &[Tv(351), Tv(352)]),
    ((359, false, AND2), &[Tv(355), Tv(358)]),
    ((376, false, AND2), &[Tv(366), Tv(375)]),
    ((394, false, OR2), &[Arg(0, 499), Tv(203)]),
    ((395, false, OR2), &[Arg(0, 507), Tv(210)]),
    ((397, false, NAND2), &[Arg(0, 507), Tv(210)]),
    ((398, false, NAND2), &[Arg(0, 499), Tv(203)]),
    ((403, false, AND2), &[Tv(401), Tv(402)]),
    ((406, false, AND2), &[Tv(404), Tv(405)]),
    ((422, false, AND2), &[Tv(418), Tv(421)]),
    ((433, false, AND2), &[Tv(423), Tv(432)]),
    ((443, false, AND2), &[Tv(439), Tv(442)]),
    ((468, false, AND2), &[Tv(444), Tv(467)]),
    ((520, false, NAND2), &[Arg(0, 426), Tv(518)]),
    ((947, false, NAND2), &[Tv(519), Tv(946)]),
    ((985, false, AND2), &[Tv(983), Tv(984)]),
    ((988, false, AND2), &[Tv(986), Tv(987)]),
    ((992, false, AND2), &[Tv(990), Tv(991)]),
    ((995, false, AND2), &[Tv(993), Tv(994)]),
    ((1101, false, NAND2), &[Arg(0, 427), Tv(518)]),
    ((1129, false, NAND2), &[Tv(519), Tv(1128)]),
    ((1157, false, NAND2), &[Arg(0, 428), Tv(518)]),
    ((1185, false, NAND2), &[Tv(519), Tv(1184)]),
    ((1213, false, NAND2), &[Arg(0, 429), Tv(518)]),
    ((1241, false, NAND2), &[Tv(519), Tv(1240)]),
    ((1269, false, NAND2), &[Arg(0, 430), Tv(518)]),
    ((1297, false, NAND2), &[Tv(519), Tv(1296)]),
    ((1325, false, NAND2), &[Arg(0, 431), Tv(518)]),
    ((1353, false, NAND2), &[Tv(519), Tv(1352)]),
    ((1381, false, NAND2), &[Arg(0, 432), Tv(518)]),
    ((1409, false, NAND2), &[Tv(519), Tv(1408)]),
    ((1437, false, NAND2), &[Arg(0, 433), Tv(518)]),
    ((1465, false, NAND2), &[Tv(519), Tv(1464)]),
    ((1493, false, NAND2), &[Arg(0, 434), Tv(518)]),
    ((1521, false, NAND2), &[Tv(519), Tv(1520)]),
    ((1549, false, NAND2), &[Arg(0, 435), Tv(518)]),
    ((1577, false, NAND2), &[Tv(519), Tv(1576)]),
    ((1605, false, NAND2), &[Arg(0, 436), Tv(518)]),
    ((1633, false, NAND2), &[Tv(519), Tv(1632)]),
    ((1661, false, NAND2), &[Arg(0, 437), Tv(518)]),
    ((1689, false, NAND2), &[Tv(519), Tv(1688)]),
    ((1717, false, NAND2), &[Arg(0, 438), Tv(518)]),
    ((1745, false, NAND2), &[Tv(519), Tv(1744)]),
    ((1773, false, NAND2), &[Arg(0, 439), Tv(518)]),
    ((1801, false, NAND2), &[Tv(519), Tv(1800)]),
    ((1829, false, NAND2), &[Arg(0, 440), Tv(518)]),
    ((1857, false, NAND2), &[Tv(519), Tv(1856)]),
    ((1885, false, NAND2), &[Arg(0, 441), Tv(518)]),
    ((1913, false, NAND2), &[Tv(519), Tv(1912)]),
    ((1941, false, NAND2), &[Arg(0, 442), Tv(518)]),
    ((1969, false, NAND2), &[Tv(519), Tv(1968)]),
    ((1997, false, NAND2), &[Arg(0, 443), Tv(518)]),
    ((2025, false, NAND2), &[Tv(519), Tv(2024)]),
    ((2053, false, NAND2), &[Arg(0, 444), Tv(518)]),
    ((2081, false, NAND2), &[Tv(519), Tv(2080)]),
    ((2109, false, NAND2), &[Arg(0, 445), Tv(518)]),
    ((2137, false, NAND2), &[Tv(519), Tv(2136)]),
    ((2165, false, NAND2), &[Arg(0, 446), Tv(518)]),
    ((2193, false, NAND2), &[Tv(519), Tv(2192)]),
    ((2221, false, NAND2), &[Arg(0, 447), Tv(518)]),
    ((2249, false, NAND2), &[Tv(519), Tv(2248)]),
    ((2277, false, NAND2), &[Arg(0, 448), Tv(518)]),
    ((2305, false, NAND2), &[Tv(519), Tv(2304)]),
    ((2333, false, NAND2), &[Arg(0, 449), Tv(518)]),
    ((2361, false, NAND2), &[Tv(519), Tv(2360)]),
    ((2389, false, NAND2), &[Arg(0, 450), Tv(518)]),
    ((2417, false, NAND2), &[Tv(519), Tv(2416)]),
];

static LEVEL_22: [((usize, bool, CellType), &[GateInput]); 56] = [
    ((35, false, INV), &[Arg(0, 539)]),
    ((38, false, INV), &[Arg(0, 547)]),
    ((41, false, INV), &[Arg(0, 578)]),
    ((46, false, INV), &[Arg(0, 586)]),
    ((63, false, INV), &[Arg(0, 667)]),
    ((283, false, NAND2), &[Arg(0, 626), Tv(255)]),
    ((284, false, OR2), &[Arg(0, 618), Tv(229)]),
    ((287, false, XNOR2), &[Tv(53), Tv(250)]),
    ((288, false, XNOR2), &[Tv(49), Tv(248)]),
    ((290, false, NAND2), &[Tv(54), Tv(256)]),
    ((297, false, AND2), &[Tv(294), Tv(296)]),
    ((322, false, NAND2), &[Arg(0, 585), Tv(255)]),
    ((325, false, AND2), &[Tv(323), Tv(324)]),
    ((329, false, AND2), &[Tv(327), Tv(328)]),
    ((338, false, AND2), &[Tv(330), Tv(337)]),
    ((354, false, AND2), &[Tv(350), Tv(353)]),
    ((377, false, AND2), &[Tv(359), Tv(376)]),
    ((386, false, XNOR2), &[Tv(34), Tv(227)]),
    ((387, false, XNOR2), &[Tv(37), Tv(223)]),
    ((396, false, AND2), &[Tv(394), Tv(395)]),
    ((399, false, AND2), &[Tv(397), Tv(398)]),
    ((407, false, AND2), &[Tv(403), Tv(406)]),
    ((434, false, AND2), &[Tv(422), Tv(433)]),
    ((469, false, AND2), &[Tv(443), Tv(468)]),
    ((470, false, NAND2), &[Tv(443), Tv(468)]),
    ((948, false, NAND2), &[Tv(520), Tv(947)]),
    ((975, false, OR2), &[Arg(0, 658), Tv(248)]),
    ((976, false, OR2), &[Arg(0, 666), Tv(250)]),
    ((978, false, NAND2), &[Arg(0, 666), Tv(250)]),
    ((979, false, NAND2), &[Arg(0, 658), Tv(248)]),
    ((989, false, AND2), &[Tv(985), Tv(988)]),
    ((996, false, AND2), &[Tv(992), Tv(995)]),
    ((1130, false, NAND2), &[Tv(1101), Tv(1129)]),
    ((1186, false, NAND2), &[Tv(1157), Tv(1185)]),
    ((1242, false, NAND2), &[Tv(1213), Tv(1241)]),
    ((1298, false, NAND2), &[Tv(1269), Tv(1297)]),
    ((1354, false, NAND2), &[Tv(1325), Tv(1353)]),
    ((1410, false, NAND2), &[Tv(1381), Tv(1409)]),
    ((1466, false, NAND2), &[Tv(1437), Tv(1465)]),
    ((1522, false, NAND2), &[Tv(1493), Tv(1521)]),
    ((1578, false, NAND2), &[Tv(1549), Tv(1577)]),
    ((1634, false, NAND2), &[Tv(1605), Tv(1633)]),
    ((1690, false, NAND2), &[Tv(1661), Tv(1689)]),
    ((1746, false, NAND2), &[Tv(1717), Tv(1745)]),
    ((1802, false, NAND2), &[Tv(1773), Tv(1801)]),
    ((1858, false, NAND2), &[Tv(1829), Tv(1857)]),
    ((1914, false, NAND2), &[Tv(1885), Tv(1913)]),
    ((1970, false, NAND2), &[Tv(1941), Tv(1969)]),
    ((2026, false, NAND2), &[Tv(1997), Tv(2025)]),
    ((2082, false, NAND2), &[Tv(2053), Tv(2081)]),
    ((2138, false, NAND2), &[Tv(2109), Tv(2137)]),
    ((2194, false, NAND2), &[Tv(2165), Tv(2193)]),
    ((2250, false, NAND2), &[Tv(2221), Tv(2249)]),
    ((2306, false, NAND2), &[Tv(2277), Tv(2305)]),
    ((2362, false, NAND2), &[Tv(2333), Tv(2361)]),
    ((2418, false, NAND2), &[Tv(2389), Tv(2417)]),
];

static LEVEL_23: [((usize, bool, CellType), &[GateInput]); 87] = [
    ((39, false, INV), &[Arg(0, 548)]),
    ((42, false, INV), &[Arg(0, 579)]),
    ((47, false, INV), &[Arg(0, 587)]),
    ((50, false, INV), &[Arg(0, 619)]),
    ((55, false, INV), &[Arg(0, 627)]),
    ((69, false, INV), &[Arg(0, 699)]),
    ((74, false, INV), &[Arg(0, 707)]),
    ((282, false, NAND2), &[Arg(0, 618), Tv(229)]),
    ((285, false, AND2), &[Tv(283), Tv(284)]),
    ((289, false, AND2), &[Tv(287), Tv(288)]),
    ((298, false, AND2), &[Tv(290), Tv(297)]),
    ((319, false, XNOR2), &[Tv(41), Tv(240)]),
    ((320, false, XNOR2), &[Tv(46), Tv(246)]),
    ((326, false, AND2), &[Tv(322), Tv(325)]),
    ((339, false, AND2), &[Tv(329), Tv(338)]),
    ((347, false, NAND2), &[Arg(0, 540), Tv(203)]),
    ((378, false, AND2), &[Tv(354), Tv(377)]),
    ((380, false, XNOR2), &[Tv(35), Tv(219)]),
    ((381, false, OR2), &[Arg(0, 540), Tv(203)]),
    ((385, false, XNOR2), &[Tv(38), Tv(217)]),
    ((388, false, AND2), &[Tv(386), Tv(387)]),
    ((400, false, AND2), &[Tv(396), Tv(399)]),
    ((435, false, AND2), &[Tv(407), Tv(434)]),
    ((471, false, NAND2), &[Arg(0, 467), Tv(469)]),
    ((949, false, NAND2), &[Tv(470), Tv(948)]),
    ((971, false, OR2), &[Arg(0, 659), Tv(229)]),
    ((972, false, NAND2), &[Tv(63), Tv(256)]),
    ((977, false, AND2), &[Tv(975), Tv(976)]),
    ((980, false, AND2), &[Tv(978), Tv(979)]),
    ((982, false, NAND2), &[Arg(0, 659), Tv(229)]),
    ((997, false, AND2), &[Tv(989), Tv(996)]),
    ((1032, false, XNOR2), &[Arg(2, 0), Arg(0, 705)]),
    ((1033, false, XNOR2), &[Arg(1, 0), Arg(0, 697)]),
    ((1034, false, XOR2), &[Arg(1, 1), Arg(0, 698)]),
    ((1035, false, XOR2), &[Arg(2, 1), Arg(0, 706)]),
    ((1075, false, XNOR2), &[Arg(1, 0), Arg(0, 738)]),
    ((1076, false, XOR2), &[Arg(1, 1), Arg(0, 739)]),
    ((1077, false, XNOR2), &[Arg(2, 0), Arg(0, 746)]),
    ((1079, false, XOR2), &[Arg(2, 1), Arg(0, 747)]),
    ((1100, false, NAND2), &[Arg(0, 468), Tv(469)]),
    ((1131, false, NAND2), &[Tv(470), Tv(1130)]),
    ((1156, false, NAND2), &[Arg(0, 469), Tv(469)]),
    ((1187, false, NAND2), &[Tv(470), Tv(1186)]),
    ((1212, false, NAND2), &[Arg(0, 470), Tv(469)]),
    ((1243, false, NAND2), &[Tv(470), Tv(1242)]),
    ((1268, false, NAND2), &[Arg(0, 471), Tv(469)]),
    ((1299, false, NAND2), &[Tv(470), Tv(1298)]),
    ((1324, false, NAND2), &[Arg(0, 472), Tv(469)]),
    ((1355, false, NAND2), &[Tv(470), Tv(1354)]),
    ((1380, false, NAND2), &[Arg(0, 473), Tv(469)]),
    ((1411, false, NAND2), &[Tv(470), Tv(1410)]),
    ((1436, false, NAND2), &[Arg(0, 474), Tv(469)]),
    ((1467, false, NAND2), &[Tv(470), Tv(1466)]),
    ((1492, false, NAND2), &[Arg(0, 475), Tv(469)]),
    ((1523, false, NAND2), &[Tv(470), Tv(1522)]),
    ((1548, false, NAND2), &[Arg(0, 476), Tv(469)]),
    ((1579, false, NAND2), &[Tv(470), Tv(1578)]),
    ((1604, false, NAND2), &[Arg(0, 477), Tv(469)]),
    ((1635, false, NAND2), &[Tv(470), Tv(1634)]),
    ((1660, false, NAND2), &[Arg(0, 478), Tv(469)]),
    ((1691, false, NAND2), &[Tv(470), Tv(1690)]),
    ((1716, false, NAND2), &[Arg(0, 479), Tv(469)]),
    ((1747, false, NAND2), &[Tv(470), Tv(1746)]),
    ((1772, false, NAND2), &[Arg(0, 480), Tv(469)]),
    ((1803, false, NAND2), &[Tv(470), Tv(1802)]),
    ((1828, false, NAND2), &[Arg(0, 481), Tv(469)]),
    ((1859, false, NAND2), &[Tv(470), Tv(1858)]),
    ((1884, false, NAND2), &[Arg(0, 482), Tv(469)]),
    ((1915, false, NAND2), &[Tv(470), Tv(1914)]),
    ((1940, false, NAND2), &[Arg(0, 483), Tv(469)]),
    ((1971, false, NAND2), &[Tv(470), Tv(1970)]),
    ((1996, false, NAND2), &[Arg(0, 484), Tv(469)]),
    ((2027, false, NAND2), &[Tv(470), Tv(2026)]),
    ((2052, false, NAND2), &[Arg(0, 485), Tv(469)]),
    ((2083, false, NAND2), &[Tv(470), Tv(2082)]),
    ((2108, false, NAND2), &[Arg(0, 486), Tv(469)]),
    ((2139, false, NAND2), &[Tv(470), Tv(2138)]),
    ((2164, false, NAND2), &[Arg(0, 487), Tv(469)]),
    ((2195, false, NAND2), &[Tv(470), Tv(2194)]),
    ((2220, false, NAND2), &[Arg(0, 488), Tv(469)]),
    ((2251, false, NAND2), &[Tv(470), Tv(2250)]),
    ((2276, false, NAND2), &[Arg(0, 489), Tv(469)]),
    ((2307, false, NAND2), &[Tv(470), Tv(2306)]),
    ((2332, false, NAND2), &[Arg(0, 490), Tv(469)]),
    ((2363, false, NAND2), &[Tv(470), Tv(2362)]),
    ((2388, false, NAND2), &[Arg(0, 491), Tv(469)]),
    ((2419, false, NAND2), &[Tv(470), Tv(2418)]),
];

static LEVEL_24: [((usize, bool, CellType), &[GateInput]); 60] = [
    ((43, false, INV), &[Arg(0, 580)]),
    ((48, false, INV), &[Arg(0, 588)]),
    ((51, false, INV), &[Arg(0, 620)]),
    ((56, false, INV), &[Arg(0, 628)]),
    ((58, false, INV), &[Arg(0, 660)]),
    ((64, false, INV), &[Arg(0, 668)]),
    ((70, false, INV), &[Arg(0, 700)]),
    ((75, false, INV), &[Arg(0, 708)]),
    ((78, false, INV), &[Arg(0, 740)]),
    ((82, false, INV), &[Arg(0, 748)]),
    ((83, false, INV), &[Arg(0, 749)]),
    ((279, false, XNOR2), &[Tv(50), Tv(240)]),
    ((280, false, XNOR2), &[Tv(55), Tv(246)]),
    ((286, false, AND2), &[Tv(282), Tv(285)]),
    ((299, false, AND2), &[Tv(289), Tv(298)]),
    ((314, false, XNOR2), &[Tv(42), Tv(227)]),
    ((315, false, XNOR2), &[Tv(47), Tv(223)]),
    ((321, false, AND2), &[Tv(319), Tv(320)]),
    ((340, false, AND2), &[Tv(326), Tv(339)]),
    ((379, false, AND2), &[Tv(347), Tv(378)]),
    ((382, false, AND2), &[Tv(380), Tv(381)]),
    ((384, false, XNOR2), &[Tv(39), Tv(210)]),
    ((389, false, AND2), &[Tv(385), Tv(388)]),
    ((436, false, AND2), &[Tv(400), Tv(435)]),
    ((437, false, NAND2), &[Tv(400), Tv(435)]),
    ((950, false, NAND2), &[Tv(471), Tv(949)]),
    ((970, false, NAND2), &[Arg(0, 667), Tv(255)]),
    ((973, false, AND2), &[Tv(971), Tv(972)]),
    ((981, false, AND2), &[Tv(977), Tv(980)]),
    ((998, false, AND2), &[Tv(982), Tv(997)]),
    ((1025, false, XNOR2), &[Tv(74), Tv(250)]),
    ((1026, false, XNOR2), &[Tv(69), Tv(248)]),
    ((1036, false, AND2), &[Tv(1032), Tv(1034)]),
    ((1037, false, AND2), &[Tv(1033), Tv(1035)]),
    ((1078, false, AND2), &[Tv(1076), Tv(1077)]),
    ((1080, false, AND2), &[Tv(1075), Tv(1079)]),
    ((1132, false, NAND2), &[Tv(1100), Tv(1131)]),
    ((1188, false, NAND2), &[Tv(1156), Tv(1187)]),
    ((1244, false, NAND2), &[Tv(1212), Tv(1243)]),
    ((1300, false, NAND2), &[Tv(1268), Tv(1299)]),
    ((1356, false, NAND2), &[Tv(1324), Tv(1355)]),
    ((1412, false, NAND2), &[Tv(1380), Tv(1411)]),
    ((1468, false, NAND2), &[Tv(1436), Tv(1467)]),
    ((1524, false, NAND2), &[Tv(1492), Tv(1523)]),
    ((1580, false, NAND2), &[Tv(1548), Tv(1579)]),
    ((1636, false, NAND2), &[Tv(1604), Tv(1635)]),
    ((1692, false, NAND2), &[Tv(1660), Tv(1691)]),
    ((1748, false, NAND2), &[Tv(1716), Tv(1747)]),
    ((1804, false, NAND2), &[Tv(1772), Tv(1803)]),
    ((1860, false, NAND2), &[Tv(1828), Tv(1859)]),
    ((1916, false, NAND2), &[Tv(1884), Tv(1915)]),
    ((1972, false, NAND2), &[Tv(1940), Tv(1971)]),
    ((2028, false, NAND2), &[Tv(1996), Tv(2027)]),
    ((2084, false, NAND2), &[Tv(2052), Tv(2083)]),
    ((2140, false, NAND2), &[Tv(2108), Tv(2139)]),
    ((2196, false, NAND2), &[Tv(2164), Tv(2195)]),
    ((2252, false, NAND2), &[Tv(2220), Tv(2251)]),
    ((2308, false, NAND2), &[Tv(2276), Tv(2307)]),
    ((2364, false, NAND2), &[Tv(2332), Tv(2363)]),
    ((2420, false, NAND2), &[Tv(2388), Tv(2419)]),
];

static LEVEL_25: [((usize, bool, CellType), &[GateInput]); 90] = [
    ((52, false, INV), &[Arg(0, 621)]),
    ((57, false, INV), &[Arg(0, 629)]),
    ((59, false, INV), &[Arg(0, 661)]),
    ((65, false, INV), &[Arg(0, 669)]),
    ((71, false, INV), &[Arg(0, 701)]),
    ((76, false, INV), &[Arg(0, 709)]),
    ((87, false, INV), &[Arg(0, 781)]),
    ((90, false, INV), &[Arg(0, 789)]),
    ((230, false, XOR2), &[Arg(2, 1), Arg(0, 788)]),
    ((231, false, XNOR2), &[Arg(1, 0), Arg(0, 779)]),
    ((232, false, XOR2), &[Arg(1, 1), Arg(0, 780)]),
    ((233, false, XNOR2), &[Arg(2, 0), Arg(0, 787)]),
    ((276, false, XNOR2), &[Tv(56), Tv(223)]),
    ((277, false, XNOR2), &[Tv(51), Tv(227)]),
    ((281, false, AND2), &[Tv(279), Tv(280)]),
    ((300, false, AND2), &[Tv(286), Tv(299)]),
    ((306, false, NAND2), &[Arg(0, 589), Tv(210)]),
    ((307, false, OR2), &[Arg(0, 581), Tv(203)]),
    ((309, false, NAND2), &[Arg(0, 581), Tv(203)]),
    ((310, false, XNOR2), &[Tv(43), Tv(219)]),
    ((313, false, XNOR2), &[Tv(48), Tv(217)]),
    ((316, false, AND2), &[Tv(314), Tv(315)]),
    ((318, false, OR2), &[Arg(0, 589), Tv(210)]),
    ((341, false, AND2), &[Tv(321), Tv(340)]),
    ((383, false, AND2), &[Tv(379), Tv(382)]),
    ((390, false, AND2), &[Tv(384), Tv(389)]),
    ((438, false, NAND2), &[Arg(0, 508), Tv(436)]),
    ((951, false, NAND2), &[Tv(437), Tv(950)]),
    ((964, false, XNOR2), &[Tv(58), Tv(240)]),
    ((965, false, XNOR2), &[Tv(64), Tv(246)]),
    ((974, false, AND2), &[Tv(970), Tv(973)]),
    ((999, false, AND2), &[Tv(981), Tv(998)]),
    ((1024, false, XNOR2), &[Tv(75), Tv(255)]),
    ((1027, false, AND2), &[Tv(1025), Tv(1026)]),
    ((1038, false, AND2), &[Tv(1036), Tv(1037)]),
    ((1039, false, XNOR2), &[Tv(70), Tv(229)]),
    ((1067, false, NAND2), &[Arg(0, 749), Tv(255)]),
    ((1068, false, OR2), &[Arg(0, 741), Tv(229)]),
    ((1071, false, XNOR2), &[Tv(82), Tv(250)]),
    ((1072, false, XNOR2), &[Tv(78), Tv(248)]),
    ((1074, false, NAND2), &[Tv(83), Tv(256)]),
    ((1081, false, AND2), &[Tv(1078), Tv(1080)]),
    ((1099, false, NAND2), &[Arg(0, 509), Tv(436)]),
    ((1133, false, NAND2), &[Tv(437), Tv(1132)]),
    ((1155, false, NAND2), &[Arg(0, 510), Tv(436)]),
    ((1189, false, NAND2), &[Tv(437), Tv(1188)]),
    ((1211, false, NAND2), &[Arg(0, 511), Tv(436)]),
    ((1245, false, NAND2), &[Tv(437), Tv(1244)]),
    ((1267, false, NAND2), &[Arg(0, 512), Tv(436)]),
    ((1301, false, NAND2), &[Tv(437), Tv(1300)]),
    ((1323, false, NAND2), &[Arg(0, 513), Tv(436)]),
    ((1357, false, NAND2), &[Tv(437), Tv(1356)]),
    ((1379, false, NAND2), &[Arg(0, 514), Tv(436)]),
    ((1413, false, NAND2), &[Tv(437), Tv(1412)]),
    ((1435, false, NAND2), &[Arg(0, 515), Tv(436)]),
    ((1469, false, NAND2), &[Tv(437), Tv(1468)]),
    ((1491, false, NAND2), &[Arg(0, 516), Tv(436)]),
    ((1525, false, NAND2), &[Tv(437), Tv(1524)]),
    ((1547, false, NAND2), &[Arg(0, 517), Tv(436)]),
    ((1581, false, NAND2), &[Tv(437), Tv(1580)]),
    ((1603, false, NAND2), &[Arg(0, 518), Tv(436)]),
    ((1637, false, NAND2), &[Tv(437), Tv(1636)]),
    ((1659, false, NAND2), &[Arg(0, 519), Tv(436)]),
    ((1693, false, NAND2), &[Tv(437), Tv(1692)]),
    ((1715, false, NAND2), &[Arg(0, 520), Tv(436)]),
    ((1749, false, NAND2), &[Tv(437), Tv(1748)]),
    ((1771, false, NAND2), &[Arg(0, 521), Tv(436)]),
    ((1805, false, NAND2), &[Tv(437), Tv(1804)]),
    ((1827, false, NAND2), &[Arg(0, 522), Tv(436)]),
    ((1861, false, NAND2), &[Tv(437), Tv(1860)]),
    ((1883, false, NAND2), &[Arg(0, 523), Tv(436)]),
    ((1917, false, NAND2), &[Tv(437), Tv(1916)]),
    ((1939, false, NAND2), &[Arg(0, 524), Tv(436)]),
    ((1973, false, NAND2), &[Tv(437), Tv(1972)]),
    ((1995, false, NAND2), &[Arg(0, 525), Tv(436)]),
    ((2029, false, NAND2), &[Tv(437), Tv(2028)]),
    ((2051, false, NAND2), &[Arg(0, 526), Tv(436)]),
    ((2085, false, NAND2), &[Tv(437), Tv(2084)]),
    ((2107, false, NAND2), &[Arg(0, 527), Tv(436)]),
    ((2141, false, NAND2), &[Tv(437), Tv(2140)]),
    ((2163, false, NAND2), &[Arg(0, 528), Tv(436)]),
    ((2197, false, NAND2), &[Tv(437), Tv(2196)]),
    ((2219, false, NAND2), &[Arg(0, 529), Tv(436)]),
    ((2253, false, NAND2), &[Tv(437), Tv(2252)]),
    ((2275, false, NAND2), &[Arg(0, 530), Tv(436)]),
    ((2309, false, NAND2), &[Tv(437), Tv(2308)]),
    ((2331, false, NAND2), &[Arg(0, 531), Tv(436)]),
    ((2365, false, NAND2), &[Tv(437), Tv(2364)]),
    ((2387, false, NAND2), &[Arg(0, 532), Tv(436)]),
    ((2421, false, NAND2), &[Tv(437), Tv(2420)]),
];

static LEVEL_26: [((usize, bool, CellType), &[GateInput]); 64] = [
    ((60, false, INV), &[Arg(0, 662)]),
    ((66, false, INV), &[Arg(0, 670)]),
    ((72, false, INV), &[Arg(0, 702)]),
    ((79, false, INV), &[Arg(0, 742)]),
    ((84, false, INV), &[Arg(0, 750)]),
    ((88, false, INV), &[Arg(0, 782)]),
    ((91, false, INV), &[Arg(0, 790)]),
    ((234, false, AND2), &[Tv(230), Tv(232)]),
    ((235, false, AND2), &[Tv(231), Tv(233)]),
    ((249, false, XNOR2), &[Tv(87), Tv(248)]),
    ((251, false, XNOR2), &[Tv(90), Tv(250)]),
    ((266, false, OR2), &[Arg(0, 630), Tv(210)]),
    ((267, false, OR2), &[Arg(0, 622), Tv(203)]),
    ((269, false, NAND2), &[Arg(0, 622), Tv(203)]),
    ((270, false, NAND2), &[Arg(0, 630), Tv(210)]),
    ((273, false, XNOR2), &[Tv(52), Tv(219)]),
    ((274, false, XNOR2), &[Tv(57), Tv(217)]),
    ((278, false, AND2), &[Tv(276), Tv(277)]),
    ((301, false, AND2), &[Tv(281), Tv(300)]),
    ((308, false, AND2), &[Tv(306), Tv(307)]),
    ((311, false, AND2), &[Tv(309), Tv(310)]),
    ((317, false, AND2), &[Tv(313), Tv(316)]),
    ((342, false, AND2), &[Tv(318), Tv(341)]),
    ((391, false, AND2), &[Tv(383), Tv(390)]),
    ((392, false, NAND2), &[Tv(383), Tv(390)]),
    ((952, false, NAND2), &[Tv(438), Tv(951)]),
    ((966, false, AND2), &[Tv(964), Tv(965)]),
    ((967, false, XNOR2), &[Tv(59), Tv(227)]),
    ((969, false, XNOR2), &[Tv(65), Tv(223)]),
    ((1000, false, AND2), &[Tv(974), Tv(999)]),
    ((1019, false, NAND2), &[Arg(0, 710), Tv(223)]),
    ((1020, false, XNOR2), &[Tv(71), Tv(240)]),
    ((1028, false, AND2), &[Tv(1024), Tv(1027)]),
    ((1029, false, XNOR2), &[Tv(76), Tv(246)]),
    ((1031, false, OR2), &[Arg(0, 710), Tv(223)]),
    ((1040, false, AND2), &[Tv(1038), Tv(1039)]),
    ((1066, false, NAND2), &[Arg(0, 741), Tv(229)]),
    ((1069, false, AND2), &[Tv(1067), Tv(1068)]),
    ((1073, false, AND2), &[Tv(1071), Tv(1072)]),
    ((1082, false, AND2), &[Tv(1074), Tv(1081)]),
    ((1134, false, NAND2), &[Tv(1099), Tv(1133)]),
    ((1190, false, NAND2), &[Tv(1155), Tv(1189)]),
    ((1246, false, NAND2), &[Tv(1211), Tv(1245)]),
    ((1302, false, NAND2), &[Tv(1267), Tv(1301)]),
    ((1358, false, NAND2), &[Tv(1323), Tv(1357)]),
    ((1414, false, NAND2), &[Tv(1379), Tv(1413)]),
    ((1470, false, NAND2), &[Tv(1435), Tv(1469)]),
    ((1526, false, NAND2), &[Tv(1491), Tv(1525)]),
    ((1582, false, NAND2), &[Tv(1547), Tv(1581)]),
    ((1638, false, NAND2), &[Tv(1603), Tv(1637)]),
    ((1694, false, NAND2), &[Tv(1659), Tv(1693)]),
    ((1750, false, NAND2), &[Tv(1715), Tv(1749)]),
    ((1806, false, NAND2), &[Tv(1771), Tv(1805)]),
    ((1862, false, NAND2), &[Tv(1827), Tv(1861)]),
    ((1918, false, NAND2), &[Tv(1883), Tv(1917)]),
    ((1974, false, NAND2), &[Tv(1939), Tv(1973)]),
    ((2030, false, NAND2), &[Tv(1995), Tv(2029)]),
    ((2086, false, NAND2), &[Tv(2051), Tv(2085)]),
    ((2142, false, NAND2), &[Tv(2107), Tv(2141)]),
    ((2198, false, NAND2), &[Tv(2163), Tv(2197)]),
    ((2254, false, NAND2), &[Tv(2219), Tv(2253)]),
    ((2310, false, NAND2), &[Tv(2275), Tv(2309)]),
    ((2366, false, NAND2), &[Tv(2331), Tv(2365)]),
    ((2422, false, NAND2), &[Tv(2387), Tv(2421)]),
];

static LEVEL_27: [((usize, bool, CellType), &[GateInput]); 81] = [
    ((61, false, INV), &[Arg(0, 663)]),
    ((67, false, INV), &[Arg(0, 671)]),
    ((73, false, INV), &[Arg(0, 703)]),
    ((77, false, INV), &[Arg(0, 711)]),
    ((80, false, INV), &[Arg(0, 743)]),
    ((85, false, INV), &[Arg(0, 751)]),
    ((89, false, INV), &[Arg(0, 783)]),
    ((236, false, AND2), &[Tv(234), Tv(235)]),
    ((237, false, XNOR2), &[Tv(88), Tv(229)]),
    ((247, false, NAND2), &[Arg(0, 791), Tv(246)]),
    ((252, false, AND2), &[Tv(249), Tv(251)]),
    ((254, false, OR2), &[Arg(0, 791), Tv(246)]),
    ((257, false, XNOR2), &[Tv(91), Tv(255)]),
    ((268, false, AND2), &[Tv(266), Tv(267)]),
    ((271, false, AND2), &[Tv(269), Tv(270)]),
    ((275, false, AND2), &[Tv(273), Tv(274)]),
    ((302, false, AND2), &[Tv(278), Tv(301)]),
    ((312, false, AND2), &[Tv(308), Tv(311)]),
    ((343, false, AND2), &[Tv(317), Tv(342)]),
    ((393, false, NAND2), &[Arg(0, 549), Tv(391)]),
    ((953, false, NAND2), &[Tv(392), Tv(952)]),
    ((959, false, XNOR2), &[Tv(66), Tv(217)]),
    ((960, false, XNOR2), &[Tv(60), Tv(219)]),
    ((968, false, AND2), &[Tv(966), Tv(967)]),
    ((1001, false, AND2), &[Tv(969), Tv(1000)]),
    ((1021, false, AND2), &[Tv(1019), Tv(1020)]),
    ((1022, false, XNOR2), &[Tv(72), Tv(227)]),
    ((1030, false, AND2), &[Tv(1028), Tv(1029)]),
    ((1041, false, AND2), &[Tv(1031), Tv(1040)]),
    ((1063, false, XNOR2), &[Tv(79), Tv(240)]),
    ((1064, false, XNOR2), &[Tv(84), Tv(246)]),
    ((1070, false, AND2), &[Tv(1066), Tv(1069)]),
    ((1083, false, AND2), &[Tv(1073), Tv(1082)]),
    ((1098, false, NAND2), &[Arg(0, 550), Tv(391)]),
    ((1135, false, NAND2), &[Tv(392), Tv(1134)]),
    ((1154, false, NAND2), &[Arg(0, 551), Tv(391)]),
    ((1191, false, NAND2), &[Tv(392), Tv(1190)]),
    ((1210, false, NAND2), &[Arg(0, 552), Tv(391)]),
    ((1247, false, NAND2), &[Tv(392), Tv(1246)]),
    ((1266, false, NAND2), &[Arg(0, 553), Tv(391)]),
    ((1303, false, NAND2), &[Tv(392), Tv(1302)]),
    ((1322, false, NAND2), &[Arg(0, 554), Tv(391)]),
    ((1359, false, NAND2), &[Tv(392), Tv(1358)]),
    ((1378, false, NAND2), &[Arg(0, 555), Tv(391)]),
    ((1415, false, NAND2), &[Tv(392), Tv(1414)]),
    ((1434, false, NAND2), &[Arg(0, 556), Tv(391)]),
    ((1471, false, NAND2), &[Tv(392), Tv(1470)]),
    ((1490, false, NAND2), &[Arg(0, 557), Tv(391)]),
    ((1527, false, NAND2), &[Tv(392), Tv(1526)]),
    ((1546, false, NAND2), &[Arg(0, 558), Tv(391)]),
    ((1583, false, NAND2), &[Tv(392), Tv(1582)]),
    ((1602, false, NAND2), &[Arg(0, 559), Tv(391)]),
    ((1639, false, NAND2), &[Tv(392), Tv(1638)]),
    ((1658, false, NAND2), &[Arg(0, 560), Tv(391)]),
    ((1695, false, NAND2), &[Tv(392), Tv(1694)]),
    ((1714, false, NAND2), &[Arg(0, 561), Tv(391)]),
    ((1751, false, NAND2), &[Tv(392), Tv(1750)]),
    ((1770, false, NAND2), &[Arg(0, 562), Tv(391)]),
    ((1807, false, NAND2), &[Tv(392), Tv(1806)]),
    ((1826, false, NAND2), &[Arg(0, 563), Tv(391)]),
    ((1863, false, NAND2), &[Tv(392), Tv(1862)]),
    ((1882, false, NAND2), &[Arg(0, 564), Tv(391)]),
    ((1919, false, NAND2), &[Tv(392), Tv(1918)]),
    ((1938, false, NAND2), &[Arg(0, 565), Tv(391)]),
    ((1975, false, NAND2), &[Tv(392), Tv(1974)]),
    ((1994, false, NAND2), &[Arg(0, 566), Tv(391)]),
    ((2031, false, NAND2), &[Tv(392), Tv(2030)]),
    ((2050, false, NAND2), &[Arg(0, 567), Tv(391)]),
    ((2087, false, NAND2), &[Tv(392), Tv(2086)]),
    ((2106, false, NAND2), &[Arg(0, 568), Tv(391)]),
    ((2143, false, NAND2), &[Tv(392), Tv(2142)]),
    ((2162, false, NAND2), &[Arg(0, 569), Tv(391)]),
    ((2199, false, NAND2), &[Tv(392), Tv(2198)]),
    ((2218, false, NAND2), &[Arg(0, 570), Tv(391)]),
    ((2255, false, NAND2), &[Tv(392), Tv(2254)]),
    ((2274, false, NAND2), &[Arg(0, 571), Tv(391)]),
    ((2311, false, NAND2), &[Tv(392), Tv(2310)]),
    ((2330, false, NAND2), &[Arg(0, 572), Tv(391)]),
    ((2367, false, NAND2), &[Tv(392), Tv(2366)]),
    ((2386, false, NAND2), &[Arg(0, 573), Tv(391)]),
    ((2423, false, NAND2), &[Tv(392), Tv(2422)]),
];

static LEVEL_28: [((usize, bool, CellType), &[GateInput]); 54] = [
    ((81, false, INV), &[Arg(0, 744)]),
    ((86, false, INV), &[Arg(0, 752)]),
    ((92, false, INV), &[Arg(0, 792)]),
    ((228, false, NAND2), &[Arg(0, 784), Tv(227)]),
    ((238, false, AND2), &[Tv(236), Tv(237)]),
    ((241, false, XNOR2), &[Tv(89), Tv(240)]),
    ((242, false, OR2), &[Arg(0, 784), Tv(227)]),
    ((253, false, AND2), &[Tv(247), Tv(252)]),
    ((258, false, AND2), &[Tv(254), Tv(257)]),
    ((272, false, AND2), &[Tv(268), Tv(271)]),
    ((303, false, AND2), &[Tv(275), Tv(302)]),
    ((344, false, AND2), &[Tv(312), Tv(343)]),
    ((345, false, NAND2), &[Tv(312), Tv(343)]),
    ((954, false, NAND2), &[Tv(393), Tv(953)]),
    ((958, false, XNOR2), &[Tv(61), Tv(203)]),
    ((961, false, AND2), &[Tv(959), Tv(960)]),
    ((963, false, XNOR2), &[Tv(67), Tv(210)]),
    ((1002, false, AND2), &[Tv(968), Tv(1001)]),
    ((1009, false, OR2), &[Arg(0, 704), Tv(203)]),
    ((1010, false, OR2), &[Arg(0, 712), Tv(210)]),
    ((1012, false, NAND2), &[Arg(0, 712), Tv(210)]),
    ((1013, false, NAND2), &[Arg(0, 704), Tv(203)]),
    ((1016, false, XNOR2), &[Tv(77), Tv(217)]),
    ((1017, false, XNOR2), &[Tv(73), Tv(219)]),
    ((1023, false, AND2), &[Tv(1021), Tv(1022)]),
    ((1042, false, AND2), &[Tv(1030), Tv(1041)]),
    ((1060, false, XNOR2), &[Tv(85), Tv(223)]),
    ((1061, false, XNOR2), &[Tv(80), Tv(227)]),
    ((1065, false, AND2), &[Tv(1063), Tv(1064)]),
    ((1084, false, AND2), &[Tv(1070), Tv(1083)]),
    ((1136, false, NAND2), &[Tv(1098), Tv(1135)]),
    ((1192, false, NAND2), &[Tv(1154), Tv(1191)]),
    ((1248, false, NAND2), &[Tv(1210), Tv(1247)]),
    ((1304, false, NAND2), &[Tv(1266), Tv(1303)]),
    ((1360, false, NAND2), &[Tv(1322), Tv(1359)]),
    ((1416, false, NAND2), &[Tv(1378), Tv(1415)]),
    ((1472, false, NAND2), &[Tv(1434), Tv(1471)]),
    ((1528, false, NAND2), &[Tv(1490), Tv(1527)]),
    ((1584, false, NAND2), &[Tv(1546), Tv(1583)]),
    ((1640, false, NAND2), &[Tv(1602), Tv(1639)]),
    ((1696, false, NAND2), &[Tv(1658), Tv(1695)]),
    ((1752, false, NAND2), &[Tv(1714), Tv(1751)]),
    ((1808, false, NAND2), &[Tv(1770), Tv(1807)]),
    ((1864, false, NAND2), &[Tv(1826), Tv(1863)]),
    ((1920, false, NAND2), &[Tv(1882), Tv(1919)]),
    ((1976, false, NAND2), &[Tv(1938), Tv(1975)]),
    ((2032, false, NAND2), &[Tv(1994), Tv(2031)]),
    ((2088, false, NAND2), &[Tv(2050), Tv(2087)]),
    ((2144, false, NAND2), &[Tv(2106), Tv(2143)]),
    ((2200, false, NAND2), &[Tv(2162), Tv(2199)]),
    ((2256, false, NAND2), &[Tv(2218), Tv(2255)]),
    ((2312, false, NAND2), &[Tv(2274), Tv(2311)]),
    ((2368, false, NAND2), &[Tv(2330), Tv(2367)]),
    ((2424, false, NAND2), &[Tv(2386), Tv(2423)]),
];

static LEVEL_29: [((usize, bool, CellType), &[GateInput]); 73] = [
    ((218, false, NAND2), &[Arg(0, 793), Tv(217)]),
    ((220, false, OR2), &[Arg(0, 785), Tv(219)]),
    ((222, false, NAND2), &[Arg(0, 785), Tv(219)]),
    ((224, false, XNOR2), &[Tv(92), Tv(223)]),
    ((239, false, AND2), &[Tv(228), Tv(238)]),
    ((243, false, AND2), &[Tv(241), Tv(242)]),
    ((245, false, OR2), &[Arg(0, 793), Tv(217)]),
    ((259, false, AND2), &[Tv(253), Tv(258)]),
    ((304, false, AND2), &[Tv(272), Tv(303)]),
    ((346, false, NAND2), &[Arg(0, 590), Tv(344)]),
    ((955, false, NAND2), &[Tv(345), Tv(954)]),
    ((962, false, AND2), &[Tv(958), Tv(961)]),
    ((1003, false, AND2), &[Tv(963), Tv(1002)]),
    ((1011, false, AND2), &[Tv(1009), Tv(1010)]),
    ((1014, false, AND2), &[Tv(1012), Tv(1013)]),
    ((1018, false, AND2), &[Tv(1016), Tv(1017)]),
    ((1043, false, AND2), &[Tv(1023), Tv(1042)]),
    ((1050, false, OR2), &[Arg(0, 745), Tv(203)]),
    ((1051, false, OR2), &[Arg(0, 753), Tv(210)]),
    ((1053, false, NAND2), &[Arg(0, 753), Tv(210)]),
    ((1054, false, NAND2), &[Arg(0, 745), Tv(203)]),
    ((1057, false, XNOR2), &[Tv(81), Tv(219)]),
    ((1058, false, XNOR2), &[Tv(86), Tv(217)]),
    ((1062, false, AND2), &[Tv(1060), Tv(1061)]),
    ((1085, false, AND2), &[Tv(1065), Tv(1084)]),
    ((1097, false, NAND2), &[Arg(0, 591), Tv(344)]),
    ((1137, false, NAND2), &[Tv(345), Tv(1136)]),
    ((1153, false, NAND2), &[Arg(0, 592), Tv(344)]),
    ((1193, false, NAND2), &[Tv(345), Tv(1192)]),
    ((1209, false, NAND2), &[Arg(0, 593), Tv(344)]),
    ((1249, false, NAND2), &[Tv(345), Tv(1248)]),
    ((1265, false, NAND2), &[Arg(0, 594), Tv(344)]),
    ((1305, false, NAND2), &[Tv(345), Tv(1304)]),
    ((1321, false, NAND2), &[Arg(0, 595), Tv(344)]),
    ((1361, false, NAND2), &[Tv(345), Tv(1360)]),
    ((1377, false, NAND2), &[Arg(0, 596), Tv(344)]),
    ((1417, false, NAND2), &[Tv(345), Tv(1416)]),
    ((1433, false, NAND2), &[Arg(0, 597), Tv(344)]),
    ((1473, false, NAND2), &[Tv(345), Tv(1472)]),
    ((1489, false, NAND2), &[Arg(0, 598), Tv(344)]),
    ((1529, false, NAND2), &[Tv(345), Tv(1528)]),
    ((1545, false, NAND2), &[Arg(0, 599), Tv(344)]),
    ((1585, false, NAND2), &[Tv(345), Tv(1584)]),
    ((1601, false, NAND2), &[Arg(0, 600), Tv(344)]),
    ((1641, false, NAND2), &[Tv(345), Tv(1640)]),
    ((1657, false, NAND2), &[Arg(0, 601), Tv(344)]),
    ((1697, false, NAND2), &[Tv(345), Tv(1696)]),
    ((1713, false, NAND2), &[Arg(0, 602), Tv(344)]),
    ((1753, false, NAND2), &[Tv(345), Tv(1752)]),
    ((1769, false, NAND2), &[Arg(0, 603), Tv(344)]),
    ((1809, false, NAND2), &[Tv(345), Tv(1808)]),
    ((1825, false, NAND2), &[Arg(0, 604), Tv(344)]),
    ((1865, false, NAND2), &[Tv(345), Tv(1864)]),
    ((1881, false, NAND2), &[Arg(0, 605), Tv(344)]),
    ((1921, false, NAND2), &[Tv(345), Tv(1920)]),
    ((1937, false, NAND2), &[Arg(0, 606), Tv(344)]),
    ((1977, false, NAND2), &[Tv(345), Tv(1976)]),
    ((1993, false, NAND2), &[Arg(0, 607), Tv(344)]),
    ((2033, false, NAND2), &[Tv(345), Tv(2032)]),
    ((2049, false, NAND2), &[Arg(0, 608), Tv(344)]),
    ((2089, false, NAND2), &[Tv(345), Tv(2088)]),
    ((2105, false, NAND2), &[Arg(0, 609), Tv(344)]),
    ((2145, false, NAND2), &[Tv(345), Tv(2144)]),
    ((2161, false, NAND2), &[Arg(0, 610), Tv(344)]),
    ((2201, false, NAND2), &[Tv(345), Tv(2200)]),
    ((2217, false, NAND2), &[Arg(0, 611), Tv(344)]),
    ((2257, false, NAND2), &[Tv(345), Tv(2256)]),
    ((2273, false, NAND2), &[Arg(0, 612), Tv(344)]),
    ((2313, false, NAND2), &[Tv(345), Tv(2312)]),
    ((2329, false, NAND2), &[Arg(0, 613), Tv(344)]),
    ((2369, false, NAND2), &[Tv(345), Tv(2368)]),
    ((2385, false, NAND2), &[Arg(0, 614), Tv(344)]),
    ((2425, false, NAND2), &[Tv(345), Tv(2424)]),
];

static LEVEL_30: [((usize, bool, CellType), &[GateInput]); 92] = [
    ((2480, false, INV), &[Arg(0, 672)]),
    ((2481, false, INV), &[Arg(0, 673)]),
    ((2482, false, INV), &[Arg(0, 674)]),
    ((2483, false, INV), &[Arg(0, 675)]),
    ((2484, false, INV), &[Arg(0, 676)]),
    ((2485, false, INV), &[Arg(0, 677)]),
    ((2486, false, INV), &[Arg(0, 678)]),
    ((2487, false, INV), &[Arg(0, 679)]),
    ((2528, false, INV), &[Arg(0, 680)]),
    ((2529, false, INV), &[Arg(0, 681)]),
    ((2530, false, INV), &[Arg(0, 682)]),
    ((2531, false, INV), &[Arg(0, 683)]),
    ((2532, false, INV), &[Arg(0, 684)]),
    ((2533, false, INV), &[Arg(0, 685)]),
    ((2534, false, INV), &[Arg(0, 686)]),
    ((2535, false, INV), &[Arg(0, 687)]),
    ((2576, false, INV), &[Arg(0, 688)]),
    ((2577, false, INV), &[Arg(0, 689)]),
    ((0, false, INV), &[Arg(0, 690)]),
    ((1, false, INV), &[Arg(0, 691)]),
    ((2, false, INV), &[Arg(0, 692)]),
    ((3, false, INV), &[Arg(0, 693)]),
    ((4, false, INV), &[Arg(0, 694)]),
    ((5, false, INV), &[Arg(0, 695)]),
    ((18, false, INV), &[Arg(0, 696)]),
    ((204, false, OR2), &[Arg(0, 786), Tv(203)]),
    ((211, false, OR2), &[Arg(0, 794), Tv(210)]),
    ((213, false, NAND2), &[Arg(0, 794), Tv(210)]),
    ((214, false, NAND2), &[Arg(0, 786), Tv(203)]),
    ((221, false, AND2), &[Tv(218), Tv(220)]),
    ((225, false, AND2), &[Tv(222), Tv(224)]),
    ((244, false, AND2), &[Tv(239), Tv(243)]),
    ((260, false, AND2), &[Tv(245), Tv(259)]),
    ((305, false, NAND2), &[Tv(272), Tv(303)]),
    ((956, false, NAND2), &[Tv(346), Tv(955)]),
    ((1004, false, AND2), &[Tv(962), Tv(1003)]),
    ((1005, false, NAND2), &[Tv(962), Tv(1003)]),
    ((1006, false, NAND2), &[Arg(0, 631), Tv(304)]),
    ((1015, false, AND2), &[Tv(1011), Tv(1014)]),
    ((1044, false, AND2), &[Tv(1018), Tv(1043)]),
    ((1052, false, AND2), &[Tv(1050), Tv(1051)]),
    ((1055, false, AND2), &[Tv(1053), Tv(1054)]),
    ((1059, false, AND2), &[Tv(1057), Tv(1058)]),
    ((1086, false, AND2), &[Tv(1062), Tv(1085)]),
    ((1138, false, NAND2), &[Tv(1097), Tv(1137)]),
    ((1140, false, NAND2), &[Arg(0, 632), Tv(304)]),
    ((1194, false, NAND2), &[Tv(1153), Tv(1193)]),
    ((1196, false, NAND2), &[Arg(0, 633), Tv(304)]),
    ((1250, false, NAND2), &[Tv(1209), Tv(1249)]),
    ((1252, false, NAND2), &[Arg(0, 634), Tv(304)]),
    ((1306, false, NAND2), &[Tv(1265), Tv(1305)]),
    ((1308, false, NAND2), &[Arg(0, 635), Tv(304)]),
    ((1362, false, NAND2), &[Tv(1321), Tv(1361)]),
    ((1364, false, NAND2), &[Arg(0, 636), Tv(304)]),
    ((1418, false, NAND2), &[Tv(1377), Tv(1417)]),
    ((1420, false, NAND2), &[Arg(0, 637), Tv(304)]),
    ((1474, false, NAND2), &[Tv(1433), Tv(1473)]),
    ((1476, false, NAND2), &[Arg(0, 638), Tv(304)]),
    ((1530, false, NAND2), &[Tv(1489), Tv(1529)]),
    ((1532, false, NAND2), &[Arg(0, 639), Tv(304)]),
    ((1586, false, NAND2), &[Tv(1545), Tv(1585)]),
    ((1588, false, NAND2), &[Arg(0, 640), Tv(304)]),
    ((1642, false, NAND2), &[Tv(1601), Tv(1641)]),
    ((1644, false, NAND2), &[Arg(0, 641), Tv(304)]),
    ((1698, false, NAND2), &[Tv(1657), Tv(1697)]),
    ((1700, false, NAND2), &[Arg(0, 642), Tv(304)]),
    ((1754, false, NAND2), &[Tv(1713), Tv(1753)]),
    ((1756, false, NAND2), &[Arg(0, 643), Tv(304)]),
    ((1810, false, NAND2), &[Tv(1769), Tv(1809)]),
    ((1812, false, NAND2), &[Arg(0, 644), Tv(304)]),
    ((1866, false, NAND2), &[Tv(1825), Tv(1865)]),
    ((1868, false, NAND2), &[Arg(0, 645), Tv(304)]),
    ((1922, false, NAND2), &[Tv(1881), Tv(1921)]),
    ((1924, false, NAND2), &[Arg(0, 646), Tv(304)]),
    ((1978, false, NAND2), &[Tv(1937), Tv(1977)]),
    ((1980, false, NAND2), &[Arg(0, 647), Tv(304)]),
    ((2034, false, NAND2), &[Tv(1993), Tv(2033)]),
    ((2036, false, NAND2), &[Arg(0, 648), Tv(304)]),
    ((2090, false, NAND2), &[Tv(2049), Tv(2089)]),
    ((2092, false, NAND2), &[Arg(0, 649), Tv(304)]),
    ((2146, false, NAND2), &[Tv(2105), Tv(2145)]),
    ((2148, false, NAND2), &[Arg(0, 650), Tv(304)]),
    ((2202, false, NAND2), &[Tv(2161), Tv(2201)]),
    ((2204, false, NAND2), &[Arg(0, 651), Tv(304)]),
    ((2258, false, NAND2), &[Tv(2217), Tv(2257)]),
    ((2260, false, NAND2), &[Arg(0, 652), Tv(304)]),
    ((2314, false, NAND2), &[Tv(2273), Tv(2313)]),
    ((2316, false, NAND2), &[Arg(0, 653), Tv(304)]),
    ((2370, false, NAND2), &[Tv(2329), Tv(2369)]),
    ((2372, false, NAND2), &[Arg(0, 654), Tv(304)]),
    ((2426, false, NAND2), &[Tv(2385), Tv(2425)]),
    ((2428, false, NAND2), &[Arg(0, 655), Tv(304)]),
];

static LEVEL_31: [((usize, bool, CellType), &[GateInput]); 83] = [
    ((212, false, AND2), &[Tv(204), Tv(211)]),
    ((215, false, AND2), &[Tv(213), Tv(214)]),
    ((226, false, AND2), &[Tv(221), Tv(225)]),
    ((261, false, AND2), &[Tv(244), Tv(260)]),
    ((957, false, NAND2), &[Tv(305), Tv(956)]),
    ((1007, false, AND2), &[Tv(1005), Tv(1006)]),
    ((1045, false, AND2), &[Tv(1015), Tv(1044)]),
    ((1046, false, NAND2), &[Tv(1015), Tv(1044)]),
    ((1047, false, NAND2), &[Tv(2480), Tv(1004)]),
    ((1056, false, AND2), &[Tv(1052), Tv(1055)]),
    ((1087, false, AND2), &[Tv(1059), Tv(1086)]),
    ((1139, false, NAND2), &[Tv(305), Tv(1138)]),
    ((1141, false, AND2), &[Tv(1005), Tv(1140)]),
    ((1143, false, NAND2), &[Tv(2481), Tv(1004)]),
    ((1195, false, NAND2), &[Tv(305), Tv(1194)]),
    ((1197, false, AND2), &[Tv(1005), Tv(1196)]),
    ((1199, false, NAND2), &[Tv(2482), Tv(1004)]),
    ((1251, false, NAND2), &[Tv(305), Tv(1250)]),
    ((1253, false, AND2), &[Tv(1005), Tv(1252)]),
    ((1255, false, NAND2), &[Tv(2483), Tv(1004)]),
    ((1307, false, NAND2), &[Tv(305), Tv(1306)]),
    ((1309, false, AND2), &[Tv(1005), Tv(1308)]),
    ((1311, false, NAND2), &[Tv(2484), Tv(1004)]),
    ((1363, false, NAND2), &[Tv(305), Tv(1362)]),
    ((1365, false, AND2), &[Tv(1005), Tv(1364)]),
    ((1367, false, NAND2), &[Tv(2485), Tv(1004)]),
    ((1419, false, NAND2), &[Tv(305), Tv(1418)]),
    ((1421, false, AND2), &[Tv(1005), Tv(1420)]),
    ((1423, false, NAND2), &[Tv(2486), Tv(1004)]),
    ((1475, false, NAND2), &[Tv(305), Tv(1474)]),
    ((1477, false, AND2), &[Tv(1005), Tv(1476)]),
    ((1479, false, NAND2), &[Tv(2487), Tv(1004)]),
    ((1531, false, NAND2), &[Tv(305), Tv(1530)]),
    ((1533, false, AND2), &[Tv(1005), Tv(1532)]),
    ((1535, false, NAND2), &[Tv(2528), Tv(1004)]),
    ((1587, false, NAND2), &[Tv(305), Tv(1586)]),
    ((1589, false, AND2), &[Tv(1005), Tv(1588)]),
    ((1591, false, NAND2), &[Tv(2529), Tv(1004)]),
    ((1643, false, NAND2), &[Tv(305), Tv(1642)]),
    ((1645, false, AND2), &[Tv(1005), Tv(1644)]),
    ((1647, false, NAND2), &[Tv(2530), Tv(1004)]),
    ((1699, false, NAND2), &[Tv(305), Tv(1698)]),
    ((1701, false, AND2), &[Tv(1005), Tv(1700)]),
    ((1703, false, NAND2), &[Tv(2531), Tv(1004)]),
    ((1755, false, NAND2), &[Tv(305), Tv(1754)]),
    ((1757, false, AND2), &[Tv(1005), Tv(1756)]),
    ((1759, false, NAND2), &[Tv(2532), Tv(1004)]),
    ((1811, false, NAND2), &[Tv(305), Tv(1810)]),
    ((1813, false, AND2), &[Tv(1005), Tv(1812)]),
    ((1815, false, NAND2), &[Tv(2533), Tv(1004)]),
    ((1867, false, NAND2), &[Tv(305), Tv(1866)]),
    ((1869, false, AND2), &[Tv(1005), Tv(1868)]),
    ((1871, false, NAND2), &[Tv(2534), Tv(1004)]),
    ((1923, false, NAND2), &[Tv(305), Tv(1922)]),
    ((1925, false, AND2), &[Tv(1005), Tv(1924)]),
    ((1927, false, NAND2), &[Tv(2535), Tv(1004)]),
    ((1979, false, NAND2), &[Tv(305), Tv(1978)]),
    ((1981, false, AND2), &[Tv(1005), Tv(1980)]),
    ((1983, false, NAND2), &[Tv(2576), Tv(1004)]),
    ((2035, false, NAND2), &[Tv(305), Tv(2034)]),
    ((2037, false, AND2), &[Tv(1005), Tv(2036)]),
    ((2039, false, NAND2), &[Tv(2577), Tv(1004)]),
    ((2091, false, NAND2), &[Tv(305), Tv(2090)]),
    ((2093, false, AND2), &[Tv(1005), Tv(2092)]),
    ((2095, false, NAND2), &[Tv(0), Tv(1004)]),
    ((2147, false, NAND2), &[Tv(305), Tv(2146)]),
    ((2149, false, AND2), &[Tv(1005), Tv(2148)]),
    ((2151, false, NAND2), &[Tv(1), Tv(1004)]),
    ((2203, false, NAND2), &[Tv(305), Tv(2202)]),
    ((2205, false, AND2), &[Tv(1005), Tv(2204)]),
    ((2207, false, NAND2), &[Tv(2), Tv(1004)]),
    ((2259, false, NAND2), &[Tv(305), Tv(2258)]),
    ((2261, false, AND2), &[Tv(1005), Tv(2260)]),
    ((2263, false, NAND2), &[Tv(3), Tv(1004)]),
    ((2315, false, NAND2), &[Tv(305), Tv(2314)]),
    ((2317, false, AND2), &[Tv(1005), Tv(2316)]),
    ((2319, false, NAND2), &[Tv(4), Tv(1004)]),
    ((2371, false, NAND2), &[Tv(305), Tv(2370)]),
    ((2373, false, AND2), &[Tv(1005), Tv(2372)]),
    ((2375, false, NAND2), &[Tv(5), Tv(1004)]),
    ((2427, false, NAND2), &[Tv(305), Tv(2426)]),
    ((2429, false, AND2), &[Tv(1005), Tv(2428)]),
    ((2431, false, NAND2), &[Tv(18), Tv(1004)]),
];

static LEVEL_32: [((usize, bool, CellType), &[GateInput]); 104] = [
    ((2488, false, INV), &[Arg(0, 754)]),
    ((2489, false, INV), &[Arg(0, 755)]),
    ((2490, false, INV), &[Arg(0, 756)]),
    ((2491, false, INV), &[Arg(0, 757)]),
    ((2492, false, INV), &[Arg(0, 758)]),
    ((2493, false, INV), &[Arg(0, 759)]),
    ((2494, false, INV), &[Arg(0, 760)]),
    ((2495, false, INV), &[Arg(0, 761)]),
    ((2536, false, INV), &[Arg(0, 762)]),
    ((2537, false, INV), &[Arg(0, 763)]),
    ((2538, false, INV), &[Arg(0, 764)]),
    ((2539, false, INV), &[Arg(0, 765)]),
    ((2540, false, INV), &[Arg(0, 766)]),
    ((2541, false, INV), &[Arg(0, 767)]),
    ((2542, false, INV), &[Arg(0, 768)]),
    ((2543, false, INV), &[Arg(0, 769)]),
    ((6, false, INV), &[Arg(0, 770)]),
    ((7, false, INV), &[Arg(0, 771)]),
    ((8, false, INV), &[Arg(0, 772)]),
    ((9, false, INV), &[Arg(0, 773)]),
    ((10, false, INV), &[Arg(0, 774)]),
    ((11, false, INV), &[Arg(0, 775)]),
    ((12, false, INV), &[Arg(0, 776)]),
    ((13, false, INV), &[Arg(0, 777)]),
    ((19, false, INV), &[Arg(0, 778)]),
    ((216, false, AND2), &[Tv(212), Tv(215)]),
    ((262, false, AND2), &[Tv(226), Tv(261)]),
    ((1008, false, NAND2), &[Tv(957), Tv(1007)]),
    ((1048, false, AND2), &[Tv(1046), Tv(1047)]),
    ((1088, false, AND2), &[Tv(1056), Tv(1087)]),
    ((1089, false, NAND2), &[Tv(1056), Tv(1087)]),
    ((1090, false, NAND2), &[Arg(0, 713), Tv(1045)]),
    ((1142, false, NAND2), &[Tv(1139), Tv(1141)]),
    ((1144, false, AND2), &[Tv(1046), Tv(1143)]),
    ((1146, false, NAND2), &[Arg(0, 714), Tv(1045)]),
    ((1198, false, NAND2), &[Tv(1195), Tv(1197)]),
    ((1200, false, AND2), &[Tv(1046), Tv(1199)]),
    ((1202, false, NAND2), &[Arg(0, 715), Tv(1045)]),
    ((1254, false, NAND2), &[Tv(1251), Tv(1253)]),
    ((1256, false, AND2), &[Tv(1046), Tv(1255)]),
    ((1258, false, NAND2), &[Arg(0, 716), Tv(1045)]),
    ((1310, false, NAND2), &[Tv(1307), Tv(1309)]),
    ((1312, false, AND2), &[Tv(1046), Tv(1311)]),
    ((1314, false, NAND2), &[Arg(0, 717), Tv(1045)]),
    ((1366, false, NAND2), &[Tv(1363), Tv(1365)]),
    ((1368, false, AND2), &[Tv(1046), Tv(1367)]),
    ((1370, false, NAND2), &[Arg(0, 718), Tv(1045)]),
    ((1422, false, NAND2), &[Tv(1419), Tv(1421)]),
    ((1424, false, AND2), &[Tv(1046), Tv(1423)]),
    ((1426, false, NAND2), &[Arg(0, 719), Tv(1045)]),
    ((1478, false, NAND2), &[Tv(1475), Tv(1477)]),
    ((1480, false, AND2), &[Tv(1046), Tv(1479)]),
    ((1482, false, NAND2), &[Arg(0, 720), Tv(1045)]),
    ((1534, false, NAND2), &[Tv(1531), Tv(1533)]),
    ((1536, false, AND2), &[Tv(1046), Tv(1535)]),
    ((1538, false, NAND2), &[Arg(0, 721), Tv(1045)]),
    ((1590, false, NAND2), &[Tv(1587), Tv(1589)]),
    ((1592, false, AND2), &[Tv(1046), Tv(1591)]),
    ((1594, false, NAND2), &[Arg(0, 722), Tv(1045)]),
    ((1646, false, NAND2), &[Tv(1643), Tv(1645)]),
    ((1648, false, AND2), &[Tv(1046), Tv(1647)]),
    ((1650, false, NAND2), &[Arg(0, 723), Tv(1045)]),
    ((1702, false, NAND2), &[Tv(1699), Tv(1701)]),
    ((1704, false, AND2), &[Tv(1046), Tv(1703)]),
    ((1706, false, NAND2), &[Arg(0, 724), Tv(1045)]),
    ((1758, false, NAND2), &[Tv(1755), Tv(1757)]),
    ((1760, false, AND2), &[Tv(1046), Tv(1759)]),
    ((1762, false, NAND2), &[Arg(0, 725), Tv(1045)]),
    ((1814, false, NAND2), &[Tv(1811), Tv(1813)]),
    ((1816, false, AND2), &[Tv(1046), Tv(1815)]),
    ((1818, false, NAND2), &[Arg(0, 726), Tv(1045)]),
    ((1870, false, NAND2), &[Tv(1867), Tv(1869)]),
    ((1872, false, AND2), &[Tv(1046), Tv(1871)]),
    ((1874, false, NAND2), &[Arg(0, 727), Tv(1045)]),
    ((1926, false, NAND2), &[Tv(1923), Tv(1925)]),
    ((1928, false, AND2), &[Tv(1046), Tv(1927)]),
    ((1930, false, NAND2), &[Arg(0, 728), Tv(1045)]),
    ((1982, false, NAND2), &[Tv(1979), Tv(1981)]),
    ((1984, false, AND2), &[Tv(1046), Tv(1983)]),
    ((1986, false, NAND2), &[Arg(0, 729), Tv(1045)]),
    ((2038, false, NAND2), &[Tv(2035), Tv(2037)]),
    ((2040, false, AND2), &[Tv(1046), Tv(2039)]),
    ((2042, false, NAND2), &[Arg(0, 730), Tv(1045)]),
    ((2094, false, NAND2), &[Tv(2091), Tv(2093)]),
    ((2096, false, AND2), &[Tv(1046), Tv(2095)]),
    ((2098, false, NAND2), &[Arg(0, 731), Tv(1045)]),
    ((2150, false, NAND2), &[Tv(2147), Tv(2149)]),
    ((2152, false, AND2), &[Tv(1046), Tv(2151)]),
    ((2154, false, NAND2), &[Arg(0, 732), Tv(1045)]),
    ((2206, false, NAND2), &[Tv(2203), Tv(2205)]),
    ((2208, false, AND2), &[Tv(1046), Tv(2207)]),
    ((2210, false, NAND2), &[Arg(0, 733), Tv(1045)]),
    ((2262, false, NAND2), &[Tv(2259), Tv(2261)]),
    ((2264, false, AND2), &[Tv(1046), Tv(2263)]),
    ((2266, false, NAND2), &[Arg(0, 734), Tv(1045)]),
    ((2318, false, NAND2), &[Tv(2315), Tv(2317)]),
    ((2320, false, AND2), &[Tv(1046), Tv(2319)]),
    ((2322, false, NAND2), &[Arg(0, 735), Tv(1045)]),
    ((2374, false, NAND2), &[Tv(2371), Tv(2373)]),
    ((2376, false, AND2), &[Tv(1046), Tv(2375)]),
    ((2378, false, NAND2), &[Arg(0, 736), Tv(1045)]),
    ((2430, false, NAND2), &[Tv(2427), Tv(2429)]),
    ((2432, false, AND2), &[Tv(1046), Tv(2431)]),
    ((2434, false, NAND2), &[Arg(0, 737), Tv(1045)]),
];

static LEVEL_33: [((usize, bool, CellType), &[GateInput]); 76] = [
    ((264, false, NAND2), &[Tv(216), Tv(262)]),
    ((1049, false, NAND2), &[Tv(1008), Tv(1048)]),
    ((1091, false, AND2), &[Tv(1089), Tv(1090)]),
    ((1093, false, NAND2), &[Tv(2488), Tv(1088)]),
    ((1145, false, NAND2), &[Tv(1142), Tv(1144)]),
    ((1147, false, AND2), &[Tv(1089), Tv(1146)]),
    ((1149, false, NAND2), &[Tv(2489), Tv(1088)]),
    ((1201, false, NAND2), &[Tv(1198), Tv(1200)]),
    ((1203, false, AND2), &[Tv(1089), Tv(1202)]),
    ((1205, false, NAND2), &[Tv(2490), Tv(1088)]),
    ((1257, false, NAND2), &[Tv(1254), Tv(1256)]),
    ((1259, false, AND2), &[Tv(1089), Tv(1258)]),
    ((1261, false, NAND2), &[Tv(2491), Tv(1088)]),
    ((1313, false, NAND2), &[Tv(1310), Tv(1312)]),
    ((1315, false, AND2), &[Tv(1089), Tv(1314)]),
    ((1317, false, NAND2), &[Tv(2492), Tv(1088)]),
    ((1369, false, NAND2), &[Tv(1366), Tv(1368)]),
    ((1371, false, AND2), &[Tv(1089), Tv(1370)]),
    ((1373, false, NAND2), &[Tv(2493), Tv(1088)]),
    ((1425, false, NAND2), &[Tv(1422), Tv(1424)]),
    ((1427, false, AND2), &[Tv(1089), Tv(1426)]),
    ((1429, false, NAND2), &[Tv(2494), Tv(1088)]),
    ((1481, false, NAND2), &[Tv(1478), Tv(1480)]),
    ((1483, false, AND2), &[Tv(1089), Tv(1482)]),
    ((1485, false, NAND2), &[Tv(2495), Tv(1088)]),
    ((1537, false, NAND2), &[Tv(1534), Tv(1536)]),
    ((1539, false, AND2), &[Tv(1089), Tv(1538)]),
    ((1541, false, NAND2), &[Tv(2536), Tv(1088)]),
    ((1593, false, NAND2), &[Tv(1590), Tv(1592)]),
    ((1595, false, AND2), &[Tv(1089), Tv(1594)]),
    ((1597, false, NAND2), &[Tv(2537), Tv(1088)]),
    ((1649, false, NAND2), &[Tv(1646), Tv(1648)]),
    ((1651, false, AND2), &[Tv(1089), Tv(1650)]),
    ((1653, false, NAND2), &[Tv(2538), Tv(1088)]),
    ((1705, false, NAND2), &[Tv(1702), Tv(1704)]),
    ((1707, false, AND2), &[Tv(1089), Tv(1706)]),
    ((1709, false, NAND2), &[Tv(2539), Tv(1088)]),
    ((1761, false, NAND2), &[Tv(1758), Tv(1760)]),
    ((1763, false, AND2), &[Tv(1089), Tv(1762)]),
    ((1765, false, NAND2), &[Tv(2540), Tv(1088)]),
    ((1817, false, NAND2), &[Tv(1814), Tv(1816)]),
    ((1819, false, AND2), &[Tv(1089), Tv(1818)]),
    ((1821, false, NAND2), &[Tv(2541), Tv(1088)]),
    ((1873, false, NAND2), &[Tv(1870), Tv(1872)]),
    ((1875, false, AND2), &[Tv(1089), Tv(1874)]),
    ((1877, false, NAND2), &[Tv(2542), Tv(1088)]),
    ((1929, false, NAND2), &[Tv(1926), Tv(1928)]),
    ((1931, false, AND2), &[Tv(1089), Tv(1930)]),
    ((1933, false, NAND2), &[Tv(2543), Tv(1088)]),
    ((1985, false, NAND2), &[Tv(1982), Tv(1984)]),
    ((1987, false, AND2), &[Tv(1089), Tv(1986)]),
    ((1989, false, NAND2), &[Tv(6), Tv(1088)]),
    ((2041, false, NAND2), &[Tv(2038), Tv(2040)]),
    ((2043, false, AND2), &[Tv(1089), Tv(2042)]),
    ((2045, false, NAND2), &[Tv(7), Tv(1088)]),
    ((2097, false, NAND2), &[Tv(2094), Tv(2096)]),
    ((2099, false, AND2), &[Tv(1089), Tv(2098)]),
    ((2101, false, NAND2), &[Tv(8), Tv(1088)]),
    ((2153, false, NAND2), &[Tv(2150), Tv(2152)]),
    ((2155, false, AND2), &[Tv(1089), Tv(2154)]),
    ((2157, false, NAND2), &[Tv(9), Tv(1088)]),
    ((2209, false, NAND2), &[Tv(2206), Tv(2208)]),
    ((2211, false, AND2), &[Tv(1089), Tv(2210)]),
    ((2213, false, NAND2), &[Tv(10), Tv(1088)]),
    ((2265, false, NAND2), &[Tv(2262), Tv(2264)]),
    ((2267, false, AND2), &[Tv(1089), Tv(2266)]),
    ((2269, false, NAND2), &[Tv(11), Tv(1088)]),
    ((2321, false, NAND2), &[Tv(2318), Tv(2320)]),
    ((2323, false, AND2), &[Tv(1089), Tv(2322)]),
    ((2325, false, NAND2), &[Tv(12), Tv(1088)]),
    ((2377, false, NAND2), &[Tv(2374), Tv(2376)]),
    ((2379, false, AND2), &[Tv(1089), Tv(2378)]),
    ((2381, false, NAND2), &[Tv(13), Tv(1088)]),
    ((2433, false, NAND2), &[Tv(2430), Tv(2432)]),
    ((2435, false, AND2), &[Tv(1089), Tv(2434)]),
    ((2437, false, NAND2), &[Tv(19), Tv(1088)]),
];

static LEVEL_34: [((usize, bool, CellType), &[GateInput]); 51] = [
    ((263, false, AND2), &[Tv(216), Tv(262)]),
    ((1092, false, NAND2), &[Tv(1049), Tv(1091)]),
    ((1094, false, AND2), &[Tv(264), Tv(1093)]),
    ((1148, false, NAND2), &[Tv(1145), Tv(1147)]),
    ((1150, false, AND2), &[Tv(264), Tv(1149)]),
    ((1204, false, NAND2), &[Tv(1201), Tv(1203)]),
    ((1206, false, AND2), &[Tv(264), Tv(1205)]),
    ((1260, false, NAND2), &[Tv(1257), Tv(1259)]),
    ((1262, false, AND2), &[Tv(264), Tv(1261)]),
    ((1316, false, NAND2), &[Tv(1313), Tv(1315)]),
    ((1318, false, AND2), &[Tv(264), Tv(1317)]),
    ((1372, false, NAND2), &[Tv(1369), Tv(1371)]),
    ((1374, false, AND2), &[Tv(264), Tv(1373)]),
    ((1428, false, NAND2), &[Tv(1425), Tv(1427)]),
    ((1430, false, AND2), &[Tv(264), Tv(1429)]),
    ((1484, false, NAND2), &[Tv(1481), Tv(1483)]),
    ((1486, false, AND2), &[Tv(264), Tv(1485)]),
    ((1540, false, NAND2), &[Tv(1537), Tv(1539)]),
    ((1542, false, AND2), &[Tv(264), Tv(1541)]),
    ((1596, false, NAND2), &[Tv(1593), Tv(1595)]),
    ((1598, false, AND2), &[Tv(264), Tv(1597)]),
    ((1652, false, NAND2), &[Tv(1649), Tv(1651)]),
    ((1654, false, AND2), &[Tv(264), Tv(1653)]),
    ((1708, false, NAND2), &[Tv(1705), Tv(1707)]),
    ((1710, false, AND2), &[Tv(264), Tv(1709)]),
    ((1764, false, NAND2), &[Tv(1761), Tv(1763)]),
    ((1766, false, AND2), &[Tv(264), Tv(1765)]),
    ((1820, false, NAND2), &[Tv(1817), Tv(1819)]),
    ((1822, false, AND2), &[Tv(264), Tv(1821)]),
    ((1876, false, NAND2), &[Tv(1873), Tv(1875)]),
    ((1878, false, AND2), &[Tv(264), Tv(1877)]),
    ((1932, false, NAND2), &[Tv(1929), Tv(1931)]),
    ((1934, false, AND2), &[Tv(264), Tv(1933)]),
    ((1988, false, NAND2), &[Tv(1985), Tv(1987)]),
    ((1990, false, AND2), &[Tv(264), Tv(1989)]),
    ((2044, false, NAND2), &[Tv(2041), Tv(2043)]),
    ((2046, false, AND2), &[Tv(264), Tv(2045)]),
    ((2100, false, NAND2), &[Tv(2097), Tv(2099)]),
    ((2102, false, AND2), &[Tv(264), Tv(2101)]),
    ((2156, false, NAND2), &[Tv(2153), Tv(2155)]),
    ((2158, false, AND2), &[Tv(264), Tv(2157)]),
    ((2212, false, NAND2), &[Tv(2209), Tv(2211)]),
    ((2214, false, AND2), &[Tv(264), Tv(2213)]),
    ((2268, false, NAND2), &[Tv(2265), Tv(2267)]),
    ((2270, false, AND2), &[Tv(264), Tv(2269)]),
    ((2324, false, NAND2), &[Tv(2321), Tv(2323)]),
    ((2326, false, AND2), &[Tv(264), Tv(2325)]),
    ((2380, false, NAND2), &[Tv(2377), Tv(2379)]),
    ((2382, false, AND2), &[Tv(264), Tv(2381)]),
    ((2436, false, NAND2), &[Tv(2433), Tv(2435)]),
    ((2438, false, AND2), &[Tv(264), Tv(2437)]),
];

static LEVEL_35: [((usize, bool, CellType), &[GateInput]); 50] = [
    ((265, false, NAND2), &[Arg(0, 795), Tv(263)]),
    ((1095, false, NAND2), &[Tv(1092), Tv(1094)]),
    ((1096, false, NAND2), &[Arg(0, 796), Tv(263)]),
    ((1151, false, NAND2), &[Tv(1148), Tv(1150)]),
    ((1152, false, NAND2), &[Arg(0, 797), Tv(263)]),
    ((1207, false, NAND2), &[Tv(1204), Tv(1206)]),
    ((1208, false, NAND2), &[Arg(0, 798), Tv(263)]),
    ((1263, false, NAND2), &[Tv(1260), Tv(1262)]),
    ((1264, false, NAND2), &[Arg(0, 799), Tv(263)]),
    ((1319, false, NAND2), &[Tv(1316), Tv(1318)]),
    ((1320, false, NAND2), &[Arg(0, 800), Tv(263)]),
    ((1375, false, NAND2), &[Tv(1372), Tv(1374)]),
    ((1376, false, NAND2), &[Arg(0, 801), Tv(263)]),
    ((1431, false, NAND2), &[Tv(1428), Tv(1430)]),
    ((1432, false, NAND2), &[Arg(0, 802), Tv(263)]),
    ((1487, false, NAND2), &[Tv(1484), Tv(1486)]),
    ((1488, false, NAND2), &[Arg(0, 803), Tv(263)]),
    ((1543, false, NAND2), &[Tv(1540), Tv(1542)]),
    ((1544, false, NAND2), &[Arg(0, 804), Tv(263)]),
    ((1599, false, NAND2), &[Tv(1596), Tv(1598)]),
    ((1600, false, NAND2), &[Arg(0, 805), Tv(263)]),
    ((1655, false, NAND2), &[Tv(1652), Tv(1654)]),
    ((1656, false, NAND2), &[Arg(0, 806), Tv(263)]),
    ((1711, false, NAND2), &[Tv(1708), Tv(1710)]),
    ((1712, false, NAND2), &[Arg(0, 807), Tv(263)]),
    ((1767, false, NAND2), &[Tv(1764), Tv(1766)]),
    ((1768, false, NAND2), &[Arg(0, 808), Tv(263)]),
    ((1823, false, NAND2), &[Tv(1820), Tv(1822)]),
    ((1824, false, NAND2), &[Arg(0, 809), Tv(263)]),
    ((1879, false, NAND2), &[Tv(1876), Tv(1878)]),
    ((1880, false, NAND2), &[Arg(0, 810), Tv(263)]),
    ((1935, false, NAND2), &[Tv(1932), Tv(1934)]),
    ((1936, false, NAND2), &[Arg(0, 811), Tv(263)]),
    ((1991, false, NAND2), &[Tv(1988), Tv(1990)]),
    ((1992, false, NAND2), &[Arg(0, 812), Tv(263)]),
    ((2047, false, NAND2), &[Tv(2044), Tv(2046)]),
    ((2048, false, NAND2), &[Arg(0, 813), Tv(263)]),
    ((2103, false, NAND2), &[Tv(2100), Tv(2102)]),
    ((2104, false, NAND2), &[Arg(0, 814), Tv(263)]),
    ((2159, false, NAND2), &[Tv(2156), Tv(2158)]),
    ((2160, false, NAND2), &[Arg(0, 815), Tv(263)]),
    ((2215, false, NAND2), &[Tv(2212), Tv(2214)]),
    ((2216, false, NAND2), &[Arg(0, 816), Tv(263)]),
    ((2271, false, NAND2), &[Tv(2268), Tv(2270)]),
    ((2272, false, NAND2), &[Arg(0, 817), Tv(263)]),
    ((2327, false, NAND2), &[Tv(2324), Tv(2326)]),
    ((2328, false, NAND2), &[Arg(0, 818), Tv(263)]),
    ((2383, false, NAND2), &[Tv(2380), Tv(2382)]),
    ((2384, false, NAND2), &[Arg(0, 819), Tv(263)]),
    ((2439, false, NAND2), &[Tv(2436), Tv(2438)]),
];

static LEVEL_36: [((usize, bool, CellType), &[GateInput]); 25] = [
    ((0, true, NAND2), &[Tv(265), Tv(1095)]),
    ((1, true, NAND2), &[Tv(1096), Tv(1151)]),
    ((2, true, NAND2), &[Tv(1152), Tv(1207)]),
    ((3, true, NAND2), &[Tv(1208), Tv(1263)]),
    ((4, true, NAND2), &[Tv(1264), Tv(1319)]),
    ((5, true, NAND2), &[Tv(1320), Tv(1375)]),
    ((6, true, NAND2), &[Tv(1376), Tv(1431)]),
    ((7, true, NAND2), &[Tv(1432), Tv(1487)]),
    ((8, true, NAND2), &[Tv(1488), Tv(1543)]),
    ((9, true, NAND2), &[Tv(1544), Tv(1599)]),
    ((10, true, NAND2), &[Tv(1600), Tv(1655)]),
    ((11, true, NAND2), &[Tv(1656), Tv(1711)]),
    ((12, true, NAND2), &[Tv(1712), Tv(1767)]),
    ((13, true, NAND2), &[Tv(1768), Tv(1823)]),
    ((14, true, NAND2), &[Tv(1824), Tv(1879)]),
    ((15, true, NAND2), &[Tv(1880), Tv(1935)]),
    ((16, true, NAND2), &[Tv(1936), Tv(1991)]),
    ((17, true, NAND2), &[Tv(1992), Tv(2047)]),
    ((18, true, NAND2), &[Tv(2048), Tv(2103)]),
    ((19, true, NAND2), &[Tv(2104), Tv(2159)]),
    ((20, true, NAND2), &[Tv(2160), Tv(2215)]),
    ((21, true, NAND2), &[Tv(2216), Tv(2271)]),
    ((22, true, NAND2), &[Tv(2272), Tv(2327)]),
    ((23, true, NAND2), &[Tv(2328), Tv(2383)]),
    ((24, true, NAND2), &[Tv(2384), Tv(2439)]),
];

static PRUNE_4: [usize; 30] = [
    691, 692, 106, 2447, 111, 694, 200, 695, 95, 640, 101, 638, 637, 639, 128, 534, 578, 623, 533,
    531, 622, 567, 522, 569, 613, 564, 115, 620, 123, 120,
];

static PRUNE_31: [usize; 92] = [
    1052, 2090, 2316, 1140, 1004, 1006, 2314, 1005, 1644, 1418, 1420, 1059, 1194, 1055, 18, 1866,
    2092, 244, 1642, 1868, 1086, 2484, 4, 3, 2258, 2529, 2260, 5, 2576, 2486, 2531, 2530, 2034,
    2485, 1, 0, 1308, 2481, 2483, 2528, 2482, 2, 2535, 1588, 2534, 956, 1362, 1138, 1364, 2532,
    2577, 2487, 2036, 1044, 1810, 1812, 2533, 1586, 2428, 2202, 1978, 2204, 260, 1252, 305, 2426,
    1306, 1756, 225, 2480, 1532, 1754, 221, 1980, 1530, 2146, 1922, 2372, 1196, 204, 1015, 2370,
    1250, 1700, 214, 213, 1476, 1698, 2148, 211, 1474, 1924,
];

static PRUNE_12: [usize; 115] = [
    1774, 2224, 1550, 2000, 2226, 824, 2222, 1274, 1998, 509, 1554, 832, 1328, 924, 1330, 833,
    1104, 2002, 1326, 1776, 1778, 1102, 1552, 2394, 905, 2168, 725, 2170, 771, 1718, 1944, 2166,
    677, 767, 1218, 634, 1942, 904, 2392, 1272, 1498, 505, 507, 506, 1946, 1494, 592, 1720, 1496,
    1722, 909, 864, 1270, 714, 2112, 759, 2338, 1886, 1888, 851, 1662, 670, 760, 1162, 2334, 2336,
    2110, 900, 1216, 854, 1666, 811, 766, 2390, 1442, 901, 762, 897, 1664, 672, 717, 671, 175,
    2114, 1438, 1440, 1890, 898, 763, 1214, 1830, 2056, 1606, 1832, 2282, 162, 794, 1106, 2278,
    2280, 2054, 167, 1384, 1610, 169, 1160, 1386, 1382, 885, 2058, 752, 1158, 1608, 706, 1834,
];

static PRUNE_5: [usize; 36] = [
    556, 781, 603, 107, 693, 602, 557, 780, 600, 599, 201, 652, 112, 559, 696, 651, 560, 641, 642,
    96, 2443, 579, 624, 535, 129, 621, 124, 532, 133, 538, 537, 208, 568, 610, 611, 209,
];

static PRUNE_24: [usize; 87] = [
    1187, 285, 1548, 1324, 282, 1635, 1996, 1411, 1772, 1915, 2276, 381, 471, 1691, 2363, 1100,
    378, 380, 2139, 289, 1131, 1492, 319, 50, 1268, 320, 997, 1940, 1579, 1716, 949, 47, 1355,
    2083, 1859, 2220, 55, 326, 2307, 1436, 1075, 39, 1076, 400, 1523, 1884, 982, 1299, 1660, 2027,
    1035, 2388, 2164, 1803, 1077, 1212, 1032, 1079, 1034, 42, 2251, 1033, 1243, 1604, 298, 1380,
    388, 339, 1467, 2052, 972, 1828, 385, 971, 69, 1971, 2332, 347, 1747, 2108, 980, 435, 74, 1156,
    2419, 2195, 977,
];

static PRUNE_36: [usize; 50] = [
    2271, 1320, 1096, 1095, 1599, 1824, 1375, 1600, 2272, 2047, 1823, 2048, 2439, 2215, 1264, 1263,
    1768, 1543, 1544, 1319, 1991, 2216, 1767, 1992, 2383, 2384, 2159, 1432, 1207, 1208, 1712, 1711,
    1488, 1487, 2160, 265, 1936, 1935, 2328, 2327, 1151, 1376, 1152, 1655, 1656, 1431, 2104, 2103,
    1880, 1879,
];

static PRUNE_14: [usize; 102] = [
    511, 1278, 826, 1728, 1504, 1726, 2176, 1500, 1502, 1952, 1276, 2230, 2232, 1780, 2006, 2228,
    1280, 829, 831, 2004, 1446, 1672, 501, 1222, 1448, 185, 497, 1444, 903, 1670, 857, 2120, 1220,
    859, 498, 1896, 2174, 2400, 1948, 1950, 1724, 1224, 2396, 502, 504, 2398, 2172, 1616, 1390,
    1392, 1166, 2064, 1388, 1838, 756, 1840, 1164, 171, 1614, 1892, 2118, 765, 674, 1668, 180,
    1894, 2344, 856, 1168, 2340, 2342, 2116, 1334, 1560, 1108, 1110, 2008, 1556, 1782, 1558, 1784,
    836, 926, 1332, 844, 1836, 934, 888, 2286, 843, 1612, 755, 710, 2062, 754, 935, 2288, 2284,
    796, 1336, 2060, 1112,
];

static PRUNE_26: [usize; 92] = [
    1413, 1323, 1189, 1099, 1861, 57, 1771, 1637, 1547, 59, 2365, 65, 2275, 383, 2141, 2051, 965,
    964, 951, 1357, 276, 1133, 231, 1267, 230, 1038, 1805, 1715, 316, 318, 1581, 1491, 1039, 2219,
    2309, 1995, 281, 2085, 232, 277, 999, 52, 233, 1525, 1074, 1435, 1301, 309, 1211, 1072, 1027,
    1883, 307, 306, 1749, 1659, 2253, 2163, 313, 2029, 90, 1939, 1081, 310, 2387, 87, 1379, 341,
    71, 1469, 1155, 974, 1245, 248, 1917, 1827, 1693, 250, 1603, 1024, 2197, 438, 76, 1071, 1973,
    2107, 300, 390, 2421, 2331, 1068, 1067,
];

static PRUNE_7: [usize; 56] = [
    150, 782, 872, 873, 869, 689, 598, 645, 103, 148, 870, 562, 155, 877, 744, 879, 743, 698, 108,
    876, 785, 605, 154, 815, 816, 140, 636, 542, 543, 731, 145, 685, 552, 732, 186, 549, 98, 819,
    684, 126, 810, 584, 540, 627, 626, 130, 581, 583, 657, 117, 700, 654, 880, 521, 701, 122,
];

static PRUNE_19: [usize; 75] = [
    1909, 465, 1683, 1685, 196, 1459, 373, 2131, 2357, 2133, 1907, 2411, 516, 2187, 427, 1461, 424,
    1235, 1237, 1853, 1627, 364, 1403, 1629, 2301, 361, 2075, 363, 2077, 1851, 415, 370, 1181, 460,
    416, 2355, 1179, 1405, 457, 458, 1571, 2021, 1347, 1797, 940, 895, 2019, 2245, 485, 1795, 36,
    1125, 360, 2299, 1123, 1573, 943, 1349, 477, 1965, 1515, 1741, 1963, 2413, 474, 429, 1739, 25,
    430, 2189, 2243, 1291, 1517, 1293, 30,
];

static PRUNE_34: [usize; 78] = [
    1819, 2269, 1593, 1595, 2045, 1369, 2267, 2041, 1049, 2043, 1817, 2321, 1373, 2097, 1149, 1371,
    1821, 1145, 1147, 1597, 1537, 1763, 2213, 1313, 1989, 1985, 2211, 1761, 1987, 2437, 1317, 1091,
    1093, 2265, 1539, 1765, 1315, 1541, 2157, 1481, 1931, 1933, 1257, 1707, 264, 1929, 2379, 1705,
    262, 2155, 2381, 216, 1261, 2433, 2435, 2209, 1709, 1483, 1485, 1259, 2101, 1649, 1875, 1651,
    1877, 1425, 2323, 2325, 1873, 2099, 2377, 2153, 1205, 1427, 1653, 1201, 1203, 1429,
];

static PRUNE_3: [usize; 19] = [
    105, 104, 110, 199, 94, 2446, 2442, 530, 577, 207, 206, 523, 114, 565, 566, 619, 119, 570, 616,
];

static PRUNE_15: [usize; 129] = [
    1503, 1729, 2179, 1279, 1505, 1955, 16, 512, 2177, 192, 1951, 1953, 2403, 1727, 21, 1283, 2231,
    830, 1281, 1731, 1507, 1447, 499, 1897, 2123, 1223, 1673, 1899, 181, 1895, 2121, 858, 496,
    1671, 2347, 1227, 2399, 2401, 2175, 1675, 503, 1449, 1451, 1225, 2563, 2518, 2067, 1841, 490,
    1843, 896, 1391, 489, 2564, 1617, 2519, 2470, 2515, 711, 937, 1839, 936, 2289, 2560, 846, 2291,
    2517, 1615, 2562, 757, 2561, 2471, 2065, 2516, 2343, 1171, 2119, 2345, 450, 2566, 446, 1619,
    2565, 1167, 1393, 1169, 493, 448, 1395, 492, 447, 2567, 1785, 2011, 1559, 1561, 930, 929, 1335,
    2007, 2233, 2009, 837, 2235, 927, 1783, 799, 2287, 889, 1339, 798, 2467, 2512, 845, 2063, 2514,
    2469, 1115, 2468, 2513, 1337, 1787, 2464, 1111, 797, 2466, 1113, 1563, 2465,
];

static PRUNE_27: [usize; 67] = [
    60, 1638, 1414, 2086, 1862, 246, 66, 2366, 967, 969, 2142, 1190, 966, 1582, 229, 274, 1358,
    952, 91, 317, 2030, 1040, 273, 1806, 235, 234, 2310, 1134, 1000, 278, 308, 1526, 1029, 84,
    1031, 1302, 1974, 79, 1750, 1073, 1028, 88, 269, 270, 1082, 2254, 311, 266, 267, 342, 251,
    1694, 1020, 1470, 72, 1019, 249, 1918, 2422, 1069, 392, 2198, 1246, 1066, 301, 391, 255,
];

static PRUNE_17: [usize; 122] = [
    2405, 1233, 2181, 2407, 463, 1681, 1229, 1455, 194, 1231, 1457, 1509, 1959, 2185, 1285, 1735,
    1961, 1957, 2183, 514, 17, 1733, 2409, 455, 410, 2349, 1401, 409, 2574, 411, 2125, 2575, 1177,
    1399, 2572, 452, 1849, 2526, 2571, 1173, 2573, 1175, 1625, 2527, 2129, 1903, 1905, 462, 1453,
    1679, 1901, 2351, 412, 2353, 1677, 2127, 2473, 2472, 1119, 1345, 2293, 1121, 2474, 1117, 486,
    892, 1343, 801, 1793, 891, 893, 487, 1569, 1847, 2569, 2073, 2524, 2523, 1621, 2568, 2478,
    2570, 1623, 2525, 2479, 1397, 495, 2521, 2476, 2520, 2069, 2475, 2295, 2477, 2071, 2522, 2297,
    1845, 1289, 28, 479, 2237, 1737, 1511, 23, 1513, 1287, 1791, 483, 31, 482, 2241, 1565, 1341,
    1567, 2017, 480, 2239, 841, 2013, 2015, 1789,
];

static PRUNE_29: [usize; 55] = [
    1864, 1640, 1009, 242, 241, 963, 2312, 238, 2088, 961, 1013, 1012, 1010, 1416, 1192, 1808,
    1584, 1042, 2256, 2032, 228, 1084, 92, 1136, 958, 1002, 954, 1360, 1976, 217, 1752, 219, 81,
    2200, 223, 1528, 86, 1304, 1920, 1065, 253, 1696, 1061, 1016, 1060, 2368, 1017, 2144, 2424,
    258, 1472, 345, 344, 1248, 1023,
];

static PRUNE_10: [usize; 43] = [
    736, 152, 779, 555, 915, 922, 157, 739, 741, 590, 861, 862, 772, 813, 812, 722, 679, 678, 182,
    723, 137, 866, 597, 868, 822, 683, 773, 143, 865, 668, 720, 719, 178, 630, 808, 883, 792, 704,
    663, 750, 164, 660, 661,
];

static PRUNE_22: [usize; 81] = [
    376, 1773, 330, 1549, 2361, 422, 328, 1185, 327, 1997, 1689, 1465, 2277, 337, 468, 1325, 2137,
    1101, 1913, 49, 995, 1717, 2305, 1493, 406, 1353, 2165, 947, 1129, 1941, 1633, 54, 1409, 2221,
    2081, 1269, 53, 1857, 324, 323, 1661, 353, 985, 443, 398, 37, 1437, 2249, 34, 1297, 350, 2109,
    395, 1885, 397, 359, 403, 992, 1577, 2389, 1213, 2025, 1801, 988, 1605, 296, 1381, 2417, 433,
    294, 2053, 1241, 1829, 520, 1745, 394, 1521, 2333, 2193, 1157, 1969,
];

static PRUNE_25: [usize; 61] = [
    286, 1188, 1636, 1412, 58, 2140, 1916, 382, 379, 64, 2364, 48, 950, 1356, 321, 1132, 1804,
    1580, 280, 2084, 279, 1860, 56, 998, 51, 2308, 83, 82, 1300, 981, 1026, 1748, 1524, 43, 314,
    1080, 315, 1037, 1036, 2028, 1078, 2252, 1244, 973, 1692, 384, 970, 70, 1468, 340, 437, 2196,
    78, 1972, 1025, 299, 389, 256, 2420, 436, 75,
];

static PRUNE_6: [usize; 42] = [
    601, 783, 558, 147, 102, 643, 690, 644, 697, 561, 563, 202, 653, 604, 784, 650, 139, 727, 138,
    144, 730, 550, 551, 728, 97, 548, 547, 729, 172, 625, 580, 125, 539, 134, 536, 177, 116, 612,
    655, 113, 656, 121,
];

static PRUNE_18: [usize; 102] = [
    1458, 1684, 1232, 195, 1234, 1460, 2132, 1680, 372, 1906, 1682, 1908, 1456, 464, 472, 1960,
    2410, 426, 473, 1736, 2186, 2412, 2408, 425, 2184, 1236, 515, 1402, 1852, 1176, 456, 1178,
    1628, 1850, 2300, 1624, 1626, 2076, 1400, 2354, 2356, 461, 1904, 371, 2130, 2352, 1404, 369,
    2128, 414, 368, 413, 1180, 1120, 1570, 894, 1796, 939, 488, 941, 445, 1346, 1572, 1568, 1794,
    2244, 1344, 2020, 2298, 2072, 2074, 1848, 1348, 1122, 942, 1124, 2296, 1740, 1514, 1516, 1290,
    2188, 1512, 1962, 1964, 1288, 476, 1738, 475, 24, 2242, 32, 2016, 1792, 2018, 484, 29, 1292,
    842, 932, 2240, 481,
];

static PRUNE_30: [usize; 77] = [
    962, 1097, 2089, 1054, 1053, 1865, 1545, 239, 1051, 2313, 1050, 1321, 1193, 1058, 1825, 1057,
    1014, 2273, 243, 1641, 245, 2049, 1011, 1417, 2033, 1809, 1489, 1265, 1085, 272, 2257, 1137,
    1993, 1003, 1769, 1585, 1043, 955, 1361, 2217, 1977, 218, 1753, 2425, 1433, 1209, 2201, 1305,
    1937, 1713, 224, 1529, 220, 2385, 222, 2161, 1153, 2145, 1018, 1921, 2369, 1601, 203, 1062,
    1377, 1881, 1249, 1657, 304, 259, 303, 2329, 1697, 346, 2105, 1473, 210,
];

static PRUNE_20: [usize; 79] = [
    466, 331, 2135, 2134, 420, 332, 1911, 197, 1910, 2359, 2358, 374, 419, 517, 1463, 1238, 20,
    428, 1239, 62, 1686, 335, 1687, 334, 1462, 2303, 2078, 1854, 2079, 365, 362, 408, 2302, 1182,
    1407, 417, 1183, 1630, 1855, 367, 1406, 459, 1631, 2022, 2247, 1798, 2023, 441, 440, 2246,
    1351, 945, 1127, 1126, 356, 1799, 1574, 1575, 944, 1350, 357, 26, 2191, 1967, 478, 1966, 27,
    2414, 68, 2415, 431, 2190, 1295, 1294, 33, 1743, 1742, 1519, 1518,
];

static PRUNE_8: [usize; 56] = [
    917, 916, 871, 646, 647, 734, 733, 553, 742, 607, 606, 787, 789, 878, 649, 920, 919, 874, 786,
    544, 817, 546, 587, 588, 776, 686, 911, 146, 687, 912, 777, 593, 141, 594, 820, 806, 809, 541,
    135, 585, 628, 131, 582, 658, 748, 702, 747, 118, 745, 790, 609, 699, 881, 665, 664, 165,
];

static PRUNE_32: [usize; 83] = [
    1007, 2315, 1367, 2091, 1143, 1365, 1591, 1139, 1141, 2095, 1419, 1869, 1871, 1195, 1645, 1867,
    2317, 1643, 2093, 2319, 1056, 2259, 1311, 2035, 1087, 1309, 1759, 226, 1535, 2039, 1047, 1587,
    1046, 1813, 1589, 1815, 957, 1363, 2261, 1045, 2263, 1811, 2037, 1255, 2427, 2429, 2203, 215,
    1477, 1703, 1253, 261, 1479, 1757, 2207, 1531, 1533, 1983, 1307, 2205, 2431, 1979, 1981, 1755,
    1199, 2371, 2373, 2147, 1647, 1421, 1423, 1197, 1475, 1701, 2151, 212, 1251, 1927, 1923, 2149,
    1699, 1925, 2375,
];

static PRUNE_13: [usize; 132] = [
    15, 2225, 827, 1277, 828, 2001, 1275, 508, 1725, 510, 1501, 825, 2005, 2456, 1779, 1781, 834,
    1329, 1555, 2457, 1777, 2227, 2229, 1553, 2003, 184, 635, 906, 1221, 2169, 2395, 1219, 768,
    902, 1669, 2393, 1445, 1723, 190, 1949, 189, 1497, 1499, 1273, 2397, 187, 1945, 2171, 1947,
    2173, 1721, 1165, 2337, 2339, 2113, 170, 1613, 1387, 1389, 848, 802, 1163, 1441, 855, 1667,
    764, 2117, 1217, 1443, 1893, 179, 2115, 176, 761, 1889, 718, 673, 1891, 899, 2341, 1665, 2281,
    2552, 1109, 2462, 2507, 2461, 2506, 2057, 2508, 163, 2553, 2463, 2283, 835, 2504, 1557, 2459,
    2458, 1105, 925, 1331, 2460, 1107, 1333, 2505, 1385, 1835, 708, 2557, 753, 2061, 2559, 1161,
    709, 2558, 1611, 1837, 886, 1833, 2555, 795, 2554, 2509, 707, 1609, 2511, 2556, 2059, 2510,
    887, 2285,
];

static PRUNE_1: [usize; 10] = [2440, 2444, 524, 573, 528, 574, 525, 572, 527, 571];

static PRUNE_28: [usize; 83] = [
    1322, 240, 1098, 1639, 61, 959, 2087, 237, 1863, 1546, 960, 2050, 968, 2367, 1826, 67, 247,
    1415, 2274, 1191, 1041, 1807, 1266, 275, 1583, 1083, 2255, 1714, 271, 227, 2031, 1490, 1994,
    1001, 236, 2311, 1770, 1359, 953, 2218, 1135, 1210, 1030, 1751, 1527, 80, 2199, 1658, 1434,
    1975, 2162, 268, 89, 1938, 1303, 85, 2386, 312, 1695, 252, 1154, 1063, 1471, 73, 1064, 343,
    2143, 1602, 1919, 1378, 77, 393, 2106, 257, 302, 2423, 1070, 1882, 1021, 1247, 254, 1022, 2330,
];

static PRUNE_9: [usize; 48] = [
    738, 648, 151, 737, 918, 914, 688, 778, 735, 149, 554, 156, 788, 608, 875, 921, 545, 682, 726,
    681, 136, 589, 821, 596, 595, 913, 142, 818, 775, 804, 173, 127, 174, 805, 666, 667, 629, 586,
    132, 703, 659, 749, 159, 160, 882, 746, 791, 168,
];

static PRUNE_21: [usize; 64] = [
    2360, 421, 375, 2136, 467, 418, 1184, 291, 1464, 336, 518, 1240, 293, 292, 1912, 423, 333,
    1688, 2304, 366, 2080, 993, 45, 1128, 994, 1632, 1408, 1856, 984, 986, 355, 2248, 444, 442,
    983, 352, 351, 404, 990, 358, 1576, 991, 405, 1352, 946, 44, 401, 2024, 987, 40, 1800, 402,
    432, 2416, 2192, 519, 295, 1520, 348, 349, 439, 1296, 1968, 1744,
];

static PRUNE_2: [usize; 16] = [
    198, 109, 93, 2445, 99, 100, 2441, 576, 575, 614, 205, 618, 529, 615, 617, 526,
];

static PRUNE_33: [usize; 102] = [
    1368, 1818, 2495, 2540, 1142, 2542, 1144, 1594, 2541, 1008, 1816, 12, 2538, 2042, 2493, 2492,
    1590, 2537, 2539, 1592, 2494, 1366, 13, 1870, 2320, 2322, 1646, 2096, 2318, 1370, 2543, 19,
    2094, 1146, 1312, 1762, 1538, 1534, 1760, 2210, 1310, 1536, 1986, 2490, 2489, 2038, 9, 2264,
    1048, 2040, 2491, 11, 2266, 10, 1814, 2536, 1089, 1088, 1314, 6, 8, 2262, 7, 1090, 2488, 1706,
    1480, 1482, 1256, 1478, 1928, 2154, 1254, 1704, 1930, 2208, 1982, 1984, 2434, 1758, 1258, 2430,
    2432, 2206, 1650, 1198, 1424, 1200, 1426, 2098, 1872, 1874, 1422, 1648, 1926, 2152, 1702, 2378,
    2374, 1202, 2150, 2376,
];

static PRUNE_11: [usize; 92] = [
    14, 2450, 1999, 2452, 2497, 2496, 2451, 1775, 823, 2449, 2448, 2223, 2501, 2546, 1103, 923,
    2548, 2502, 2547, 153, 2453, 1551, 2498, 1327, 2545, 2455, 2500, 740, 2499, 2544, 2454, 680,
    1943, 770, 860, 907, 1719, 591, 2391, 632, 814, 769, 2167, 183, 724, 633, 867, 1495, 863, 908,
    910, 1271, 774, 1887, 669, 849, 850, 715, 2335, 713, 803, 2111, 712, 1215, 721, 676, 631, 675,
    807, 852, 1663, 716, 1439, 853, 793, 2551, 2055, 161, 1831, 884, 2549, 158, 2503, 2279, 2550,
    1159, 751, 1607, 705, 662, 166, 1383,
];

static PRUNE_23: [usize; 56] = [
    287, 377, 2362, 1410, 283, 329, 284, 1186, 1914, 338, 1690, 63, 469, 288, 470, 290, 2138, 996,
    46, 948, 1354, 407, 1130, 325, 1858, 1634, 322, 2306, 2082, 399, 38, 354, 35, 1298, 396, 1802,
    1578, 2250, 2026, 989, 41, 387, 297, 2418, 975, 1466, 386, 1242, 979, 1746, 1522, 976, 2194,
    434, 1970, 978,
];

static PRUNE_35: [usize; 51] = [
    2270, 2044, 2046, 1820, 1094, 2268, 1148, 1374, 1822, 1596, 1372, 1598, 1988, 2214, 1764, 2436,
    2212, 2438, 1092, 1542, 1318, 1540, 1990, 1316, 1766, 263, 1932, 2382, 1708, 2158, 2380, 2156,
    1260, 1486, 1262, 1934, 1484, 1710, 2100, 2326, 2102, 1876, 1150, 2324, 1430, 1204, 1206, 1878,
    1652, 1654, 1428,
];

static PRUNE_16: [usize; 95] = [
    1954, 2180, 1730, 1956, 2406, 1230, 2402, 2404, 193, 2178, 1508, 1734, 22, 1284, 1510, 1506,
    1732, 513, 2182, 1282, 1958, 1898, 500, 2348, 454, 1674, 2124, 2350, 2346, 1398, 451, 2122,
    1174, 453, 1678, 1452, 1454, 191, 1228, 2126, 1450, 1900, 1902, 1226, 188, 1676, 2292, 2294,
    1842, 2068, 2290, 1342, 938, 758, 2066, 847, 1118, 449, 1396, 494, 1622, 1170, 1172, 2070,
    1618, 491, 1844, 1620, 1846, 1394, 838, 2236, 928, 2010, 2012, 1786, 839, 1286, 2234, 1340,
    1790, 933, 1114, 800, 890, 1116, 1566, 931, 1788, 2238, 840, 1562, 1564, 2014, 1338,
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
    state: &Vec<FheBool<E>>,
    x: &Vec<FheBool<E>>,
    y: &Vec<FheBool<E>>,
) -> Vec<FheBool<E>> {
    let args: &[&Vec<FheBool<E>>] = &[state, x, y];

    let mut temp_nodes = HashMap::new();
    let mut out = Vec::new();
    out.resize(625, None);

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
    run_level(&mut temp_nodes, &LEVEL_13);
    prune(&mut temp_nodes, &PRUNE_13);
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
    run_level(&mut temp_nodes, &LEVEL_20);
    prune(&mut temp_nodes, &PRUNE_20);
    run_level(&mut temp_nodes, &LEVEL_21);
    prune(&mut temp_nodes, &PRUNE_21);
    run_level(&mut temp_nodes, &LEVEL_22);
    prune(&mut temp_nodes, &PRUNE_22);
    run_level(&mut temp_nodes, &LEVEL_23);
    prune(&mut temp_nodes, &PRUNE_23);
    run_level(&mut temp_nodes, &LEVEL_24);
    prune(&mut temp_nodes, &PRUNE_24);
    run_level(&mut temp_nodes, &LEVEL_25);
    prune(&mut temp_nodes, &PRUNE_25);
    run_level(&mut temp_nodes, &LEVEL_26);
    prune(&mut temp_nodes, &PRUNE_26);
    run_level(&mut temp_nodes, &LEVEL_27);
    prune(&mut temp_nodes, &PRUNE_27);
    run_level(&mut temp_nodes, &LEVEL_28);
    prune(&mut temp_nodes, &PRUNE_28);
    run_level(&mut temp_nodes, &LEVEL_29);
    prune(&mut temp_nodes, &PRUNE_29);
    run_level(&mut temp_nodes, &LEVEL_30);
    prune(&mut temp_nodes, &PRUNE_30);
    run_level(&mut temp_nodes, &LEVEL_31);
    prune(&mut temp_nodes, &PRUNE_31);
    run_level(&mut temp_nodes, &LEVEL_32);
    prune(&mut temp_nodes, &PRUNE_32);
    run_level(&mut temp_nodes, &LEVEL_33);
    prune(&mut temp_nodes, &PRUNE_33);
    run_level(&mut temp_nodes, &LEVEL_34);
    prune(&mut temp_nodes, &PRUNE_34);
    run_level(&mut temp_nodes, &LEVEL_35);
    prune(&mut temp_nodes, &PRUNE_35);
    run_level(&mut temp_nodes, &LEVEL_36);
    prune(&mut temp_nodes, &PRUNE_36);

    let constant_false = Some(state[0].clone());
    out[100] = constant_false.clone();
    out[101] = constant_false.clone();
    out[102] = constant_false.clone();
    out[103] = constant_false.clone();
    out[104] = constant_false.clone();
    out[105] = constant_false.clone();
    out[106] = constant_false.clone();
    out[107] = constant_false.clone();
    out[108] = constant_false.clone();
    out[109] = constant_false.clone();
    out[110] = constant_false.clone();
    out[111] = constant_false.clone();
    out[112] = constant_false.clone();
    out[113] = constant_false.clone();
    out[114] = constant_false.clone();
    out[115] = constant_false.clone();
    out[116] = constant_false.clone();
    out[117] = constant_false.clone();
    out[118] = constant_false.clone();
    out[119] = constant_false.clone();
    out[120] = constant_false.clone();
    out[121] = constant_false.clone();
    out[122] = constant_false.clone();
    out[123] = constant_false.clone();
    out[124] = constant_false.clone();
    out[125] = constant_false.clone();
    out[126] = constant_false.clone();
    out[127] = constant_false.clone();
    out[128] = constant_false.clone();
    out[129] = constant_false.clone();
    out[130] = constant_false.clone();
    out[131] = constant_false.clone();
    out[132] = constant_false.clone();
    out[133] = constant_false.clone();
    out[134] = constant_false.clone();
    out[135] = constant_false.clone();
    out[136] = constant_false.clone();
    out[137] = constant_false.clone();
    out[138] = constant_false.clone();
    out[139] = constant_false.clone();
    out[140] = constant_false.clone();
    out[141] = constant_false.clone();
    out[142] = constant_false.clone();
    out[143] = constant_false.clone();
    out[144] = constant_false.clone();
    out[145] = constant_false.clone();
    out[146] = constant_false.clone();
    out[147] = constant_false.clone();
    out[148] = constant_false.clone();
    out[149] = constant_false.clone();
    out[150] = constant_false.clone();
    out[151] = constant_false.clone();
    out[152] = constant_false.clone();
    out[153] = constant_false.clone();
    out[154] = constant_false.clone();
    out[155] = constant_false.clone();
    out[156] = constant_false.clone();
    out[157] = constant_false.clone();
    out[158] = constant_false.clone();
    out[159] = constant_false.clone();
    out[160] = constant_false.clone();
    out[161] = constant_false.clone();
    out[162] = constant_false.clone();
    out[163] = constant_false.clone();
    out[164] = constant_false.clone();
    out[165] = constant_false.clone();
    out[166] = constant_false.clone();
    out[167] = constant_false.clone();
    out[168] = constant_false.clone();
    out[169] = constant_false.clone();
    out[170] = constant_false.clone();
    out[171] = constant_false.clone();
    out[172] = constant_false.clone();
    out[173] = constant_false.clone();
    out[174] = constant_false.clone();
    out[175] = constant_false.clone();
    out[176] = constant_false.clone();
    out[177] = constant_false.clone();
    out[178] = constant_false.clone();
    out[179] = constant_false.clone();
    out[180] = constant_false.clone();
    out[181] = constant_false.clone();
    out[182] = constant_false.clone();
    out[183] = constant_false.clone();
    out[184] = constant_false.clone();
    out[185] = constant_false.clone();
    out[186] = constant_false.clone();
    out[187] = constant_false.clone();
    out[188] = constant_false.clone();
    out[189] = constant_false.clone();
    out[190] = constant_false.clone();
    out[191] = constant_false.clone();
    out[192] = constant_false.clone();
    out[193] = constant_false.clone();
    out[194] = constant_false.clone();
    out[195] = constant_false.clone();
    out[196] = constant_false.clone();
    out[197] = constant_false.clone();
    out[198] = constant_false.clone();
    out[199] = constant_false.clone();
    out[200] = constant_false.clone();
    out[201] = constant_false.clone();
    out[202] = constant_false.clone();
    out[203] = constant_false.clone();
    out[204] = constant_false.clone();
    out[205] = constant_false.clone();
    out[206] = constant_false.clone();
    out[207] = constant_false.clone();
    out[208] = constant_false.clone();
    out[209] = constant_false.clone();
    out[210] = constant_false.clone();
    out[211] = constant_false.clone();
    out[212] = constant_false.clone();
    out[213] = constant_false.clone();
    out[214] = constant_false.clone();
    out[215] = constant_false.clone();
    out[216] = constant_false.clone();
    out[217] = constant_false.clone();
    out[218] = constant_false.clone();
    out[219] = constant_false.clone();
    out[220] = constant_false.clone();
    out[221] = constant_false.clone();
    out[222] = constant_false.clone();
    out[223] = constant_false.clone();
    out[224] = constant_false.clone();
    out[225] = constant_false.clone();
    out[226] = constant_false.clone();
    out[227] = constant_false.clone();
    out[228] = constant_false.clone();
    out[229] = constant_false.clone();
    out[230] = constant_false.clone();
    out[231] = constant_false.clone();
    out[232] = constant_false.clone();
    out[233] = constant_false.clone();
    out[234] = constant_false.clone();
    out[235] = constant_false.clone();
    out[236] = constant_false.clone();
    out[237] = constant_false.clone();
    out[238] = constant_false.clone();
    out[239] = constant_false.clone();
    out[240] = constant_false.clone();
    out[241] = constant_false.clone();
    out[242] = constant_false.clone();
    out[243] = constant_false.clone();
    out[244] = constant_false.clone();
    out[245] = constant_false.clone();
    out[246] = constant_false.clone();
    out[247] = constant_false.clone();
    out[248] = constant_false.clone();
    out[249] = constant_false.clone();
    out[250] = constant_false.clone();
    out[251] = constant_false.clone();
    out[252] = constant_false.clone();
    out[253] = constant_false.clone();
    out[254] = constant_false.clone();
    out[255] = constant_false.clone();
    out[256] = constant_false.clone();
    out[257] = constant_false.clone();
    out[258] = constant_false.clone();
    out[259] = constant_false.clone();
    out[25] = constant_false.clone();
    out[260] = constant_false.clone();
    out[261] = constant_false.clone();
    out[262] = constant_false.clone();
    out[263] = constant_false.clone();
    out[264] = constant_false.clone();
    out[265] = constant_false.clone();
    out[266] = constant_false.clone();
    out[267] = constant_false.clone();
    out[268] = constant_false.clone();
    out[269] = constant_false.clone();
    out[26] = constant_false.clone();
    out[270] = constant_false.clone();
    out[271] = constant_false.clone();
    out[272] = constant_false.clone();
    out[273] = constant_false.clone();
    out[274] = constant_false.clone();
    out[275] = constant_false.clone();
    out[276] = constant_false.clone();
    out[277] = constant_false.clone();
    out[278] = constant_false.clone();
    out[279] = constant_false.clone();
    out[27] = constant_false.clone();
    out[280] = constant_false.clone();
    out[281] = constant_false.clone();
    out[282] = constant_false.clone();
    out[283] = constant_false.clone();
    out[284] = constant_false.clone();
    out[285] = constant_false.clone();
    out[286] = constant_false.clone();
    out[287] = constant_false.clone();
    out[288] = constant_false.clone();
    out[289] = constant_false.clone();
    out[28] = constant_false.clone();
    out[290] = constant_false.clone();
    out[291] = constant_false.clone();
    out[292] = constant_false.clone();
    out[293] = constant_false.clone();
    out[294] = constant_false.clone();
    out[295] = constant_false.clone();
    out[296] = constant_false.clone();
    out[297] = constant_false.clone();
    out[298] = constant_false.clone();
    out[299] = constant_false.clone();
    out[29] = constant_false.clone();
    out[300] = constant_false.clone();
    out[301] = constant_false.clone();
    out[302] = constant_false.clone();
    out[303] = constant_false.clone();
    out[304] = constant_false.clone();
    out[305] = constant_false.clone();
    out[306] = constant_false.clone();
    out[307] = constant_false.clone();
    out[308] = constant_false.clone();
    out[309] = constant_false.clone();
    out[30] = constant_false.clone();
    out[310] = constant_false.clone();
    out[311] = constant_false.clone();
    out[312] = constant_false.clone();
    out[313] = constant_false.clone();
    out[314] = constant_false.clone();
    out[315] = constant_false.clone();
    out[316] = constant_false.clone();
    out[317] = constant_false.clone();
    out[318] = constant_false.clone();
    out[319] = constant_false.clone();
    out[31] = constant_false.clone();
    out[320] = constant_false.clone();
    out[321] = constant_false.clone();
    out[322] = constant_false.clone();
    out[323] = constant_false.clone();
    out[324] = constant_false.clone();
    out[325] = constant_false.clone();
    out[326] = constant_false.clone();
    out[327] = constant_false.clone();
    out[328] = constant_false.clone();
    out[329] = constant_false.clone();
    out[32] = constant_false.clone();
    out[330] = constant_false.clone();
    out[331] = constant_false.clone();
    out[332] = constant_false.clone();
    out[333] = constant_false.clone();
    out[334] = constant_false.clone();
    out[335] = constant_false.clone();
    out[336] = constant_false.clone();
    out[337] = constant_false.clone();
    out[338] = constant_false.clone();
    out[339] = constant_false.clone();
    out[33] = constant_false.clone();
    out[340] = constant_false.clone();
    out[341] = constant_false.clone();
    out[342] = constant_false.clone();
    out[343] = constant_false.clone();
    out[344] = constant_false.clone();
    out[345] = constant_false.clone();
    out[346] = constant_false.clone();
    out[347] = constant_false.clone();
    out[348] = constant_false.clone();
    out[349] = constant_false.clone();
    out[34] = constant_false.clone();
    out[350] = constant_false.clone();
    out[351] = constant_false.clone();
    out[352] = constant_false.clone();
    out[353] = constant_false.clone();
    out[354] = constant_false.clone();
    out[355] = constant_false.clone();
    out[356] = constant_false.clone();
    out[357] = constant_false.clone();
    out[358] = constant_false.clone();
    out[359] = constant_false.clone();
    out[35] = constant_false.clone();
    out[360] = constant_false.clone();
    out[361] = constant_false.clone();
    out[362] = constant_false.clone();
    out[363] = constant_false.clone();
    out[364] = constant_false.clone();
    out[365] = constant_false.clone();
    out[366] = constant_false.clone();
    out[367] = constant_false.clone();
    out[368] = constant_false.clone();
    out[369] = constant_false.clone();
    out[36] = constant_false.clone();
    out[370] = constant_false.clone();
    out[371] = constant_false.clone();
    out[372] = constant_false.clone();
    out[373] = constant_false.clone();
    out[374] = constant_false.clone();
    out[375] = constant_false.clone();
    out[376] = constant_false.clone();
    out[377] = constant_false.clone();
    out[378] = constant_false.clone();
    out[379] = constant_false.clone();
    out[37] = constant_false.clone();
    out[380] = constant_false.clone();
    out[381] = constant_false.clone();
    out[382] = constant_false.clone();
    out[383] = constant_false.clone();
    out[384] = constant_false.clone();
    out[385] = constant_false.clone();
    out[386] = constant_false.clone();
    out[387] = constant_false.clone();
    out[388] = constant_false.clone();
    out[389] = constant_false.clone();
    out[38] = constant_false.clone();
    out[390] = constant_false.clone();
    out[391] = constant_false.clone();
    out[392] = constant_false.clone();
    out[393] = constant_false.clone();
    out[394] = constant_false.clone();
    out[395] = constant_false.clone();
    out[396] = constant_false.clone();
    out[397] = constant_false.clone();
    out[398] = constant_false.clone();
    out[399] = constant_false.clone();
    out[39] = constant_false.clone();
    out[400] = constant_false.clone();
    out[401] = constant_false.clone();
    out[402] = constant_false.clone();
    out[403] = constant_false.clone();
    out[404] = constant_false.clone();
    out[405] = constant_false.clone();
    out[406] = constant_false.clone();
    out[407] = constant_false.clone();
    out[408] = constant_false.clone();
    out[409] = constant_false.clone();
    out[40] = constant_false.clone();
    out[410] = constant_false.clone();
    out[411] = constant_false.clone();
    out[412] = constant_false.clone();
    out[413] = constant_false.clone();
    out[414] = constant_false.clone();
    out[415] = constant_false.clone();
    out[416] = constant_false.clone();
    out[417] = constant_false.clone();
    out[418] = constant_false.clone();
    out[419] = constant_false.clone();
    out[41] = constant_false.clone();
    out[420] = constant_false.clone();
    out[421] = constant_false.clone();
    out[422] = constant_false.clone();
    out[423] = constant_false.clone();
    out[424] = constant_false.clone();
    out[425] = constant_false.clone();
    out[426] = constant_false.clone();
    out[427] = constant_false.clone();
    out[428] = constant_false.clone();
    out[429] = constant_false.clone();
    out[42] = constant_false.clone();
    out[430] = constant_false.clone();
    out[431] = constant_false.clone();
    out[432] = constant_false.clone();
    out[433] = constant_false.clone();
    out[434] = constant_false.clone();
    out[435] = constant_false.clone();
    out[436] = constant_false.clone();
    out[437] = constant_false.clone();
    out[438] = constant_false.clone();
    out[439] = constant_false.clone();
    out[43] = constant_false.clone();
    out[440] = constant_false.clone();
    out[441] = constant_false.clone();
    out[442] = constant_false.clone();
    out[443] = constant_false.clone();
    out[444] = constant_false.clone();
    out[445] = constant_false.clone();
    out[446] = constant_false.clone();
    out[447] = constant_false.clone();
    out[448] = constant_false.clone();
    out[449] = constant_false.clone();
    out[44] = constant_false.clone();
    out[450] = constant_false.clone();
    out[451] = constant_false.clone();
    out[452] = constant_false.clone();
    out[453] = constant_false.clone();
    out[454] = constant_false.clone();
    out[455] = constant_false.clone();
    out[456] = constant_false.clone();
    out[457] = constant_false.clone();
    out[458] = constant_false.clone();
    out[459] = constant_false.clone();
    out[45] = constant_false.clone();
    out[460] = constant_false.clone();
    out[461] = constant_false.clone();
    out[462] = constant_false.clone();
    out[463] = constant_false.clone();
    out[464] = constant_false.clone();
    out[465] = constant_false.clone();
    out[466] = constant_false.clone();
    out[467] = constant_false.clone();
    out[468] = constant_false.clone();
    out[469] = constant_false.clone();
    out[46] = constant_false.clone();
    out[470] = constant_false.clone();
    out[471] = constant_false.clone();
    out[472] = constant_false.clone();
    out[473] = constant_false.clone();
    out[474] = constant_false.clone();
    out[475] = constant_false.clone();
    out[476] = constant_false.clone();
    out[477] = constant_false.clone();
    out[478] = constant_false.clone();
    out[479] = constant_false.clone();
    out[47] = constant_false.clone();
    out[480] = constant_false.clone();
    out[481] = constant_false.clone();
    out[482] = constant_false.clone();
    out[483] = constant_false.clone();
    out[484] = constant_false.clone();
    out[485] = constant_false.clone();
    out[486] = constant_false.clone();
    out[487] = constant_false.clone();
    out[488] = constant_false.clone();
    out[489] = constant_false.clone();
    out[48] = constant_false.clone();
    out[490] = constant_false.clone();
    out[491] = constant_false.clone();
    out[492] = constant_false.clone();
    out[493] = constant_false.clone();
    out[494] = constant_false.clone();
    out[495] = constant_false.clone();
    out[496] = constant_false.clone();
    out[497] = constant_false.clone();
    out[498] = constant_false.clone();
    out[499] = constant_false.clone();
    out[49] = constant_false.clone();
    out[500] = constant_false.clone();
    out[501] = constant_false.clone();
    out[502] = constant_false.clone();
    out[503] = constant_false.clone();
    out[504] = constant_false.clone();
    out[505] = constant_false.clone();
    out[506] = constant_false.clone();
    out[507] = constant_false.clone();
    out[508] = constant_false.clone();
    out[509] = constant_false.clone();
    out[50] = constant_false.clone();
    out[510] = constant_false.clone();
    out[511] = constant_false.clone();
    out[512] = constant_false.clone();
    out[513] = constant_false.clone();
    out[514] = constant_false.clone();
    out[515] = constant_false.clone();
    out[516] = constant_false.clone();
    out[517] = constant_false.clone();
    out[518] = constant_false.clone();
    out[519] = constant_false.clone();
    out[51] = constant_false.clone();
    out[520] = constant_false.clone();
    out[521] = constant_false.clone();
    out[522] = constant_false.clone();
    out[523] = constant_false.clone();
    out[524] = constant_false.clone();
    out[525] = constant_false.clone();
    out[526] = constant_false.clone();
    out[527] = constant_false.clone();
    out[528] = constant_false.clone();
    out[529] = constant_false.clone();
    out[52] = constant_false.clone();
    out[530] = constant_false.clone();
    out[531] = constant_false.clone();
    out[532] = constant_false.clone();
    out[533] = constant_false.clone();
    out[534] = constant_false.clone();
    out[535] = constant_false.clone();
    out[536] = constant_false.clone();
    out[537] = constant_false.clone();
    out[538] = constant_false.clone();
    out[539] = constant_false.clone();
    out[53] = constant_false.clone();
    out[540] = constant_false.clone();
    out[541] = constant_false.clone();
    out[542] = constant_false.clone();
    out[543] = constant_false.clone();
    out[544] = constant_false.clone();
    out[545] = constant_false.clone();
    out[546] = constant_false.clone();
    out[547] = constant_false.clone();
    out[548] = constant_false.clone();
    out[549] = constant_false.clone();
    out[54] = constant_false.clone();
    out[550] = constant_false.clone();
    out[551] = constant_false.clone();
    out[552] = constant_false.clone();
    out[553] = constant_false.clone();
    out[554] = constant_false.clone();
    out[555] = constant_false.clone();
    out[556] = constant_false.clone();
    out[557] = constant_false.clone();
    out[558] = constant_false.clone();
    out[559] = constant_false.clone();
    out[55] = constant_false.clone();
    out[560] = constant_false.clone();
    out[561] = constant_false.clone();
    out[562] = constant_false.clone();
    out[563] = constant_false.clone();
    out[564] = constant_false.clone();
    out[565] = constant_false.clone();
    out[566] = constant_false.clone();
    out[567] = constant_false.clone();
    out[568] = constant_false.clone();
    out[569] = constant_false.clone();
    out[56] = constant_false.clone();
    out[570] = constant_false.clone();
    out[571] = constant_false.clone();
    out[572] = constant_false.clone();
    out[573] = constant_false.clone();
    out[574] = constant_false.clone();
    out[575] = constant_false.clone();
    out[576] = constant_false.clone();
    out[577] = constant_false.clone();
    out[578] = constant_false.clone();
    out[579] = constant_false.clone();
    out[57] = constant_false.clone();
    out[580] = constant_false.clone();
    out[581] = constant_false.clone();
    out[582] = constant_false.clone();
    out[583] = constant_false.clone();
    out[584] = constant_false.clone();
    out[585] = constant_false.clone();
    out[586] = constant_false.clone();
    out[587] = constant_false.clone();
    out[588] = constant_false.clone();
    out[589] = constant_false.clone();
    out[58] = constant_false.clone();
    out[590] = constant_false.clone();
    out[591] = constant_false.clone();
    out[592] = constant_false.clone();
    out[593] = constant_false.clone();
    out[594] = constant_false.clone();
    out[595] = constant_false.clone();
    out[596] = constant_false.clone();
    out[597] = constant_false.clone();
    out[598] = constant_false.clone();
    out[599] = constant_false.clone();
    out[59] = constant_false.clone();
    out[600] = constant_false.clone();
    out[601] = constant_false.clone();
    out[602] = constant_false.clone();
    out[603] = constant_false.clone();
    out[604] = constant_false.clone();
    out[605] = constant_false.clone();
    out[606] = constant_false.clone();
    out[607] = constant_false.clone();
    out[608] = constant_false.clone();
    out[609] = constant_false.clone();
    out[60] = constant_false.clone();
    out[610] = constant_false.clone();
    out[611] = constant_false.clone();
    out[612] = constant_false.clone();
    out[613] = constant_false.clone();
    out[614] = constant_false.clone();
    out[615] = constant_false.clone();
    out[616] = constant_false.clone();
    out[617] = constant_false.clone();
    out[618] = constant_false.clone();
    out[619] = constant_false.clone();
    out[61] = constant_false.clone();
    out[620] = constant_false.clone();
    out[621] = constant_false.clone();
    out[622] = constant_false.clone();
    out[623] = constant_false.clone();
    out[624] = constant_false.clone();
    out[62] = constant_false.clone();
    out[63] = constant_false.clone();
    out[64] = constant_false.clone();
    out[65] = constant_false.clone();
    out[66] = constant_false.clone();
    out[67] = constant_false.clone();
    out[68] = constant_false.clone();
    out[69] = constant_false.clone();
    out[70] = constant_false.clone();
    out[71] = constant_false.clone();
    out[72] = constant_false.clone();
    out[73] = constant_false.clone();
    out[74] = constant_false.clone();
    out[75] = constant_false.clone();
    out[76] = constant_false.clone();
    out[77] = constant_false.clone();
    out[78] = constant_false.clone();
    out[79] = constant_false.clone();
    out[80] = constant_false.clone();
    out[81] = constant_false.clone();
    out[82] = constant_false.clone();
    out[83] = constant_false.clone();
    out[84] = constant_false.clone();
    out[85] = constant_false.clone();
    out[86] = constant_false.clone();
    out[87] = constant_false.clone();
    out[88] = constant_false.clone();
    out[89] = constant_false.clone();
    out[90] = constant_false.clone();
    out[91] = constant_false.clone();
    out[92] = constant_false.clone();
    out[93] = constant_false.clone();
    out[94] = constant_false.clone();
    out[95] = constant_false.clone();
    out[96] = constant_false.clone();
    out[97] = constant_false.clone();
    out[98] = constant_false.clone();
    out[99] = constant_false.clone();

    out.into_iter().map(|c| c.unwrap()).collect()
}
