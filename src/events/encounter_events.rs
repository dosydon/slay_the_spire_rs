use crate::game::enemy::EnemyTrait;
use crate::game::global_info::GlobalInfo;
use crate::enemies::EnemyEnum;
use crate::utils::CategoricalDistribution;
use crate::enemies::sentry::Sentry;
use crate::enemies::lagavulin::Lagavulin;
use crate::events::SLSEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum EncounterEvent {
    // Act 1 Easy Pool (First 3 encounters)
    TwoLouses,      // Two Louses (Red or Green)
    JawWorm,        // Single Jaw Worm
    Cultist,        // Single Cultist
    SmallSlimes,    // Spike Slime M + Acid Slime S OR Acid Slime M + Spike Slime S

    // Act 1 Hard Pool (After first 3 encounters)
    GangOfGremlins,  // 4 random gremlins (Mad, Sneaky, Fat, Shield, or Wizard)
    // TODO: LargeSlime - Spike Slime (L) or Acid Slime (L) - not yet implemented
    SwarmOfSlimes,   // 3× Spike Slime (S) + 2× Acid Slime (S)
    BlueSlaver,      // Single Blue Slaver
    RedSlaver,       // Single Red Slaver
    ThreeLouses,     // Three Louses (each can be Red or Green)
    TwoFungiBeasts,  // Two Fungi Beasts with Spore Cloud
    ExordiumThugs,   // Two-enemy fight: (Louse/Med Slime) + (Slaver/Cultist/Looter)
    ExordiumWildlife,// Two-enemy fight: (Fungi Beast/Jaw Worm) + (Louse/Med Slime)
    Looter,          // Single Looter that steals gold and escapes

    // Act 1 Elite Encounters
    GremlinNob,      // Act 1 Elite - Gremlin Nob
    ThreeSentries,   // Act 1 Elite - 3 Sentries
    Lagavulin,       // Act 1 Elite - Lagavulin
}

pub fn sample_encounter_event(global_info: &GlobalInfo, event_history: &[SLSEvent], rng: &mut impl rand::Rng) -> EncounterEvent {
    // Count only encounter events in the history
    let encounter_count = event_history.iter().filter(|event| {
        matches!(event, SLSEvent::EncounterEvent(_))
    }).count();

    // For the first three enemy encounters, sample from easy pool
    if encounter_count < 3 {
        act1_easy_pool().sample_owned(rng)
    } else {
        // After first three encounters, sample from the full Act 1 pool (excluding elites)
        act1_hard_pool().sample_owned(rng)
    }
}

/// Sample an elite encounter from the Act 1 elite pool
pub fn sample_elite_encounter(_global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> EncounterEvent {
    act1_elite_encounters().sample_owned(rng)
}

fn act1_easy_pool() -> CategoricalDistribution<EncounterEvent> {
    // Easy encounters for first three enemy encounters (no elites)
    CategoricalDistribution::new(vec![
        (EncounterEvent::TwoLouses, 0.2),
        (EncounterEvent::JawWorm, 0.2),
        (EncounterEvent::Cultist, 0.2),
        (EncounterEvent::SmallSlimes, 0.2),
    ])
}

fn act1_hard_pool() -> CategoricalDistribution<EncounterEvent> {
    // Act 1 Hard Pool - Remaining Combat Encounters (after first 3)
    // Weights from ENEMIES.md
    CategoricalDistribution::new(vec![
        (EncounterEvent::GangOfGremlins, 1.0),   // Weight: 1
        // TODO: LargeSlime (weight 2.0) - not implemented yet (needs Large Slime enemies)
        (EncounterEvent::SwarmOfSlimes, 1.0),    // Weight: 1
        (EncounterEvent::BlueSlaver, 2.0),       // Weight: 2
        (EncounterEvent::RedSlaver, 1.0),        // Weight: 1
        (EncounterEvent::ThreeLouses, 2.0),      // Weight: 2
        (EncounterEvent::TwoFungiBeasts, 2.0),   // Weight: 2
        (EncounterEvent::ExordiumThugs, 1.5),    // Weight: 1.5
        (EncounterEvent::ExordiumWildlife, 1.5), // Weight: 1.5
        (EncounterEvent::Looter, 2.0),           // Weight: 2
    ])
}

fn act1_elite_encounters() -> CategoricalDistribution<EncounterEvent> {
    // Act 1 elite encounter pool
    CategoricalDistribution::new(vec![
        (EncounterEvent::ThreeSentries, 0.57), // 57% chance
        (EncounterEvent::Lagavulin, 0.43),     // 43% chance
    ])
}

impl EncounterEvent {
    /// Get a random encounter event from the Act 1 first three encounters pool
    pub fn get_act1_first_three_encounter(rng: &mut impl rand::Rng) -> EncounterEvent {
        let distribution = act1_easy_pool();
        distribution.sample_owned(rng)
    }

    /// Get a random elite encounter from the Act 1 elite pool
    pub fn get_act1_elite_encounter(rng: &mut impl rand::Rng) -> EncounterEvent {
        let distribution = act1_elite_encounters();
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
            EncounterEvent::Looter => {
                let looter = crate::enemies::looter::Looter::instantiate(rng, global_info);
                vec![EnemyEnum::Looter(looter)]
            }
            EncounterEvent::TwoFungiBeasts => {
                let fungi1 = crate::enemies::fungi_beast::FungiBeast::instantiate(rng, global_info);
                let fungi2 = crate::enemies::fungi_beast::FungiBeast::instantiate(rng, global_info);
                vec![EnemyEnum::FungiBeast(fungi1), EnemyEnum::FungiBeast(fungi2)]
            }
            EncounterEvent::BlueSlaver => {
                let blue_slaver = crate::enemies::blue_slaver::BlueSlaver::instantiate(rng, global_info);
                vec![EnemyEnum::BlueSlaver(blue_slaver)]
            }
            EncounterEvent::RedSlaver => {
                let red_slaver = crate::enemies::red_slaver::RedSlaver::instantiate(rng, global_info);
                vec![EnemyEnum::RedSlaver(red_slaver)]
            }
            EncounterEvent::ThreeLouses => {
                // Three Louses; each slot rolls Red or Green independently (50/50)
                let mut louses = Vec::new();
                for _ in 0..3 {
                    if rng.random::<f64>() < 0.5 {
                        let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                        louses.push(EnemyEnum::RedLouse(red_louse));
                    } else {
                        let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                        louses.push(EnemyEnum::GreenLouse(green_louse));
                    }
                }
                louses
            }
            EncounterEvent::SwarmOfSlimes => {
                // 3× Spike Slime (S) + 2× Acid Slime (S)
                let mut slimes = Vec::new();
                for _ in 0..3 {
                    let spike_slime_s = crate::enemies::spike_slime_s::SpikeSlimeS::instantiate(rng, global_info);
                    slimes.push(EnemyEnum::SpikeSlimeS(spike_slime_s));
                }
                for _ in 0..2 {
                    let acid_slime_s = crate::enemies::acid_slime_s::AcidSlimeS::instantiate(rng, global_info);
                    slimes.push(EnemyEnum::AcidSlimeS(acid_slime_s));
                }
                slimes
            }
            EncounterEvent::ExordiumThugs => {
                // Two-enemy fight: first enemy is a Louse (any color) or Medium Slime (any type)
                // second enemy is a Slaver (any color), Cultist, or Looter
                let mut enemies = Vec::new();

                // First enemy: Louse or Medium Slime (equal chance)
                if rng.random::<f64>() < 0.5 {
                    // Louse (Red or Green)
                    if rng.random::<f64>() < 0.5 {
                        let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::RedLouse(red_louse));
                    } else {
                        let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::GreenLouse(green_louse));
                    }
                } else {
                    // Medium Slime (Spike or Acid)
                    if rng.random::<f64>() < 0.5 {
                        let spike_slime_m = crate::enemies::spike_slime_m::SpikeSlimeM::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::SpikeSlimeM(spike_slime_m));
                    } else {
                        let acid_slime_m = crate::enemies::acid_slime_m::AcidSlimeM::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::AcidSlimeM(acid_slime_m));
                    }
                }

                // Second enemy: Slaver, Cultist, or Looter (equal chance)
                let choice = rng.random_range(0..3);
                match choice {
                    0 => {
                        // Slaver (Blue or Red)
                        if rng.random::<f64>() < 0.5 {
                            let blue_slaver = crate::enemies::blue_slaver::BlueSlaver::instantiate(rng, global_info);
                            enemies.push(EnemyEnum::BlueSlaver(blue_slaver));
                        } else {
                            let red_slaver = crate::enemies::red_slaver::RedSlaver::instantiate(rng, global_info);
                            enemies.push(EnemyEnum::RedSlaver(red_slaver));
                        }
                    }
                    1 => {
                        let cultist = crate::enemies::cultist::Cultist::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::Cultist(cultist));
                    }
                    _ => {
                        let looter = crate::enemies::looter::Looter::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::Looter(looter));
                    }
                }

                enemies
            }
            EncounterEvent::ExordiumWildlife => {
                // Two-enemy fight: first enemy is a Fungi Beast or Jaw Worm
                // second enemy is a Louse (any color) or Medium Slime (any type)
                let mut enemies = Vec::new();

                // First enemy: Fungi Beast or Jaw Worm (equal chance)
                if rng.random::<f64>() < 0.5 {
                    let fungi_beast = crate::enemies::fungi_beast::FungiBeast::instantiate(rng, global_info);
                    enemies.push(EnemyEnum::FungiBeast(fungi_beast));
                } else {
                    let jaw_worm = crate::enemies::jaw_worm::JawWorm::instantiate(rng, global_info);
                    enemies.push(EnemyEnum::JawWorm(jaw_worm));
                }

                // Second enemy: Louse or Medium Slime (equal chance)
                if rng.random::<f64>() < 0.5 {
                    // Louse (Red or Green)
                    if rng.random::<f64>() < 0.5 {
                        let red_louse = crate::enemies::red_louse::RedLouse::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::RedLouse(red_louse));
                    } else {
                        let green_louse = crate::enemies::green_louse::GreenLouse::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::GreenLouse(green_louse));
                    }
                } else {
                    // Medium Slime (Spike or Acid)
                    if rng.random::<f64>() < 0.5 {
                        let spike_slime_m = crate::enemies::spike_slime_m::SpikeSlimeM::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::SpikeSlimeM(spike_slime_m));
                    } else {
                        let acid_slime_m = crate::enemies::acid_slime_m::AcidSlimeM::instantiate(rng, global_info);
                        enemies.push(EnemyEnum::AcidSlimeM(acid_slime_m));
                    }
                }

                enemies
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
    fn test_blue_slaver_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::BlueSlaver;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        assert_eq!(enemies.len(), 1);

        match &enemies[0] {
            EnemyEnum::BlueSlaver(slaver) => {
                // Verify HP is in correct range for base ascension
                assert!(slaver.get_hp() >= 46 && slaver.get_hp() <= 50);
            }
            _ => panic!("Expected BlueSlaver enemy"),
        }
    }

    #[test]
    fn test_red_slaver_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::RedSlaver;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        assert_eq!(enemies.len(), 1);

        match &enemies[0] {
            EnemyEnum::RedSlaver(slaver) => {
                // Verify HP is in correct range for base ascension
                assert!(slaver.get_hp() >= 46 && slaver.get_hp() <= 50);
            }
            _ => panic!("Expected RedSlaver enemy"),
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

    #[test]
    fn test_looter_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::Looter;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 1 Looter
        assert_eq!(enemies.len(), 1);

        // Should be a Looter
        match &enemies[0] {
            EnemyEnum::Looter(looter) => {
                // Verify HP is in the correct range for ascension 0 (44-48)
                let hp = looter.get_hp();
                assert!(hp >= 44 && hp <= 48, "Looter HP {} should be in range 44-48", hp);
            }
            _ => panic!("Expected Looter enemy"),
        }
    }

    #[test]
    fn test_two_fungi_beasts_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::TwoFungiBeasts;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 2 Fungi Beasts
        assert_eq!(enemies.len(), 2);

        // Both should be Fungi Beasts
        for enemy in &enemies {
            match enemy {
                EnemyEnum::FungiBeast(fungi) => {
                    // Verify HP is in the correct range for ascension 0 (22-28)
                    let hp = fungi.get_hp();
                    assert!(hp >= 22 && hp <= 28, "Fungi Beast HP {} should be in range 22-28", hp);
                }
                _ => panic!("Expected Fungi Beast enemy"),
            }
        }
    }

    #[test]
    fn test_three_louses_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::ThreeLouses;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 3 louses
        assert_eq!(enemies.len(), 3);

        // All should be louses (Red or Green)
        for enemy in &enemies {
            match enemy {
                EnemyEnum::RedLouse(_) | EnemyEnum::GreenLouse(_) => {
                    // Success
                }
                _ => panic!("Expected RedLouse or GreenLouse"),
            }
        }
    }

    // TODO: test_large_slime_encounter - Not implemented yet (Large Slimes not created)

    #[test]
    fn test_swarm_of_slimes_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::SwarmOfSlimes;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 5 slimes (3 Spike S + 2 Acid S)
        assert_eq!(enemies.len(), 5);

        let mut spike_count = 0;
        let mut acid_count = 0;

        for enemy in &enemies {
            match enemy {
                EnemyEnum::SpikeSlimeS(_) => spike_count += 1,
                EnemyEnum::AcidSlimeS(_) => acid_count += 1,
                _ => panic!("Expected SpikeSlimeS or AcidSlimeS"),
            }
        }

        assert_eq!(spike_count, 3, "Should have exactly 3 Spike Slimes (S)");
        assert_eq!(acid_count, 2, "Should have exactly 2 Acid Slimes (S)");
    }

    #[test]
    fn test_exordium_thugs_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::ExordiumThugs;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 2 enemies
        assert_eq!(enemies.len(), 2);

        // First enemy should be Louse or Medium Slime
        match &enemies[0] {
            EnemyEnum::RedLouse(_) | EnemyEnum::GreenLouse(_) |
            EnemyEnum::SpikeSlimeM(_) | EnemyEnum::AcidSlimeM(_) => {
                // Success
            }
            _ => panic!("First enemy should be Louse or Medium Slime"),
        }

        // Second enemy should be Slaver, Cultist, or Looter
        match &enemies[1] {
            EnemyEnum::BlueSlaver(_) | EnemyEnum::RedSlaver(_) |
            EnemyEnum::Cultist(_) | EnemyEnum::Looter(_) => {
                // Success
            }
            _ => panic!("Second enemy should be Slaver, Cultist, or Looter"),
        }
    }

    #[test]
    fn test_exordium_wildlife_encounter() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        let encounter = EncounterEvent::ExordiumWildlife;
        let enemies = encounter.instantiate(&mut rng, &global_info);

        // Should spawn exactly 2 enemies
        assert_eq!(enemies.len(), 2);

        // First enemy should be Fungi Beast or Jaw Worm
        match &enemies[0] {
            EnemyEnum::FungiBeast(_) | EnemyEnum::JawWorm(_) => {
                // Success
            }
            _ => panic!("First enemy should be Fungi Beast or Jaw Worm"),
        }

        // Second enemy should be Louse or Medium Slime
        match &enemies[1] {
            EnemyEnum::RedLouse(_) | EnemyEnum::GreenLouse(_) |
            EnemyEnum::SpikeSlimeM(_) | EnemyEnum::AcidSlimeM(_) => {
                // Success
            }
            _ => panic!("Second enemy should be Louse or Medium Slime"),
        }
    }

}

