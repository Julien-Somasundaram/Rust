use std::{
    any::Any,
    collections::{HashMap, VecDeque},
};

use crate::maps::{
    map::Map,
    tile::{MapTile, Resource, TileType},
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RobotType {
    Explorer,
    Harvester,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum RobotState {
    Exploring,
    Harvesting,
    ReturningToBase,
    Reporting,
    Idle,
}

pub trait Robot: Any {
    fn new(x: usize, y: usize, id: usize) -> Self
    where
        Self: Sized;
    fn get_id(&self) -> usize;
    fn get_position(&self) -> (usize, usize);
    fn get_state(&self) -> RobotState;
    fn set_state(&mut self, state: RobotState);
    fn get_type(&self) -> RobotType;
    fn update(&mut self, map: &mut Map);
    fn get_current_resource(&self) -> Option<(usize, usize, Resource, bool)>;
    fn set_target_resource(&mut self, target_resource: Option<(usize, usize, Resource, bool)>);

    fn move_to(&mut self, x: usize, y: usize, map: &mut Map) {
        if map.is_valid(x, y) {
            let (old_x, old_y) = self.get_position();
            let (base_x, base_y) = map.base_position;

            if old_x == base_x && old_y == base_y {
                map.set(MapTile::new(old_x, old_y, TileType::Base));
            } else {
                map.set(MapTile::new(old_x, old_y, TileType::Empty));
            }

            map.set(MapTile::new(x, y, TileType::Robot(self.get_type())));

            self.set_position(x, y);
        } else {
            eprintln!("Invalid move to position ({}, {})", x, y);
        }
    }

    fn calculate_next_step(
        &self,
        target_x: usize,
        target_y: usize,
        map: &Map,
    ) -> Option<(usize, usize)> {
        let (start_x, start_y) = self.get_position();
        if start_x == target_x && start_y == target_y {
            return None;
        }
    
        let mut queue = VecDeque::new();
        let mut came_from = HashMap::new();
    
        queue.push_back((start_x, start_y));
        came_from.insert((start_x, start_y), None);
    
        while let Some((x, y)) = queue.pop_front() {
            if x == target_x && y == target_y {
                break;
            }
    
            // Se déplacer uniquement en ligne droite (haut, bas, gauche, droite)
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x as isize + dx;
                let new_y = y as isize + dy;
    
                if new_x < 0 || new_y < 0 {
                    continue;
                }
    
                let new_x = new_x as usize;
                let new_y = new_y as usize;
    
                if map.is_valid(new_x, new_y) && !came_from.contains_key(&(new_x, new_y)) {
                    queue.push_back((new_x, new_y));
                    came_from.insert((new_x, new_y), Some((x, y)));
                }
            }
        }
    
        if !came_from.contains_key(&(target_x, target_y)) {
            // Essayer toutes les directions possibles jusqu'à en trouver une valide
            let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
            for &(dx, dy) in &directions {
                let new_x = start_x as isize + dx;
                let new_y = start_y as isize + dy;
    
                if new_x >= 0 && new_y >= 0 {
                    let new_x = new_x as usize;
                    let new_y = new_y as usize;
                    if map.is_valid(new_x, new_y) {
                        return Some((new_x, new_y));
                    }
                }
            }
            return None;
        }
    
        let mut path = Vec::new();
        let mut current = Some((target_x, target_y));
    
        while let Some(pos) = current {
            path.push(pos);
            current = came_from.get(&pos).cloned().flatten();
        }
    
        path.reverse();
        if path.len() > 2 {
            Some(path[1])
        } else {
            None
        }
    }
    
    
    

    fn set_position(&mut self, x: usize, y: usize);

    fn return_to_base(&mut self, map: &mut Map) {
        let (base_x, base_y) = map.base_position;

        match self.calculate_next_step(base_x, base_y, map) {
            Some((x, y)) => {
                self.move_to(x, y, map);
            }
            None => {
                self.set_state(RobotState::Reporting);
            }
        }
    }
}
