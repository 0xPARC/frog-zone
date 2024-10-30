#include "frogzone.h"

#pragma hls_top
CellDatas5 get_five_cells(
                          Coord player_coord,
                          Coords5 query_coords,
                          ItemsWithId items,
                          PlayersWithId players
                         ) {
  CellDatas5 cells;

  #pragma hls_unroll yes
  for (int i = 0; i < 5; i++) {
    Coord query_coord = query_coords.values[i];
    CellData cell;
    if (invalid_coord(player_coord, query_coord)) {
      CellData ret = CellData();
      ret.entity_type = Invalid;
      cell = ret;
    } else {
      cell = get_cell_no_check(query_coord, items, players);
    }
    cells.values[i] = cell;
  }

  return cells;
}

