#![allow(arithmetic_overflow)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct Encrypted<T> {
    pub val: T,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EncryptedCoord {
    pub x: Encrypted<u8>,
    pub y: Encrypted<u8>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PlayerEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: Encrypted<u8>,
    pub atk: Encrypted<u8>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Player {
    pub id: usize,
    pub data: PlayerEncryptedData,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ItemEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: Encrypted<u8>,
    pub atk: Encrypted<u8>,
    pub is_consumed: Encrypted<bool>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Item {
    pub id: usize,
    pub data: ItemEncryptedData,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub struct CellEncryptedData {
    pub entity_type: Encrypted<EntityType>,
    pub entity_id: Encrypted<usize>,
    pub hp: Encrypted<u8>,
    pub atk: Encrypted<u8>,
}

#[derive(Clone, Copy, Deserialize, Debug)]
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
}

pub fn fhe_apply_move_raw(
    old_coords: EncryptedCoord,
    direction: Encrypted<Direction>,
    height: u8,
    width: u8,
) -> EncryptedCoord {
    let mut new_coords = old_coords;

    match direction {
        Encrypted { val: Direction::Up } => {
            if new_coords.y.val > 0 {
                new_coords.y.val -= 1;
            }
        }
        Encrypted {
            val: Direction::Down,
        } => {
            if new_coords.y.val < height - 1 {
                new_coords.y.val += 1;
            }
        }
        Encrypted {
            val: Direction::Left,
        } => {
            if new_coords.x.val > 0 {
                new_coords.x.val -= 1;
            }
        }
        Encrypted {
            val: Direction::Right,
        } => {
            if new_coords.x.val < width - 1 {
                new_coords.x.val += 1;
            }
        }
    }

    new_coords
}

pub fn fhe_apply_move_check_collisions(
    old_coords: EncryptedCoord,
    direction: Encrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
) -> EncryptedCoord {
    let new_coords = fhe_apply_move_raw(old_coords, direction, height, width);

    for obstacle in obstacles {
        if new_coords == obstacle {
            // note that if fhe_apply_move_raw returned the original coordinates,
            // we'll end up in here because the player's old_coords are part of obstacles
            // this is fine since we're returning old_coords anyways
            return old_coords;
        }
    }

    new_coords
}

pub fn fhe_apply_move(
    player_data: PlayerEncryptedData,
    direction: Encrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
    items: [ItemEncryptedData; 2],
) -> (PlayerEncryptedData, [ItemEncryptedData; 2]) {
    let new_coords =
        fhe_apply_move_check_collisions(player_data.loc, direction, height, width, obstacles);

    let mut new_player_data = player_data;
    new_player_data.loc = new_coords;
    let mut new_item_data = items;

    for (idx, item) in items.iter().enumerate() {
        if new_coords == item.loc && !item.is_consumed.val {
            new_item_data[idx].is_consumed.val = true;
            new_player_data.atk.val += new_item_data[idx].atk.val;
            new_player_data.hp.val += new_item_data[idx].hp.val;
        }
    }

    (new_player_data, new_item_data)
}

fn fhe_get_cell_no_check(
    coord: EncryptedCoord,
    items: [Item; 2],
    players: [Player; 4],
) -> CellEncryptedData {
    let mut cell = CellEncryptedData::default();

    for item in items {
        if coord == item.data.loc && !item.data.is_consumed.val {
            cell.entity_type = Encrypted::<EntityType> {
                val: EntityType::Item,
            };
            cell.entity_id = Encrypted::<usize> { val: item.id };
            cell.hp = Encrypted::<u8> {
                val: item.data.hp.val,
            };
            cell.atk = Encrypted::<u8> {
                val: item.data.atk.val,
            };
        }
    }

    for player in players {
        if coord == player.data.loc {
            cell.entity_type = Encrypted::<EntityType> {
                val: EntityType::Player,
            };
            cell.entity_id = Encrypted::<usize> { val: player.id };
            cell.hp = Encrypted::<u8> {
                val: player.data.hp.val,
            };
            cell.atk = Encrypted::<u8> {
                val: player.data.atk.val,
            };
        }
    }

    cell
}

fn fhe_get_cell(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [Item; 2],
    players: [Player; 4],
) -> CellEncryptedData {
    // coord's x and y values must be within [-2, +2] of player's x and y values
    if query_coord.x.val.abs_diff(player_coord.x.val) > 2
        || query_coord.y.val.abs_diff(player_coord.y.val) > 2
    {
        let mut ret = CellEncryptedData::default();
        ret.entity_type = Encrypted::<EntityType> {
            val: EntityType::Invalid,
        };
        return ret;
    }

    fhe_get_cell_no_check(query_coord, items, players)
}

impl Zone {
    pub fn new(width: u8, height: u8) -> Self {
        let players = [
            Player {
                id: 0,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 1 },
                        y: Encrypted { val: 0 },
                    },
                    hp: Encrypted { val: 5 },
                    atk: Encrypted { val: 1 },
                },
            },
            Player {
                id: 1,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 11 },
                        y: Encrypted { val: 0 },
                    },
                    hp: Encrypted { val: 5 },
                    atk: Encrypted { val: 1 },
                },
            },
            Player {
                id: 2,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 21 },
                        y: Encrypted { val: 0 },
                    },
                    hp: Encrypted { val: 5 },
                    atk: Encrypted { val: 1 },
                },
            },
            Player {
                id: 3,
                data: PlayerEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 2 },
                        y: Encrypted { val: 0 },
                    },
                    hp: Encrypted { val: 5 },
                    atk: Encrypted { val: 1 },
                },
            },
        ];

        let items = [
            Item {
                id: 0,
                data: ItemEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 5 },
                        y: Encrypted { val: 5 },
                    },
                    hp: Encrypted { val: 1 },
                    atk: Encrypted { val: 1 },
                    is_consumed: Encrypted { val: false },
                },
            },
            Item {
                id: 1,
                data: ItemEncryptedData {
                    loc: EncryptedCoord {
                        x: Encrypted { val: 15 },
                        y: Encrypted { val: 15 },
                    },
                    hp: Encrypted { val: 1 },
                    atk: Encrypted { val: 1 },
                    is_consumed: Encrypted { val: false },
                },
            },
        ];

        let filler_coord = EncryptedCoord {
            x: Encrypted { val: 255 },
            y: Encrypted { val: 255 },
        };
        let mut obstacles: [EncryptedCoord; 96] = [filler_coord; 96];
        obstacles[0] = EncryptedCoord {
            x: Encrypted { val: 0 },
            y: Encrypted { val: 0 },
        };

        Self {
            width,
            height,
            players,
            items,
            obstacles,
        }
    }

    pub fn move_player(
        &mut self,
        player_id: usize,
        direction: Encrypted<Direction>,
    ) -> EncryptedCoord {
        if player_id >= self.players.len() {
            return EncryptedCoord {
                x: Encrypted { val: 255 },
                y: Encrypted { val: 255 },
            };
        }

        let player = self.players[player_id];
        let player_data = player.data;

        let item_data = self.items.map(|i| i.data);

        let filler_coord = EncryptedCoord {
            x: Encrypted { val: 255 },
            y: Encrypted { val: 255 },
        };
        let mut obstacles = [filler_coord; 100];

        let mut count = 0;
        for obstacle in self.obstacles {
            obstacles[count] = obstacle;
            count += 1;
        }
        for test_player in self.players {
            obstacles[count] = test_player.data.loc;
            count += 1;
        }

        let (new_player_data, new_item_data) = fhe_apply_move(
            player_data,
            direction,
            self.height,
            self.width,
            obstacles,
            item_data,
        );

        self.players[player_id].data = new_player_data;

        for i in 0..self.items.len() {
            self.items[i].data = new_item_data[i];
        }

        return player.data.loc;
    }

    pub fn get_cells(
        &self,
        player_id: usize,
        coords: Vec<EncryptedCoord>,
    ) -> Vec<CellEncryptedData> {
        let mut cells = Vec::new();
        let player_coord = self.players[player_id].data.loc;

        for coord in coords.iter() {
            cells.push(fhe_get_cell(
                player_coord,
                coord.clone(),
                self.items,
                self.players,
            ));
        }

        let len = coords.len();

        cells
    }
}
