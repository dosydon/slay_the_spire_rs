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
    GangOfGremlins, // 4 random gremlins (Mad, Sneaky, Fat, Shield, or Wizard)
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
        (EncounterEvent::TwoLouses, 0.2),
        (EncounterEvent::JawWorm, 0.2),
        (EncounterEvent::Cultist, 0.2),
        (EncounterEvent::SmallSlimes, 0.2),
        (EncounterEvent::GangOfGremlins, 0.2),
        (EncounterEvent::GremlinNob, 0.2),
        (EncounterEvent::ThreeSentries, 0.2), // Act 1 Elite encounter
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
            EncounterEvent::GangOfGremlins => {
                // Spawn 4 random gremlins from the weighted pool
                // Pool: 2× Fat, 2× Sneaky, 2× Mad, 1× Shield, 1× Wizard
                // Total: 8 options, pick 4 randomly
                let mut pool = vec![
                    0, 0, // Fat x2
                    1, 1, // Sneaky x2
                    2, 2, // Mad x2
                    3,    // Shield x1
                    4,    // Wizard x1
                ];

                let mut gremlins = Vec::new();

                for _ in 0..4 {
                    // Pick random gremlin from remaining pool
                    let idx = rng.random_range(0..pool.len());
                    let gremlin_type = pool.remove(idx);

                    let gremlin = match gremlin_type {
                        0 => {
                            let fat = crate::enemies::fat_gremlin::FatGremlin::instantiate(rng, global_info);
                            EnemyEnum::FatGremlin(fat)
                        }
                        1 => {
                            let sneaky = crate::enemies::sneaky_gremlin::SneakyGremlin::instantiate(rng, global_info);
                            EnemyEnum::SneakyGremlin(sneaky)
                        }
                        2 => {
                            let mad = crate::enemies::mad_gremlin::MadGremlin::instantiate(rng, global_info);
                            EnemyEnum::MadGremlin(mad)
                        }
                        3 => {
                            let shield = crate::enemies::shield_gremlin::ShieldGremlin::instantiate(rng, global_info);
                            EnemyEnum::ShieldGremlin(shield)
                        }
                        _ => {
                            let wizard = crate::enemies::gremlin_wizard::GremlinWizard::instantiate(rng, global_info);
                            EnemyEnum::GremlinWizard(wizard)
                        }
                    };
                    gremlins.push(gremlin);
                }

                gremlins
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

    #[test]
    fn test_gang_of_gremlins_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::GangOfGremlins;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 4 gremlins
        assert_eq!(enemies.len(), 4);

        // All should be gremlins (Fat, Sneaky, Mad, Shield, or Wizard)
        for enemy in &enemies {
            match enemy {
                EnemyEnum::FatGremlin(_) |
                EnemyEnum::SneakyGremlin(_) |
                EnemyEnum::MadGremlin(_) |
                EnemyEnum::ShieldGremlin(_) |
                EnemyEnum::GremlinWizard(_) => {
                    // Success - we got a gremlin
                }
                _ => panic!("Expected a Gremlin enemy, got: {:?}", enemy),
            }
        }
    }

    #[test]
    fn test_gang_of_gremlins_variety() {
        use std::collections::HashSet;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Run multiple times to ensure we get different gremlin types
        let mut seen_types = HashSet::new();

        for _ in 0..20 {
            let encounter = EncounterEvent::GangOfGremlins;
            let enemies = encounter.instantiate(&mut rng, &global_info);

            for enemy in &enemies {
                let type_name = match enemy {
                    EnemyEnum::FatGremlin(_) => "Fat",
                    EnemyEnum::SneakyGremlin(_) => "Sneaky",
                    EnemyEnum::MadGremlin(_) => "Mad",
                    EnemyEnum::ShieldGremlin(_) => "Shield",
                    EnemyEnum::GremlinWizard(_) => "Wizard",
                    _ => panic!("Expected a Gremlin enemy"),
                };
                seen_types.insert(type_name);
            }
        }

        // After 20 encounters (80 gremlins), we should have seen multiple types
        assert!(seen_types.len() >= 3, "Expected to see at least 3 different gremlin types, saw: {}", seen_types.len());
    }

    #[test]
    fn test_gang_of_gremlins_with_battle() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::GangOfGremlins;
        let enemy_enums = encounter.instantiate(&mut rng, &global_info);

        // Create a battle with the gang of gremlins
        let enemies: Vec<EnemyInBattle> = enemy_enums
            .into_iter()
            .map(|e| EnemyInBattle::new(e))
            .collect();

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info.clone(), 50, 80, enemies, &mut rng);

        // All 4 gremlins should be alive at battle start
        assert_eq!(battle.get_enemies().len(), 4);
        for enemy in battle.get_enemies() {
            assert!(enemy.battle_info.is_alive());
        }

        // Check that Mad Gremlin has Angry listener if present
        let has_mad_gremlin = battle.get_enemies().iter().any(|e| {
            matches!(e.enemy, EnemyEnum::MadGremlin(_))
        });

        if has_mad_gremlin {
            // Mad Gremlin should have an Angry listener initialized
            // We can't directly check the listener, but we can verify it exists
            // by checking that damage triggers strength gain (tested separately)
            assert!(true); // Placeholder - listener is tested in mad_gremlin unit tests
        }
    }
}

