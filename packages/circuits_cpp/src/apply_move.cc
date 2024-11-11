#include "frogzone.h"

const Coord obstacles[] = {
  Coord{29, 0},
  Coord{30, 0},
  Coord{1, 1},
  Coord{2, 1},
  Coord{3, 1},
  Coord{29, 1},
  Coord{30, 1},
  Coord{1, 2},
  Coord{2, 2},
  Coord{3, 2},
  Coord{4, 2},
  Coord{15, 2},
  Coord{16, 2},
  Coord{17, 2},
  Coord{18, 2},
  Coord{19, 2},
  Coord{16, 3},
  Coord{17, 3},
  Coord{18, 3},
  Coord{19, 3},
  Coord{29, 4},
  Coord{30, 4},
  Coord{31, 4},
  Coord{3, 5},
  Coord{4, 5},
  Coord{5, 5},
  Coord{29, 5},
  Coord{3, 6},
  Coord{6, 6},
  Coord{9, 6},
  Coord{10, 6},
  Coord{20, 6},
  Coord{21, 6},
  Coord{29, 6},
  Coord{1, 7},
  Coord{2, 7},
  Coord{7, 7},
  Coord{9, 7},
  Coord{11, 7},
  Coord{12, 7},
  Coord{13, 7},
  Coord{19, 7},
  Coord{22, 7},
  Coord{23, 7},
  Coord{28, 7},
  Coord{0, 8},
  Coord{6, 8},
  Coord{9, 8},
  Coord{14, 8},
  Coord{15, 8},
  Coord{16, 8},
  Coord{17, 8},
  Coord{18, 8},
  Coord{24, 8},
  Coord{25, 8},
  Coord{26, 8},
  Coord{29, 8},
  Coord{6, 9},
  Coord{9, 9},
  Coord{26, 9},
  Coord{29, 9},
  Coord{3, 10},
  Coord{4, 10},
  Coord{5, 10},
  Coord{6, 10},
  Coord{9, 10},
  Coord{26, 10},
  Coord{29, 10},
  Coord{1, 11},
  Coord{2, 11},
  Coord{10, 11},
  Coord{11, 11},
  Coord{15, 11},
  Coord{16, 11},
  Coord{17, 11},
  Coord{26, 11},
  Coord{28, 11},
  Coord{0, 12},
  Coord{12, 12},
  Coord{14, 12},
  Coord{18, 12},
  Coord{19, 12},
  Coord{26, 12},
  Coord{29, 12},
  Coord{13, 13},
  Coord{14, 13},
  Coord{20, 13},
  Coord{25, 13},
  Coord{30, 13},
  Coord{31, 13},
  Coord{20, 14},
  Coord{24, 14},
  Coord{10, 15},
  Coord{11, 15},
  Coord{12, 15},
  Coord{21, 15},
  Coord{22, 15},
  Coord{23, 15},
  Coord{0, 16},
  Coord{8, 16},
  Coord{9, 16},
  Coord{13, 16},
  Coord{14, 16},
  Coord{1, 17},
  Coord{8, 17},
  Coord{15, 17},
  Coord{30, 17},
  Coord{31, 17},
  Coord{2, 18},
  Coord{7, 18},
  Coord{14, 18},
  Coord{15, 18},
  Coord{31, 18},
  Coord{3, 19},
  Coord{8, 19},
  Coord{9, 19},
  Coord{10, 19},
  Coord{11, 19},
  Coord{12, 19},
  Coord{13, 19},
  Coord{3, 20},
  Coord{20, 20},
  Coord{21, 20},
  Coord{22, 20},
  Coord{1, 21},
  Coord{2, 21},
  Coord{17, 21},
  Coord{18, 21},
  Coord{19, 21},
  Coord{23, 21},
  Coord{24, 21},
  Coord{25, 21},
  Coord{26, 21},
  Coord{0, 22},
  Coord{18, 22},
  Coord{27, 22},
  Coord{28, 22},
  Coord{15, 23},
  Coord{16, 23},
  Coord{19, 23},
  Coord{28, 23},
  Coord{12, 24},
  Coord{13, 24},
  Coord{14, 24},
  Coord{17, 24},
  Coord{19, 24},
  Coord{28, 24},
  Coord{9, 25},
  Coord{10, 25},
  Coord{11, 25},
  Coord{16, 25},
  Coord{19, 25},
  Coord{27, 25},
  Coord{8, 26},
  Coord{16, 26},
  Coord{18, 26},
  Coord{19, 26},
  Coord{27, 26},
  Coord{9, 27},
  Coord{10, 27},
  Coord{11, 27},
  Coord{12, 27},
  Coord{14, 27},
  Coord{15, 27},
  Coord{20, 27},
  Coord{25, 27},
  Coord{26, 27},
  Coord{0, 28},
  Coord{1, 28},
  Coord{13, 28},
  Coord{20, 28},
  Coord{21, 28},
  Coord{22, 28},
  Coord{23, 28},
  Coord{24, 28},
  Coord{2, 29},
  Coord{3, 29},
  Coord{4, 29},
  Coord{5, 29},
  Coord{6, 30},
  Coord{7, 30},
  Coord{8, 31},
  Coord{9, 31},
};
const int obstacles_len = 183;

Coord apply_move_raw(
                     Coord old_coords,
                     Direction direction) {
  Coord new_coords = old_coords;
  switch (direction) {
    case Up:
      if (new_coords.y > 0) {
        new_coords.y -= 1;
      }
      break;
    case Down:
      if (new_coords.y < HEIGHT-1) {
        new_coords.y += 1;
      }
      break;
    case Left:
      if (new_coords.x > 0) {
        new_coords.x -= 1;
      }
      break;
    case Right:
      if (new_coords.x < WIDTH-1) {
        new_coords.x += 1;
      }
      break;
  }
  return new_coords;
}

Coord apply_move_check_collisions(
                                  Coord old_coords,
                                  Direction direction,
                                  Obstacles4 players_coords) {
  Coord new_coords = apply_move_raw(old_coords, direction);

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_PLAYERS; i++) {
    Coord obstacle = players_coords.values[i];
    if (new_coords == obstacle) {
      return old_coords;
    }
  }
  #pragma hls_unroll yes
  for (int i = 0; i < obstacles_len; i++) {
    Coord obstacle = obstacles[i];
    if (new_coords == obstacle) {
      return old_coords;
    }
  }
  return new_coords;
}

#pragma hls_top
ApplyMoveOut apply_move(
                        PlayerData player_data,
                        Direction direction,
                        Obstacles4 players_coords,
                        Monsters monsters,
                        Items items) {
  Coord old_coords = player_data.loc;
  Coord new_coords = apply_move_check_collisions(player_data.loc, direction, players_coords);

  PlayerData new_player_data = player_data;
  Items new_items = items;
  Monsters new_monsters = monsters;

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_ITEMS; i++) {
    ItemData item = new_items.values[i];
    if ((new_coords == item.loc) && (!item.is_consumed)) {
      new_items.values[i].is_consumed = true;
      new_player_data.atk += item.atk;
      new_player_data.hp += item.hp;
      new_player_data.points += item.points;
    }
  }

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_MONSTERS; i++) {
    MonsterData monster = new_monsters.values[i];
    if ((new_coords == monster.loc) && (monster.hp != 0)) {
        if (player_data.hp <= monster.atk) {
            new_player_data.hp = 0;
        } else {
            new_player_data.hp -= monster.atk;
        }

        if (player_data.atk >= monster.hp) {
            new_monsters.values[i].hp = 0;
            new_player_data.atk += monster.atk;
            new_player_data.points += monster.points;
        } else {
            new_monsters.values[i].hp -= player_data.atk;
        }

        new_coords = old_coords;
    }
  }

  new_player_data.loc = new_coords;

  return ApplyMoveOut{
    .player_data = new_player_data,
    .items = new_items,
    .monsters = new_monsters,
  };
}
