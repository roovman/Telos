// src/entities/mod.rs 

pub mod entity; // <--- Оголошує, що існує файл entity.rs
pub mod ai;
pub mod powerup;

pub use entity::Entity;
pub use powerup::PowerupType;
