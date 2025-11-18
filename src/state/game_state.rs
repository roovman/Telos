// src/state/game_state.rs

use crate::map::{
    map::Map, 
    position::MapPosition,
};
use crate::specials::{
    entity::Entity,
    powerup::PowerupType,
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Move,
    Build,
}
use super::actions::Action;
pub struct GameState {
    // --- Основні Компоненти Світу ---
    
    /// Зберігає сітку клітинок та її розміри.
    pub map: Map, 

    /// Зберігає всі динамічні об'єкти (гравця, ворогів, тощо).
    pub entities: Vec<Entity>, 

    // --- Стан Гри та Керування ---
    pub current_mode: Mode,
    /// Унікальний ідентифікатор гравця (для швидкого пошуку у векторі entities).
    /// Ми використовуємо u32, якщо Entity::id теж u32.
    pub player_id: u32, 

    /// Чи повинен головний цикл продовжувати роботу.
    pub is_running: bool,
    
    // --- TUI / Debug ---

    /// Повідомлення для відображення у панелі статусу (Debug/Status panel).
    pub debug_message: String,
}

// src/state/game_state.rs (impl GameState)

impl GameState {
    /// Ініціалізує ігровий стан, створюючи карту та розміщуючи гравця.
    pub fn new() -> Self {
        // 1. Створення карти
        let map_width = 80;
        let map_height = 25;
        let mut map = Map::new(map_width, map_height);
        
        // 2. Ініціалізація гравця
        let player_id: u32 = 1;
        let player_start_pos = MapPosition::new(map_width / 2, map_height / 2); // Центр карти
        
        let player = Entity {
            id: player_id, 
            position: player_start_pos, 
            symbol: '@',
            health: 100,
        };
        
        // 3. Реєстрація гравця на клітинці
        // Ми використовуємо u8 для tile.entity_id, тому приводимо u32 ID
        if let Some(tile) = map.get_tile_mut(&player_start_pos) {
            tile.entity_id = Some(player_id as u32); 
        }

        // 4. Початкове розміщення предмета (для тестування take_powerup)
        let powerup_pos = player_start_pos.right(3).up(2); // Поруч із гравцем
        if let Some(tile) = map.get_tile_mut(&powerup_pos) {
             // Ми знаємо, що powerup: PowerupType, тому використовуємо його.
             tile.powerup = PowerupType::HealingPotion;
        }

        GameState {
            map,
            entities: vec![player],
            player_id,
            is_running: true,
            current_mode: Mode::Move,
            debug_message: "Ласкаво просимо до роґаліка на Rust!".to_string(),
        }
    }

    /// Повертає змінне посилання на гравця.
    fn get_player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.iter_mut().find(|e| e.id == self.player_id)
    }

    /// Повертає незмінне посилання на гравця.
    fn get_player(&self) -> Option<&Entity> {
        self.entities.iter().find(|e| e.id == self.player_id)
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.is_running = false,
            
            Action::Move { unit_id, target_pos } => {
                // Всі сутності можуть рухатися, але зараз обробляємо лише гравця
                if unit_id == self.player_id {
                    self.handle_player_move(target_pos);
                }
            }
            
            Action::BuildWall { target_pos } => {
                if self.map.build_wall(&target_pos) {
                    self.debug_message = format!("Стіна побудована на {:?}", target_pos);
                } else {
                    self.debug_message = format!("Неможливо побудувати стіну на {:?}", target_pos);
                }
            }
            Action::ToggleBuildMode => {
                self.current_mode = match self.current_mode {
                    Mode::Move => Mode::Build,
                    Mode::Build => Mode::Move,
                };
                self.debug_message = format!("Режим змінено на: {:?}", self.current_mode);
            }
            
            // ... інші дії
            Action::Attack { .. } => self.debug_message = "Атака ще не реалізована.".to_string(),
            Action::EndTurn => self.debug_message = "Кінець ходу.".to_string(),
            Action::Click { pos } => self.debug_message = format!("Клік на позиції {:?}", pos),
        }
    }
    
    /// Обробляє рух гравця: перевіряє валідність та оновлює старий/новий Tile.
    fn handle_player_move(&mut self, target_pos: MapPosition) {
        let current_pos;
        let player_id = self.player_id;
        let is_standable;
        let mut pickup_message: Option<String> = None;
        
        // 1. ФАЗА ПЕРЕВІРКИ (Без змінних запозичень)
        {
            let player = match self.get_player() { Some(p) => p, None => return, };
            current_pos = player.position;
            is_standable = self.map.is_standable(&target_pos);
        } 

        // 2. ФАЗА ОНОВЛЕННЯ (Map)
        if is_standable {
            
            if let Some(old_tile) = self.map.get_tile_mut(&current_pos) {
                old_tile.entity_id = None;
            }
            
            if let Some(new_tile) = self.map.get_tile_mut(&target_pos) {
                new_tile.entity_id = Some(player_id as u32); 
                
                if new_tile.has_powerup() {
                    let powerup = new_tile.take_powerup(); 
                    
                    match powerup {
                        PowerupType::HealingPotion => {
                            // Ми оновимо HP гравця пізніше
                            pickup_message = Some(format!("Підібрав Зілля Лікування!"));
                        }
                        _ => {
                            pickup_message = Some(format!("Підібрав Powerup: {:?}", powerup));
                        }
                    }
                }
            }

            if let Some(player) = self.get_player_mut() {
                player.position = target_pos; 

                if let Some(msg) = pickup_message {
                    if msg.contains("Зілля Лікування") {
                        player.health += 25; 
                        self.debug_message = format!("{} HP: {}", msg, player.health);
                    } else {
                        self.debug_message = msg;
                    }
                } else {
                    self.debug_message = format!("Рух до {:?}", target_pos);
                }
            }
        } else {
            self.debug_message = "Рух неможливий: клітинка зайнята або непрохідна.".to_string();
        }
    }
}