pub mod burning_blood;
pub mod anchor;
pub mod blood_vial;

pub use burning_blood::BurningBloodRelic;
pub use anchor::AnchorRelic;
pub use blood_vial::BloodVialRelic;

#[derive(Debug, Clone, PartialEq)]
pub enum Relic {
    BurningBlood,
    Anchor,
    BloodVial,
}

impl Relic {
    /// Convert this relic to a game event listener
    pub fn to_game_event_listener(self) -> Option<Box<dyn crate::game::game_event::GameEventListener>> {
        match self {
            Relic::BurningBlood => Some(Box::new(BurningBloodRelic::new())),
            _ => None,
        }
    }

    /// Convert this relic to a battle event listener
    pub fn to_battle_event_listener(self) -> Option<Box<dyn crate::battle::events::EventListener>> {
        match self {
            Relic::Anchor => Some(Box::new(AnchorRelic::new(crate::battle::target::Entity::Player))),
            Relic::BloodVial => Some(Box::new(BloodVialRelic::new(crate::battle::target::Entity::Player))),
            _ => None,
        }
    }
}