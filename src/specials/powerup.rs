// src/specials/powerup.rs

// Це те, на що натякав наш PowerupTypeID! 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerupType {
    None,
    HealingPotion,
    SpeedBoost,
    WallBreaker,
}

impl PowerupType {
    /// Метод, який повертає ID для збереження в Tile (якщо Tile зберігає ID)
    pub fn to_id(&self) -> u8 {
        match self {
            PowerupType::None => 0,
            PowerupType::HealingPotion => 1,
            PowerupType::SpeedBoost => 2,
            PowerupType::WallBreaker => 3,
        }
    }
    
    pub fn is_some(&self) -> bool{
        match self {
            PowerupType::None => false,
            _ => true,
        }
    } 
    
    pub fn take(&mut self)-> PowerupType {
        use std::mem;
        mem::replace(self, PowerupType::None)
    }
     // Метод, який інкапсулює логіку дії
    // Пізніше: fn apply_effect(&self, target: &mut Entity)
}