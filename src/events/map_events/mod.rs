//! SLS Events - Non-combat events that offer choices to the player
//!
//! Events present the player with multiple choices, each leading to effects or subsequent choices.

mod big_fish;
mod the_cleric;
mod dead_adventurer;
mod golden_idol;
mod shining_light;
mod world_of_goop;
mod wing_statue;
mod purifier;
mod living_wall;

use crate::game::effect::Effect;
use crate::game::global_info::GlobalInfo;
use rand::prelude::IndexedRandom;

/// All non-combat events in Slay the Spire
#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum MapEvent {
    /// The Big Fish event - A large fish appears with multiple interaction options
    BigFish,
    /// The Cleric - A cleric offers healing services for gold
    TheCleric,
    /// Dead Adventurer - Find a dead adventurer's belongings
    DeadAdventurer,
    /// Golden Idol - Steal a golden idol from a trap
    GoldenIdol,
    /// Shining Light - Divine light offers to upgrade cards
    ShiningLight,
    /// World of Goop - Slime world offers gold but debuffs you
    WorldOfGoop,
    /// Wing Statue - A statue with a missing wing
    WingStatue,
    /// Purifier - Shrine that removes cards from deck
    Purifier,
    /// Living Wall - A mysterious wall offering deck improvements
    LivingWall,
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

/// Context needed to generate event choices with game state
pub struct EventContext {
    pub floor: u32,
    pub player_hp: u32,
    pub player_max_hp: u32,
    pub gold: u32,
    pub ascension: u32,
}

impl MapEvent {
    /// Get the initial choices for this event (basic version without game context)
    /// Note: Some events may have simplified choices without game context.
    /// Prefer using get_choices_with_context for full game integration.
    pub fn get_choices(&self) -> Vec<EventChoice> {
        match self {
            MapEvent::BigFish => big_fish::big_fish_choices(80),  // Default max HP
            MapEvent::TheCleric => the_cleric::the_cleric_choices(0, 0, 0),
            MapEvent::DeadAdventurer => dead_adventurer::dead_adventurer_choices(),
            MapEvent::GoldenIdol => golden_idol::golden_idol_choices(),
            MapEvent::ShiningLight => shining_light::shining_light_choices(80, 0),  // Default max HP, ascension 0
            MapEvent::WorldOfGoop => world_of_goop::world_of_goop_choices(),
            MapEvent::WingStatue => wing_statue::wing_statue_choices(),
            MapEvent::Purifier => purifier::purifier_choices(),
            MapEvent::LivingWall => living_wall::living_wall_choices(),
        }
    }

    /// Get the initial choices for this event with game context
    /// This allows events to adjust their choices based on game state
    /// (e.g., gold cost scaling with floor, heal amounts based on current HP)
    pub fn get_choices_with_context(&self, ctx: &EventContext) -> Vec<EventChoice> {
        match self {
            MapEvent::BigFish => big_fish::big_fish_choices(ctx.player_max_hp),
            MapEvent::TheCleric => the_cleric::the_cleric_choices(ctx.floor, ctx.player_hp, ctx.player_max_hp),
            MapEvent::DeadAdventurer => dead_adventurer::dead_adventurer_choices(),
            MapEvent::GoldenIdol => golden_idol::golden_idol_choices(),
            MapEvent::ShiningLight => shining_light::shining_light_choices(ctx.player_max_hp, ctx.ascension),
            MapEvent::WorldOfGoop => world_of_goop::world_of_goop_choices(),
            MapEvent::WingStatue => wing_statue::wing_statue_choices(),
            MapEvent::Purifier => purifier::purifier_choices(),
            MapEvent::LivingWall => living_wall::living_wall_choices(),
        }
    }

    /// Get the event title/description text
    pub fn get_description(&self) -> &'static str {
        match self {
            MapEvent::BigFish => big_fish::big_fish_description(),
            MapEvent::TheCleric => the_cleric::the_cleric_description(),
            MapEvent::DeadAdventurer => dead_adventurer::dead_adventurer_description(),
            MapEvent::GoldenIdol => golden_idol::golden_idol_description(),
            MapEvent::ShiningLight => shining_light::shining_light_description(),
            MapEvent::WorldOfGoop => world_of_goop::world_of_goop_description(),
            MapEvent::WingStatue => wing_statue::wing_statue_description(),
            MapEvent::Purifier => purifier::purifier_description(),
            MapEvent::LivingWall => living_wall::living_wall_description(),
        }
    }
}

/// Sample a random SLS Event based on the current game state
pub fn sample_sls_event(_global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> MapEvent {
    // All Act 1 events currently implemented
    let act1_events = vec![
        MapEvent::BigFish,
        MapEvent::TheCleric,
        MapEvent::DeadAdventurer,
        MapEvent::GoldenIdol,
        MapEvent::ShiningLight,
        MapEvent::WorldOfGoop,
        MapEvent::WingStatue,
        MapEvent::Purifier,
        MapEvent::LivingWall,
    ];

    // TODO: In the future, this could be expanded to:
    // - Filter events based on act number
    // - Weight events differently based on floor
    // - Track event history to avoid repeats
    // - Apply ascension-specific event pool changes

    *act1_events.choose(rng).unwrap()
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
        assert!(banana.text.contains("Banana"));
        assert!(banana.text.contains("Gain 5 Max HP"));

        match &banana.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Battle(crate::game::effect::BattleEffect::HealAndIncreaseMaxHp(5)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_donut_choice() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();

        let donut = &choices[1];
        assert!(donut.text.contains("Donut"));
        assert!(donut.text.contains("Heal"));
        assert!(donut.text.contains("26 HP")); // 80 / 3 = 26 (default max HP)

        match &donut.outcome {
            EventOutcome::Effects(effects) => {
                assert_eq!(effects.len(), 1);
                // Heal(0) is a placeholder that means: calculate as max_hp / 3
                assert_eq!(effects[0], Effect::Battle(crate::game::effect::BattleEffect::Heal(0)));
            }
            _ => panic!("Expected Effects outcome"),
        }
    }

    #[test]
    fn test_big_fish_box_choice() {
        let event = MapEvent::BigFish;
        let choices = event.get_choices();

        let box_choice = &choices[2];
        assert!(box_choice.text.contains("Box"));
        assert!(box_choice.text.contains("relic"));

        match &box_choice.outcome {
            EventOutcome::Effects(effects) => {
                // Box should give a random relic
                assert_eq!(effects.len(), 1);
                assert_eq!(effects[0], Effect::Game(crate::game::effect::GameEffect::ObtainRandomRelic));
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
