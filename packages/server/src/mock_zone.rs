use core::array::from_fn;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use crate::client::{Direction, EntityType};
use crate::initial_data::{get_all_items, get_all_monsters, get_all_obstacles};

const NUM_ITEMS: usize = 12;
const NUM_MONSTERS: usize = 23;
const NUM_OBSTACLES: usize = 193;

pub type MockEncrypted<T> = T;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MockEncryptedCoord {
    pub x: MockEncrypted<u8>,
    pub y: MockEncrypted<u8>,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEncryptedData {
    pub loc: MockEncryptedCoord,
    pub hp: MockEncrypted<u8>,
    pub atk: MockEncrypted<u8>,
    pub points: MockEncrypted<u8>,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: usize,
    pub data: PlayerEncryptedData,
}

#[derive(Copy, Clone, Debug)]
pub struct PlayerWithEncryptedId {
    pub id: MockEncrypted<u8>,
    pub data: PlayerEncryptedData,
}

#[derive(Copy, Clone, Debug)]
pub struct ItemEncryptedData {
    pub loc: MockEncryptedCoord,
    pub hp: MockEncrypted<u8>,
    pub atk: MockEncrypted<u8>,
    pub is_consumed: MockEncrypted<bool>,
    pub points: MockEncrypted<u8>,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub id: usize,
    pub data: ItemEncryptedData,
}

#[derive(Copy, Clone, Debug)]
pub struct ItemWithEncryptedId {
    pub id: MockEncrypted<u8>,
    pub data: ItemEncryptedData,
}

#[derive(Copy, Clone, Debug)]
pub struct MonsterEncryptedData {
    pub loc: MockEncryptedCoord,
    pub hp: MockEncrypted<u8>,
    pub atk: MockEncrypted<u8>,
    pub points: MockEncrypted<u8>,
}

#[derive(Clone, Debug)]
pub struct Monster {
    pub id: usize,
    pub data: MonsterEncryptedData,
}

#[derive(Copy, Clone, Debug)]
pub struct MonsterWithEncryptedId {
    pub id: MockEncrypted<u8>,
    pub data: MonsterEncryptedData,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct CellEncryptedData {
    pub entity_type: MockEncrypted<EntityType>,
    pub entity_id: MockEncrypted<u8>,
    pub hp: MockEncrypted<u8>,
    pub atk: MockEncrypted<u8>,
    pub points: MockEncrypted<u8>,
}

#[derive(Clone, Debug)]
pub struct MockZone {
    pub width: u8,
    pub height: u8,
    pub players: [Player; 4],
    pub items: [Item; NUM_ITEMS],
    pub monsters: [Monster; NUM_MONSTERS],
    pub obstacles: [MockEncryptedCoord; NUM_OBSTACLES],
    pub random_state: u8,
    pub precomputed_ids: [MockEncrypted<u8>; 34],
}

pub fn fhe_apply_move_raw(
    old_coords: MockEncryptedCoord,
    direction: MockEncrypted<Direction>,
    height: u8,
    width: u8,
) -> MockEncryptedCoord {
    let mut new_coords = old_coords;

    match direction {
        Direction::Up => {
            if new_coords.y > 0 {
                new_coords.y -= 1;
            }
        }
        Direction::Down => {
            if new_coords.y < height - 1 {
                new_coords.y += 1;
            }
        }
        Direction::Left => {
            if new_coords.x > 0 {
                new_coords.x -= 1;
            }
        }
        Direction::Right => {
            if new_coords.x < width - 1 {
                new_coords.x += 1;
            }
        }
    }

    new_coords
}

pub fn fhe_apply_move_check_collisions(
    old_coords: MockEncryptedCoord,
    direction: MockEncrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [MockEncryptedCoord; NUM_OBSTACLES + 4],
) -> MockEncryptedCoord {
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
    direction: MockEncrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [MockEncryptedCoord; NUM_OBSTACLES + 4],
    monsters: [MonsterEncryptedData; NUM_MONSTERS],
    items: [ItemEncryptedData; NUM_ITEMS],
) -> (
    PlayerEncryptedData,
    [ItemEncryptedData; NUM_ITEMS],
    [MonsterEncryptedData; NUM_MONSTERS],
) {
    let old_coords = player_data.loc;

    let mut new_coords =
        fhe_apply_move_check_collisions(player_data.loc, direction, height, width, obstacles);

    let mut new_player_data = player_data;
    let mut new_item_data = items;
    let mut new_monster_data = monsters;

    for (idx, item) in items.iter().enumerate() {
        if new_coords == item.loc && !item.is_consumed {
            new_item_data[idx].is_consumed = true;
            new_player_data.atk += new_item_data[idx].atk;
            new_player_data.hp += new_item_data[idx].hp;
            new_player_data.points += new_item_data[idx].points;
        }
    }

    for (idx, monster) in monsters.iter().enumerate() {
        if new_coords == monster.loc && monster.hp != 0 {
            // apply monster's attack
            if player_data.hp <= monster.atk {
                new_player_data.hp = 0;
            } else {
                new_player_data.hp -= monster.atk;
            }

            // apply player's attack
            if monster.hp <= player_data.atk {
                new_monster_data[idx].hp = 0;
                new_player_data.atk += monster.atk;
                new_player_data.points += monster.points;
            } else {
                new_monster_data[idx].hp -= player_data.atk
            }

            // revert player back to their coords
            new_coords = old_coords;
        }
    }

    new_player_data.loc = new_coords;

    (new_player_data, new_item_data, new_monster_data)
}

fn fhe_get_cell_no_check(
    coord: MockEncryptedCoord,
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> CellEncryptedData {
    let mut cell = CellEncryptedData::default();

    for monster in monsters {
        if coord == monster.data.loc && monster.data.hp > 0 {
            cell.entity_type = EntityType::Monster;
            cell.entity_id = monster.id;
            cell.hp = monster.data.hp;
            cell.atk = monster.data.atk;
            cell.points = monster.data.points;
        }
    }

    for item in items {
        if coord == item.data.loc && !item.data.is_consumed {
            cell.entity_type = EntityType::Item;
            cell.entity_id = item.id;
            cell.hp = item.data.hp;
            cell.atk = item.data.atk;
            cell.points = item.data.points;
        }
    }

    for player in players {
        if coord == player.data.loc {
            cell.entity_type = EntityType::Player;
            cell.entity_id = player.id;
            cell.hp = player.data.hp;
            cell.atk = player.data.atk;
            cell.points = player.data.points;
        }
    }

    cell
}

fn fhe_get_cell(
    player_coord: MockEncryptedCoord,
    query_coord: MockEncryptedCoord,
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> CellEncryptedData {
    // coord's x and y values must be within [-2, +2] of player's x and y values
    // can ignore this check if necessary for performance
    if query_coord.x.abs_diff(player_coord.x) > 2 || query_coord.y.abs_diff(player_coord.y) > 2 {
        let mut ret = CellEncryptedData::default();
        ret.entity_type = EntityType::Invalid;
        return ret;
    }

    fhe_get_cell_no_check(query_coord, monsters, items, players)
}

fn fhe_get_five_cells(
    player_coord: MockEncryptedCoord,
    query_coords: [MockEncryptedCoord; 5],
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut cells = [CellEncryptedData::default(); 5];

    for (idx, query_coord) in query_coords.iter().enumerate() {
        // coord's x and y values must be within [-2, +2] of player's x and y values
        // can ignore this check if necessary for performance
        if query_coord.x.abs_diff(player_coord.x) > 2 || query_coord.y.abs_diff(player_coord.y) > 2
        {
            let mut ret = CellEncryptedData::default();
            ret.entity_type = EntityType::Invalid;
            cells[idx] = ret;
        } else {
            cells[idx] = fhe_get_cell_no_check(query_coord.clone(), monsters, items, players);
        }
    }

    cells
}

fn fhe_get_cross_cells(
    player_coord: MockEncryptedCoord,
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut cells = [CellEncryptedData::default(); 5];

    let query_coords = [
        player_coord,
        MockEncryptedCoord {
            x: player_coord.x,
            y: player_coord.y + 1,
        },
        MockEncryptedCoord {
            x: player_coord.x,
            y: player_coord.y - 1,
        },
        MockEncryptedCoord {
            x: player_coord.x + 1,
            y: player_coord.y,
        },
        MockEncryptedCoord {
            x: player_coord.x - 1,
            y: player_coord.y,
        },
    ];

    for (idx, query_coord) in query_coords.iter().enumerate() {
        cells[idx] = fhe_get_cell_no_check(query_coord.clone(), monsters, items, players);
    }

    cells
}

fn fhe_get_vertical_cells(
    center_coord: MockEncryptedCoord,
    query_coord: MockEncryptedCoord,
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut cells = [CellEncryptedData::default(); 5];

    // can ignore this check if necessary for performance
    if query_coord.y != center_coord.y || query_coord.x.abs_diff(center_coord.x) > 2 {
        for i in 0..5 {
            cells[i].entity_type = EntityType::Invalid;
        }
        return cells;
    }

    let query_coords = [
        MockEncryptedCoord {
            x: query_coord.x,
            y: query_coord.y - 2,
        },
        MockEncryptedCoord {
            x: query_coord.x,
            y: query_coord.y - 1,
        },
        query_coord,
        MockEncryptedCoord {
            x: query_coord.x,
            y: query_coord.y + 1,
        },
        MockEncryptedCoord {
            x: query_coord.x,
            y: query_coord.y + 2,
        },
    ];

    for (idx, query_coord) in query_coords.iter().enumerate() {
        cells[idx] = fhe_get_cell_no_check(query_coord.clone(), monsters, items, players);
    }

    cells
}

fn fhe_get_horizontal_cells(
    center_coord: MockEncryptedCoord,
    query_coord: MockEncryptedCoord,
    monsters: [MonsterWithEncryptedId; NUM_MONSTERS],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut cells = [CellEncryptedData::default(); 5];

    // can ignore this check if necessary for performance
    if query_coord.x != center_coord.x || query_coord.y.abs_diff(center_coord.y) > 2 {
        for i in 0..5 {
            cells[i].entity_type = EntityType::Invalid;
        }
        return cells;
    }

    let query_coords = [
        MockEncryptedCoord {
            x: query_coord.x - 2 as MockEncrypted<u8>,
            y: query_coord.y,
        },
        MockEncryptedCoord {
            x: query_coord.x - 1,
            y: query_coord.y,
        },
        query_coord,
        MockEncryptedCoord {
            x: query_coord.x + 1,
            y: query_coord.y,
        },
        MockEncryptedCoord {
            x: query_coord.x + 2,
            y: query_coord.y,
        },
    ];

    for (idx, query_coord) in query_coords.iter().enumerate() {
        cells[idx] = fhe_get_cell_no_check(query_coord.clone(), monsters, items, players);
    }

    cells
}

fn pk_encrypt<T>(value: T) -> MockEncrypted<T> {
    return value;
}

impl MockZone {
    pub fn new(width: u8, height: u8) -> Self {
        let players = [
            Player {
                id: 0,
                data: PlayerEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(3),
                        y: pk_encrypt(27),
                    },
                    hp: pk_encrypt(5),
                    atk: pk_encrypt(1),
                    points: pk_encrypt(0),
                },
            },
            Player {
                id: 1,
                data: PlayerEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(19),
                        y: pk_encrypt(27),
                    },
                    hp: pk_encrypt(5),
                    atk: pk_encrypt(1),
                    points: pk_encrypt(0),
                },
            },
            Player {
                id: 2,
                data: PlayerEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(28),
                        y: pk_encrypt(28),
                    },
                    hp: pk_encrypt(5),
                    atk: pk_encrypt(1),
                    points: pk_encrypt(0),
                },
            },
            Player {
                id: 3,
                data: PlayerEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(12),
                        y: pk_encrypt(29),
                    },
                    hp: pk_encrypt(5),
                    atk: pk_encrypt(1),
                    points: pk_encrypt(0),
                },
            },
        ];

        let filler_item = Item {
            id: 0,
            data: ItemEncryptedData {
                loc: MockEncryptedCoord {
                    x: pk_encrypt(0),
                    y: pk_encrypt(0),
                },
                hp: pk_encrypt(0),
                atk: pk_encrypt(0),
                points: pk_encrypt(0),
                is_consumed: pk_encrypt(false),
            },
        };
        let mut items: [Item; NUM_ITEMS] = from_fn(|_| filler_item.clone());
        let plaintext_items = get_all_items();
        for (idx, plaintext_item) in plaintext_items.iter().enumerate() {
            items[idx] = Item {
                id: idx,
                data: ItemEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(plaintext_item.x),
                        y: pk_encrypt(plaintext_item.y),
                    },
                    hp: pk_encrypt(plaintext_item.hp),
                    atk: pk_encrypt(plaintext_item.atk),
                    points: pk_encrypt(plaintext_item.points),
                    is_consumed: pk_encrypt(false),
                },
            };
        }

        let filler_monster = Monster {
            id: 0,
            data: MonsterEncryptedData {
                loc: MockEncryptedCoord {
                    x: pk_encrypt(0),
                    y: pk_encrypt(0),
                },
                hp: pk_encrypt(0),
                atk: pk_encrypt(0),
                points: pk_encrypt(0),
            },
        };
        let mut monsters: [Monster; NUM_MONSTERS] = from_fn(|_| filler_monster.clone());
        let plaintext_monsters = get_all_monsters();
        for (idx, plaintext_monster) in plaintext_monsters.iter().enumerate() {
            monsters[idx] = Monster {
                id: idx,
                data: MonsterEncryptedData {
                    loc: MockEncryptedCoord {
                        x: pk_encrypt(plaintext_monster.x),
                        y: pk_encrypt(plaintext_monster.y),
                    },
                    hp: pk_encrypt(plaintext_monster.hp),
                    atk: pk_encrypt(plaintext_monster.atk),
                    points: pk_encrypt(plaintext_monster.points),
                },
            };
        }

        let filler_coord = MockEncryptedCoord {
            x: pk_encrypt(255),
            y: pk_encrypt(255),
        };
        let mut obstacles: [MockEncryptedCoord; NUM_OBSTACLES] = from_fn(|_| filler_coord.clone());
        let plaintext_obstacles = get_all_obstacles();
        for (idx, plaintext_obstacle) in plaintext_obstacles.iter().enumerate() {
            obstacles[idx] = MockEncryptedCoord {
                x: pk_encrypt(plaintext_obstacle.x),
                y: pk_encrypt(plaintext_obstacle.y),
            };
        }

        let random_state = thread_rng().gen();

        let precomputed_ids = [
            pk_encrypt(0),
            pk_encrypt(1),
            pk_encrypt(2),
            pk_encrypt(3),
            pk_encrypt(4),
            pk_encrypt(5),
            pk_encrypt(6),
            pk_encrypt(7),
            pk_encrypt(8),
            pk_encrypt(9),
            pk_encrypt(10),
            pk_encrypt(11),
            pk_encrypt(12),
            pk_encrypt(13),
            pk_encrypt(14),
            pk_encrypt(15),
            pk_encrypt(16),
            pk_encrypt(17),
            pk_encrypt(18),
            pk_encrypt(19),
            pk_encrypt(20),
            pk_encrypt(21),
            pk_encrypt(22),
            pk_encrypt(23),
            pk_encrypt(24),
            pk_encrypt(25),
            pk_encrypt(26),
            pk_encrypt(27),
            pk_encrypt(28),
            pk_encrypt(29),
            pk_encrypt(30),
            pk_encrypt(31),
            pk_encrypt(32),
            pk_encrypt(33),
        ];

        Self {
            width,
            height,
            players,
            items,
            monsters,
            obstacles,
            random_state,
            precomputed_ids,
        }
    }

    pub fn move_player(
        &mut self,
        player_id: usize,
        direction: MockEncrypted<Direction>,
    ) -> MockEncryptedCoord {
        assert!(player_id < self.players.len());

        let player_data = self.players[player_id].data.clone();

        let item_data = self.items.each_ref().map(|i| i.data.clone());

        let monster_data = self.monsters.each_ref().map(|i| i.data.clone());

        let obstacles: [_; NUM_OBSTACLES + 4] = from_fn(|i| {
            if i < NUM_OBSTACLES {
                self.obstacles[i].clone()
            } else {
                self.players[i - NUM_OBSTACLES].data.loc.clone()
            }
        });

        let (new_player_data, new_item_data, new_monster_data) = fhe_apply_move(
            player_data,
            direction,
            self.height,
            self.width,
            obstacles,
            monster_data,
            item_data,
        );

        self.players[player_id].data = new_player_data;

        for i in 0..NUM_ITEMS {
            self.items[i].data = new_item_data[i].clone();
        }

        for i in 0..NUM_MONSTERS {
            self.monsters[i].data = new_monster_data[i].clone();
        }

        self.players[player_id].data.loc.clone()
    }

    pub fn mix_random_input(&mut self, player_id: usize, random_input: u8) {
        assert!(player_id < self.players.len());

        self.random_state ^= random_input;
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

    fn fully_encrypted_items(&self) -> [ItemWithEncryptedId; NUM_ITEMS] {
        from_fn(|i| {
            let item = self.items[i].clone();
            ItemWithEncryptedId {
                id: self.precomputed_ids[item.id].clone(),
                data: item.data,
            }
        })
    }

    fn fully_encrypted_monsters(&self) -> [MonsterWithEncryptedId; NUM_MONSTERS] {
        from_fn(|i| {
            let monster = self.monsters[i].clone();
            MonsterWithEncryptedId {
                id: self.precomputed_ids[monster.id].clone(),
                data: monster.data,
            }
        })
    }

    pub fn get_cells(
        &self,
        player_id: usize,
        coords: Vec<MockEncryptedCoord>,
    ) -> Vec<CellEncryptedData> {
        let mut cells = Vec::new();
        let player_coord = &self.players[player_id].data.loc;

        for coord in coords.iter() {
            cells.push(fhe_get_cell(
                player_coord.clone(),
                coord.clone(),
                self.fully_encrypted_monsters(),
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
        coords: [MockEncryptedCoord; 5],
    ) -> [CellEncryptedData; 5] {
        fhe_get_five_cells(
            self.players[player_id].data.loc.clone(),
            coords,
            self.fully_encrypted_monsters(),
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_cross_cells(&self, player_id: usize) -> [CellEncryptedData; 5] {
        fhe_get_cross_cells(
            self.players[player_id].data.loc.clone(),
            self.fully_encrypted_monsters(),
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_vertical_cells(
        &self,
        player_id: usize,
        center: MockEncryptedCoord,
    ) -> [CellEncryptedData; 5] {
        fhe_get_vertical_cells(
            self.players[player_id].data.loc.clone(),
            center,
            self.fully_encrypted_monsters(),
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }

    pub fn get_horizontal_cells(
        &self,
        player_id: usize,
        center: MockEncryptedCoord,
    ) -> [CellEncryptedData; 5] {
        fhe_get_horizontal_cells(
            self.players[player_id].data.loc.clone(),
            center,
            self.fully_encrypted_monsters(),
            self.fully_encrypted_items(),
            self.fully_encrypted_players(),
        )
    }
}
