use crate::maps::map::Map;
use crate::maps::tile::{MapTile, Resource, TileType};
use crate::robots::robot::{RobotState, RobotType};
use crate::robots::{explorer::Explorer, harvester::Harvester, robot::Robot};
use crate::UI::utils::open_window;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
pub struct Simulation {
    pub map: Arc<RwLock<Map>>,
    pub energy_count: Arc<Mutex<u32>>,
    pub running: Arc<AtomicBool>,
    pub speed: Arc<Mutex<u64>>,
    frame_count: u64,
    pub fps: f32,
    last_frame_time: std::time::Instant,
    explorer_threads: Arc<Mutex<HashMap<usize, thread::JoinHandle<()>>>>,
    harvester_threads: Arc<Mutex<HashMap<usize, thread::JoinHandle<()>>>>,
    pub located_resources: Arc<Mutex<VecDeque<Vec<(usize, usize, Resource)>>>>,
}

impl Simulation {
    pub fn new(map_seed: u32) -> Self {
        let map = Arc::new(RwLock::new(Map::new(25, 25, map_seed)));

        Simulation {
            map,
            energy_count: Arc::new(Mutex::new(0)),
            speed: Arc::new(Mutex::new(500)),
            running: Arc::new(AtomicBool::new(false)),
            frame_count: 0,
            fps: 0.0,
            last_frame_time: std::time::Instant::now(),
            explorer_threads: Arc::new(Mutex::new(HashMap::new())),
            harvester_threads: Arc::new(Mutex::new(HashMap::new())),
            located_resources: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn play(&self) {
        self.running.store(true, Ordering::SeqCst);
    }

    pub fn pause(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    pub fn run(&mut self) {
        let _ = open_window(self);
    }

    pub fn compute_fps(&mut self) {
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_frame_time = now;
        }
    }

    pub fn increase_speed(&mut self) {
        let mut speed = self.speed.lock().unwrap();
        if *speed > 100 {
            *speed -= 100;
        }
    }

    pub fn decrease_speed(&mut self) {
        let mut speed = self.speed.lock().unwrap();
        if *speed < 500 {
            *speed += 100;
        }
    }

    pub fn send_robot<F>(&mut self, robot_type: RobotType, call_bak: F)
    where
        F: FnOnce(&mut Box<dyn Robot + Send>) + Send + 'static,
    {
        let map_guard = self.map.read().unwrap();
        let base_pos = map_guard.base_position;
        drop(map_guard);

        let mut robot: Box<dyn Robot + Send> = match robot_type {
            RobotType::Explorer => Box::new(Explorer::new(
                base_pos.0,
                base_pos.1,
                self.explorer_threads.lock().unwrap().len(),
            )),
            RobotType::Harvester => Box::new(Harvester::new(
                base_pos.0,
                base_pos.1,
                self.harvester_threads.lock().unwrap().len(),
            )),
        };

        if robot_type == RobotType::Harvester {
            call_bak(&mut robot);
        }

        let map = Arc::clone(&self.map);
        let running = Arc::clone(&self.running);
        let speed = Arc::clone(&self.speed);
        let mut self_clone = self.clone();
        let robot_id = robot.get_id();
        let thread_handle = thread::spawn(move || loop {
            if robot.get_state() == RobotState::Reporting {
                self_clone.robot_came_back(&mut robot);
            }
            let sleep_time = {
                let speed = speed.lock().unwrap();
                *speed
            };
            if !running.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(sleep_time));
                continue;
            }

            let mut map_guard = map.write().unwrap();
            robot.update(&mut map_guard);
            drop(map_guard);

            thread::sleep(Duration::from_millis(sleep_time));
            if robot.get_state() == RobotState::Idle {
                break;
            }
        });

        match robot_type {
            RobotType::Explorer => {
                self.frame_count += 1;
                let mut explorer_threads = self.explorer_threads.lock().unwrap();
                explorer_threads.insert(robot_id, thread_handle);
            }
            RobotType::Harvester => {
                let mut harvester_threads = self.harvester_threads.lock().unwrap();
                harvester_threads.insert(robot_id, thread_handle);
            }
        }
    }

    fn robot_came_back(&mut self, robot: &mut Box<dyn Robot + Send>) {
        let mut self_clone = self.clone();
        match robot.get_type() {
            RobotType::Explorer => {
                let found_resource = robot.get_current_resource();
                if let Some((res_x, res_y, resource, _)) = found_resource {
                    let mut located_resources = self.located_resources.lock().unwrap();
                    let resource_exists = located_resources.iter().any(|resources| {
                        resources.iter().any(|(x, y, _)| *x == res_x && *y == res_y)
                    });
                    if !resource_exists {
                        located_resources.push_back(vec![(res_x, res_y, resource.clone())]);
                        self_clone.send_robot(RobotType::Harvester, move |harvester| {
                            harvester.set_target_resource(Some((
                                res_x,
                                res_y,
                                resource,
                                true,
                            )));
                        });
                    }
                }
                self.join_thread(robot);
            }
            RobotType::Harvester => {
                let target = robot.get_current_resource();
                if let Some((_, _, resource, remind)) = target {
                    match resource.resource_type {
                        crate::maps::tile::ResourceType::Energy => {
                            let mut energy_count = self_clone.energy_count.lock().unwrap();
                            *energy_count += resource.scale;
                        }
                        _ => {}
                    }

                    match remind {
                        true => robot.set_state(RobotState::Harvesting),
                        _ => self.join_thread(robot),
                    }
                }
            }
        }
    }

    fn join_thread(&mut self, robot: &mut Box<dyn Robot + Send>) {
        self.map.write().unwrap().set(MapTile::new(
            robot.get_position().0,
            robot.get_position().1,
            TileType::Empty,
        ));
        robot.set_state(RobotState::Idle);
    }
}
