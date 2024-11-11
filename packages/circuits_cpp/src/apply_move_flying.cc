#include "frogzone.h"

Coord apply_move_raw_flying(
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

#pragma hls_top
Coord apply_move_flying(
                        Coord old_coords,
                        Direction direction,
                        Obstacles4 players_coords,
                        Obstacles23 monster_coords,
                        Obstacles12 item_coords) {
  Coord new_coords = apply_move_raw_flying(old_coords, direction);

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_PLAYERS; i++) {
    Coord obstacle = players_coords.values[i];
    if (new_coords == obstacle) {
      return old_coords;
    }
  }

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_MONSTERS; i++) {
    Coord obstacle = monster_coords.values[i];
    if (new_coords == obstacle) {
      return old_coords;
    }
  }

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_ITEMS; i++) {
    Coord obstacle = item_coords.values[i];
    if (new_coords == obstacle) {
      return old_coords;
    }
  }

  return new_coords;
}
