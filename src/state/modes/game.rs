// src/state/modes/game.rs

// --- IMPORTS ---
use super::super::world_state::WorldState; 
use crate::specials::entity::EntityID;
use crate::map::position::MapPosition;

// === GAME MODE ===
pub struct GameMode {
    pub world_state: WorldState,
    pub selected_entity_id: Option<EntityID>,
    pub debug_message: String,
    // Note: You would re-add pub reachable_tiles here when you restore BFS/reachability
}

impl GameMode {
    pub fn new(world_state: WorldState) -> Self {
        GameMode {
            world_state,
            selected_entity_id: None,
            debug_message: String::from("Game Mode. Click to Select/Move."),
        }
    }

    pub fn handle_click(&mut self, pos: MapPosition) {
        if let Some(selected_id) = self.selected_entity_id {
            if let Some(clicked_id) = self.world_state.get_entity_id_at(pos) {
                if clicked_id != selected_id {
                    self.select_entity(Some(clicked_id));
                    return;
                }
            }
            self.try_move_entity(selected_id, pos);
        } else {
            let clicked_id = self.world_state.get_entity_id_at(pos);
            self.select_entity(clicked_id);
        }
    }

    pub fn handle_menu_click(&mut self, screen_x: i32, screen_y: i32) {
        // Assume Y=3 is the [T] End Turn button region
        match screen_y {
            3 => {
                self.end_turn(); // Call the turn logic
            },
            // Assuming Y=4 could be a Deselect/Cancel button
            4 => {
                self.select_entity(None); // Deselects the current unit
            }
            _ => {
                self.debug_message = format!("Game Menu Clicked at ({}, {}).", screen_x, screen_y);
            }
        }
    }

    fn select_entity(&mut self, id: Option<EntityID>) {
        if let Some(prev) = self.selected_entity_id {
            if let Some(e) = self.world_state.get_entity_mut(prev) { 
                e.set_selected(false); 
            }
        }
        self.selected_entity_id = id;
        if let Some(new) = id {
            if let Some(e) = self.world_state.get_entity_mut(new) { 
                e.set_selected(true); 
            }
            self.debug_message = format!("Selected Unit #{}", new);
        } else {
            self.debug_message = String::from("Selection cleared.");
        }
    }

    fn try_move_entity(&mut self, id: EntityID, target_pos: MapPosition) {
        // NOTE: Energy check and pathfinding/distance check will go here when restored!

        if !self.world_state.map.is_standable(&target_pos) {
            self.debug_message = String::from("Blocked path!");
            return;
        }

        let old_pos;
        {
            let e = self.world_state.get_entity(id).unwrap();
            old_pos = e.position(); 
        }

        // --- FIX 1: Use set_entity_id mutator to clear the old tile ---
        self.world_state.map.get_tile_mut(&old_pos).unwrap().set_entity_id(None);
        
        let mut msg = format!("Unit moved.");
        if let Some(tile) = self.world_state.map.get_tile_mut(&target_pos) {
            // --- FIX 2: Use set_entity_id mutator to set the new tile ---
            tile.set_entity_id(Some(id));
            
            if tile.has_powerup() {
                 let p = tile.take_powerup();
                 msg = format!("Moved & Got {:?}", p);
            }
        }

        if let Some(e) = self.world_state.get_entity_mut(id) {
            e.set_position(target_pos);
        }
        self.debug_message = msg;
    }
    
    pub fn end_turn(&mut self) {
        for entity in self.world_state.entities.iter_mut() {
            entity.reduce_stun();
            entity.refill_energy();
        }
        
        self.debug_message = String::from("Turn ended. All units maintained state.");
        
        // if let Some(id) = self.selected_entity_id {
        //     // self.recalculate_reachability(id); // Keep this commented
        // }
    }
}