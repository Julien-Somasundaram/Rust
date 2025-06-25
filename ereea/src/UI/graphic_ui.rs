use iced::widget::{Column, Container, Row, Space, Text, Toggler};
use iced::{executor, time, Application, Command, Element, Font, Length, Subscription, Theme};

use crate::robots::robot::RobotType;
use crate::simulation::simulation::Simulation;

use super::map_grid::MapGrid;
use super::utils::create_button;

pub struct MapWindow {
    simulation: Simulation,
    map_grid: MapGrid,
    auto_explore: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    SendExplorer,
    Pause,
    Play,
    UpSpeed,
    DownSpeed,
    ToggleAutoExplore(bool),
}

impl Application for MapWindow {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = Simulation;

    fn new(simulation: Simulation) -> (Self, Command<Message>) {
        let font = if cfg!(target_os = "windows") {
            Font::with_name("Segoe UI Emoji")
        } else if cfg!(target_os = "macos") {
            Font::with_name("Apple Color Emoji")
        } else {
            Font::with_name("Noto Color Emoji")
        };

        let map_grid = MapGrid::new(simulation.map.clone(), font);

        (
            MapWindow {
                simulation,
                map_grid,
                auto_explore: false,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("EREEA - Map View")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick => {
                self.simulation.compute_fps();
                if let Ok(mut map) = self.simulation.map.write() {
                    self.map_grid.update(&mut map);
                }
                if self.auto_explore {
                    self.simulation.send_robot(RobotType::Explorer, |_| {});
                }
            }
            Message::SendExplorer => self.simulation.send_robot(RobotType::Explorer, |_| {}),
            Message::Pause => self.simulation.pause(),
            Message::Play => self.simulation.play(),
            Message::UpSpeed => self.simulation.increase_speed(),
            Message::DownSpeed => self.simulation.decrease_speed(),
            Message::ToggleAutoExplore(val) => self.auto_explore = val,
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(33)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<Message> {
        let is_running = self
            .simulation
            .running
            .load(std::sync::atomic::Ordering::SeqCst);

        let located_resources_count = {
            let located_resources = self.simulation.located_resources.lock().unwrap();
            located_resources.len()
        };

        let energy_count = self.simulation.energy_count.lock().unwrap();

        let stats = format!(
            "FPS: {}\nResources found: {}\nEnergy: {}",
            self.simulation.fps, located_resources_count, energy_count
        );

        let status_icon = if is_running { "Running" } else { "Paused" };

        let controls = Column::new()
            .spacing(15)
            .padding(15)
            .push(Text::new("Controls").size(20))
            .push(Text::new(status_icon).size(16))
            .push(Space::with_height(10))
            .push(Text::new(stats))
            .push(Space::with_height(20))
            .push(create_button("Play", Message::Play, !is_running))
            .push(create_button("Pause", Message::Pause, is_running))
            .push(Space::with_height(10))
            .push(create_button("Send Explorer", Message::SendExplorer, is_running))
            .push(
                Row::new()
                    .spacing(10)
                    .push(create_button("+ Speed", Message::UpSpeed, true))
                    .push(create_button("- Speed", Message::DownSpeed, true)),
            )
            .push(
                    Toggler::new(Some("Auto-Explore".into()), self.auto_explore, Message::ToggleAutoExplore)
                    .spacing(10),
            );

        let map = self.map_grid.view().map(|_| Message::Tick);

        Row::new()
            .push(Container::new(controls).width(Length::FillPortion(3)))
            .push(Container::new(map).width(Length::FillPortion(9)))
            .into()
    }
}
