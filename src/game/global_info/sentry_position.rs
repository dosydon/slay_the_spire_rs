use crate::game::global_info::SentryPosition;

pub fn create_sentry_positions() -> [SentryPosition; 3] {
    [
        SentryPosition::First,   // Left Sentry (Outer position)
        SentryPosition::Second,  // Middle Sentry
        SentryPosition::Third,   // Right Sentry (Outer position)
    ]
}

// Simple flag to track which Sentry gets the first move (left or right)
pub fn create_first_move_flag() -> bool {
    true  // Left Sentry gets first move by default
}

pub fn instantiate_sentry_encounter(global_info: &crate::game::global_info::GlobalInfo, rng: &mut impl rand::Rng, first_move_left: bool) -> crate::game::global_info::GlobalInfo {
    let positions = Self::create_sentry_positions(first_move_left);
    crate::game::global_info::GlobalInfo {
        ascention: global_info.ascention,
        current_floor: global_info.current_floor,
        sentry_positions: Some(positions),
    }
}