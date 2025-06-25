use noise::{NoiseFn, Perlin};
use rand::Rng;

use super::tile::{MapTile, Resource, ResourceType, TileType};

pub const TERRAIN_SCALE: f64 = 6.0;
pub const RESOURCE_SCALE: f64 = 2.0;
const RESOURCE_PROBABILITY: f64 = 0.1;
const THRESHOLD: f64 = 0.3;

#[derive(Debug, Clone)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<MapTile>,
    pub seed: u32,
    pub base_position: (usize, usize),
}

impl Map {
    pub fn new(width: usize, height: usize, seed: u32) -> Self {
        let mut map = Map {
            width,
            height,
            grid: vec![MapTile::new(0, 0, TileType::Empty); width * height],
            seed,
            base_position: (0, 0),
        };

        map.generate_terrain();
        map.place_resources();
        map.place_science_base();
        map
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> MapTile {
        self.grid[self.get_index(x, y)]
    }

    pub fn set(&mut self, tile: MapTile) {
        let idx = self.get_index(tile.x, tile.y);
        self.grid[idx] = tile;
    }

    fn generate_terrain(&mut self) {
        let perlin: Perlin = Perlin::new(self.seed);

        for y in 0..self.height {
            for x in 0..self.width {
                let noise_value = perlin.get([x as f64 / TERRAIN_SCALE, y as f64 / TERRAIN_SCALE]);
                if noise_value > THRESHOLD {
                    self.set(MapTile::new(x, y, TileType::Terrain));
                }
            }
        }
    }

    fn place_resources(&mut self) {
        let perlin = Perlin::new(self.seed);
        let mut rng = rand::rng();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y).tile != TileType::Empty {
                    continue;
                }

                let noise_value =
                    perlin.get([x as f64 / RESOURCE_SCALE, y as f64 / RESOURCE_SCALE]);
                if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(MapTile::new(
                        x,
                        y,
                        TileType::Resource(Resource::new(10, ResourceType::Energy)),
                    ));
                } else if noise_value > THRESHOLD && rng.random_bool(RESOURCE_PROBABILITY) {
                    self.set(MapTile::new(
                        x,
                        y,
                        TileType::Resource(Resource::new(10, ResourceType::Mineral)),
                    ));
                }
            }
        }
    }

    fn place_science_base(&mut self) {
        let mut rng = rand::rng();

        loop {
            let x = rng.random_range(1..self.width - 1);
            let y = rng.random_range(1..self.height - 1);

            if self.get(x, y).tile == TileType::Empty
                && self.get(x - 1, y).tile == TileType::Empty
                && self.get(x + 1, y).tile == TileType::Empty
                && self.get(x, y - 1).tile == TileType::Empty
                && self.get(x, y + 1).tile == TileType::Empty
            {
                self.set(MapTile::new(x, y, TileType::Base));
                self.base_position = (x, y);
                break;
            }
        }
    }

    pub fn is_valid(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height && (self.get(x, y).tile == TileType::Empty)
    }
}
