mod director;
mod move_to;
mod pathfinding;

use pathfinding::*;
use rogue_lib::prelude::*;

use crate::move_to::MoveTo as MT;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode, size,
    },
};
use rand::prelude::*;
use std::error::Error;
use std::io::{Write, stdout};

const MAX_ENEMIES: u16 = 5;

/// Director handles the spawning of enemies.
struct Director {
    budget: u16,
    current: u16,
    max_enemies: u16,
}

pub struct Player {
    pub position: Vector2<u16>,
    icon: char,
}

impl Player {
    fn new() -> Self {
        Player {
            position: Vector2::new(10, 5),
            icon: '@',
        }
    }

    fn move_player(&mut self, direction: char, terminal_width: u16, terminal_height: u16) {
        match direction {
            'w' if self.position.x > 0 => self.position.y -= 1,
            's' if self.position.y < terminal_height - 1 => self.position.y += 1,
            'a' if self.position.x > 0 => self.position.x -= 1,
            'd' if self.position.y < terminal_width - 1 => self.position.x += 1,
            _ => {} // Invalid move or boundary hit
        }
    }
}

fn draw_game(
    stdout: &mut std::io::Stdout,
    player: &Player,
    enemies: &[Enemy],
) -> Result<(), Box<dyn Error>> {
    // Clear the screen
    queue!(stdout, Clear(ClearType::All))?;

    // Move cursor to player position and draw player
    queue!(stdout, MoveTo(player.position.x, player.position.y))?;
    queue!(stdout, Print(player.icon))?;
    for enemy in enemies {
        queue!(
            stdout,
            MoveTo(enemy.position.x, enemy.position.y),
            Print(enemy.icon())
        )?;
    }

    // Move cursor to bottom and show instructions
    let (_width, height) = size()?;
    queue!(stdout, MoveTo(0, height - 1))?;
    queue!(stdout, Print("Use WASD to move, Q to quit"))?;

    // Flush all queued commands
    stdout.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Enable raw mode and enter alternate screen
    let mut enemies = Vec::new();
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?; // Hide cursor for cleaner look

    let mut player = Player::new();

    // Initial draw
    draw_game(&mut stdout, &player, &enemies)?;
    let (width, height) = size()?;
    let map = Map::new(width, height);
    let graph = map.into_graph();

    // 2. Main application loop
    loop {
        // Poll for events with a timeout to prevent blocking indefinitely
        if event::poll(std::time::Duration::from_millis(50))? {
            if enemies.len() < 5 {
                let mut rng = rand::rng();
                let size = size()?;
                let random_x = rng.random_range(0..=size.0);
                let random_y = rng.random_range(0..=size.1);
                let enemy = Enemy::builder()
                    .position(Vector2::new(random_x, random_y))
                    .icon('E')
                    .name(String::from("Skeleton"))
                    .build();

                enemies.push(enemy);
            }

            for mut enemy in &mut enemies {
                let start_node = graph.get_node(enemy.position).unwrap();
                let goal_node = graph.get_node(player.position).unwrap();
                let path = pathfind(start_node, goal_node, &graph);
                if let Some(path) = path {
                    enemy.move_to(path[1].position.x, path[1].position.y);
                }
            }
            draw_game(&mut stdout, &player, &enemies)?;

            // Read the event
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => break, // Exit
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        let (width, height) = size()?;
                        player.move_player('w', width, height);
                        draw_game(&mut stdout, &player, &enemies)?;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        let (width, height) = size()?;
                        player.move_player('s', width, height);
                        draw_game(&mut stdout, &player, &enemies)?;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        let (width, height) = size()?;
                        player.move_player('a', width, height);
                        draw_game(&mut stdout, &player, &enemies)?;
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        let (width, height) = size()?;
                        player.move_player('d', width, height);
                        draw_game(&mut stdout, &player, &enemies)?;
                    }
                    _ => {} // Ignore other keys
                }
            }
        }
    }

    // 3. Restore the terminal state (crucial for a clean exit)
    execute!(stdout, Show, LeaveAlternateScreen)?; // Show cursor again
    disable_raw_mode()?;

    Ok(())
}
