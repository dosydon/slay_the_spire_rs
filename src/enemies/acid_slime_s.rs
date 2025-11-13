use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}};

#[derive(Clone)]
pub struct AcidSlimeS {
    hp: u32,
    next_move_is_lick: bool,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AcidSlimeSMove {
    Lick,
    Tackle,
}

impl AcidSlimeS {
    pub fn new(hp: u32) -> Self {
        AcidSlimeS {
            hp,
            next_move_is_lick: true, // Always starts with Lick
        }
    }

    pub fn calculate_tackle_damage(global_info: &GlobalInfo) -> u32 {
        if global_info.ascention >= 2 {
            4
        } else {
            3
        }
    }

    pub fn calculate_hp_range(global_info: &GlobalInfo) -> (u32, u32) {
        if global_info.ascention >= 7 {
            (11, 15)
        } else {
            (8, 12)
        }
    }

    pub fn get_move_effects(&self, move_type: AcidSlimeSMove, global_info: &GlobalInfo) -> Vec<Effect> {
        match move_type {
            AcidSlimeSMove::Lick => {
                vec![Effect::ApplyWeak(1)]
            }
            AcidSlimeSMove::Tackle => {
                vec![Effect::AttackToTarget { 
                    amount: Self::calculate_tackle_damage(global_info), 
                    num_attacks: 1 
                }]
            }
        }
    }

    fn choose_next_move(&mut self) -> AcidSlimeSMove {
        // AcidSlimeS alternates between Lick and Tackle, starting with Lick
        if self.next_move_is_lick {
            self.next_move_is_lick = false;
            AcidSlimeSMove::Lick
        } else {
            self.next_move_is_lick = true;
            AcidSlimeSMove::Tackle
        }
    }
}

impl EnemyTrait for AcidSlimeS {
    type MoveType = AcidSlimeSMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let (hp_min, hp_max) = Self::calculate_hp_range(global_info);
        let hp = hp_min + rng.random_range(0..=(hp_max - hp_min));
        
        AcidSlimeS::new(hp)
    }

    fn get_name() -> String {
        "Acid Slime (S)".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (AcidSlimeSMove, Vec<Effect>) {
        let selected_move = self.choose_next_move();
        let effects = self.get_move_effects(selected_move, global_info);
        (selected_move, effects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::global_info::GlobalInfo;

    #[test]
    fn test_acid_slime_s_creation() {
        let acid_slime = AcidSlimeS::new(10);
        assert_eq!(acid_slime.hp, 10);
        assert!(acid_slime.next_move_is_lick);
    }

    #[test]
    fn test_acid_slime_s_ascension_scaling() {
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test damage scaling
        assert_eq!(AcidSlimeS::calculate_tackle_damage(&global_info_asc0), 3);
        assert_eq!(AcidSlimeS::calculate_tackle_damage(&global_info_asc2), 4);

        // Test HP scaling
        assert_eq!(AcidSlimeS::calculate_hp_range(&global_info_asc0), (8, 12));
        assert_eq!(AcidSlimeS::calculate_hp_range(&global_info_asc7), (11, 15));
    }

    #[test]
    fn test_acid_slime_s_move_alternation() {
        let mut acid_slime = AcidSlimeS::new(10);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let mut rng = rand::rng();

        // Should start with Lick
        let (move1, _effects1) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move1, AcidSlimeSMove::Lick);

        // Should then use Tackle
        let (move2, _effects2) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move2, AcidSlimeSMove::Tackle);

        // Should go back to Lick
        let (move3, _effects3) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move3, AcidSlimeSMove::Lick);

        // Should use Tackle again
        let (move4, _effects4) = acid_slime.choose_move_and_effects(&global_info, &mut rng);
        assert_eq!(move4, AcidSlimeSMove::Tackle);
    }

    #[test]
    fn test_acid_slime_s_instantiation() {
        let mut rng = rand::rng();
        let global_info_asc0 = GlobalInfo { ascention: 0, current_floor: 1 };
        let global_info_asc7 = GlobalInfo { ascention: 7, current_floor: 1 };

        // Test normal ascension instantiation
        let acid_slime_asc0 = AcidSlimeS::instantiate(&mut rng, &global_info_asc0);
        assert!(acid_slime_asc0.hp >= 8 && acid_slime_asc0.hp <= 12);
        assert!(acid_slime_asc0.next_move_is_lick);

        // Test high ascension instantiation
        let acid_slime_asc7 = AcidSlimeS::instantiate(&mut rng, &global_info_asc7);
        assert!(acid_slime_asc7.hp >= 11 && acid_slime_asc7.hp <= 15);
        assert!(acid_slime_asc7.next_move_is_lick);
    }

    #[test]
    fn test_acid_slime_s_effects() {
        let acid_slime = AcidSlimeS::new(10);
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };

        // Test Lick effects
        let lick_effects = acid_slime.get_move_effects(AcidSlimeSMove::Lick, &global_info);
        assert_eq!(lick_effects, vec![Effect::ApplyWeak(1)]);

        // Test Tackle effects
        let tackle_effects = acid_slime.get_move_effects(AcidSlimeSMove::Tackle, &global_info);
        assert_eq!(tackle_effects, vec![
            Effect::AttackToTarget { amount: 3, num_attacks: 1 }
        ]);

        // Test ascension damage scaling
        let global_info_asc2 = GlobalInfo { ascention: 2, current_floor: 1 };
        let tackle_effects_asc2 = acid_slime.get_move_effects(AcidSlimeSMove::Tackle, &global_info_asc2);
        assert_eq!(tackle_effects_asc2, vec![
            Effect::AttackToTarget { amount: 4, num_attacks: 1 }
        ]);
    }

    #[test]
    fn test_acid_slime_s_name() {
        assert_eq!(AcidSlimeS::get_name(), "Acid Slime (S)");
    }

    #[test]
    fn test_acid_slime_s_battle_integration() {
        use crate::battle::{Battle, enemy_in_battle::EnemyInBattle};
        use crate::enemies::EnemyEnum;
        use crate::cards::ironclad::starter_deck::starter_deck;
        
        let deck = starter_deck();
        let mut rng = rand::rng();
        let global_info = GlobalInfo { ascention: 0, current_floor: 1 };
        let acid_slime = AcidSlimeS::instantiate(&mut rng, &global_info);
        let enemies = vec![EnemyInBattle::new(EnemyEnum::AcidSlimeS(acid_slime))];
        let battle = Battle::new(deck, global_info, 80, 80, enemies, &mut rng);
        
        // Test that the enemy is properly set up
        assert_eq!(battle.get_enemies().len(), 1);
        assert!(battle.get_enemies()[0].battle_info.is_alive());
        let enemy_hp = battle.get_enemies()[0].battle_info.get_hp();
        assert!(enemy_hp >= 8 && enemy_hp <= 12);
        
        // Test move generation using a separate AcidSlimeS instance
        let mut test_slime = AcidSlimeS::instantiate(&mut rng, &global_info);
        let (enemy_move, effects) = test_slime.choose_move_and_effects(&global_info, &mut rng);
        
        // First move should be Lick (applies Weak)
        assert_eq!(enemy_move, AcidSlimeSMove::Lick);
        assert_eq!(effects, vec![Effect::ApplyWeak(1)]);
    }
}