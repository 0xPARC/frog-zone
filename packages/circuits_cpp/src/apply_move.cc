#include "frogzone.h"

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
                                  Obstacles100 obstacles) {
  Coord new_coords = apply_move_raw(old_coords, direction);

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_OBSTACLES; i++) {
    Coord obstacle = obstacles.values[i];
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
                        Obstacles100 obstacles,
                        Items items) {
  Coord new_coords = apply_move_check_collisions(player_data.loc, direction, obstacles);

  PlayerData new_player_data = player_data;
  new_player_data.loc = new_coords;
  Items new_items = items;

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_ITEMS; i++) {
    ItemData item = new_items.values[i];
    if ((new_coords == item.loc) && (!item.is_consumed)) {
      new_items.values[i].is_consumed = true;
      new_player_data.atk += item.atk;
      new_player_data.hp += item.hp;
    }
  }

  return ApplyMoveOut{
    .player_data = new_player_data,
    .items = new_items,
  };
}
