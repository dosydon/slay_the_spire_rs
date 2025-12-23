use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Living Wall event
/// Based on: https://slay-the-spire.fandom.com/wiki/Living_Wall
/// A mysterious living wall offers to help improve your deck
pub fn living_wall_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Forget (Remove a card from your deck)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::EnterSelectCardsToRemove { count: 1 }),
            ]),
        },
        EventChoice {
            text: "Change (Transform a card in your deck)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::EnterSelectCardsToTransform { count: 1 }),
            ]),
        },
        EventChoice {
            text: "Grow (Upgrade a card in your deck)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::EnterSelectCardsToUpgrade { count: 1 }),
            ]),
        },
    ]
}

pub fn living_wall_description() -> &'static str {
    "You encounter a peculiar living wall. Its surface ripples and shifts, \
     and you sense it can help you alter your deck in various ways."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_living_wall_has_three_choices() {
        let choices = living_wall_choices();
        assert_eq!(choices.len(), 3);
    }

    #[test]
    fn test_living_wall_forget_choice() {
        let choices = living_wall_choices();

        let forget = &choices[0];
        assert!(forget.text.contains("Forget"));
        assert!(forget.text.contains("Remove a card"));

        match &forget.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(GameEffect::EnterSelectCardsToRemove { count: 1 }));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_living_wall_change_choice() {
        let choices = living_wall_choices();

        let change = &choices[1];
        assert!(change.text.contains("Change"));
        assert!(change.text.contains("Transform a card"));

        match &change.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(GameEffect::EnterSelectCardsToTransform { count: 1 }));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_living_wall_grow_choice() {
        let choices = living_wall_choices();

        let grow = &choices[2];
        assert!(grow.text.contains("Grow"));
        assert!(grow.text.contains("Upgrade a card"));

        match &grow.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(GameEffect::EnterSelectCardsToUpgrade { count: 1 }));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_living_wall_description() {
        let desc = living_wall_description();
        assert!(desc.contains("wall"));
        assert!(desc.len() > 0);
    }
}
