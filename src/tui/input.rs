// src/lib/tui/input.rs (ОНОВЛЕНА ВЕРСІЯ)
use crate::state::game_state::Mode;
use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use std::time::Duration;

use crate::state::actions::Action;
use crate::state::GameState;
use crate::map::position::MapPosition; // ⭐️ Використовуємо нашу абстракцію
use color_eyre::Result;

pub fn handle_events(game_state: &GameState) -> Result<Option<Action>> {
    if event::poll(Duration::from_millis(16))? {
        let event = event::read()?;
        
        match event {
            // Клавіатура
            Event::Key(key) => {
                match key.code {
                    KeyCode::Char('q') => return Ok(Some(Action::Quit)),
                    
                    // ⭐️ NEW: Клавіша 'b' для перемикання режиму
                    KeyCode::Char('b') => return Ok(Some(Action::ToggleBuildMode)),
                    
                    _ => {}
                }
            }
            // Миша
            Event::Mouse(mouse) => {
                let x = mouse.column as i32;
                let y = mouse.row as i32;
                
                // Зміщення на -1, щоб отримати координати на карті всередині рамки
                let map_pos = MapPosition::new(x - 1, y - 1); 

                // Перевірка валідності позиції на карті
                if map_pos.x() < 0 || map_pos.y() < 0 
                    || map_pos.x() >= game_state.map.width 
                    || map_pos.y() >= game_state.map.height {
                    // Клік був поза межами карти
                    return Ok(None);
                }

                match mouse.kind {
                    // 1. ЛІВИЙ КЛІК: Рух гравця
                    MouseEventKind::Down(MouseButton::Left) => {
                        match game_state.current_mode{
                            Mode::Move => {
                            let player_id = game_state.player_id;

                            return Ok(Some(Action::Move {
                                unit_id: player_id,
                                target_pos: map_pos,
                            }));
                            }
                            Mode::Build => {
                                return Ok(Some(Action::BuildWall {
                                    target_pos: map_pos,
                                }));
                            }
                            _ =>{}
                        }
                    }
    
                    _ => {}
                }
            }
            _ => {}
        }
    }
    // Подій не було
    Ok(None)
}