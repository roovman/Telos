// // src/specials/ai.rs
// use super::entity::Entity; // Сутність, якою керуємо
// use crate::state::{GameState, actions::Action};

// /// 💡 Абстракція: Трейт, який визначає, що має вміти штучний інтелект.
// /// Це дозволяє поліморфно викликати метод `decide_action` на різних типах AI.
// pub trait AI {
//     // Вхід: Поточний стан сутності, карта, GameState (залежності)
//     // Вихід: Дія, яку потрібно виконати (наприклад, Action::Move)
//     fn decide_action(&self, entity: &Entity, game_state: &GameState) -> Option<Action>;
// }

// /// Реалізація конкретного типу AI
// pub struct ChasingAI;

// impl AI for ChasingAI {
//     fn decide_action(&self, entity: &Entity, game_state: &GameState) -> Option<Action> {

//         None
//     }
// }