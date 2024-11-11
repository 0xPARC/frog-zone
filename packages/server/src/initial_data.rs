#[derive(Copy, Clone, Debug)]
pub struct PlaintextMonster {
    pub id: usize,
    pub x: u8,
    pub y: u8,
    pub hp: u8,
    pub atk: u8,
    pub points: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct PlaintextItem {
    pub id: usize,
    pub x: u8,
    pub y: u8,
    pub hp: u8,
    pub atk: u8,
    pub points: u8,
}

#[derive(Copy, Clone, Debug)]
pub struct PlaintextObstacle {
    pub x: u8,
    pub y: u8,
}

pub fn get_all_monsters() -> Vec<PlaintextMonster> {
    vec![
        PlaintextMonster {
            id: 0,
            x: 13,
            y: 3,
            hp: 100,
            atk: 1,
            points: 25,
        },
        PlaintextMonster {
            id: 1,
            x: 0,
            y: 13,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 2,
            x: 8,
            y: 14,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 3,
            x: 14,
            y: 14,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 4,
            x: 24,
            y: 15,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 5,
            x: 25,
            y: 15,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 6,
            x: 24,
            y: 20,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 7,
            x: 13,
            y: 22,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 8,
            x: 1,
            y: 24,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 9,
            x: 2,
            y: 24,
            hp: 1,
            atk: 1,
            points: 1,
        },
        PlaintextMonster {
            id: 10,
            x: 27,
            y: 7,
            hp: 15,
            atk: 1,
            points: 5,
        },
        PlaintextMonster {
            id: 11,
            x: 27,
            y: 11,
            hp: 15,
            atk: 1,
            points: 5,
        },
        PlaintextMonster {
            id: 12,
            x: 3,
            y: 14,
            hp: 15,
            atk: 1,
            points: 5,
        },
        PlaintextMonster {
            id: 13,
            x: 11,
            y: 20,
            hp: 5,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 14,
            x: 19,
            y: 20,
            hp: 5,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 15,
            x: 27,
            y: 20,
            hp: 5,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 16,
            x: 5,
            y: 23,
            hp: 5,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 17,
            x: 8,
            y: 8,
            hp: 2,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 18,
            x: 29,
            y: 15,
            hp: 2,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 19,
            x: 5,
            y: 16,
            hp: 2,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 20,
            x: 13,
            y: 17,
            hp: 2,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 21,
            x: 16,
            y: 17,
            hp: 2,
            atk: 1,
            points: 2,
        },
        PlaintextMonster {
            id: 22,
            x: 26,
            y: 17,
            hp: 2,
            atk: 1,
            points: 2,
        },
    ]
}

pub fn get_all_items() -> Vec<PlaintextItem> {
    vec![
        PlaintextItem {
            id: 0,
            x: 7,
            y: 3,
            hp: 10,
            atk: 2,
            points: 1,
        },
        PlaintextItem {
            id: 1,
            x: 24,
            y: 7,
            hp: 10,
            atk: 2,
            points: 1,
        },
        PlaintextItem {
            id: 2,
            x: 15,
            y: 20,
            hp: 5,
            atk: 0,
            points: 1,
        },
        PlaintextItem {
            id: 3,
            x: 29,
            y: 22,
            hp: 5,
            atk: 0,
            points: 1,
        },
        PlaintextItem {
            id: 4,
            x: 8,
            y: 25,
            hp: 5,
            atk: 0,
            points: 1,
        },
        PlaintextItem {
            id: 5,
            x: 8,
            y: 7,
            hp: 0,
            atk: 4,
            points: 1,
        },
        PlaintextItem {
            id: 6,
            x: 22,
            y: 14,
            hp: 0,
            atk: 4,
            points: 1,
        },
        PlaintextItem {
            id: 7,
            x: 17,
            y: 14,
            hp: 0,
            atk: 2,
            points: 1,
        },
        PlaintextItem {
            id: 8,
            x: 25,
            y: 16,
            hp: 0,
            atk: 2,
            points: 1,
        },
        PlaintextItem {
            id: 9,
            x: 7,
            y: 21,
            hp: 0,
            atk: 1,
            points: 1,
        },
        PlaintextItem {
            id: 10,
            x: 14,
            y: 28,
            hp: 0,
            atk: 1,
            points: 1,
        },
        PlaintextItem {
            id: 11,
            x: 11,
            y: 12,
            hp: 0,
            atk: 0,
            points: 10,
        },
    ]
}

pub fn get_all_obstacles() -> Vec<PlaintextObstacle> {
    vec![
        PlaintextObstacle { x: 29, y: 0 },
        PlaintextObstacle { x: 30, y: 0 },
        PlaintextObstacle { x: 1, y: 1 },
        PlaintextObstacle { x: 2, y: 1 },
        PlaintextObstacle { x: 3, y: 1 },
        PlaintextObstacle { x: 29, y: 1 },
        PlaintextObstacle { x: 30, y: 1 },
        PlaintextObstacle { x: 1, y: 2 },
        PlaintextObstacle { x: 2, y: 2 },
        PlaintextObstacle { x: 3, y: 2 },
        PlaintextObstacle { x: 4, y: 2 },
        PlaintextObstacle { x: 15, y: 2 },
        PlaintextObstacle { x: 16, y: 2 },
        PlaintextObstacle { x: 17, y: 2 },
        PlaintextObstacle { x: 18, y: 2 },
        PlaintextObstacle { x: 19, y: 2 },
        PlaintextObstacle { x: 16, y: 3 },
        PlaintextObstacle { x: 17, y: 3 },
        PlaintextObstacle { x: 18, y: 3 },
        PlaintextObstacle { x: 19, y: 3 },
        PlaintextObstacle { x: 29, y: 4 },
        PlaintextObstacle { x: 30, y: 4 },
        PlaintextObstacle { x: 31, y: 4 },
        PlaintextObstacle { x: 3, y: 5 },
        PlaintextObstacle { x: 4, y: 5 },
        PlaintextObstacle { x: 5, y: 5 },
        PlaintextObstacle { x: 29, y: 5 },
        // PlaintextObstacle { x: 31, y: 5 },
        PlaintextObstacle { x: 3, y: 6 },
        PlaintextObstacle { x: 6, y: 6 },
        PlaintextObstacle { x: 9, y: 6 },
        PlaintextObstacle { x: 10, y: 6 },
        PlaintextObstacle { x: 20, y: 6 },
        PlaintextObstacle { x: 21, y: 6 },
        PlaintextObstacle { x: 29, y: 6 },
        // PlaintextObstacle { x: 30, y: 6 },
        PlaintextObstacle { x: 1, y: 7 },
        PlaintextObstacle { x: 2, y: 7 },
        PlaintextObstacle { x: 7, y: 7 },
        PlaintextObstacle { x: 9, y: 7 },
        PlaintextObstacle { x: 11, y: 7 },
        PlaintextObstacle { x: 12, y: 7 },
        PlaintextObstacle { x: 13, y: 7 },
        PlaintextObstacle { x: 19, y: 7 },
        // PlaintextObstacle { x: 21, y: 7 },
        PlaintextObstacle { x: 22, y: 7 },
        PlaintextObstacle { x: 23, y: 7 },
        PlaintextObstacle { x: 28, y: 7 },
        // PlaintextObstacle { x: 31, y: 7 },
        PlaintextObstacle { x: 0, y: 8 },
        PlaintextObstacle { x: 6, y: 8 },
        PlaintextObstacle { x: 9, y: 8 },
        PlaintextObstacle { x: 14, y: 8 },
        PlaintextObstacle { x: 15, y: 8 },
        PlaintextObstacle { x: 16, y: 8 },
        PlaintextObstacle { x: 17, y: 8 },
        PlaintextObstacle { x: 18, y: 8 },
        // PlaintextObstacle { x: 19, y: 8 },
        // PlaintextObstacle { x: 22, y: 8 },
        PlaintextObstacle { x: 24, y: 8 },
        PlaintextObstacle { x: 25, y: 8 },
        PlaintextObstacle { x: 26, y: 8 },
        PlaintextObstacle { x: 29, y: 8 },
        PlaintextObstacle { x: 6, y: 9 },
        PlaintextObstacle { x: 9, y: 9 },
        // PlaintextObstacle { x: 13, y: 9 },
        // PlaintextObstacle { x: 15, y: 9 },
        // PlaintextObstacle { x: 20, y: 9 },
        // PlaintextObstacle { x: 21, y: 9 },
        PlaintextObstacle { x: 26, y: 9 },
        PlaintextObstacle { x: 29, y: 9 },
        PlaintextObstacle { x: 3, y: 10 },
        PlaintextObstacle { x: 4, y: 10 },
        PlaintextObstacle { x: 5, y: 10 },
        PlaintextObstacle { x: 6, y: 10 },
        PlaintextObstacle { x: 9, y: 10 },
        // PlaintextObstacle { x: 14, y: 10 },
        // PlaintextObstacle { x: 22, y: 10 },
        PlaintextObstacle { x: 26, y: 10 },
        PlaintextObstacle { x: 29, y: 10 },
        PlaintextObstacle { x: 1, y: 11 },
        PlaintextObstacle { x: 2, y: 11 },
        PlaintextObstacle { x: 10, y: 11 },
        PlaintextObstacle { x: 11, y: 11 },
        PlaintextObstacle { x: 15, y: 11 },
        PlaintextObstacle { x: 16, y: 11 },
        PlaintextObstacle { x: 17, y: 11 },
        // PlaintextObstacle { x: 21, y: 11 },
        // PlaintextObstacle { x: 23, y: 11 },
        PlaintextObstacle { x: 26, y: 11 },
        PlaintextObstacle { x: 28, y: 11 },
        PlaintextObstacle { x: 0, y: 12 },
        PlaintextObstacle { x: 12, y: 12 },
        PlaintextObstacle { x: 14, y: 12 },
        PlaintextObstacle { x: 18, y: 12 },
        PlaintextObstacle { x: 19, y: 12 },
        // PlaintextObstacle { x: 21, y: 12 },
        // PlaintextObstacle { x: 23, y: 12 },
        PlaintextObstacle { x: 26, y: 12 },
        PlaintextObstacle { x: 29, y: 12 },
        PlaintextObstacle { x: 13, y: 13 },
        PlaintextObstacle { x: 14, y: 13 },
        PlaintextObstacle { x: 20, y: 13 },
        // PlaintextObstacle { x: 21, y: 13 },
        // PlaintextObstacle { x: 23, y: 13 },
        PlaintextObstacle { x: 25, y: 13 },
        PlaintextObstacle { x: 30, y: 13 },
        PlaintextObstacle { x: 31, y: 13 },
        PlaintextObstacle { x: 20, y: 14 },
        // PlaintextObstacle { x: 21, y: 14 },
        // PlaintextObstacle { x: 23, y: 14 },
        PlaintextObstacle { x: 24, y: 14 },
        PlaintextObstacle { x: 10, y: 15 },
        PlaintextObstacle { x: 11, y: 15 },
        PlaintextObstacle { x: 12, y: 15 },
        PlaintextObstacle { x: 21, y: 15 },
        PlaintextObstacle { x: 22, y: 15 },
        PlaintextObstacle { x: 23, y: 15 },
        PlaintextObstacle { x: 0, y: 16 },
        PlaintextObstacle { x: 8, y: 16 },
        PlaintextObstacle { x: 9, y: 16 },
        PlaintextObstacle { x: 13, y: 16 },
        PlaintextObstacle { x: 14, y: 16 },
        PlaintextObstacle { x: 1, y: 17 },
        PlaintextObstacle { x: 8, y: 17 },
        PlaintextObstacle { x: 15, y: 17 },
        PlaintextObstacle { x: 30, y: 17 },
        PlaintextObstacle { x: 31, y: 17 },
        PlaintextObstacle { x: 2, y: 18 },
        PlaintextObstacle { x: 7, y: 18 },
        PlaintextObstacle { x: 14, y: 18 },
        PlaintextObstacle { x: 15, y: 18 },
        PlaintextObstacle { x: 31, y: 18 },
        PlaintextObstacle { x: 3, y: 19 },
        PlaintextObstacle { x: 8, y: 19 },
        PlaintextObstacle { x: 9, y: 19 },
        PlaintextObstacle { x: 10, y: 19 },
        PlaintextObstacle { x: 11, y: 19 },
        PlaintextObstacle { x: 12, y: 19 },
        PlaintextObstacle { x: 13, y: 19 },
        PlaintextObstacle { x: 3, y: 20 },
        PlaintextObstacle { x: 20, y: 20 },
        PlaintextObstacle { x: 21, y: 20 },
        PlaintextObstacle { x: 22, y: 20 },
        PlaintextObstacle { x: 1, y: 21 },
        PlaintextObstacle { x: 2, y: 21 },
        PlaintextObstacle { x: 17, y: 21 },
        PlaintextObstacle { x: 18, y: 21 },
        PlaintextObstacle { x: 19, y: 21 },
        PlaintextObstacle { x: 23, y: 21 },
        PlaintextObstacle { x: 24, y: 21 },
        PlaintextObstacle { x: 25, y: 21 },
        PlaintextObstacle { x: 26, y: 21 },
        PlaintextObstacle { x: 0, y: 22 },
        PlaintextObstacle { x: 18, y: 22 },
        PlaintextObstacle { x: 27, y: 22 },
        PlaintextObstacle { x: 28, y: 22 },
        PlaintextObstacle { x: 15, y: 23 },
        PlaintextObstacle { x: 16, y: 23 },
        PlaintextObstacle { x: 19, y: 23 },
        PlaintextObstacle { x: 28, y: 23 },
        PlaintextObstacle { x: 12, y: 24 },
        PlaintextObstacle { x: 13, y: 24 },
        PlaintextObstacle { x: 14, y: 24 },
        PlaintextObstacle { x: 17, y: 24 },
        PlaintextObstacle { x: 19, y: 24 },
        PlaintextObstacle { x: 28, y: 24 },
        PlaintextObstacle { x: 9, y: 25 },
        PlaintextObstacle { x: 10, y: 25 },
        PlaintextObstacle { x: 11, y: 25 },
        PlaintextObstacle { x: 16, y: 25 },
        PlaintextObstacle { x: 19, y: 25 },
        PlaintextObstacle { x: 27, y: 25 },
        PlaintextObstacle { x: 8, y: 26 },
        PlaintextObstacle { x: 16, y: 26 },
        PlaintextObstacle { x: 18, y: 26 },
        PlaintextObstacle { x: 19, y: 26 },
        PlaintextObstacle { x: 27, y: 26 },
        PlaintextObstacle { x: 9, y: 27 },
        PlaintextObstacle { x: 10, y: 27 },
        PlaintextObstacle { x: 11, y: 27 },
        PlaintextObstacle { x: 12, y: 27 },
        PlaintextObstacle { x: 14, y: 27 },
        PlaintextObstacle { x: 15, y: 27 },
        PlaintextObstacle { x: 20, y: 27 },
        PlaintextObstacle { x: 25, y: 27 },
        PlaintextObstacle { x: 26, y: 27 },
        PlaintextObstacle { x: 0, y: 28 },
        PlaintextObstacle { x: 1, y: 28 },
        PlaintextObstacle { x: 13, y: 28 },
        PlaintextObstacle { x: 20, y: 28 },
        PlaintextObstacle { x: 21, y: 28 },
        PlaintextObstacle { x: 22, y: 28 },
        PlaintextObstacle { x: 23, y: 28 },
        PlaintextObstacle { x: 24, y: 28 },
        PlaintextObstacle { x: 2, y: 29 },
        PlaintextObstacle { x: 3, y: 29 },
        PlaintextObstacle { x: 4, y: 29 },
        PlaintextObstacle { x: 5, y: 29 },
        PlaintextObstacle { x: 6, y: 30 },
        PlaintextObstacle { x: 7, y: 30 },
        PlaintextObstacle { x: 8, y: 31 },
        PlaintextObstacle { x: 9, y: 31 },
    ]
}
