// src/state/modes/mod.rs

pub mod editor; // Declares the editor.rs file
pub mod game;   // Declares the game.rs file

// Optional: Re-export the main structs for cleaner imports elsewhere
pub use editor::EditorMode;
pub use game::GameMode;