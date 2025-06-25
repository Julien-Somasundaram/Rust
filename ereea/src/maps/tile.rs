use std::char;

use crate::robots::robot::RobotType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Terrain,
    Base,
    Resource(Resource),
    Robot(RobotType),
}

impl TileType {
    pub fn char(&self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Terrain => '⛰',
            TileType::Base => '🏠',
            TileType::Resource(resource) => match resource.resource_type {
                ResourceType::Energy => '⚡',
                ResourceType::Mineral => '💎',
            },
            TileType::Robot(robot) => match robot {
                RobotType::Explorer => '🚜',
                RobotType::Harvester => '🤖',
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MapTile {
    pub x: usize,
    pub y: usize,
    pub tile: TileType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceType {
    Energy,
    Mineral,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Resource {
    pub scale: u32,
    pub resource_type: ResourceType,
}

impl Resource {
    pub fn new(scale: u32, resource_type: ResourceType) -> Self {
        Resource { scale, resource_type }
    }
}

impl MapTile {
    pub fn new(x: usize, y: usize, tile: TileType) -> Self {
        MapTile { x, y, tile }
    }
}