#include "frogzone.h"

#pragma hls_top
CellData get_cell(
                  Coord player_coord,
                  Coord query_coord,
                  MonstersWithId monsters,
                  ItemsWithId items,
                  PlayersWithId players
                  ) {
  if (invalid_coord(player_coord, query_coord)) {
    CellData ret = CellData();
    ret.entity_type = Invalid;
    return ret;
  }

  return get_cell_no_check(query_coord, monsters, items, players);
}
