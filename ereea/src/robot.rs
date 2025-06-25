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
}
