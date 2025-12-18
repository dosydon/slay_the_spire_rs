//! SLS Events - Non-combat events that offer choices to the player
//!
//! Events present the player with multiple choices, each leading to effects or subsequent choices.

use crate::game::effect::Effect;
use crate::game::global_info::GlobalInfo;
use crate::utils::CategoricalDistribution;

/// All non-combat events in Slay the Spire
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum MapEvent {
    /// The Big Fish event - A large fish appears with multiple interaction options
    BigFish,
}

/// Represents a choice the player can make in an event
#[derive(Debug, Clone, PartialEq)]
pub struct EventChoice {
    /// Display text for this choice
    pub text: String,
    /// What happens when this choice is selected
    pub outcome: EventOutcome,
}

/// What happens after making a choice
#[derive(Debug, Clone, PartialEq)]
pub enum EventOutcome {
    /// Apply a list of effects and end the event
    Effects(Vec<Effect>),
    /// Transition to another set of choices (for multi-stage events)
    NextChoices(Vec<EventChoice>),
}

impl MapEvent {
    /// Get the initial choices for this event
    pub fn get_choices(&self) -> Vec<EventChoice> {
        match self {
            MapEvent::BigFish => big_fish_choices(),
        }
    }

    /// Get the event title/description text
    pub fn get_description(&self) -> &'static str {
        match self {
            MapEvent::BigFish => {
                "A massive fish emerges from the depths, its scales shimmering with an otherworldly glow. \
                 It regards you with ancient, knowing eyes."
            }
        }
    }
}

/// Sample a random SLS Event based on the current game state
pub fn sample_sls_event(global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> MapEvent {
    // For now, only return BigFish since that's the only event we have implemented
    // In the future, this could be expanded to sample from a distribution based on floor, act, etc.
    MapEvent::BigFish
}

/// Create a distribution for Act 1 events (for future expansion)
fn act1_event_distribution() -> CategoricalDistribution<MapEvent> {
    CategoricalDistribution::new(vec![
        (MapEvent::BigFish, 1.0), // Currently the only event
    ])
}

/// Big Fish event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Big_Fish
/// - Banana: Gain 5 Max HP
/// - Donut: Heal for 1/3 of Max HP (implementation note: exact amount depends on player max HP)
/// - Box: Obtain a random relic
fn big_fish_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Banana".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::HealAndIncreaseMaxHp(5),  // Gain 5 Max HP
            ]),
        },
        EventChoice {
            text: "Donut".to_string(),
            outcome: EventOutcome::Effects(vec![
                // Heal for 1/3 of Max HP
                // Note: The actual heal amount needs to be calculated based on player's max HP
                // This will need special handling in the event resolution system
                // For now, we use a placeholder value that will need to be overridden
                Effect::Heal(0),  // 0 indicates: calculate as max_hp / 3
            ]),
        },
        EventChoice {
            text: "Box".to_string(),
            outcome: EventOutcome::Effects(vec![
                // Obtain a random relic
                // TODO: This will need a new effect type like Effect::ObtainRandomRelic
                // or special handling in the event system to add a relic
                // For now, we'll leave it as an empty effect list
            ]),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_fish_has_three_choices() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();
        assert_eq!(choices.len(), 3);
    }

    #[test]
    fn test_big_fish_banana_choice() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();

        let banana = &choices[0];
        assert_eq!(banana.text, "Banana");

        match &banana.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::HealAndIncreaseMaxHp(5));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_donut_choice() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();

        let donut = &choices[1];
        assert_eq!(donut.text, "Donut");

        match &donut.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                // Heal(0) is a placeholder that means: calculate as max_hp / 3
                assert_eq!(effects[0], Effect::Heal(0));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_box_choice() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();

        let box_choice = &choices[2];
        assert_eq!(box_choice.text, "Box");

        match &box_choice.outcome {
            EventOutcome::Effects(effects) => {
                // Box should give a relic, but we haven't implemented that effect yet
                assert_eq!(effects.len(), 0);
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_event_description() {
        let event = MapEvent::BigFish;
        let desc = event.get_description();
        assert!(desc.contains("fish"));
        assert!(desc.len() > 0);
    }
}
