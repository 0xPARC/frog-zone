#include "frogzone.h"

void test_apply_move() {
  PlayerData player_data;
  Direction direction;
  Obstacles4 obstacles;
  Monsters monsters;
  Items items;
  apply_move(player_data, direction, obstacles, monsters, items);
}

void test_apply_move_monster() {
  Coord old_coords;
  Direction direction;
  Obstacles4 player_coords;
  Obstacles23 monster_coords;
  Obstacles12 item_coords;
  apply_move_monster(old_coords, direction, player_coords, monster_coords, item_coords);
}

void test_apply_move_flying() {
  Coord old_coords;
  Direction direction;
  Obstacles4 player_coords;
  Obstacles23 monster_coords;
  Obstacles12 item_coords;
  apply_move_flying(old_coords, direction, player_coords, monster_coords, item_coords);
}

void test_get_cell() {
  Coord player_coord;
  Coord query_coord;
  ItemsWithId items;
  MonstersWithId monsters;
  PlayersWithId players;
  get_cell(player_coord, query_coord, monsters, items, players);
}

void test_get_five_cells() {
  Coord player_coord;
  Coords5 query_coords;
  ItemsWithId items;
  MonstersWithId monsters;
  PlayersWithId players;
  get_five_cells(player_coord, query_coords, monsters, items, players);
}

void test_get_cross_cells() {
  Coord player_coord;
  MonstersWithId monsters;
  ItemsWithId items;
  PlayersWithId players;
  get_cross_cells(player_coord, monsters, items, players);
}

void test_get_horizontal_cells() {
  Coord player_coord;
  Coord query_coord;
  MonstersWithId monsters;
  ItemsWithId items;
  PlayersWithId players;
  get_horizontal_cells(player_coord, query_coord, monsters, items, players);
}

void test_get_vertical_cells() {
  Coord player_coord;
  Coord query_coord;
  MonstersWithId monsters;
  ItemsWithId items;
  PlayersWithId players;
  get_vertical_cells(player_coord, query_coord, monsters, items, players);
}

int main() {
  test_apply_move();
  test_apply_move_monster();
  test_apply_move_flying();
  test_get_cell();
  test_get_five_cells();
  test_get_cross_cells();
  test_get_horizontal_cells();
  test_get_vertical_cells();
  return 0;
}
