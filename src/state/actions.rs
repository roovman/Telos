// src/lib/state/actions.rs (ОНОВЛЕНО)

use crate::map::position::MapPosition; // Використовуємо нашу чисту абстракцію для координат
// Використовуємо u32 для ID, щоб узгодити з Entity::id (як ми планували)
// Або використовуємо EntityID, якщо ви його створили у specials/ids.rs
pub type EntityID = u32; 

#[derive(Debug, Clone)]
pub enum Action {
    /// Рух сутності з певним ID до цільової позиції.
    Move { 
        unit_id: EntityID, 
        target_pos: MapPosition, 
    },
    
    /// Атака однієї сутності іншою.
    Attack { 
        attacker_id: EntityID, 
        target_id: EntityID, 
    },
    
    /// Будівництво стіни на цільовій позиції.
    BuildWall { 
        target_pos: MapPosition, 
    },

    /// Обробка кліка мишею на позиції екрану (Screen Coordinates).
    Click { 
        pos: MapPosition, 
    }, 
    ToggleBuildMode,
    
    /// Завершення ходу (для покрокової логіки).
    EndTurn,
    
    /// Вихід із гри.
    Quit,
}