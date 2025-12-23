use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Shining Light event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Shining_Light
/// - Pray: Upgrade 2 random cards, take 20% of max HP in damage (30% at Ascension 15+)
/// - Enter the light: Does nothing
pub fn shining_light_choices(player_max_hp: u32, ascension: u32) -> Vec<EventChoice> {
    // Calculate damage based on ascension level
    // 20% at Ascension 0-14, 30% at Ascension 15+
    let damage_percent = if ascension >= 15 { 0.30 } else { 0.20 };
    let damage = (player_max_hp as f64 * damage_percent).round() as u32;

    vec![
        EventChoice {
            text: format!("Pray (Upgrade 2 random cards, take {} damage)", damage),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::UpgradeRandomCards { count: 2 }),
                Effect::Battle(BattleEffect::LoseHp(damage)),
            ]),
        },
        EventChoice {
            text: "Enter the light (No effect)".to_string(),
            outcome: EventOutcome::Effects(vec![
                // Does nothing - leave empty
            ]),
        },
    ]
}

pub fn shining_light_description() -> &'static str {
    "A radiant light descends from above, bathing you in warmth. \
     A divine presence offers to purify your techniques."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shining_light_has_two_choices() {
        let choices = shining_light_choices(80, 0);
        assert_eq!(choices.len(), 2);
    }

    #[test]
    fn test_shining_light_pray_choice_ascension_0() {
        let choices = shining_light_choices(80, 0);

        let pray = &choices[0];
        assert!(pray.text.contains("Pray"));
        assert!(pray.text.contains("Upgrade 2 random cards"));
        assert!(pray.text.contains("16 damage")); // 20% of 80 = 16

        match &pray.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 2);
                assert_eq!(effects[0], Effect::Game(GameEffect::UpgradeRandomCards { count: 2 }));
                assert_eq!(effects[1], Effect::Battle(BattleEffect::LoseHp(16)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_shining_light_pray_choice_ascension_15() {
        let choices = shining_light_choices(80, 15);

        let pray = &choices[0];
        assert!(pray.text.contains("Pray"));
        assert!(pray.text.contains("24 damage")); // 30% of 80 = 24

        match &pray.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 2);
                assert_eq!(effects[0], Effect::Game(GameEffect::UpgradeRandomCards { count: 2 }));
                assert_eq!(effects[1], Effect::Battle(BattleEffect::LoseHp(24)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_shining_light_enter_light_choice() {
        let choices = shining_light_choices(80, 0);

        let enter_light = &choices[1];
        assert!(enter_light.text.contains("Enter the light"));
        assert!(enter_light.text.contains("No effect"));

        match &enter_light.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 0); // Does nothing
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_event_description() {
        let desc = shining_light_description();
        assert!(desc.contains("light"));
        assert!(desc.len() > 0);
    }
}
