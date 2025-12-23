use crate::game::enemy::EnemyTrait;
use crate::game::effect::BattleEffect;
use crate::game::global_info::GlobalInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlueSlaver {
    hp: u32,
    move_history: Vec<BlueSlaverMove>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlueSlaverMove {
    Stab,
    Rake,
}

impl BlueSlaver {
    /// Calculate Blue Slaver's HP based on ascension level
    /// Base: 46-50, A7+: 48-52
    pub fn calculate_hp(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 7 {
            rng.random_range(48..=52)
        } else {
            rng.random_range(46..=50)
        }
    }

    /// Calculate Stab damage based on ascension level
    /// Base: 12, A2+: 13
    pub fn calculate_stab_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            13
        } else {
            12
        }
    }

    /// Calculate Rake damage based on ascension level
    /// Base: 7, A2+: 8
    pub fn calculate_rake_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            8
        } else {
            7
        }
    }

    /// Calculate Rake weak stacks based on ascension level
    /// Base: 1, A17+: 2
    pub fn calculate_rake_weak(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 17 {
            2
        } else {
            1
        }
    }

    /// Get the effects for a given move
    pub fn get_move_effects(&self, move_type: BlueSlaverMove, global_info: &GlobalInfo) -> Vec<BattleEffect> {
        match move_type {
            BlueSlaverMove::Stab => {
                let damage = Self::calculate_stab_damage(global_info);
                vec![
                    BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                ]
            }
            BlueSlaverMove::Rake => {
                let damage = Self::calculate_rake_damage(global_info);
                let weak_duration = Self::calculate_rake_weak(global_info);
                vec![
                    BattleEffect::AttackToTarget { amount: damage, num_attacks: 1, strength_multiplier: 1 },
                    BattleEffect::ApplyWeak { duration: weak_duration },
                ]
            }
        }
    }

    /// Check if a move can be used based on move history and ascension level
    fn can_use_move(&self, move_type: BlueSlaverMove, global_info: &GlobalInfo) -> bool {
        let history_len = self.move_history.len();

        // Cannot use the same move 3 times in a row (all ascensions)
        if history_len >= 2
            && self.move_history[history_len - 1] == move_type
            && self.move_history[history_len - 2] == move_type {
            return false;
        }

        // A17+: Cannot use Rake twice in a row
        if global_info.ascention >= 17
            && move_type == BlueSlaverMove::Rake
            && history_len >= 1
            && self.move_history[history_len - 1] == BlueSlaverMove::Rake {
            return false;
        }

        true
    }
}

impl EnemyTrait for BlueSlaver {
    type MoveType = BlueSlaverMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let hp = Self::calculate_hp(rng, global_info);
        BlueSlaver {
            hp,
            move_history: Vec::new(),
        }
    }

    fn get_name() -> String {
        "Blue Slaver".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(
        &mut self,
        global_info: &GlobalInfo,
        rng: &mut impl rand::Rng,
    ) -> (BlueSlaverMove, Vec<BattleEffect>) {
        // Pattern: 40% chance to use Rake, 60% chance to use Stab
        // Cannot use the same move 3 times in a row
        // A17+: Cannot use Rake twice in a row

        let mut available_moves = Vec::new();
        let mut probabilities = Vec::new();

        // Check Rake (40% base chance)
        if self.can_use_move(BlueSlaverMove::Rake, global_info) {
            available_moves.push(BlueSlaverMove::Rake);
            probabilities.push(0.4);
        }

        // Check Stab (60% base chance)
        if self.can_use_move(BlueSlaverMove::Stab, global_info) {
            available_moves.push(BlueSlaverMove::Stab);
            probabilities.push(0.6);
        }

        // If only one move is available, use it
        let move_type = if available_moves.len() == 1 {
            available_moves[0]
        } else {
            // Normalize probabilities and choose
            let total: f64 = probabilities.iter().sum();
            let normalized: Vec<f64> = probabilities.iter().map(|p| p / total).collect();

            let roll = rng.random::<f64>();
            let mut cumulative = 0.0;
            let mut chosen_move = available_moves[0];

            for (i, prob) in normalized.iter().enumerate() {
                cumulative += prob;
                if roll < cumulative {
                    chosen_move = available_moves[i];
                    break;
                }
            }
            chosen_move
        };

        // Record the move in history
        self.move_history.push(move_type);

        let effects = self.get_move_effects(move_type, global_info);
        (move_type, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blue_slaver_hp_ranges() {
        let mut rng = rand::rng();

        // Base ascension: 46-50 HP
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        for _ in 0..20 {
            let slaver = BlueSlaver::instantiate(&mut rng, &global_info_base);
            assert!(slaver.get_hp() >= 46 && slaver.get_hp() <= 50);
        }

        // A7+: 48-52 HP
        let global_info_a7 = GlobalInfo { ascention: 7, current_floor: 1 };
        for _ in 0..20 {
            let slaver = BlueSlaver::instantiate(&mut rng, &global_info_a7);
            assert!(slaver.get_hp() >= 48 && slaver.get_hp() <= 52);
        }
    }

    #[test]
    fn test_stab_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_stab_damage(&global_info_base), 12);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_stab_damage(&global_info_a2), 13);
    }

    #[test]
    fn test_rake_damage_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_rake_damage(&global_info_base), 7);

        let global_info_a2 = GlobalInfo { ascention: 2, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_rake_damage(&global_info_a2), 8);
    }

    #[test]
    fn test_rake_weak_scaling() {
        let global_info_base = GlobalInfo { ascention: 0, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_rake_weak(&global_info_base), 1);

        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        assert_eq!(BlueSlaver::calculate_rake_weak(&global_info_a17), 2);
    }

    #[test]
    fn test_stab_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let slaver = BlueSlaver::instantiate(&mut rng, &global_info);

        let effects = slaver.get_move_effects(BlueSlaverMove::Stab, &global_info);

        // Should have Attack only
        assert_eq!(effects.len(), 1);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 12, .. }));
    }

    #[test]
    fn test_rake_effects() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let slaver = BlueSlaver::instantiate(&mut rng, &global_info);

        let effects = slaver.get_move_effects(BlueSlaverMove::Rake, &global_info);

        // Should have Attack and ApplyWeak
        assert_eq!(effects.len(), 2);
        assert!(matches!(effects[0], BattleEffect::AttackToTarget { amount: 7, .. }));
        assert!(matches!(effects[1], BattleEffect::ApplyWeak { duration: 1 }));
    }

    #[test]
    fn test_cannot_use_same_move_three_times() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = BlueSlaver::instantiate(&mut rng, &global_info);

        // Simulate using Stab twice
        slaver.move_history = vec![BlueSlaverMove::Stab, BlueSlaverMove::Stab];

        // Should not be able to use Stab a third time
        assert!(!slaver.can_use_move(BlueSlaverMove::Stab, &global_info));
        // But should be able to use Rake
        assert!(slaver.can_use_move(BlueSlaverMove::Rake, &global_info));
    }

    #[test]
    fn test_a17_cannot_use_rake_twice() {
        let mut rng = rand::rng();
        let global_info_a17 = GlobalInfo { ascention: 17, current_floor: 1 };
        let mut slaver = BlueSlaver::instantiate(&mut rng, &global_info_a17);

        // Simulate using Rake once
        slaver.move_history = vec![BlueSlaverMove::Rake];

        // Should not be able to use Rake again at A17+
        assert!(!slaver.can_use_move(BlueSlaverMove::Rake, &global_info_a17));
        // But should be able to use Stab
        assert!(slaver.can_use_move(BlueSlaverMove::Stab, &global_info_a17));
    }

    #[test]
    fn test_move_distribution() {
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut slaver = BlueSlaver::instantiate(&mut rng, &global_info);

        let mut rake_count = 0;
        let mut stab_count = 0;

        // Simulate 100 moves
        for _ in 0..100 {
            let (move_type, _) = slaver.choose_move_and_effects(&global_info, &mut rng);
            match move_type {
                BlueSlaverMove::Rake => rake_count += 1,
                BlueSlaverMove::Stab => stab_count += 1,
            }
        }

        // Should be roughly 40% Rake, 60% Stab (with some tolerance for randomness)
        // Allow for a wide range due to small sample size
        assert!(rake_count >= 20 && rake_count <= 60);
        assert!(stab_count >= 40 && stab_count <= 80);
    }
}
