use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdout, Result};
use crossterm::{execute, terminal::{Clear, ClearType}};

use crate::map::{Map, Cellule};
use crate::robot::{Robot, TypeRobot};

pub fn render_map(map: &Map, tick: usize) -> Result<()> {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All))?;
    let backend = CrosstermBackend::new(&mut out);
    let mut terminal = Terminal::new(backend)?;



    terminal.draw(|f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
            .split(size);

        // Génération de la carte en couleur
        let mut text_lines = vec![];
        for y in 0..map.hauteur {
            let mut line = vec![];
            for x in 0..map.largeur {
                if (x, y) == map.base {
                    line.push(Span::styled("B", Style::default().fg(Color::White).bg(Color::DarkGray)));
                } else if let Some(robot) = map.robots.iter().find(|r| r.x == x && r.y == y) {
                    let (ch, color) = match robot.kind {
                        TypeRobot::Explorateur => ('E', Color::Cyan),
                        TypeRobot::Recolteur => ('R', Color::Yellow),
                        TypeRobot::Scientifique => ('S', Color::Green),
                    };
                    line.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else {
                    let (ch, color) = match map.grille[y][x] {
                        Cellule::Vide => (' ', Color::Reset),
                        Cellule::Obstacle => ('#', Color::DarkGray),
                        Cellule::Energie => ('+', Color::Red),
                        Cellule::Minerai => ('*', Color::Magenta),
                        Cellule::Scientifique => ('?', Color::Blue),
                    };
                    line.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                }

            }
            text_lines.push(Line::from(line));
        }

        let map_widget = Paragraph::new(text_lines)
            .block(Block::default().title("Carte").borders(Borders::ALL));
        f.render_widget(map_widget, chunks[0]);

        // HUD avec infos de simulation
      let hud_text = vec![
        Line::from(vec![Span::raw(format!("Tick actuel : {}", tick))]),
        Line::from(vec![Span::raw(format!("Robots : {}", map.robots.len()))]),
        Line::from("Ressources collectées :"),
        Line::from(format!("⚡ Energie : {}", map.collecte.energie)),
        Line::from(format!("⛏ Minerai : {}", map.collecte.minerai)),
        Line::from(format!("🔬 Science : {}", map.collecte.science)),
    ];


        let hud = Paragraph::new(hud_text)
            .block(Block::default().title("Statut").borders(Borders::ALL));
        f.render_widget(hud, chunks[1]);
    })?;

    Ok(())
}
