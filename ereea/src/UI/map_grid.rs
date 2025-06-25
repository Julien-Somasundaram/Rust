use crate::maps::map::Map;
use iced::widget::{Column, Container, Row, Text};
use iced::{Element, Font, Length};
use std::sync::{Arc, RwLock};

use super::graphic_ui::Message;

pub struct MapGrid {
    map: Arc<RwLock<Map>>,
    pub font: Font,
    map_content: String,
}

impl MapGrid {
    pub fn new(map: Arc<RwLock<Map>>, font: Font) -> Self {
        Self { map, font, map_content: String::new() }
    }

    pub fn update(&mut self, map: &mut Map) {
        let mut map_content = String::new();
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = map.get(x, y).tile;
                map_content.push(tile.char());
            }
            map_content.push('\n');
        }
        self.map_content = map_content;
    }

    pub fn view(&self) -> Element<Message> {
        if let Ok(map) = self.map.read() {
            let mut rows = Column::new().spacing(1);

            for y in 0..map.height {
                let mut row = Row::new().spacing(1);

                for x in 0..map.width {
                    let emoji = map.get(x, y).tile.char().to_string();
                    let cell = Container::new(Text::new(emoji).font(self.font))
                        .width(Length::Fixed(30.0))
                        .height(Length::Fixed(30.0))
                        .center_x()
                        .center_y();

                    row = row.push(cell);
                }

                rows = rows.push(row);
            }

            Container::new(rows)
                .width(Length::Fill) 
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            Container::new(Text::new("Failed to lock map").font(self.font))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        }
    }
}