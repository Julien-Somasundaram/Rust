mod map;
mod robot;

use map::Map;
use std::{thread, time::Duration};

fn main() {
    let seed = 587;
    let mut carte = Map::new(40, 20, seed);

    for tour in 0..20 {
        println!("--- Tour {} ---", tour);
        carte.afficher();
        carte.tick();
        thread::sleep(Duration::from_millis(300));
        println!("\x1B[2J\x1B[1;1H"); // Clear écran terminal
    }
}
