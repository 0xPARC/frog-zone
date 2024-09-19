use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Coord {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub hp: u32,
    pub atk: u32,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Item {
    pub hp: u32,
    pub atk: u32,
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
    PlayerAdd(Player, Coord),
    PlayerMove(usize, Coord),
    ItemAdd(Item, Coord),
    ItemPickup(usize, Item, u32, u32, u32, u32, Coord),
}

pub struct Zone {
    pub width: u16,
    pub height: u16,
    pub players: HashMap<Coord, Player>,
    pub items: HashMap<Coord, Item>,
    pub tick_rate: u64,
}

impl Zone {
    pub fn new(width: u16, height: u16, tick_rate: u64) -> Self {
        Self {
            width,
            height,
            players: HashMap::new(),
            items: HashMap::new(),
            tick_rate,
        }
    }

    pub fn move_player(&mut self, player_id: usize, direction: Direction) -> Vec<Event> {
        let mut events = Vec::new();
        let old_coord = match self.players.iter().find(|(_, p)| p.id == player_id) {
            Some((coord, _)) => *coord,
            None => return events,
        };

        let new_coord = match direction {
            Direction::Up => Coord {
                x: old_coord.x,
                y: old_coord.y.saturating_sub(1),
            },
            Direction::Down => Coord {
                x: old_coord.x,
                y: (old_coord.y + 1).min(self.height - 1),
            },
            Direction::Left => Coord {
                x: old_coord.x.saturating_sub(1),
                y: old_coord.y,
            },
            Direction::Right => Coord {
                x: (old_coord.x + 1).min(self.width - 1),
                y: old_coord.y,
            },
        };

        if new_coord == old_coord || self.players.contains_key(&new_coord) {
            return events;
        }

        let player = self.players.remove(&old_coord).unwrap();
        events.push(Event::PlayerMove(player_id, new_coord));

        if let Some(item) = self.items.remove(&new_coord) {
            let old_hp = player.hp;
            let old_atk = player.atk;
            let new_hp = player.hp + item.hp;
            let new_atk = player.atk + item.atk;
            let player = self.players.entry(new_coord).or_insert(player);
            player.hp = new_hp;
            player.atk = new_atk;
            events.push(Event::ItemPickup(
                player_id, item, old_hp, new_hp, old_atk, new_atk, new_coord,
            ));
        } else {
            self.players.insert(new_coord, player);
        }

        events
    }

    pub fn add_player(&mut self, player: Player, coord: Coord) -> Event {
        if self.players.contains_key(&coord) || self.items.contains_key(&coord) {
            panic!("occupied");
        }
        self.players.insert(coord, player);
        Event::PlayerAdd(player, coord)
    }

    pub fn add_item(&mut self, item: Item, coord: Coord) -> Event {
        if self.players.contains_key(&coord) || self.items.contains_key(&coord) {
            panic!("occupied");
        }
        self.items.insert(coord, item);
        Event::ItemAdd(item, coord)
    }
}
