// Public modules
pub mod battle_action;
pub mod character_battle_info;
pub mod target;
pub mod battle_events;
pub mod battle_result;
pub mod player;
pub mod deck_hand_pile;
pub mod enemy_in_battle;
pub mod battle_state;
pub mod listeners;
pub mod battle_error;
pub mod event_listener_enum;

// Private modules
mod battle;
mod turn_flow;
mod eval_action;
mod play_card;
mod eval_effect;
mod enemy_manager;
mod listener_manager;

// Re-export commonly used types for easier access
pub use target::Entity;
pub use battle_result::BattleResult;
pub use battle_error::BattleError;
pub use battle::Battle;
pub use event_listener_enum::EventListenerEnum;
