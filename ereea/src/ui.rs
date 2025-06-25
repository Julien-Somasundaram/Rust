use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdout, Result};

use crate::map::{Map, Cellule};
use crate::robot::Robot;

pub fn render_map(map: &Map) -> Result<()> {
    let mut stdout = stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Carte EREEA").borders(Borders::ALL);
        let chunks = Layout::default()
            .constraints([Constraint::Min(0)].as_ref())
            .split(size);

        let mut text = String::new();
        for y in 0..map.hauteur {
            for x in 0..map.largeur {
                if let Some(robot) = map.robots.iter().find(|r| r.x == x && r.y == y) {
                    text.push(robot.symbole());
                } else {
                    text.push(match map.grille[y][x] {
                        Cellule::Vide => ' ',
                        Cellule::Obstacle => '#',
                        Cellule::Energie => '+',
                        Cellule::Minerai => '*',
                        Cellule::Scientifique => '?',
                    });
                }
            }
            text.push('\n');
        }

        let para = Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(Color::White));
        f.render_widget(para, chunks[0]);
    })?;

    Ok(())
}
