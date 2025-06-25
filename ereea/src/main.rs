mod map;

use map::Map;
use rand::Rng;

fn main() {
    let seed = rand::thread_rng().gen_range(0..10000);
    let carte = Map::new(40, 20, seed);

    println!("Carte générée avec la seed : {}", seed);
    carte.afficher();
}
