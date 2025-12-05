use super::Battle;
use crate::enemies::enemy_enum::{EnemyEnum, EnemyMove};
use crate::game::{effect::Effect, global_info::GlobalInfo};
use crate::battle::target::Entity;
use crate::enemies::red_louse::CurlUpListener;

impl Battle {
    /// Initialize event listeners for enemies based on their type
    pub(in crate::battle) fn initialize_enemy_listeners(&mut self, global_info: &GlobalInfo, rng: &mut impl rand::Rng) {
        for (i, enemy) in self.enemies.iter().enumerate() {
            match &enemy.enemy {
                EnemyEnum::RedLouse(_) => {
                    // Red Louse gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::GreenLouse(_) => {
                    // Green Louse also gets a curl up listener with randomly generated block amount
                    let curl_up = CurlUpListener::new(Entity::Enemy(i), global_info.ascention, rng);
                    self.event_listeners.push(Box::new(curl_up));
                }
                EnemyEnum::JawWorm(_) => {
                    // Jaw Worm has no special listeners
                }
                EnemyEnum::Cultist(_) => {
                    // Cultist has no special listeners
                }
                EnemyEnum::SpikeSlimeS(_) => {
                    // Spike Slime (S) has no special listeners
                }
                EnemyEnum::SpikeSlimeM(_) => {
                    // Spike Slime (M) has no special listeners
                }
                EnemyEnum::AcidSlimeS(_) => {
                    // Acid Slime (S) has no special listeners
                }
                EnemyEnum::AcidSlimeM(_) => {
                    // Acid Slime (M) has no special listeners
                }
                EnemyEnum::GremlinNob(_) => {
                    // Gremlin Nob gets an enrage listener only AFTER it uses its first move (Bellow)
                    // This will be added dynamically when the first move is executed
                }
                EnemyEnum::Lagavulin(_) => {
                    // Lagavulin gets a listener for wake-from-damage, initial block, and Stunned→Awake transition
                    let lagavulin_listener = crate::enemies::lagavulin::LagavulinListener::new(i);
                    self.event_listeners.push(Box::new(lagavulin_listener));

                    // Lagavulin starts with Metallicize 8 while asleep (removed when awakened)
                    let metallicize_listener = crate::cards::ironclad::metallicize::MetallicizeListener::new(Entity::Enemy(i), 8);
                    self.event_listeners.push(Box::new(metallicize_listener));
                }
                EnemyEnum::Sentry(_) => {
                    // Sentry gets 1 Artifact at combat start
                    let sentry_listener = crate::enemies::sentry::SentryListener::new(i);
                    self.event_listeners.push(Box::new(sentry_listener));
                }
                EnemyEnum::FatGremlin(_) => {
                    // Fat Gremlin has no special listeners
                }
                EnemyEnum::SneakyGremlin(_) => {
                    // Sneaky Gremlin has no special listeners
                }
                EnemyEnum::MadGremlin(mad_gremlin) => {
                    // Mad Gremlin gets an Angry listener at combat start
                    use crate::enemies::mad_gremlin::AngryListener;
                    let angry_amount = mad_gremlin.get_angry_stacks();
                    let listener = AngryListener::new(Entity::Enemy(i), angry_amount);
                    self.event_listeners.push(Box::new(listener));
                }
                EnemyEnum::ShieldGremlin(_) => {
                    // Shield Gremlin has no special listeners
                }
                EnemyEnum::GremlinWizard(_) => {
                    // Gremlin Wizard has no special listeners
                }
            }
        }
    }

    /// Sample and store the next action and effects for all enemies
    pub(crate) fn sample_enemy_actions(&mut self, rng: &mut impl rand::Rng) {
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            let (enemy_move, effects) = enemy.enemy.sample_move_and_effects(&self.global_info, rng);
            self.enemy_actions[i] = Some((enemy_move, effects));
        }
    }
    
    /// Get the stored move for a specific enemy
    pub fn get_enemy_move(&self, enemy_index: usize) -> Option<&EnemyMove> {
        self.enemy_actions.get(enemy_index).and_then(|pair| pair.as_ref().map(|(enemy_move, _)| enemy_move))
    }
    
    /// Get the stored move and effects for a specific enemy
    pub fn get_enemy_move_and_effects(&self, enemy_index: usize) -> Option<(&EnemyMove, &Vec<Effect>)> {
        self.enemy_actions.get(enemy_index).and_then(|pair| pair.as_ref().map(|(enemy_move, effects)| (enemy_move, effects)))
    }
    
    /// Get all stored enemy moves
    pub(in crate::battle) fn get_all_enemy_moves(&self) -> Vec<Option<&EnemyMove>> {
        self.enemy_actions.iter().map(|pair| pair.as_ref().map(|(enemy_move, _)| enemy_move)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;
    use crate::battle::enemy_in_battle::EnemyInBattle;
    use crate::enemies::{red_louse::RedLouse, enemy_enum::EnemyEnum};
    use crate::game::{global_info::GlobalInfo, enemy::EnemyTrait};

    #[test]
    fn test_red_louse_curl_up_event_system() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Initially enemy should have 0 block
        assert_eq!(battle.enemies[0].battle_info.get_block(), 0);
        
        // Deal damage to the enemy to trigger curl up
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_dealt = battle.apply_damage(Entity::Enemy(0), 6);

        // Check that damage was dealt and curl up was triggered (enemy gained block)
        assert_eq!(damage_dealt, 6);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 6);
        
        // Curl up gives 3-7 block for ascension 0
        let curl_up_block = battle.enemies[0].battle_info.get_block();
        assert!(curl_up_block >= 3 && curl_up_block <= 7);
        
        // Deal damage again - curl up should not trigger a second time
        let hp_before_second_damage = battle.enemies[0].battle_info.get_hp();
        let second_damage = battle.apply_damage(Entity::Enemy(0), 4);

        // Calculate expected outcome based on curl up block amount
        let expected_damage = if curl_up_block >= 4 { 0 } else { 4 - curl_up_block };
        let expected_remaining_block = if curl_up_block >= 4 { curl_up_block - 4 } else { 0 };
        
        assert_eq!(second_damage, expected_damage);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), hp_before_second_damage - expected_damage);
        assert_eq!(battle.enemies[0].battle_info.get_block(), expected_remaining_block);
    }

    #[test]
    fn test_curl_up_ascension_scaling() {
        
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        
        // Test normal ascension (0-6): should give 3-7 block
        let normal_global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &normal_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut normal_battle = Battle::new(deck.clone(), normal_global_info, 80, 80, enemies, &mut rng);
        normal_battle.apply_damage(Entity::Enemy(0), 6);
        let normal_block = normal_battle.enemies[0].battle_info.get_block();
        assert!(normal_block >= 3 && normal_block <= 7);
        
        // Test mid ascension (7-16): should give 4-8 block
        let mid_global_info = GlobalInfo { ascention: 10, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &mid_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut mid_battle = Battle::new(deck.clone(), mid_global_info, 80, 80, enemies, &mut rng);
        mid_battle.apply_damage(Entity::Enemy(0), 6);
        let mid_block = mid_battle.enemies[0].battle_info.get_block();
        assert!(mid_block >= 4 && mid_block <= 8);
        
        // Test high ascension (17+): should give 9-12 block
        let high_global_info = GlobalInfo { ascention: 17, current_floor: 1 };
        let red_louse = RedLouse::instantiate(&mut rng, &high_global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::RedLouse(red_louse))];
        let mut high_battle = Battle::new(deck, high_global_info, 80, 80, enemies, &mut rng);
        high_battle.apply_damage(Entity::Enemy(0), 6);
        let high_block = high_battle.enemies[0].battle_info.get_block();
        assert!(high_block >= 9 && high_block <= 12);
    }
}