use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;
use crate::robot::{Robot, TypeRobot, Module};


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
    pub bases: Vec<(usize, usize)>,
    pub explorée: Vec<Vec<bool>>

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
        let bases = vec![(0, 0), (largeur - 1, hauteur - 1)];


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
        let kind = types[rng.gen_range(0..types.len())];
        let modules = match kind {
            TypeRobot::Explorateur => vec![Module::Capteur],
            TypeRobot::Recolteur => vec![Module::Foreuse],
            TypeRobot::Scientifique => vec![Module::Analyseur],
        };
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
                        kind,
                        sac: Ressources::default(),
                        capacite: 5,
                        retour_base: false,
                        modules: modules.clone(),
                        last_pos: None,
                    });
                    break;
                }
            }
        }

        let explorée = vec![vec![false; largeur]; hauteur];

        Self {
        largeur,
        hauteur,
        grille,
        robots,
        collecte: Ressources::default(),
        bases,
        explorée: explorée,
        
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

    let cibles: Vec<Option<(usize, usize)>> = self.robots.iter().map(|r| {
        if r.retour_base {
            Some(self.base_la_plus_proche(r.x, r.y))
        } else {
            self.case_non_exploree_la_plus_proche(r.x, r.y)
        }
    }).collect();

    for (robot, cible) in self.robots.iter_mut().zip(cibles) {
        let other_positions: Vec<(usize, usize)> = positions
            .iter()
            .filter(|&&(x, y)| x != robot.x || y != robot.y)
            .copied()
            .collect();

        robot.deplacer_vers(
            self.largeur,
            self.hauteur,
            |x, y| !matches!(self.grille[y][x], Cellule::Obstacle),
            &other_positions,
            cible,
        );

        let (x, y) = (robot.x, robot.y);

        // 🚧 Collecte
        if !robot.retour_base {
            let libre = robot.capacite
                - (robot.sac.energie + robot.sac.minerai + robot.sac.science);

            if libre > 0 {
                match self.grille[y][x] {
                    Cellule::Energie if robot.modules.contains(&Module::Foreuse) => {
                        robot.sac.energie += 1;
                        self.grille[y][x] = Cellule::Vide;
                    }
                    Cellule::Minerai if robot.modules.contains(&Module::Foreuse) => {
                        robot.sac.minerai += 1;
                        self.grille[y][x] = Cellule::Vide;
                    }
                    Cellule::Scientifique if robot.modules.contains(&Module::Analyseur) => {
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

        // 🏁 Dépôt à la base + mise à jour explorée
        if robot.retour_base && self.bases.contains(&(x, y)) {
            let vision_range = 1;
            for dy in -(vision_range as isize)..=(vision_range as isize) {
                for dx in -(vision_range as isize)..=(vision_range as isize) {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && ny >= 0
                        && nx < self.largeur as isize
                        && ny < self.hauteur as isize
                    {
                        self.explorée[ny as usize][nx as usize] = true;
                    }
                }
            }

            self.collecte.energie += robot.sac.energie;
            self.collecte.minerai += robot.sac.minerai;
            self.collecte.science += robot.sac.science;
            robot.sac = Ressources::default();
            robot.retour_base = false;
        }
    }
}


    pub fn case_non_exploree_la_plus_proche(&self, x: usize, y: usize) -> Option<(usize, usize)> {
        let mut file = vec![(x, y)];
        let mut visitée = vec![vec![false; self.largeur]; self.hauteur];
        visitée[y][x] = true;

        while !file.is_empty() {
        let (cx, cy) = file.remove(0);
            if !self.explorée[cy][cx] && !matches!(self.grille[cy][cx], Cellule::Obstacle) {
                return Some((cx, cy));
            }

            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;
                if nx >= 0 && ny >= 0
                    && (nx as usize) < self.largeur
                    && (ny as usize) < self.hauteur
                {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if !visitée[ny][nx] {
                        visitée[ny][nx] = true;
                        file.push((nx, ny));
                    }
                }
            }
        }

        None
    }
    pub fn base_la_plus_proche(&self, x: usize, y: usize) -> (usize, usize) {
        *self.bases
            .iter()
            .min_by_key(|(bx, by)| {
                (bx.abs_diff(x) + by.abs_diff(y))
            })
            .unwrap()
    }
}

