#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Obstacle,
    Energy,
    Mineral,
    Science,
}

impl Tile {
    pub fn symbol(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Obstacle => '#',
            Tile::Energy => '+',
            Tile::Mineral => '*',
            Tile::Science => '?',
        }
    }
}
