use super::robot::{Robot, RobotState, RobotType};
use crate::maps::{
    map::Map,
    tile::{Resource, TileType},
};
use rand::Rng;

pub struct Explorer {
    id: usize,
    x: usize,
    y: usize,
    resource: Option<(usize, usize, Resource, bool)>,
    state: RobotState,
}

impl Robot for Explorer {
    fn new(x: usize, y: usize, id: usize) -> Self {
        Explorer {
            id: id,
            x: x,
            y: y,
            resource: None,
            state: RobotState::Exploring,
        }
    }

    fn get_type(&self) -> RobotType {
        RobotType::Explorer
    }

    fn get_id(&self) -> usize {
        self.id
    }

    fn get_position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn get_state(&self) -> RobotState {
        self.state.clone()
    }

    fn set_state(&mut self, state: RobotState) {
        self.state = state;
    }

    fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Exploring => {
                self.explore(map);
            }
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            }
            _ => {}
        }
    }

    fn get_current_resource(&self) -> Option<(usize, usize, Resource, bool)> {
        self.resource
    }

    fn set_target_resource(&mut self, _: Option<(usize, usize, Resource, bool)>) {}
}

impl Explorer {
    pub fn explore(&mut self, map: &mut Map) {
        let mut rng = rand::rng();

        let move_horizontal = rng.random_bool(0.5);

        let (direction_x, direction_y) = if move_horizontal {
            (if rng.random_bool(0.5) { 1isize } else { -1isize }, 0isize)
        } else {
            (0isize, if rng.random_bool(0.5) { 1isize } else { -1isize })
        };

        let new_x = (self.x as isize + direction_x).max(0) as usize;
        let new_y = (self.y as isize + direction_y).max(0) as usize;

        if new_x <= map.width && new_y <= map.height {
            match map.get(new_x, new_y).tile {
                TileType::Resource(resource) => {
                    self.resource = Some((new_x, new_y, resource, true));
                    self.set_state(RobotState::ReturningToBase);
                }
                _ => {
                    self.move_to(new_x, new_y, map);
                }
            }
        }
    }
}
