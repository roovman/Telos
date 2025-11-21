// src/state/modes/editor.rs

// --- IMPORTS ---
// Note the relative path changes: GameState and BuildTool are now accessible from super::super
use super::super::game_state::GameState; 
use super::super::actions::BuildTool;
use crate::map::position::MapPosition;

// === EDITOR MODE ===
pub struct EditorMode {
    pub game_state: GameState,
    pub current_tool: BuildTool,
    pub debug_message: String,
}

impl EditorMode {
    pub fn new() -> Self {
        EditorMode {
            game_state: GameState::new(),
            current_tool: BuildTool::Wall,
            debug_message: String::from("Editor: Tab->Tool, S->Save, Q->Menu"),
        }
    }

    pub fn save_map(&mut self) {
        if let Err(e) = self.game_state.save("map.json") {
            self.debug_message = format!("Save Failed: {}", e);
        } else {
            self.debug_message = String::from("Saved to 'map.json'!");
        }
    }

    pub fn cycle_tool(&mut self) {
        self.current_tool = match self.current_tool {
            BuildTool::Wall => BuildTool::Floor,
            BuildTool::Floor => BuildTool::Unit,
            BuildTool::Unit => BuildTool::Wall,
        };
        self.debug_message = format!("Tool: {:?}", self.current_tool);
    }

    pub fn handle_click(&mut self, pos: MapPosition) {
        match self.current_tool {
            BuildTool::Wall => {
                if self.game_state.build_wall(pos) {
                    self.debug_message = format!("Built Wall at {:?}", pos);
                }
            }
            BuildTool::Floor => {
                self.game_state.build_floor(pos);
                self.debug_message = format!("Cleared at {:?}", pos);
            }
            BuildTool::Unit => {
                // --- UPDATED SPAWN CALL ---
                if let Some(id) = self.game_state.spawn_entity(
                    pos, 
                    'E', // Editor defaults to 'E' for enemy/entity
                    String::from("Editor Unit"),
                    50, // Smaller health
                    3,  // Smaller energy
                    2,  // Team 2 (Enemy)
                    false // Is AI
                ) {
                    self.debug_message = format!("Spawned Unit #{} (Team 2, AI)", id);
                } else {
                    self.debug_message = String::from("Blocked!");
                }
                // --------------------------
            }
        }
    }
}