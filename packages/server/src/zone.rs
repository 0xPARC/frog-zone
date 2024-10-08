use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Deserialize)]
pub enum EntityType {
    None,
    Player,
    Item,
    Monster,
}

pub struct CellEncryptedData {
    pub entity_type: Encrypted<EntityType>,
    pub entity_id: Encrypted<usize>,
    pub hp: Encrypted<u8>,
    pub atk: Encrypted<u8>,
    pub is_consumed: Encrypted<bool>,
}

#[derive(Clone, Copy, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Serialize, Clone)]
pub enum Event {
    PlayerMove(usize),
}

pub struct Zone {
    pub width: u8,
    pub height: u8,
    pub players: [Player; 4],
    pub items: [Item; 2],
    pub obstacles: [EncryptedCoord; 2],
}

pub fn apply_move_raw(
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

pub fn apply_move_check_collisions(
    old_coords: EncryptedCoord,
    direction: Encrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
) -> EncryptedCoord {
    let new_coords = apply_move_raw(old_coords, direction, height, width);

    for obstacle in obstacles {
        if new_coords == obstacle {
            return old_coords;
        }
    }

    new_coords
}

pub fn apply_move(
    player_data: PlayerEncryptedData,
    direction: Encrypted<Direction>,
    height: u8,
    width: u8,
    obstacles: [EncryptedCoord; 100],
    items: [ItemEncryptedData; 2],
) -> (PlayerEncryptedData, [ItemEncryptedData; 2]) {
    let new_coords =
        apply_move_check_collisions(player_data.loc, direction, height, width, obstacles);

    let mut new_player_data = player_data.clone();
    new_player_data.loc = new_coords;
    let mut new_item_data = items.clone();

    for (idx, item) in items.iter().enumerate() {
        if new_coords == item.loc {
            new_item_data[idx].is_consumed.val = true;
            new_player_data.atk.val += new_item_data[idx].atk.val;
            new_player_data.hp.val += new_item_data[idx].hp.val;
        }
    }

    (new_player_data, new_item_data)
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
                        x: Encrypted { val: 31 },
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

        let obstacles = [
            EncryptedCoord {
                x: Encrypted { val: 0 },
                y: Encrypted { val: 0 },
            },
            EncryptedCoord {
                x: Encrypted { val: 20 },
                y: Encrypted { val: 20 },
            },
        ];

        Self {
            width,
            height,
            players,
            items,
            obstacles,
        }
    }

    pub fn move_player(&mut self, player_id: usize, direction: Encrypted<Direction>) -> Vec<Event> {
        let mut events = Vec::new();
        if player_id >= self.players.len() {
            return events;
        }

        let mut player = self.players[player_id];
        let player_data = player.data.clone();

        let item_data = self.items.map(|i| i.data.clone());

        let filler_coord = EncryptedCoord {
            x: Encrypted { val: 255 },
            y: Encrypted { val: 255 },
        };
        let mut obstacles = vec![filler_coord; 100];

        let mut count = 0;
        for obstacle in &self.obstacles {
            obstacles[count] = obstacle.clone();
            count += 1;
        }
        for player in &self.players {
            if player.id != player_id {
                obstacles[count] = player.data.loc.clone();
                count += 1;
            }
        }

        let (new_player_data, new_item_data) = apply_move(
            player_data,
            direction,
            self.height,
            self.width,
            obstacles.try_into().unwrap(),
            item_data,
        );

        player.data = new_player_data;

        for i in 0..self.items.len() {
            self.items[i].data = new_item_data[i];
        }

        events.push(Event::PlayerMove(player_id));

        events
    }

    pub fn get_viewport(&self, player_id: usize, claimed_coords: EncryptedCoord) -> [[]]
}
