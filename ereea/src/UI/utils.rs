use iced::{widget::{Button, Text}, Application, Length, Settings};
use crate::simulation::simulation::Simulation;
use super::graphic_ui::{MapWindow, Message};

pub fn create_button(label: &str, message: Message, enabled: bool ) -> Button<Message> {
    let mut button = Button::new(
        Text::new(label)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .width(Length::FillPortion(10)
        )
    )
    .style(if enabled {
        iced::theme::Button::Primary
    } else {
        iced::theme::Button::Secondary
    })
    .width(Length::Fill);

    if enabled {
        button = button.on_press(message);
    }

    button
}

pub fn open_window(simulation: &Simulation) -> iced::Result {
    let tile_size = 30;
    let control_width = 200;
    let padding = 10;

    let (window_width, window_height) = {
        let map_guard = simulation.map.read().unwrap();
        let width = (map_guard.width as u32 * tile_size) + control_width + padding;
        let height = map_guard.height as u32 * tile_size + padding;
        (width, height)
    };
    
    println!("Fenêtre ajustée : {}x{}", window_width, window_height);

    let settings = Settings {
        window: iced::window::Settings {
            size: (window_width, window_height),
            resizable: false,
            ..Default::default()
        },
        antialiasing: true,
        exit_on_close_request: true,
        ..Settings::with_flags(simulation.clone())
    };

    MapWindow::run(settings)?;
    Ok(())
}