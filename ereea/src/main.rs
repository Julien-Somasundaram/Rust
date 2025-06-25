mod map;
mod robot;

use map::Map;

fn main() {
    let seed = 587;
    let carte = Map::new(40, 20, seed);

    println!("Carte générée avec la seed : {}", seed);
    carte.afficher();
}
