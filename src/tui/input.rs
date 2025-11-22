// src/tui/input.rs

use crossterm::event::{self, Event, KeyCode, MouseEventKind, MouseButton};
use crossterm::terminal; // NEW IMPORT: Required to get terminal size
use std::time::Duration;
use crate::state::actions::{Action, MenuSelection};
use crate::state::application_state::{ApplicationState, AppState};
use crate::map::position::MapPosition;
use color_eyre::Result;

pub fn handle_input(app: &ApplicationState) -> Result<Option<Action>> {
    if event::poll(Duration::from_millis(16))? {
        let event = event::read()?;
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Char('q') | KeyCode::Char('Q') => {
                    match app.state {
                        AppState::Menu => return Ok(Some(Action::QuitApp)),
                        _ => return Ok(Some(Action::BackToMenu)),
                    }
                },
                // Menu
                KeyCode::Char('1') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode))),
                KeyCode::Char('2') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode))),
                KeyCode::Char('3') if matches!(app.state, AppState::Menu) => 
                    return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest))),

                // Editor
                KeyCode::Tab if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::CycleBuildTool)),
                KeyCode::Char('s') if matches!(app.state, AppState::Editor(_)) => 
                    return Ok(Some(Action::SaveMap)), 

                _ => {}
            },
            
            Event::Mouse(mouse) => {
                let x = mouse.column as i32;
                let y = mouse.row as i32;

                // Handle Menu clicks first, as they are full-width
                if matches!(app.state, AppState::Menu) {
                    if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                        // Calculate approximate Y-coordinate ranges for menu buttons
                        let (_, total_height) = terminal::size()?;
                        let start_y = (total_height as f32 * 0.30).round() as i32 + 1; // Start Y of the first button block
                        
                        match y {
                            _ if y >= start_y && y < start_y + 3 => {
                                // Y range for "1. BUILD MODE"
                                return Ok(Some(Action::MenuSelect(MenuSelection::EnterBuildMode)));
                            },
                            _ if y >= start_y + 3 && y < start_y + 6 => {
                                // Y range for "2. PLAY NEW GAME"
                                return Ok(Some(Action::MenuSelect(MenuSelection::EnterPlayMode)));
                            },
                            _ if y >= start_y + 6 && y < start_y + 9 => {
                                // Y range for "3. LOAD LAST MAP"
                                return Ok(Some(Action::MenuSelect(MenuSelection::LoadLatest)));
                            },
                            _ if y >= start_y + 9 && y < start_y + 12 => {
                                // Y range for "Q. QUIT APPLICATION"
                                return Ok(Some(Action::QuitApp));
                            },
                            _ => {}
                        }
                    }
                    return Ok(None); // Stop processing mouse events if we are in the menu
                }

                // --- 1. Determine the Sidebar Boundary (Logic for Editor/Simulation) ---
                // ... (rest of the Editor/Simulation mouse routing logic remains here)
                let sidebar_x_start = (terminal::size()?.0 as f32 * 0.75).round() as i32; 
                let is_in_sidebar = x >= sidebar_x_start; 
                let map_pos = MapPosition::new(x - 1, y - 1); 

                // --- 2. Route Click Event (Left Button Down) for Editor/Simulation ---
                if let MouseEventKind::Down(MouseButton::Left) = mouse.kind {
                    match app.state {
                        AppState::Editor(_) => {
                            if is_in_sidebar {
                                return Ok(Some(Action::EditorMenuClick { screen_x: x, screen_y: y })); 
                            } else if x > 0 && y > 0 && x < sidebar_x_start { 
                                return Ok(Some(Action::EditorClick { pos: map_pos }));
                            }
                        }
                        AppState::Simulation(_) => {
                            if is_in_sidebar {
                                return Ok(Some(Action::GameMenuClick { screen_x: x, screen_y: y })); 
                            } else if x > 0 && y > 0 && x < sidebar_x_start { 
                                return Ok(Some(Action::GameClick { pos: map_pos }));
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}