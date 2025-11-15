pub mod burning_blood;
pub mod factory;

pub use burning_blood::BurningBloodRelic;
pub use factory::create_burning_blood_relic;

#[derive(Debug, Clone, PartialEq)]
pub enum Relic {
    BurningBlood,
}