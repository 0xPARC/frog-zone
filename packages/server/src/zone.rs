use core::array::from_fn;
use itertools::{chain, Itertools};
use phantom::{PhantomBool, PhantomCt, PhantomEvaluator};

const NUM_ITEMS: usize = 16;

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

impl EncryptedCoord {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![&self.x, &self.y]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }

    pub fn from_cts(
        cts: &mut impl Iterator<Item = PhantomCt>,
        evaluator: &PhantomEvaluator,
    ) -> Self {
        Self {
            x: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
            y: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlayerEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: EncryptedU8,
    pub atk: EncryptedU8,
}

impl PlayerEncryptedData {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![self.loc.bits(), &self.hp, &self.atk]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }

    pub fn from_cts(
        cts: &mut impl Iterator<Item = PhantomCt>,
        evaluator: &PhantomEvaluator,
    ) -> Self {
        Self {
            loc: EncryptedCoord::from_cts(cts, evaluator),
            hp: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
            atk: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
        }
    }
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

impl PlayerWithEncryptedId {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![&self.id, self.data.bits()]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }
}

#[derive(Clone, Debug)]
pub struct ItemEncryptedData {
    pub loc: EncryptedCoord,
    pub hp: EncryptedU8,
    pub atk: EncryptedU8,
    pub is_consumed: EncryptedBool,
}

impl ItemEncryptedData {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![self.loc.bits(), &self.hp, &self.atk, [&self.is_consumed]]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }

    pub fn from_cts(
        cts: &mut impl Iterator<Item = PhantomCt>,
        evaluator: &PhantomEvaluator,
    ) -> Self {
        Self {
            loc: EncryptedCoord::from_cts(cts, evaluator),
            hp: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
            atk: from_fn(|_| evaluator.wrap(cts.next().unwrap())),
            is_consumed: evaluator.wrap(cts.next().unwrap()),
        }
    }
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

impl ItemWithEncryptedId {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![&self.id, self.data.bits()]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }
}

#[derive(Clone, Debug)]
pub enum EntityType {
    Invalid,
    Player,
    Item,
    Monster,
    None,
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

impl CellEncryptedData {
    /// Returns concatenation of each field as bits in little-endian.
    pub fn bits(&self) -> impl Iterator<Item = &PhantomBool> {
        chain![&self.entity_type, &self.entity_id, &self.hp, &self.atk]
    }

    pub fn cts(&self) -> impl Iterator<Item = &PhantomCt> {
        self.bits().map(|bit| bit.ct())
    }
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
    pub items: [Item; NUM_ITEMS],
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
    items: [ItemEncryptedData; NUM_ITEMS],
) -> (PlayerEncryptedData, [ItemEncryptedData; NUM_ITEMS]) {
    let mut output_bits = phantom_benchs::frogzone_apply_move_rs_fhe_lib::apply_move(
        &direction.to_vec(),
        &items
            .iter()
            .flat_map(|item| item.bits())
            .cloned()
            .collect_vec(),
        &obstacles
            .iter()
            .flat_map(|obstacle| obstacle.bits())
            .cloned()
            .collect_vec(),
        &player_data.bits().cloned().collect_vec(),
    )
    .into_iter();
    let output = (
        PlayerEncryptedData {
            loc: EncryptedCoord {
                x: from_fn(|_| output_bits.next().unwrap()),
                y: from_fn(|_| output_bits.next().unwrap()),
            },
            hp: from_fn(|_| output_bits.next().unwrap()),
            atk: from_fn(|_| output_bits.next().unwrap()),
        },
        from_fn(|_| ItemEncryptedData {
            loc: EncryptedCoord {
                x: from_fn(|_| output_bits.next().unwrap()),
                y: from_fn(|_| output_bits.next().unwrap()),
            },
            hp: from_fn(|_| output_bits.next().unwrap()),
            atk: from_fn(|_| output_bits.next().unwrap()),
            is_consumed: output_bits.next().unwrap(),
        }),
    );
    assert!(output_bits.next().is_none());
    output
}

fn fhe_get_cell_no_check(
    coord: EncryptedCoord,
    items: [ItemWithEncryptedId; NUM_ITEMS],
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
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> CellEncryptedData {
    let mut output_bits = phantom_benchs::frogzone_get_cell_rs_fhe_lib::get_cell(
        &items
            .iter()
            .flat_map(|item| item.bits())
            .cloned()
            .collect_vec(),
        &player_coord.bits().cloned().collect_vec(),
        &players
            .iter()
            .flat_map(|player| player.bits())
            .cloned()
            .collect_vec(),
        &query_coord.bits().cloned().collect_vec(),
    )
    .into_iter();
    let output = CellEncryptedData {
        entity_type: from_fn(|_| output_bits.next().unwrap()),
        entity_id: from_fn(|_| output_bits.next().unwrap()),
        hp: from_fn(|_| output_bits.next().unwrap()),
        atk: from_fn(|_| output_bits.next().unwrap()),
    };
    assert!(output_bits.next().is_none());
    output
}

fn fhe_get_five_cells(
    player_coord: EncryptedCoord,
    query_coords: [EncryptedCoord; 5],
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut output_bits = phantom_benchs::frogzone_get_five_cells_rs_fhe_lib::get_five_cells(
        &items
            .iter()
            .flat_map(|item| item.bits())
            .cloned()
            .collect_vec(),
        &player_coord.bits().cloned().collect_vec(),
        &players
            .iter()
            .flat_map(|player| player.bits())
            .cloned()
            .collect_vec(),
        &query_coords
            .iter()
            .flat_map(|query_coord| query_coord.bits())
            .cloned()
            .collect_vec(),
    )
    .into_iter();
    let output = from_fn(|_| CellEncryptedData {
        entity_type: from_fn(|_| output_bits.next().unwrap()),
        entity_id: from_fn(|_| output_bits.next().unwrap()),
        hp: from_fn(|_| output_bits.next().unwrap()),
        atk: from_fn(|_| output_bits.next().unwrap()),
    });
    assert!(output_bits.next().is_none());
    output
}

fn fhe_get_cross_cells(
    player_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut output_bits = phantom_benchs::frogzone_get_cross_cells_rs_fhe_lib::get_cross_cells(
        &items
            .iter()
            .flat_map(|item| item.bits())
            .cloned()
            .collect_vec(),
        &player_coord.bits().cloned().collect_vec(),
        &players
            .iter()
            .flat_map(|player| player.bits())
            .cloned()
            .collect_vec(),
    )
    .into_iter();
    let output = from_fn(|_| CellEncryptedData {
        entity_type: from_fn(|_| output_bits.next().unwrap()),
        entity_id: from_fn(|_| output_bits.next().unwrap()),
        hp: from_fn(|_| output_bits.next().unwrap()),
        atk: from_fn(|_| output_bits.next().unwrap()),
    });
    assert!(output_bits.next().is_none());
    output
}

fn fhe_get_vertical_cells(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut output_bits =
        phantom_benchs::frogzone_get_vertical_cells_rs_fhe_lib::get_vertical_cells(
            &items
                .iter()
                .flat_map(|item| item.bits())
                .cloned()
                .collect_vec(),
            &player_coord.bits().cloned().collect_vec(),
            &players
                .iter()
                .flat_map(|player| player.bits())
                .cloned()
                .collect_vec(),
            &query_coord.bits().cloned().collect_vec(),
        )
        .into_iter();
    let output = from_fn(|_| CellEncryptedData {
        entity_type: from_fn(|_| output_bits.next().unwrap()),
        entity_id: from_fn(|_| output_bits.next().unwrap()),
        hp: from_fn(|_| output_bits.next().unwrap()),
        atk: from_fn(|_| output_bits.next().unwrap()),
    });
    assert!(output_bits.next().is_none());
    output
}

fn fhe_get_horizontal_cells(
    player_coord: EncryptedCoord,
    query_coord: EncryptedCoord,
    items: [ItemWithEncryptedId; NUM_ITEMS],
    players: [PlayerWithEncryptedId; 4],
) -> [CellEncryptedData; 5] {
    let mut output_bits =
        phantom_benchs::frogzone_get_horizontal_cells_rs_fhe_lib::get_horizontal_cells(
            &items
                .iter()
                .flat_map(|item| item.bits())
                .cloned()
                .collect_vec(),
            &player_coord.bits().cloned().collect_vec(),
            &players
                .iter()
                .flat_map(|player| player.bits())
                .cloned()
                .collect_vec(),
            &query_coord.bits().cloned().collect_vec(),
        )
        .into_iter();
    let output = from_fn(|_| CellEncryptedData {
        entity_type: from_fn(|_| output_bits.next().unwrap()),
        entity_id: from_fn(|_| output_bits.next().unwrap()),
        hp: from_fn(|_| output_bits.next().unwrap()),
        atk: from_fn(|_| output_bits.next().unwrap()),
    });
    assert!(output_bits.next().is_none());
    output
}

fn pk_encrypt<const N: usize>(evaluator: &PhantomEvaluator, value: u8) -> [EncryptedBool; N] {
    evaluator
        .unbatch(&evaluator.batched_pk_encrypt((0..N).map(|i| (value >> i) & 1 == 1)))
        .try_into()
        .unwrap()
}

impl Zone {
    pub fn new(width: u8, height: u8, evaluator: &PhantomEvaluator) -> Self {
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

        let items = from_fn(|i| Item {
            id: i,
            data: ItemEncryptedData {
                loc: EncryptedCoord {
                    x: pk_encrypt(evaluator, i as _),
                    y: pk_encrypt(evaluator, i as _),
                },
                hp: pk_encrypt(evaluator, 1),
                atk: pk_encrypt(evaluator, 1),
                is_consumed: pk_encrypt::<1>(evaluator, 0)[0].clone(),
            },
        });

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
        assert!(player_id < self.players.len());

        let player_data = self.players[player_id].data.clone();

        let item_data = self.items.each_ref().map(|i| i.data.clone());

        let obstacles: [_; 100] = from_fn(|i| {
            if i < 96 {
                self.obstacles[i].clone()
            } else {
                self.players[i - 96].data.loc.clone()
            }
        });

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
            self.items[i].data = new_item_data[i].clone();
        }

        self.players[player_id].data.loc.clone()
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

    // For syncing with workers

    pub fn cts(&self) -> Vec<PhantomCt> {
        chain![
            self.players.iter().flat_map(|player| player.data.cts()),
            self.items.iter().flat_map(|item| item.data.cts()),
            self.obstacles.iter().flat_map(|obstacle| obstacle.cts())
        ]
        .cloned()
        .collect()
    }

    pub fn from_cts(
        width: u8,
        height: u8,
        cts: Vec<PhantomCt>,
        evaluator: &PhantomEvaluator,
    ) -> Self {
        let mut cts = cts.into_iter();
        Zone {
            width,
            height,
            players: from_fn(|id| Player {
                id,
                data: PlayerEncryptedData::from_cts(&mut cts, evaluator),
            }),
            items: from_fn(|id| Item {
                id,
                data: ItemEncryptedData::from_cts(&mut cts, evaluator),
            }),
            obstacles: from_fn(|_| EncryptedCoord::from_cts(&mut cts, evaluator)),
            precomputed_ids: from_fn(|id| pk_encrypt(evaluator, id as _)),
        }
    }

    pub fn cts_diff(&self, flags: [bool; 4]) -> ZoneDiff {
        (
            from_fn(|id| flags[id].then(|| self.players[id].data.cts().cloned().collect())),
            self.items
                .iter()
                .flat_map(|item| item.data.cts())
                .cloned()
                .collect(),
        )
    }

    pub fn apply_diff(&mut self, (players, items): ZoneDiff, evaluator: &PhantomEvaluator) {
        for (id, player) in players.into_iter().enumerate() {
            if let Some(player) = player {
                self.players[id] = Player {
                    id,
                    data: PlayerEncryptedData::from_cts(&mut player.into_iter(), evaluator),
                };
            }
        }
        let mut items = items.into_iter();
        self.items = from_fn(|id| Item {
            id,
            data: ItemEncryptedData::from_cts(&mut items, evaluator),
        })
    }
}

/// Diff of `players` and concatenation of `items` bits after some
/// `Zone::move_player`, used to sync with workers. If player is not updated
/// during the time, `players[id]` will be `None`.
pub type ZoneDiff = ([Option<Vec<PhantomCt>>; 4], Vec<PhantomCt>);
