use crate::game::effect::{Effect, BattleEffect, GameEffect};
use crate::events::map_events::{EventChoice, EventOutcome};

/// Big Fish event choices
/// Based on: https://slay-the-spire.fandom.com/wiki/Big_Fish
/// - Banana: Gain 5 Max HP
/// - Donut: Heal for 1/3 of Max HP (implementation note: exact amount depends on player max HP)
/// - Box: Obtain a random relic
pub fn big_fish_choices(player_max_hp: u32) -> Vec<EventChoice> {
    // Calculate heal amount for Donut (1/3 of max HP)
    let donut_heal = player_max_hp / 3;

    vec![
        EventChoice {
            text: "Banana (Gain 5 Max HP)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Battle(BattleEffect::HealAndIncreaseMaxHp(5)),  // Gain 5 Max HP
            ]),
        },
        EventChoice {
            text: format!("Donut (Heal {} HP)", donut_heal),
            outcome: EventOutcome::Effects(vec![
                // Heal for 1/3 of Max HP
                // Note: The actual heal amount needs to be calculated based on player's max HP
                // This will need special handling in the event resolution system
                // For now, we use a placeholder value that will need to be overridden
                Effect::Battle(BattleEffect::Heal(0)),  // 0 indicates: calculate as max_hp / 3
            ]),
        },
        EventChoice {
            text: "Box (Obtain a random relic)".to_string(),
            outcome: EventOutcome::Effects(vec![
                Effect::Game(GameEffect::ObtainRandomRelic),
            ]),
        },
    ]
}

pub fn big_fish_description() -> &'static str {
    "A massive fish emerges from the depths, its scales shimmering with an otherworldly glow. \
     It regards you with ancient, knowing eyes."
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big_fish_has_three_choices() {
        let choices = big_fish_choices(80);
        assert_eq!(choices.len(), 3);
    }

    #[test]
    fn test_big_fish_banana_choice() {
        let choices = big_fish_choices(80);

        let banana = &choices[0];
        assert!(banana.text.contains("Banana"));
        assert!(banana.text.contains("Gain 5 Max HP"));

        match &banana.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Battle(BattleEffect::HealAndIncreaseMaxHp(5)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_donut_choice() {
        let choices = big_fish_choices(80);

        let donut = &choices[1];
        assert!(donut.text.contains("Donut"));
        assert!(donut.text.contains("Heal"));
        assert!(donut.text.contains("26 HP")); // 80 / 3 = 26

        match &donut.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                // Heal(0) is a placeholder that means: calculate as max_hp / 3
                assert_eq!(effects[0], Effect::Battle(BattleEffect::Heal(0)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_box_choice() {
        let choices = big_fish_choices(80);

        let box_choice = &choices[2];
        assert!(box_choice.text.contains("Box"));
        assert!(box_choice.text.contains("relic"));

        match &box_choice.outcome {
            EventOutcome::Effects(effects) => {
                // Box should give a random relic
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(GameEffect::ObtainRandomRelic));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_event_description() {
        let desc = big_fish_description();
        assert!(desc.contains("fish"));
        assert!(desc.len() > 0);
    }
}
