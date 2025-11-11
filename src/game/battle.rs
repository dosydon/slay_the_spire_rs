use crate::{enemies::red_louse::RedLouse, game::{action::Action, card::Card, character_battle_info::CharacterBattleInfo, deck::Deck, effect::{Effect, EffectWithTarget}, enemy::{EnemyInGame, EnemyTrait}, target::Target, global_info::GlobalInfo}};

#[derive(Debug)]
pub struct Player {
    pub character: CharacterBattleInfo,
}

impl Player {
    pub fn new(hp: u32, energy: u32) -> Self {
        Player {
            character: CharacterBattleInfo::new(hp, energy),
        }
    }

    /// Delegate methods to character
    pub fn take_damage(&mut self, damage: u32) -> u32 {
        self.character.take_damage(damage)
    }

    pub fn gain_block(&mut self, amount: u32) {
        self.character.gain_block(amount);
    }

    pub fn spend_energy(&mut self, amount: u32) -> bool {
        self.character.spend_energy(amount)
    }

    pub fn get_hp(&self) -> u32 {
        self.character.get_hp()
    }

    pub fn get_energy(&self) -> u32 {
        self.character.get_energy()
    }

    pub fn get_block(&self) -> u32 {
        self.character.get_block()
    }

    pub fn apply_vulnerable(&mut self, turns: u32) {
        self.character.apply_vulnerable(turns);
    }

    pub fn is_vulnerable(&self) -> bool {
        self.character.is_vulnerable()
    }

    pub fn start_turn(&mut self) {
        self.character.start_turn();
        // Player gets 3 energy at start of turn
        self.character.energy = 3;
    }

    pub fn is_alive(&self) -> bool {
        self.character.is_alive()
    }
}

pub enum Phase {
    MainPhase,
    SelectEnemyPhase,
}

pub enum GameError {
    InvalidAction,
    NotEnoughEnergy,
    CardNotInHand,
}

pub struct Battle {
    player: Player,
    enemies: Vec<EnemyInGame>,
    hand: Vec<Card>,
    deck: Deck,
    phase: Phase,
}

impl Battle {
    pub fn new(deck: Deck, global_info: &GlobalInfo, rng: &mut impl rand::Rng) -> Self {
        let (deck, hand) = deck.initialize_game(rng);
        Battle {
            player: Player::new(80, 3),
            enemies: vec![RedLouse::instantiate(rng, global_info)],
            hand: hand,
            deck,
            phase: Phase::MainPhase,
        }
    }
    
    pub fn get_player(&self) -> &Player {
        &self.player
    }
    
    pub fn get_enemies(&self) -> &Vec<EnemyInGame> {
        &self.enemies
    }
    
    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
    
    pub fn get_phase(&self) -> &Phase {
        &self.phase
    }
    
    pub fn is_battle_over(&self) -> bool {
        !self.player.is_alive() || self.enemies.iter().all(|e| !e.battle_info.is_alive())
    }
    
    pub fn is_valid_target(&self, target: &Target) -> bool {
        match target {
            Target::Enemy(idx) => *idx < self.enemies.len(),
            Target::Player => true,  // Player is always a valid target
            Target::None => false,   // None is not a valid target
        }
    }
    
    pub fn eval_action(&mut self, action: Action) {
        match action {
            Action::PlayCard(idx, target) => {
                if idx < self.hand.len() && self.is_valid_target(&target) {
                    self.play_card(idx, target);
                }
            }
            Action::EndTurn => {
                self.end_turn();
            }
//            Action::SelectEnemy(idx) => {
//                if idx < self.enemies.len() {
//                    self.select_enemy(idx);
//                }
//            }
        }
    }

    pub fn play_card(&mut self, idx: usize, target: Target) {
        if idx >= self.hand.len() { return; }
        
        let card = &self.hand[idx];
        if !self.player.spend_energy(card.get_cost()) { return; }
        
        let card_effects = card.get_effects().clone();
        self.hand.remove(idx);
        
        for effect in card_effects {
            self.eval_effect_with_target(&EffectWithTarget::from_effect(effect, target));
        }
    }
    
    pub fn eval_effect_with_target(&mut self, effect: &EffectWithTarget) {
        match effect {
            EffectWithTarget::AttackToTarget { target, amount, num_attacks } => {
                for _ in 0..*num_attacks {
                    match target {
                        Target::Enemy(idx) => {
                            if *idx < self.enemies.len() {
                                let damage_with_strength = self.player.character.calculate_damage(*amount);
                                self.enemies[*idx].take_damage(damage_with_strength as i32);
                            }
                        },
                        Target::Player => {
                            // When enemies attack player, they use their own strength
                            // For now, since we don't track which entity is doing the attack,
                            // we'll just use the base damage. This could be improved later.
                            self.player.take_damage(*amount);
                        },
                        Target::None => {
                            // No target - effect does nothing
                        }
                    }
                }
            },
            EffectWithTarget::GainDefense { amount } => {
                // Defense effects always apply to the player
                self.player.gain_block(*amount);
            },
            EffectWithTarget::Vulnerable { duration } => {
                // Apply vulnerable to the first enemy for now
                // In the future, this should be targeted like damage
                if !self.enemies.is_empty() {
                    self.enemies[0].battle_info.apply_vulnerable(*duration);
                }
            },
            EffectWithTarget::GainStrength { amount } => {
                // Strength effects apply to the player for now
                // Enemy strength will be handled by enemy-specific logic
                self.player.character.gain_strength(*amount);
            },
        }
    }

    fn enemy_turn(&mut self, rng: &mut impl rand::Rng) {
        for (idx, enemy) in self.enemies.iter_mut().enumerate() {
            let mv = enemy.battle_info.choose_next_move(rng);
            let effects = enemy.get_move_effects(mv, &enemy.battle_info);
            for effect in effects {
                let with_target = match effect {
                    Effect::AttackToTarget { amount, num_attacks } => {
                        EffectWithTarget::AttackToTarget { target: Target::Player, amount, num_attacks }
                    }
                    Effect::GainStrength(amount) => {
                        // if enemy strength buff should apply to itself, adapt storage accordingly
                        EffectWithTarget::GainStrength { amount }
                    }
                    Effect::GainDefense(amount) => {
                        EffectWithTarget::GainDefense { amount }
                    }
                    Effect::Vulnerable(duration) => {
                        EffectWithTarget::Vulnerable { duration }
                    }
                };
                self.eval_effect_with_target(&with_target);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cards::ironclad::starter_deck::starter_deck;

    #[test]
    fn test_battle_initialization() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let battle = Battle::new(deck, &global_info, &mut rng);
        assert_eq!(battle.player.get_hp(), 80);
        assert_eq!(battle.player.get_block(), 0);
        assert_eq!(battle.player.get_energy(), 3);
        assert!(!battle.enemies.is_empty());
        
        println!("{:?}", battle.deck);
        println!("{:?}", battle.hand);
    }

    #[test]
    fn test_eval_effect_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = EffectWithTarget::AttackToTarget {
            target: Target::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        
        battle.eval_effect_with_target(&damage_effect);
        
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - 10);
    }

    #[test]
    fn test_play_card_with_target() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        let initial_energy = battle.player.get_energy();
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        
        // Find a Strike card in the hand
        let strike_idx = battle.hand.iter().position(|card| card.get_name() == "Strike");
        
        if let Some(idx) = strike_idx {
            // Play the Strike card targeting enemy 0
            let action = Action::PlayCard(idx, Target::Enemy(0));
            battle.eval_action(action);
            
            // Check that energy was spent and enemy took damage
            assert!(battle.player.get_energy() < initial_energy);
            assert!(battle.enemies[0].battle_info.get_hp() < initial_enemy_hp);
        } else {
            panic!("No Strike card found in hand");
        }
    }

    #[test]
    fn test_vulnerable_effect_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        // Apply vulnerable to enemy
        let vulnerable_effect = EffectWithTarget::Vulnerable { duration: 2 };
        battle.eval_effect_with_target(&vulnerable_effect);
        
        // Check that enemy is vulnerable
        assert!(battle.enemies[0].battle_info.is_vulnerable());
        assert_eq!(battle.enemies[0].battle_info.get_vulnerable_turns(), 2);
        
        // Apply damage - should be increased by 50%
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        
        let damage_effect = EffectWithTarget::AttackToTarget {
            target: Target::Enemy(0),
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_effect_with_target(&damage_effect);
        
        // 10 damage * 1.5 = 15 damage should be dealt (but capped by enemy's HP)
        let expected_damage = 15u32.min(initial_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - expected_damage);
    }

    #[test]
    fn test_character_block_integration() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        // Give enemy some block
        battle.enemies[0].battle_info.gain_block(5);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 5);
        
        let initial_hp = battle.enemies[0].battle_info.get_hp();
        let damage_effect = EffectWithTarget::AttackToTarget {
            target: Target::Enemy(0),
            amount: 8,
            num_attacks: 1,
        };
        battle.eval_effect_with_target(&damage_effect);
        
        // 8 damage - 5 block = 3 actual damage
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_hp - 3);
        assert_eq!(battle.enemies[0].battle_info.get_block(), 0);
    }

    #[test]
    fn test_damage_to_player() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        let initial_hp = battle.player.get_hp();
        
        // Create an effect that damages the player
        let damage_effect = EffectWithTarget::AttackToTarget {
            target: Target::Player,
            amount: 10,
            num_attacks: 1,
        };
        battle.eval_effect_with_target(&damage_effect);
        
        // Player should take 10 damage
        assert_eq!(battle.player.get_hp(), initial_hp - 10);
    }

    #[test]
    fn test_attack_with_strength() {
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut battle = Battle::new(deck, &global_info, &mut rng);
        
        // Give player some strength
        battle.player.character.gain_strength(3);
        assert_eq!(battle.player.character.get_strength(), 3);
        
        let initial_enemy_hp = battle.enemies[0].battle_info.get_hp();
        let attack_effect = EffectWithTarget::AttackToTarget {
            target: Target::Enemy(0),
            amount: 6,
            num_attacks: 1,
        };
        battle.eval_effect_with_target(&attack_effect);
        
        // 6 base damage + 3 strength = 9 total damage
        let expected_damage = 9u32.min(initial_enemy_hp);
        assert_eq!(battle.enemies[0].battle_info.get_hp(), initial_enemy_hp - expected_damage);
    }

}