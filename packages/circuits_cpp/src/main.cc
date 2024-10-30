#include "frogzone.h"

void test_apply_move() {
  PlayerData player_data;
  Direction direction;
  Obstacles100 obstacles;
  Items items;
  apply_move(player_data, direction, obstacles, items);
}

void test_get_cell() {
  Coord player_coord;
  Coord query_coord;
  ItemsWithId items;
  PlayersWithId players;
  get_cell(player_coord, query_coord, items, players);
}

void test_get_five_cells() {
  Coord player_coord;
  Coords5 query_coords;
  ItemsWithId items;
  PlayersWithId players;
  get_five_cells(player_coord, query_coords, items, players);
}

void test_get_cross_cells() {
  Coord player_coord;
  ItemsWithId items;
  PlayersWithId players;
  get_cross_cells(player_coord,  items, players);
}

void test_get_horizontal_cells() {
  Coord player_coord;
  Coord query_coord;
  ItemsWithId items;
  PlayersWithId players;
  get_horizontal_cells(player_coord, query_coord, items, players);
}

void test_get_vertical_cells() {
  Coord player_coord;
  Coord query_coord;
  ItemsWithId items;
  PlayersWithId players;
  get_vertical_cells(player_coord, query_coord, items, players);
}

int main() {
  test_apply_move();
  test_get_cell();
  test_get_five_cells();
  test_get_cross_cells();
  test_get_horizontal_cells();
  test_get_vertical_cells();
  return 0;
}
