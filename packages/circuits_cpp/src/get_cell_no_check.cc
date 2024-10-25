#include "frogzone.h"

CellData get_cell_no_check(
                           Coord coord,
                           ItemsWithId items,
                           PlayersWithId players) {
  CellData cell = CellData();

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_ITEMS; i++) {
    ItemWithId item = items.values[i];
    if ((coord == item.data.loc) && (!item.data.is_consumed)) {
      cell.entity_type = Item;
      cell.entity_id = item.id;
      cell.hp = item.data.hp;
      cell.atk = item.data.atk;
    }
  }

  #pragma hls_unroll yes
  for (int i = 0; i < NUM_PLAYERS; i++) {
    PlayerWithId player = players.values[i];
    if (coord == player.data.loc) {
      cell.entity_type = Player;
      cell.entity_id = player.id;
      cell.hp = player.data.hp;
      cell.atk = player.data.atk;
    }
  }

  return cell;
}

bool invalid_coord_x(
                 Coord player_coord,
                 Coord query_coord
                 ) {
  unsigned char abs_delta_x = player_coord.x > query_coord.x ?
    player_coord.x - query_coord.x : query_coord.x - player_coord.x;
  return abs_delta_x > 2;
}

bool invalid_coord_y(
                 Coord player_coord,
                 Coord query_coord
                 ) {
  unsigned char abs_delta_y = player_coord.y > query_coord.y ?
    player_coord.y - query_coord.y : query_coord.y - player_coord.y;
  return abs_delta_y > 2;
}

bool invalid_coord(
                 Coord player_coord,
                 Coord query_coord
                 ) {
  return invalid_coord_x(player_coord, query_coord) || invalid_coord_y(player_coord, query_coord);
}
