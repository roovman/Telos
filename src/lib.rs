// src/lib.rs

pub mod map;
pub mod specials;
pub mod state;
pub mod tui;

// Експортуємо головну функцію запуску
pub use tui::engine::run;