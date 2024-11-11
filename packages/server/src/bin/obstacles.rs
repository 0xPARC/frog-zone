//! Helper util to dump the obstacles in C source code.

use server::initial_data::get_all_obstacles;

fn main() {
    let obstacles = get_all_obstacles();

    println!("const Coord obstacles[] = {{");
    for obstacle in &obstacles {
        println!("  Coord{{{}, {}}},", obstacle.x, obstacle.y);
    }
    println!("}};");
    println!("const int obstacles_len = {};", obstacles.len());
}
