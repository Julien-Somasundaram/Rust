use super::robot::{Robot, RobotState, RobotType};
use crate::maps::{
    map::Map,
    tile::{MapTile, Resource, TileType},
};

pub struct Harvester {
    id: usize,
    x: usize,
    y: usize,
    cargo_capacity: u32,
    state: RobotState,
    target_resource: Option<(usize, usize, Resource, bool)>,
}

impl Robot for Harvester {
    fn new(x: usize, y: usize, id: usize) -> Self {
        Harvester {
            id: id,
            x: x,
            y: y,
            cargo_capacity: 5,
            state: RobotState::Harvesting,
            target_resource: None,
        }
    }
    fn get_id(&self) -> usize {
        self.id
    }
    fn get_type(&self) -> RobotType {
        RobotType::Harvester
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

    fn get_current_resource(&self) -> Option<(usize, usize, Resource, bool)> {
        if let Some((x, y, resource, remind)) = self.target_resource {
            Some((x, y, resource, remind))
        } else {
            None
        }
    }

    fn set_target_resource(&mut self, target: Option<(usize, usize, Resource, bool)>) {
        self.target_resource = target;
    }

    fn update(&mut self, map: &mut Map) {
        match self.state {
            RobotState::Harvesting => {
                self.harvest(map);
            }
            RobotState::ReturningToBase => {
                self.return_to_base(map);
            }
            _ => {}
        }
    }
}

impl Harvester {
    pub fn harvest(&mut self, map: &mut Map) {
        if let Some((x, y, _, _)) = self.target_resource {
            let step = self.calculate_next_step(x, y, map);
            match step {
                Some((next_x, next_y)) => {
                    self.move_to(next_x, next_y, map);
                }
                None => {
                    let tile = map.get(x, y);
                    match tile.tile {
                        TileType::Resource(res) => {
                            if res.scale > self.cargo_capacity {
                                self.set_target_resource(Some((
                                    x,
                                    y,
                                    Resource::new(self.cargo_capacity, res.resource_type),
                                    true,
                                )));
                                map.set(MapTile::new(
                                    x,
                                    y,
                                    TileType::Resource(Resource::new(
                                        res.scale - self.cargo_capacity,
                                        res.resource_type,
                                    )),
                                ));
                            } else {
                                self.set_target_resource(Some((
                                    x,
                                    y,
                                    Resource::new(res.scale, res.resource_type),
                                    false,
                                )));
                                map.set(MapTile::new(x, y, TileType::Empty));
                            }
                        }
                        _ => {}
                    }
                    self.set_state(RobotState::ReturningToBase);
                }
            }
        }
    }
}
