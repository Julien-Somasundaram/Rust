use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum TypeRobot {
    Explorateur,
    Recolteur,
    Scientifique,
}

#[derive(Debug, Clone, Copy)]
pub struct Robot {
    pub id: u32,
    pub x: usize,
    pub y: usize,
    pub kind: TypeRobot,
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
}

