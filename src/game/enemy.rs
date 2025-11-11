use crate::{enemies::enemy_enum::EnemyEnum, battle::character_battle_info::CharacterBattleInfo, game::global_info::GlobalInfo, utils::CategoricalDistribution};

pub struct EnemyInGame {
    pub enemy: EnemyEnum,
    pub battle_info: CharacterBattleInfo,
}

impl EnemyInGame {
    pub fn new(enemy: EnemyEnum, hp: u32) -> Self {
        EnemyInGame {
            enemy,
            battle_info: CharacterBattleInfo::new_enemy(hp),
        }
    }

    /// Delegate damage to character (keeping this for the i32 -> u32 conversion)
    pub fn take_damage(&mut self, damage: i32) -> u32 {
        if damage <= 0 {
            return 0;
        }
        self.battle_info.take_damage(damage as u32)
    }
}

pub trait EnemyTrait {
    type MoveType;
    fn instantiate(rng: &mut impl rand::Rng, _global_info: &GlobalInfo) -> Self;
    fn hp_lb() -> u32;
    fn hp_ub() -> u32;
    fn choose_next_move(&self, global_info: &GlobalInfo) -> CategoricalDistribution<Self::MoveType>;
    fn get_name() -> String;
}