// src/tui/draw.rs

use ratatui::{
    Frame, 
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Style, Color, Modifier}, 
    layout::{Constraint, Layout, Direction, Rect},
    prelude::Alignment,
    text::{Span, Line}
};
use crate::state::application_state::{ApplicationState, AppState};
use crate::state::world_state::WorldState;
use crate::map::tile::TileType;
// Import structs from the refactored modes folder (Assuming you finalize this soon)
use crate::state::modes::{EditorMode, GameMode}; 

// --- HELPERS ---

fn draw_map_tiles(f: &mut Frame, world_state: &WorldState, inner_area: Rect) {
    for y in 0..world_state.map.height() {
        for x in 0..world_state.map.width() {
            let tile = world_state.map.get_tile_i32(x, y);
            if let Some(tile) = tile {
                
                let (mut symbol, mut style) = match tile.tile_type() { 
                    TileType::WalkableGeneric => (tile.symbol(), Style::default().fg(Color::DarkGray)),
                    TileType::Wall => (tile.symbol(), Style::default().fg(Color::Rgb(255, 165, 0))),
                };

                if let Some(id) = tile.entity_id() { 
                    if let Some(e) = world_state.get_entity(id) {
                        symbol = e.symbol(); // Use accessor
                        if e.is_selected() { // Use accessor
                            style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
                        } else {
                            style = Style::default().fg(Color::White);
                        }
                    }
                }

                if (x as u16) < inner_area.width && (y as u16) < inner_area.height {
                    f.buffer_mut()
                        .get_mut(inner_area.x + x as u16, inner_area.y + y as u16)
                        .set_symbol(&symbol.to_string())
                        .set_style(style);
                }
            }
        }
    }
}

// --- NEW MENU/SIDEBAR RENDERING FUNCTIONS ---


fn draw_editor_menu(f: &mut Frame, mode: &EditorMode, area: Rect) {
    
    let tool_info = format!("Current Tool: {:?}\n\n[W] Wall\n[F] Floor\n[U] Unit\n\n[S] Save Map\n[Q] Quit", mode.current_tool);
    
    // Add multiple blank lines to push the log to the bottom of the sidebar,
    // making the most use of the vertical space before the log starts.
    let vertical_padding = "\n\n\n\n\n\n"; 
    
    let log_header = format!("--- Logs ---\n");
    
    let full_content = format!("{}{}{}{}", tool_info, vertical_padding, log_header, mode.debug_message);

    // FIX: Add .wrap(Wrap { trim: false }) to enable word wrapping
    let p = Paragraph::new(full_content)
        .wrap(ratatui::widgets::Wrap { trim: false }) // <--- FIX HERE
        .block(Block::default().title("EDITOR MENU").borders(Borders::ALL).style(Style::default().fg(Color::White)));
    
    f.render_widget(p, area);
}

fn draw_game_menu(f: &mut Frame, mode: &GameMode, area: Rect) {
    // We can show selected unit stats and action buttons here
    let mut menu_content = String::from("--- ACTIONS ---\n");
    menu_content.push_str("[T] End Turn\n\n");
    
    if let Some(id) = mode.selected_entity_id {
        if let Some(e) = mode.world_state.get_entity(id) {
            menu_content.push_str(&format!("UNIT: {}\n", e.display_name()));
            menu_content.push_str(&format!("HP: {}/{}\n", e.health(), e.max_health()));
            menu_content.push_str(&format!("EN: {}/{}\n", e.energy(), e.max_energy()));
            menu_content.push_str("-------------------\n");
        }
    } else {
        menu_content.push_str("Click a unit to select.\n");
    }

    let log_info = format!("\n--- Logs ---\n{}", mode.debug_message);

    let p = Paragraph::new(menu_content + &log_info)
        .block(Block::default().title("GAME COMMANDS").borders(Borders::ALL).style(Style::default().fg(Color::White)));
    f.render_widget(p, area);
}

// --- NEW: Game Wrapper with Sidebar Structure ---
fn draw_game_wrapper(f: &mut Frame, world_state: &WorldState, mode_specific_drawer: impl FnOnce(&mut Frame, Rect)) {
    // Split the entire area vertically: Map (75%) | Menu/Logs (25%)
    let chunks = Layout::default().direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(75), Constraint::Percentage(25)]).split(f.area());
    
    let map_area = chunks[0];
    let menu_area = chunks[1];

    let map_block = Block::default().title(" Map ").borders(Borders::ALL);
    f.render_widget(map_block.clone(), map_area);
    
    // Draw the map into the inner area of the map block
    draw_map_tiles(f, world_state, map_block.inner(map_area));
    
    // Call the specific menu drawing function for the mode
    mode_specific_drawer(f, menu_area);
}

fn draw_greeting_menu(f: &mut Frame, ){
    let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),         
                    Constraint::Length(10),      
                    Constraint::Min(0),         
                ])
                .split(f.area());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(30), 
                    Constraint::Percentage(40), 
                    Constraint::Percentage(30), 
                ])
                .split(chunks[1]);

            let menu_area = horizontal_chunks[1];

            let menu_lines = vec![
                Line::from(Span::styled("--- RUST TUI CORE ---", Style::default().fg(Color::LightCyan).add_modifier(Modifier::BOLD))),
                Line::from(vec![Span::raw("")]),
                Line::from(Span::styled("1. START EDITOR (BUILD MODE)", Style::default().fg(Color::Green))),
                Line::from(Span::styled("2. START NEW GAME (PLAY MODE)", Style::default().fg(Color::Yellow))),
                Line::from(Line::from(Span::styled("3. LOAD LAST SAVED MAP ('map.json')", Style::default().fg(Color::Blue)))),
                Line::from(vec![Span::raw("")]),
                Line::from(Span::styled("Q. QUIT APPLICATION", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
            ];

            let p = Paragraph::new(menu_lines) // Use menu_lines (Vec<Line>)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: false })
                .block(Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))); // Simple style, no reversal, minimal overhead

            f.render_widget(p, menu_area); 
}
// --- MAIN DISPATCHER ---
pub fn app_ui(f: &mut Frame, app: &ApplicationState) {
    match &app.state {
        AppState::Menu => {
           draw_greeting_menu(f)
        },
        // --- UPDATED DISPATCH FOR SIDEBAR ---
        AppState::Editor(mode) => {
            draw_game_wrapper(f, &mode.world_state, |f, area| {
                draw_editor_menu(f, mode, area);
            });
        },
        AppState::Simulation(mode) => {
            draw_game_wrapper(f, &mode.world_state, |f, area| {
                draw_game_menu(f, mode, area);
            });
        },
        _ => {}
    }
}