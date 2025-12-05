use crate::game::enemy::EnemyTrait;
use crate::game::global_info::GlobalInfo;
use crate::enemies::EnemyEnum;
use crate::utils::CategoricalDistribution;
use crate::enemies::sentry::Sentry;
use crate::enemies::lagavulin::Lagavulin;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncounterEvent {
    TwoLouses,
    JawWorm,
    Cultist,
    SmallSlimes,
    GremlinNob,
    ThreeSentries,  // Act 1 Elite encounter
    Lagavulin,      // Act 1 Elite encounter
}

pub fn sample_encounter_event(_global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> EncounterEvent {
    // For simplicity, we only implement Act 1 first three encounters sampling here
    act1_first_three_encounters().sample_owned(rng)
}

fn act1_first_three_encounters() -> CategoricalDistribution<EncounterEvent> {
    CategoricalDistribution::new(vec![
        (EncounterEvent::TwoLouses, 0.25),
        (EncounterEvent::JawWorm, 0.25),
        (EncounterEvent::Cultist, 0.25),
        (EncounterEvent::SmallSlimes, 0.25),
        (EncounterEvent::GremlinNob, 0.25),
        (EncounterEvent::ThreeSentries, 0.25), // Act 1 Elite encounter
    ])
}

impl EncounterEvent {
    /// Get a random encounter event from the Act 1 first three encounters pool
    pub fn get_act1_first_three_encounter(rng: &mut impl rand::Rng) -> EncounterEvent {
        let distribution = act1_first_three_encounters();
        distribution.sample_owned(rng)
    }

    pub fn instantiate(&self, rng: &mut impl rand::Rng, global_info: &crate::game::global_info::GlobalInfo) -> Vec<EnemyEnum> {
        match self {
            EncounterEvent::TwoLouses => {
                
                // First louse: 50% chance of red or green
                let louse1 = if rng.random::<f64>() < 0.5 {
                    let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                    EnemyEnum::RedLouse(red_louse)
                } else {
                    let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                    EnemyEnum::GreenLouse(green_louse)
                };
                
                // Second louse: 50% chance of red or green
                let louse2 = if rng.random::<f64>() < 0.5 {
                    let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                    EnemyEnum::RedLouse(red_louse)
                } else {
                    let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                    EnemyEnum::GreenLouse(green_louse)
                };
                
                vec![louse1, louse2]
            }
            EncounterEvent::JawWorm => {
                let jaw_worm = crate::enemies::jaw_worm::JawWorm::instantiate(rng, global_info);
                vec![EnemyEnum::JawWorm(jaw_worm)]
            }
            EncounterEvent::Cultist => {
                let cultist = crate::enemies::cultist::Cultist::instantiate(rng, global_info);
                vec![EnemyEnum::Cultist(cultist)]
            }
            EncounterEvent::SmallSlimes => {
                if rng.random::<f64>() < 0.5 {
                    let slime1 = crate::enemies::spike_slime_m::SpikeSlimeM::instantiate(rng, global_info);
                    let slime2 = crate::enemies::acid_slime_s::AcidSlimeS::instantiate(rng, global_info);
                    return vec![EnemyEnum::SpikeSlimeM(slime1), EnemyEnum::AcidSlimeS(slime2)];
                } else {
                    let slime1 = crate::enemies::acid_slime_m::AcidSlimeM::instantiate(rng, global_info);
                    let slime2 = crate::enemies::spike_slime_s::SpikeSlimeS::instantiate(rng, global_info);
                    return vec![EnemyEnum::AcidSlimeM(slime1), EnemyEnum::SpikeSlimeS(slime2)];
                }
            }
            EncounterEvent::GremlinNob => {
                let gremlin_nob = crate::enemies::gremlin_nob::GremlinNob::instantiate(rng, global_info);
                vec![EnemyEnum::GremlinNob(gremlin_nob)]
            }
            EncounterEvent::ThreeSentries => {
                // Create 3 Sentries with proper move patterns using the helper method
                let sentries = Sentry::create_sentry_group(rng, global_info.ascention);
                sentries.into_iter().map(|s| EnemyEnum::Sentry(s)).collect()
            }
            EncounterEvent::Lagavulin => {
                let lagavulin = Lagavulin::instantiate(rng, global_info);
                vec![EnemyEnum::Lagavulin(lagavulin)]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_jaw_worm_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        
        let encounter = EncounterEvent::JawWorm;
        let enemies = encounter.instantiate(&mut rng, &global_info);
        
        assert_eq!(enemies.len(), 1);
        
        match &enemies[0] {
            EnemyEnum::JawWorm(_) => {
                // Success - we got a JawWorm
            }
            _ => panic!("Expected JawWorm enemy"),
        }
    }

    #[test]
    fn test_two_louses_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::TwoLouses;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        assert_eq!(enemies.len(), 2);

        // Both should be louses (either red or green)
        for enemy in &enemies {
            match enemy {
                EnemyEnum::RedLouse(_) | EnemyEnum::GreenLouse(_) => {
                    // Success - we got a louse
                }
                _ => panic!("Expected RedLouse or GreenLouse enemy"),
            }
        }
    }

    #[test]
    fn test_three_sentries_start_with_artifact() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::ThreeSentries;
        let enemy_enums = encounter.instantiate(&mut rng, &global_info);

        // Should have exactly 3 Sentries
        assert_eq!(enemy_enums.len(), 3);

        // All should be Sentries
        for enemy in &enemy_enums {
            assert!(matches!(enemy, EnemyEnum::Sentry(_)));
        }

        // Create a battle to test artifact via listener
        let enemies: Vec<EnemyInBattle> = enemy_enums
            .into_iter()
            .map(|e| EnemyInBattle::new(e))
            .collect();

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info.clone(), 50, 80, enemies, &mut rng);

        // Each Sentry should have 1 Artifact after combat start
        for i in 0..3 {
            assert_eq!(
                battle.get_enemies()[i].battle_info.get_artifact(),
                1,
                "Sentry {} should start with 1 Artifact",
                i
            );
        }
    }
}

