// src/specials/entity.rs

use crate::map::position::MapPosition; // Абстракція позиції


/// Базова структура, що представляє будь-яку рухому/інтерактивну сутність у світі.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    pub id: u32, 
    pub position: MapPosition, 
    pub health: u32,
    pub symbol: char,
}

impl Entity {
    pub fn new(id: u32, position: MapPosition, symbol: char) -> Self {
        // ...
        Entity { id, position, health: 100, symbol }
    }
}