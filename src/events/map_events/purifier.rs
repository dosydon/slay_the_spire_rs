use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Purifier shrine event
/// Based on: https://slay-the-spire.fandom.com/wiki/Purifier
/// Simple shrine that allows removing 1 card from deck
pub fn purifier_choices() -> Vec<EventChoice> {
    vec![
        EventChoice {
            text: "Pray (Remove a card from your deck)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::EnterSelectCardsToRemove { count: 1 }),
            ]),
        },
        EventChoice {
            text: "Leave".to_string(),
            outcome: EventOutcome::Effects(vec![]),
        },
    ]
}

pub fn purifier_description() -> &'static str {
    "An ancient shrine radiates a purifying energy. \
     You sense it could cleanse your deck of unwanted cards."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purifier_has_two_choices() {
        let choices = purifier_choices();
        assert_eq!(choices.len(), 2);
    }

    #[test]
    fn test_purifier_pray_choice() {
        let choices = purifier_choices();

        let pray = &choices[0];
        assert!(pray.text.contains("Pray"));
        assert!(pray.text.contains("Remove a card"));

        match &pray.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(GameEffect::EnterSelectCardsToRemove { count: 1 }));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_purifier_leave_choice() {
        let choices = purifier_choices();

        let leave = &choices[1];
        assert!(leave.text.contains("Leave"));

        match &leave.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 0);
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_purifier_description() {
        let desc = purifier_description();
        assert!(desc.contains("shrine"));
        assert!(desc.len() > 0);
    }
}
