pub mod burning_blood;
pub mod anchor;
pub mod factory;

pub use burning_blood::BurningBloodRelic;
pub use anchor::AnchorRelic;
pub use factory::{create_burning_blood_relic, create_anchor_relic};

#[derive(Debug, Clone, PartialEq)]
pub enum Relic {
    BurningBlood,
    Anchor,
}