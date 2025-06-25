mod map;
mod robot;
mod ui;

use std::{thread, time::Duration};
use map::Map;
use crossterm::{execute, terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}};
use std::io::stdout;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map = Map::new(40, 20, 42);

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    for tick in 0..100 {
        ui::render_map(&map, tick)?;
        map.tick();
        thread::sleep(Duration::from_millis(200));
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
