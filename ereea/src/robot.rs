use rand::Rng;
use crate::map::Ressources;

#[derive(Debug, Clone, Copy)]
pub enum TypeRobot {
    Explorateur,
    Recolteur,
    Scientifique,
}

#[derive(Debug, Clone)]
pub struct Robot {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    pub kind: TypeRobot,
    pub sac: Ressources,
    pub capacite: u32,
    pub retour_base: bool,
    pub modules: Vec<Module>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Capteur,
    Foreuse,
    Analyseur,
}
impl Robot {
    pub fn symbole(&self) -> char {
        match self.kind {
            TypeRobot::Explorateur => 'E',
            TypeRobot::Recolteur => 'R',
            TypeRobot::Scientifique => 'S',
        }
    }
    
    pub fn deplacer(&mut self, largeur: usize, hauteur: usize, is_cellule_valide: impl Fn(usize, usize) -> bool, positions_occupees: &[(usize, usize)]) {
        let directions = [
            (0isize, -1), // haut
            (0, 1),       // bas
            (-1, 0),      // gauche
            (1, 0),       // droite
        ];

        let mut rng = rand::thread_rng();
        let mut essais = 0;
        while essais < 10 {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let nx = self.x as isize + dx;
            let ny = self.y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let (nx, ny) = (nx as usize, ny as usize);
                if nx < largeur && ny < hauteur && is_cellule_valide(nx, ny) {
                    if !positions_occupees.contains(&(nx, ny)) {
                        self.x = nx;
                        self.y = ny;
                        break;
                    }
                }
            }
            essais += 1;
        }
    }
    pub fn deplacer_vers(
    &mut self,
    largeur: usize,
    hauteur: usize,
    is_cellule_valide: impl Fn(usize, usize) -> bool,
    positions_occupees: &[(usize, usize)],
    cible: Option<(usize, usize)>,
) {
    let directions = [
        (0isize, -1),
        (0, 1),
        (-1, 0),
        (1, 0),
    ];

    let mut rng = rand::thread_rng();
    let mut essais = 0;

    while essais < 10 {
        let (nx, ny) = if let Some((cx, cy)) = cible {
            let dx = (cx as isize - self.x as isize).signum();
            let dy = (cy as isize - self.y as isize).signum();
            (self.x as isize + dx, self.y as isize + dy)
        } else {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            (self.x as isize + dx, self.y as isize + dy)
        };

        if nx >= 0 && ny >= 0 {
            let (nx, ny) = (nx as usize, ny as usize);
            if nx < largeur && ny < hauteur && is_cellule_valide(nx, ny) {
                if !positions_occupees.contains(&(nx, ny)) {
                    self.x = nx;
                    self.y = ny;
                    break;
                }
            }
        }

        essais += 1;
    }
}

}

