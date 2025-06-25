use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use crate::robot::{Robot, TypeRobot};


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
    pub robots: Vec<Robot>,
    pub collecte: Ressources,
    pub base: (usize, usize),
}
#[derive(Debug, Default, Clone, Copy)]
pub struct Ressources {
    pub energie: u32,
    pub minerai: u32,
    pub science: u32,
}
impl Map {
    pub fn new(largeur: usize, hauteur: usize, seed: u32) -> Self {
        use rand::{SeedableRng, rngs::StdRng};
        let mut grille = vec![vec![Cellule::Vide; largeur]; hauteur];
        let perlin = noise::Perlin::new(seed);

        for y in 0..hauteur {
            for x in 0..largeur {
                let nx = x as f64 / largeur as f64;
                let ny = y as f64 / hauteur as f64;
                let bruit = perlin.get([nx * 5.0, ny * 5.0]);
                grille[y][x] = match bruit {
                    b if b < -0.4 => Cellule::Obstacle,
                    b if b < -0.1 => Cellule::Energie,
                    b if b < 0.2 => Cellule::Minerai,
                    b if b < 0.6 => Cellule::Scientifique,
                    _ => Cellule::Vide,
                };
            }
        }

        // Génération des robots
        let mut rng = StdRng::seed_from_u64(seed as u64);
        let mut robots: Vec<Robot> = vec![];

        let types = [TypeRobot::Explorateur, TypeRobot::Recolteur, TypeRobot::Scientifique];

        for id in 0..5 {
            loop {
                let x = rng.gen_range(0..largeur);
                let y = rng.gen_range(0..hauteur);

                if !matches!(grille[y][x], Cellule::Obstacle) &&
                   !robots.iter().any(|r| r.x == x && r.y == y)
                {
                    robots.push(Robot {
                        id,
                        x,
                        y,
                        kind: types[rng.gen_range(0..types.len())],
                        sac: Ressources::default(),
                        capacite: 5,
                        retour_base: false,
                    });
                    break;
                }
            }
        }
        let base = (0, 0); // position fixe de la station de base

        Self {
        largeur,
        hauteur,
        grille,
        robots,
        collecte: Ressources::default(),
        base,
        
}

    }

    pub fn afficher(&self) {
        for y in 0..self.hauteur {
            for x in 0..self.largeur {
                if let Some(robot) = self.robots.iter().find(|r| r.x == x && r.y == y) {
                    print!("{}", robot.symbole());
                } else {
                    let symbole = match self.grille[y][x] {
                        Cellule::Vide => ' ',
                        Cellule::Obstacle => '#',
                        Cellule::Energie => '+',
                        Cellule::Minerai => '*',
                        Cellule::Scientifique => '?',
                    };
                    print!("{}", symbole);
                }
            }
            println!();
        }
    }
    pub fn tick(&mut self) {
    let positions: Vec<(usize, usize)> = self.robots.iter().map(|r| (r.x, r.y)).collect();
    for robot in &mut self.robots {
    let other_positions: Vec<(usize, usize)> = positions
        .iter()
        .filter(|&&(x, y)| x != robot.x || y != robot.y)
        .copied()
        .collect();

    let cible = if robot.retour_base {
        Some(self.base)
    } else {
        None
    };

    robot.deplacer_vers(
        self.largeur,
        self.hauteur,
        |x, y| !matches!(self.grille[y][x], Cellule::Obstacle),
        &other_positions,
        cible,
    );

    let (x, y) = (robot.x, robot.y);

    // Si sur une ressource et pas plein
    if !robot.retour_base {
        let libre = robot.capacite
            - (robot.sac.energie + robot.sac.minerai + robot.sac.science);
        if libre > 0 {
            match self.grille[y][x] {
                Cellule::Energie => {
                    robot.sac.energie += 1;
                    self.grille[y][x] = Cellule::Vide;
                }
                Cellule::Minerai => {
                    robot.sac.minerai += 1;
                    self.grille[y][x] = Cellule::Vide;
                }
                Cellule::Scientifique => {
                    robot.sac.science += 1;
                    self.grille[y][x] = Cellule::Vide;
                }
                _ => {}
            }
        }

        let total = robot.sac.energie + robot.sac.minerai + robot.sac.science;
        if total >= robot.capacite {
            robot.retour_base = true;
        }
    }

    // Déposer à la base
    if robot.retour_base && (x, y) == self.base {
        self.collecte.energie += robot.sac.energie;
        self.collecte.minerai += robot.sac.minerai;
        self.collecte.science += robot.sac.science;
        robot.sac = Ressources::default();
        robot.retour_base = false;
    }
}
   
    
}

}