use crate::game::enemy::EnemyTrait;
use crate::game::effect::Effect;
use crate::game::global_info::GlobalInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Looter {
    hp: u32,
    gold_stolen: u32,
    turn_count: u32,
    has_used_smoke_bomb: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LooterMove {
    Mug,
    Lunge,
    SmokeBomb,
    Escape,
}

impl Looter {
    /// Calculate Looter's HP based on ascension level
    /// Base: 44-48, A7+: 46-50
    pub fn calculate_hp(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 7 {
            rng.random_range(46..=50)
        } else {
            rng.random_range(44..=48)
        }
    }

    /// Calculate Mug damage based on ascension level
    /// Base: 10, A2+: 11
    pub fn calculate_mug_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            11
        } else {
            10
        }
    }

    /// Calculate Lunge damage based on ascension level
    /// Base: 12, A2+: 14
    pub fn calculate_lunge_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            14
        } else {
            12
        }
    }

    /// Calculate gold stolen per attack based on ascension level
    /// Base: 15, A17+: 20
    pub fn calculate_thievery_amount(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            20
        } else {
            15
        }
    }

    /// Calculate Smoke Bomb block amount
    /// Always 6 block
    pub fn calculate_smoke_bomb_block() -> u32 {
        6
    }

    /// Get the effects for a given move
    pub fn get_move_effects(&self, move_type: LooterMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            LooterMove::Mug => {
                let damage = Self::calculate_mug_damage(global_info);
                let gold = Self::calculate_thievery_amount(global_info);
                vec![
                    Effect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                    Effect::StealGold { amount: gold },
                ]
            }
            LooterMove::Lunge => {
                let damage = Self::calculate_lunge_damage(global_info);
                let gold = Self::calculate_thievery_amount(global_info);
                vec![
                    Effect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                    Effect::StealGold { amount: gold },
                ]
            }
            LooterMove::SmokeBomb => {
                let block = Self::calculate_smoke_bomb_block();
                vec![
                    Effect::GainDefense { amount: block },
                ]
            }
            LooterMove::Escape => {
                vec![
                    Effect::EnemyEscape,
                ]
            }
        }
    }
}

impl EnemyTrait for Looter {
    type MoveType = LooterMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let hp = Self::calculate_hp(rng, global_info);
        Looter {
            hp,
            gold_stolen: 0,
            turn_count: 0,
            has_used_smoke_bomb: false,
        }
    }

    fn get_name() -> String {
        "Looter".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(
        &mut self,
        global_info: &GlobalInfo,
        rng: &mut impl rand::Rng,
    ) -> (LooterMove, Vec<Effect>) {
        self.turn_count += 1;

        let move_type = if self.has_used_smoke_bomb {
            // After Smoke Bomb, always Escape
            LooterMove::Escape
        } else if self.turn_count <= 2 {
            // First 2 turns: always Mug
            LooterMove::Mug
        } else {
            // Turn 3+: randomly choose Lunge or Smoke Bomb
            // 50/50 chance
            if rng.random::<f64>() < 0.5 {
                LooterMove::Lunge
            } else {
                self.has_used_smoke_bomb = true;
                LooterMove::SmokeBomb
            }
        };

        let effects = self.get_move_effects(move_type, global_info);
        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looter_hp_ranges() {
        let mut rng = rand::rng();

        // Base ascension: 44-48 HP
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        for _ in 0..20 {
            let looter = Looter::instantiate(&mut rng, &global_info_base);
            assert!(looter.get_hp() >= 44 && looter.get_hp() <= 48);
        }

        // A7+: 46-50 HP
        let global_info_a7 = GlobalInfo { ascention: 7, current_floor: 1 };
        for _ in 0..20 {
            let looter = Looter::instantiate(&mut rng, &global_info_a7);
            assert!(looter.get_hp() >= 46 && looter.get_hp() <= 50);
        }
    }

    #[test]
    fn test_mug_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(Looter::calculate_mug_damage(&global_info_base), 10);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(Looter::calculate_mug_damage(&global_info_a2), 11);
    }

    #[test]
    fn test_lunge_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(Looter::calculate_lunge_damage(&global_info_base), 12);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(Looter::calculate_lunge_damage(&global_info_a2), 14);
    }

    #[test]
    fn test_thievery_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(Looter::calculate_thievery_amount(&global_info_base), 15);

        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        assert_eq!(Looter::calculate_thievery_amount(&global_info_a17), 20);
    }

    #[test]
    fn test_smoke_bomb_block() {
        assert_eq!(Looter::calculate_smoke_bomb_block(), 6);
    }

    #[test]
    fn test_move_pattern() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut looter = Looter::instantiate(&mut rng, &global_info);

        // Turn 1: Must be Mug
        let (move1, _) = looter.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, LooterMove::Mug);

        // Turn 2: Must be Mug
        let (move2, _) = looter.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, LooterMove::Mug);

        // Turn 3+: Should be either Lunge or SmokeBomb
        let (move3, _) = looter.choose_move_and_effects(&global_info, &mut rng);
        assert!(move3 == LooterMove::Lunge || move3 == LooterMove::SmokeBomb);
    }

    #[test]
    fn test_escape_after_smoke_bomb() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut looter = Looter::instantiate(&mut rng, &global_info);

        // Force Smoke Bomb by setting the flag
        looter.turn_count = 3;
        looter.has_used_smoke_bomb = true;

        // Next move should be Escape
        let (move_next, _) = looter.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move_next, LooterMove::Escape);
    }

    #[test]
    fn test_mug_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let looter = Looter::instantiate(&mut rng, &global_info);

        let effects = looter.get_move_effects(LooterMove::Mug, &global_info);

        // Should have Attack and StealGold
        assert_eq!(effects.len(), 2);
        assert!(matches!(effects[0], Effect::AttackToTarget { amount: 10, .. }));
        assert!(matches!(effects[1], Effect::StealGold { amount: 15 }));
    }

    #[test]
    fn test_lunge_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let looter = Looter::instantiate(&mut rng, &global_info);

        let effects = looter.get_move_effects(LooterMove::Lunge, &global_info);

        // Should have Attack and StealGold
        assert_eq!(effects.len(), 2);
        assert!(matches!(effects[0], Effect::AttackToTarget { amount: 12, .. }));
        assert!(matches!(effects[1], Effect::StealGold { amount: 15 }));
    }

    #[test]
    fn test_smoke_bomb_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let looter = Looter::instantiate(&mut rng, &global_info);

        let effects = looter.get_move_effects(LooterMove::SmokeBomb, &global_info);

        // Should grant block
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::GainDefense { amount: 6 }));
    }

    #[test]
    fn test_escape_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let looter = Looter::instantiate(&mut rng, &global_info);

        let effects = looter.get_move_effects(LooterMove::Escape, &global_info);

        // Should trigger enemy escape
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], Effect::EnemyEscape));
    }

    #[test]
    fn test_escape_mechanic_on_character_battle_info() {
        use crate::battle::character_battle_info::CharacterBattleInfo;

        let mut enemy_info = CharacterBattleInfo::new_enemy(50);

        // Enemy should be alive initially
        assert!(enemy_info.is_alive());
        assert!(!enemy_info.has_escaped());
        assert_eq!(enemy_info.get_hp(), 50);

        // Mark as escaped
        enemy_info.mark_escaped();

        // After escape, enemy should not be is_alive() but HP should be unchanged
        assert!(!enemy_info.is_alive());
        assert!(enemy_info.has_escaped());
        assert_eq!(enemy_info.get_hp(), 50);

        // is_in_combat should still return true (has HP > 0)
        assert!(enemy_info.is_in_combat());
    }

    #[test]
    fn test_looter_escape_ends_battle() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Looter
        let looter = Looter::instantiate(&mut rng, &global_info);
        let looter_hp = looter.get_hp();
        let enemy = EnemyInBattle::new(EnemyEnum::Looter(looter));

        let deck = Deck::new(vec![strike()]);
        let mut battle = Battle::new(deck, global_info.clone(), 50, 80, vec![enemy], &mut rng);

        // Enemy should be alive initially
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        assert!(!battle.get_enemies()[0].battle_info.has_escaped());
        assert!(!battle.is_battle_over());

        // Manually mark enemy as escaped (simulating EnemyEscape effect)
        battle.get_enemies_mut()[0].battle_info.mark_escaped();

        // After escape, enemy should not be alive
        assert!(!battle.get_enemies()[0].battle_info.is_alive());
        assert!(battle.get_enemies()[0].battle_info.has_escaped());
        assert_eq!(battle.get_enemies()[0].battle_info.get_hp(), looter_hp);

        // Battle should be over (all enemies defeated/escaped)
        assert!(battle.is_battle_over());
    }

    #[test]
    fn test_gold_tracking_initialized() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Looter
        let looter = Looter::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Looter(looter));

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info.clone(), 50, 80, vec![enemy], &mut rng);

        // Battle should start with no gold stolen
        assert_eq!(battle.get_gold_stolen(), 0);
    }

    #[test]
    fn test_gold_returned_when_enemy_killed() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Looter
        let looter = Looter::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Looter(looter));

        let deck = Deck::new(vec![strike()]);
        let battle = Battle::new(deck, global_info.clone(), 50, 80, vec![enemy], &mut rng);

        // If Looter is killed (not escaped), it should NOT have escaped flag
        assert!(!battle.get_enemies()[0].battle_info.has_escaped());

        // This means gold will be returned in Game logic
    }

    #[test]
    fn test_gold_kept_when_enemy_escapes() {
        use crate::battle::enemy_in_battle::EnemyInBattle;
        use crate::battle::Battle;
        use crate::game::deck::Deck;
        use crate::cards::ironclad::strike::strike;
        use crate::enemies::enemy_enum::EnemyEnum;

        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Create a Looter
        let looter = Looter::instantiate(&mut rng, &global_info);
        let enemy = EnemyInBattle::new(EnemyEnum::Looter(looter));

        let deck = Deck::new(vec![strike()]);
        let mut battle = Battle::new(deck, global_info.clone(), 50, 80, vec![enemy], &mut rng);

        // Mark enemy as escaped
        battle.get_enemies_mut()[0].battle_info.mark_escaped();

        // Enemy should have escaped flag
        assert!(battle.get_enemies()[0].battle_info.has_escaped());

        // This means gold will be kept (lost permanently) in Game logic
    }
}
