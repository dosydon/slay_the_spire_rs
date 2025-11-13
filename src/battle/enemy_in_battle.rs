use crate::{enemies::enemy_enum::EnemyEnum, battle::character_battle_info::CharacterBattleInfo};

pub struct EnemyInBattle {
    pub enemy: EnemyEnum,
    pub battle_info: CharacterBattleInfo,
}

impl EnemyInBattle {
    pub fn new(enemy: EnemyEnum) -> Self {
        let hp = enemy.get_hp();
        EnemyInBattle {
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