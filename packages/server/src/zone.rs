use core::array::from_fn;
use phantom::{PhantomBool, PhantomEvaluator};
use std::sync::Arc;

/// Encrypted [`bool`]
pub type EncryptedBool = PhantomBool;

/// Encrypted [`u8`] in little-endian 8-bits
pub type EncryptedU8 = [EncryptedBool; 8];

/// Encrypted [`Direction`] in little-endian 2-bits
pub type EncryptedDirection = [EncryptedBool; 2];

/// Encrypted [`EntityType`] in little-endian 3-bits
pub type EncryptedEntityType = [EncryptedBool; 3];

#[derive(Clone, Debug)]
pub struct EncryptedCoord {
    pub x: EncryptedU8,
    pub y: EncryptedU8,
}

#[derive(Clone, Debug)]
pub struct PlayerEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: EncryptedU8,
    pub atk: EncryptedU8,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub data: PlayerEncryptedData,
}

#[derive(Clone, Debug)]
pub struct PlayerWithEncryptedId {
    pub id: EncryptedU8,
    pub data: PlayerEncryptedData,
}

#[derive(Clone, Debug)]
pub struct ItemEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: EncryptedU8,
    pub atk: EncryptedU8,
    pub is_consumed: EncryptedBool,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub data: ItemEncryptedData,
}

#[derive(Clone, Debug)]
pub struct ItemWithEncryptedId {
    pub id: EncryptedU8,
    pub data: ItemEncryptedData,
}

#[derive(Clone, Debug)]
pub enum EntityType {
    None,
    Player,
    Item,
    Monster,
    Invalid,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::None
    }
}

#[derive(Clone, Debug)]
pub struct CellEncryptedData {
    pub entity_type: EncryptedEntityType,
    pub entity_id: EncryptedU8,
    pub hp: EncryptedU8,
    pub atk: EncryptedU8,
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Zone {
    pub width: u8,
    pub height: u8,
    pub players: [Player; 4],
    pub items: [Item; 2],
    pub obstacles: [EncryptedCoord; 96],
    pub precomputed_ids: [EncryptedU8; 20],
}

pub fn fhe_apply_move_raw(
    old_coords: EncryptedCoord,
    direction: EncryptedDirection,
    height: u8,
    width: u8,
) -> EncryptedCoord {
    todo!()
    // let mut new_coords = old_coords;

    // match direction {
    //     Encrypted { val: Direction::Up } => {
    //         if new_coords.y.val > 0 {
    //             new_coords.y.val -= 1;
    //         }
    //     }
    //     Encrypted {
    //         val: Direction::Down,
    //     } => {
    //         if new_coords.y.val < height - 1 {
    //             new_coords.y.val += 1;
    //         }
    //     }
    //     Encrypted {
    //         val: Direction::Left,
    //     } => {
    //         if new_coords.x.val > 0 {
    //             new_coords.x.val -= 1;
    //         }
    //     }
    //     Encrypted {
    //         val: Direction::Right,
    //     } => {
    //         if new_coords.x.val < width - 1 {
    //             new_coords.x.val += 1;
    //         }
    //     }
    // }

    // new_coords
}

pub fn fhe_apply_move_check_collisions(
    old_coords: EncryptedCoord,
    direction: EncryptedDirection,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
) -> EncryptedCoord {
    todo!()
    // let new_coords = fhe_apply_move_raw(old_coords, direction, height, width);

    // for obstacle in obstacles {
    //     if new_coords == obstacle {
    //         // note that if fhe_apply_move_raw returned the original coordinates,
    //         // we'll end up in here because the player's old_coords are part of obstacles
    //         // this is fine since we're returning old_coords anyways
    //         return old_coords;
    //     }
    // }

    // new_coords
}

pub fn fhe_apply_move(
    player_data: PlayerEncryptedData,
    direction: EncryptedDirection,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
    items: [ItemEncryptedData; 2],
) -> (PlayerEncryptedData, [ItemEncryptedData; 2]) {
    todo!()
    // let new_coords =
    //     fhe_apply_move_check_collisions(player_data.loc, direction, height, width, obstacles);

    // let mut new_player_data = player_data;
    // new_player_data.loc = new_coords;
    // let mut new_item_data = items;

    // for (idx, item) in items.iter().enumerate() {
    //     if new_coords == item.loc && !item.is_consumed.val {
    //         new_item_data[idx].is_consumed.val = true;
    //         new_player_data.atk.val += new_item_data[idx].atk.val;
    //         new_player_data.hp.val += new_item_data[idx].hp.val;
    //     }
    // }

    // (new_player_data, new_item_data)
}

fn fhe_get_cell_no_check(
    coord: EncryptedCoord,
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> CellEncryptedData {
    todo!()
    // let mut cell = CellEncryptedData::default();

    // for item in items {
    //     if coord == item.data.loc && !item.data.is_consumed.val {
    //         cell.entity_type = Encrypted::<EntityType> {
    //             val: EntityType::Item,
    //         };
    //         cell.entity_id = item.id;
    //         cell.hp = Encrypted::<u8> {
    //             val: item.data.hp.val,
    //         };
    //         cell.atk = Encrypted::<u8> {
    //             val: item.data.atk.val,
    //         };
    //     }
    // }

    // for player in players {
    //     if coord == player.data.loc {
    //         cell.entity_type = Encrypted::<EntityType> {
    //             val: EntityType::Player,
    //         };
    //         cell.entity_id = player.id;
    //         cell.hp = Encrypted::<u8> {
    //             val: player.data.hp.val,
    //         };
    //         cell.atk = Encrypted::<u8> {
    //             val: player.data.atk.val,
    //         };
    //     }
    // }

    // cell
}

fn fhe_get_cell(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> CellEncryptedData {
    todo!()
    // // coord's x and y values must be within [-2, +2] of player's x and y values
    // // can ignore this check if necessary for performance
    // if query_coord.x.val.abs_diff(player_coord.x.val) > 2
    //     || query_coord.y.val.abs_diff(player_coord.y.val) > 2
    // {
    //     let mut ret = CellEncryptedData::default();
    //     ret.entity_type = Encrypted::<EntityType> {
    //         val: EntityType::Invalid,
    //     };
    //     return ret;
    // }

    // fhe_get_cell_no_check(query_coord, items, players)
}

fn fhe_get_five_cells(
    player_coord: EncryptedCoord,
    query_coords: [EncryptedCoord; 5],
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    todo!()
    // let mut cells = [CellEncryptedData::default(); 5];

    // for (idx, query_coord) in query_coords.iter().enumerate() {
    //     // coord's x and y values must be within [-2, +2] of player's x and y values
    //     // can ignore this check if necessary for performance
    //     if query_coord.x.val.abs_diff(player_coord.x.val) > 2
    //         || query_coord.y.val.abs_diff(player_coord.y.val) > 2
    //     {
    //         let mut ret = CellEncryptedData::default();
    //         ret.entity_type = Encrypted::<EntityType> {
    //             val: EntityType::Invalid,
    //         };
    //         cells[idx] = ret;
    //     } else {
    //         cells[idx] = fhe_get_cell_no_check(query_coord.clone(), items, players);
    //     }
    // }

    // cells
}

fn fhe_get_cross_cells(
    player_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    todo!()
    // let mut cells = [CellEncryptedData::default(); 5];

    // let query_coords = [
    //     player_coord,
    //     EncryptedCoord {
    //         x: player_coord.x,
    //         y: Encrypted::<u8> {
    //             val: player_coord.y.val + 1,
    //         },
    //     },
    //     EncryptedCoord {
    //         x: player_coord.x,
    //         y: Encrypted::<u8> {
    //             val: player_coord.y.val - 1,
    //         },
    //     },
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: player_coord.x.val + 1,
    //         },
    //         y: player_coord.y,
    //     },
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: player_coord.x.val - 1,
    //         },
    //         y: player_coord.y,
    //     },
    // ];

    // for (idx, query_coord) in query_coords.iter().enumerate() {
    //     cells[idx] = fhe_get_cell_no_check(query_coord.clone(), items, players);
    // }

    // cells
}

fn fhe_get_vertical_cells(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    todo!()
    // let mut cells = [CellEncryptedData::default(); 5];

    // // can ignore this check if necessary for performance
    // if query_coord.y.val != player_coord.y.val || query_coord.x.val.abs_diff(player_coord.x.val) > 2
    // {
    //     for i in 0..5 {
    //         cells[i].entity_type = Encrypted::<EntityType> {
    //             val: EntityType::Invalid,
    //         };
    //     }
    //     return cells;
    // }

    // let query_coords = [
    //     EncryptedCoord {
    //         x: query_coord.x,
    //         y: Encrypted::<u8> {
    //             val: query_coord.y.val - 2,
    //         },
    //     },
    //     EncryptedCoord {
    //         x: query_coord.x,
    //         y: Encrypted::<u8> {
    //             val: query_coord.y.val - 1,
    //         },
    //     },
    //     query_coord,
    //     EncryptedCoord {
    //         x: query_coord.x,
    //         y: Encrypted::<u8> {
    //             val: query_coord.y.val + 1,
    //         },
    //     },
    //     EncryptedCoord {
    //         x: query_coord.x,
    //         y: Encrypted::<u8> {
    //             val: query_coord.y.val + 2,
    //         },
    //     },
    // ];

    // for (idx, query_coord) in query_coords.iter().enumerate() {
    //     cells[idx] = fhe_get_cell_no_check(query_coord.clone(), items, players);
    // }

    // cells
}

fn fhe_get_horizontal_cells(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; 2],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    todo!()
    // let mut cells = [CellEncryptedData::default(); 5];

    // // can ignore this check if necessary for performance
    // if query_coord.x.val != player_coord.x.val || query_coord.y.val.abs_diff(player_coord.y.val) > 2
    // {
    //     for i in 0..5 {
    //         cells[i].entity_type = Encrypted::<EntityType> {
    //             val: EntityType::Invalid,
    //         };
    //     }
    //     return cells;
    // }

    // let query_coords = [
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: query_coord.x.val - 2,
    //         },
    //         y: query_coord.y,
    //     },
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: query_coord.x.val - 1,
    //         },
    //         y: query_coord.y,
    //     },
    //     query_coord,
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: query_coord.x.val + 1,
    //         },
    //         y: query_coord.y,
    //     },
    //     EncryptedCoord {
    //         x: Encrypted::<u8> {
    //             val: query_coord.x.val + 2,
    //         },
    //         y: query_coord.y,
    //     },
    // ];

    // for (idx, query_coord) in query_coords.iter().enumerate() {
    //     cells[idx] = fhe_get_cell_no_check(query_coord.clone(), items, players);
    // }

    // cells
}

fn pk_encrypt<const N: usize>(evaluator: &Arc<PhantomEvaluator>, value: u8) -> [EncryptedBool; N] {
    evaluator
        .unbatch(&evaluator.batched_pk_encrypt((0..N).map(|i| (value >> i) & 1 == 1)))
        .try_into()
        .unwrap()
}

impl Zone {
    pub fn new(width: u8, height: u8, evaluator: &Arc<PhantomEvaluator>) -> Self {
        let players = [
            Player {
                id: 0,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 1),
                        y: pk_encrypt(evaluator, 0),
                    },
                    hp: pk_encrypt(evaluator, 5),
                    atk: pk_encrypt(evaluator, 1),
                },
            },
            Player {
                id: 1,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 11),
                        y: pk_encrypt(evaluator, 0),
                    },
                    hp: pk_encrypt(evaluator, 5),
                    atk: pk_encrypt(evaluator, 1),
                },
            },
            Player {
                id: 2,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 21),
                        y: pk_encrypt(evaluator, 0),
                    },
                    hp: pk_encrypt(evaluator, 5),
                    atk: pk_encrypt(evaluator, 1),
                },
            },
            Player {
                id: 3,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 2),
                        y: pk_encrypt(evaluator, 0),
                    },
                    hp: pk_encrypt(evaluator, 5),
                    atk: pk_encrypt(evaluator, 1),
                },
            },
        ];

        let items = [
            Item {
                id: 0,
                data: ItemEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 5),
                        y: pk_encrypt(evaluator, 5),
                    },
                    hp: pk_encrypt(evaluator, 1),
                    atk: pk_encrypt(evaluator, 1),
                    is_consumed: pk_encrypt::<1>(evaluator, 0)[0].clone(),
                },
            },
            Item {
                id: 1,
                data: ItemEncryptedData {
                    loc: EncryptedCoord {
                        x: pk_encrypt(evaluator, 15),
                        y: pk_encrypt(evaluator, 15),
                    },
                    hp: pk_encrypt(evaluator, 1),
                    atk: pk_encrypt(evaluator, 1),
                    is_consumed: pk_encrypt::<1>(evaluator, 0)[0].clone(),
                },
            },
        ];

        let filler_coord = EncryptedCoord {
            x: pk_encrypt(evaluator, 255),
            y: pk_encrypt(evaluator, 255),
        };
        let mut obstacles: [EncryptedCoord; 96] = from_fn(|_| filler_coord.clone());
        obstacles[0] = EncryptedCoord {
            x: pk_encrypt(evaluator, 0),
            y: pk_encrypt(evaluator, 0),
        };

        let precomputed_ids = [
            pk_encrypt(evaluator, 0),
            pk_encrypt(evaluator, 1),
            pk_encrypt(evaluator, 2),
            pk_encrypt(evaluator, 3),
            pk_encrypt(evaluator, 4),
            pk_encrypt(evaluator, 5),
            pk_encrypt(evaluator, 6),
            pk_encrypt(evaluator, 7),
            pk_encrypt(evaluator, 8),
            pk_encrypt(evaluator, 9),
            pk_encrypt(evaluator, 10),
            pk_encrypt(evaluator, 11),
            pk_encrypt(evaluator, 12),
            pk_encrypt(evaluator, 13),
            pk_encrypt(evaluator, 14),
            pk_encrypt(evaluator, 15),
            pk_encrypt(evaluator, 16),
            pk_encrypt(evaluator, 17),
            pk_encrypt(evaluator, 18),
            pk_encrypt(evaluator, 19),
        ];

        Self {
            width,
            height,
            players,
            items,
            obstacles,
            precomputed_ids,
        }
    }

    pub fn move_player(
        &mut self,
        player_id: usize,
        direction: EncryptedDirection,
    ) -> EncryptedCoord {
        todo!()
        // if player_id >= self.players.len() {
        //     return EncryptedCoord {
        //         x: Encrypted { val: 255 },
        //         y: Encrypted { val: 255 },
        //     };
        // }

        // let player = self.players[player_id];
        // let player_data = player.data;

        // let item_data = self.items.map(|i| i.data);

        // let filler_coord = EncryptedCoord {
        //     x: Encrypted { val: 255 },
        //     y: Encrypted { val: 255 },
        // };
        // let mut obstacles = [filler_coord; 100];

        // let mut count = 0;
        // for obstacle in self.obstacles {
        //     obstacles[count] = obstacle;
        //     count += 1;
        // }
        // for test_player in self.players {
        //     obstacles[count] = test_player.data.loc;
        //     count += 1;
        // }

        // let (new_player_data, new_item_data) = fhe_apply_move(
        //     player_data,
        //     direction,
        //     self.height,
        //     self.width,
        //     obstacles,
        //     item_data,
        // );

        // self.players[player_id].data = new_player_data;

        // for i in 0..self.items.len() {
        //     self.items[i].data = new_item_data[i];
        // }

        // return player.data.loc;
    }

    fn fully_encrypted_players(&self) -> [PlayerWithEncryptedId; 4] {
        from_fn(|i| {
            let player = self.players[i].clone();
            PlayerWithEncryptedId {
                id: self.precomputed_ids[player.id].clone(),
                data: player.data,
            }
        })
    }

    fn fully_encrypted_items(&self) -> [ItemWithEncryptedId; 2] {
        from_fn(|i| {
            let item = self.items[i].clone();
            ItemWithEncryptedId {
                id: self.precomputed_ids[item.id].clone(),
                data: item.data,
            }
        })
    }

    pub fn get_cells(
        &self,
        player_id: usize,
        coords: Vec<EncryptedCoord>,
    ) -> Vec<CellEncryptedData> {
        let mut cells = Vec::new();
        let player_coord = &self.players[player_id].data.loc;

        for coord in coords.iter() {
            cells.push(fhe_get_cell(
                player_coord.clone(),
                coord.clone(),
                self.fully_encrypted_items(),
                self.fully_encrypted_players(),
            ));
        }

        cells
    }

    pub fn get_player(&self, player_id: usize) -> PlayerEncryptedData {
        self.players[player_id].data.clone()
    }

    pub fn get_five_cells(
        &self,
        player_id: usize,
        coords: [EncryptedCoord; 5],
    ) -> [CellEncryptedData; 5] {
        fhe_get_five_cells(
            self.players[player_id].data.loc.clone(),
            coords,
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_cross_cells(&self, player_id: usize) -> [CellEncryptedData; 5] {
        fhe_get_cross_cells(
            self.players[player_id].data.loc.clone(),
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_vertical_cells(
        &self,
        player_id: usize,
        center: EncryptedCoord,
    ) -> [CellEncryptedData; 5] {
        fhe_get_vertical_cells(
            self.players[player_id].data.loc.clone(),
            center,
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_horizontal_cells(
        &self,
        player_id: usize,
        center: EncryptedCoord,
    ) -> [CellEncryptedData; 5] {
        fhe_get_horizontal_cells(
            self.players[player_id].data.loc.clone(),
            center,
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }
}
