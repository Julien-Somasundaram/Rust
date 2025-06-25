use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum Cellule {
    Vide,
    Obstacle,
    Energie,
    Minerai,
    Scientifique,
}

pub struct Map {
    pub largeur: usize,
    pub hauteur: usize,
    pub grille: Vec<Vec<Cellule>>,
}

impl Map {
    pub fn new(largeur: usize, hauteur: usize, seed: u32) -> Self {
        let mut grille = vec![vec![Cellule::Vide; largeur]; hauteur];
        let perlin = Perlin::new(seed);

        for y in 0..hauteur {
            for x in 0..largeur {
                let nx = x as f64 / largeur as f64;
                let ny = y as f64 / hauteur as f64;
                let bruit = perlin.get([nx * 5.0, ny * 5.0]); // scale le bruit

                grille[y][x] = match bruit {
                    b if b < -0.4 => Cellule::Obstacle,
                    b if b < -0.1 => Cellule::Energie,
                    b if b < 0.2 => Cellule::Minerai,
                    b if b < 0.6 => Cellule::Scientifique,
                    _ => Cellule::Vide,
                };
            }
        }

        Map {
            largeur,
            hauteur,
            grille,
        }
    }

    pub fn afficher(&self) {
        for ligne in &self.grille {
            for cellule in ligne {
                let symbole = match cellule {
                    Cellule::Vide => ' ',
                    Cellule::Obstacle => '#',
                    Cellule::Energie => 'E',       // ⚡ → E
                    Cellule::Minerai => 'M',       // ⛏ → M
                    Cellule::Scientifique => 'S',  // ? → S
                };

                print!("{}", symbole);
            }
            println!();
        }
    }
}
