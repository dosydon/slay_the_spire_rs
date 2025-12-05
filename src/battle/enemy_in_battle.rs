use crate::{enemies::enemy_enum::EnemyEnum, battle::character_battle_info::CharacterBattleInfo};

pub struct EnemyInBattle {
    pub enemy: EnemyEnum,
    pub battle_info: CharacterBattleInfo,
}

impl EnemyInBattle {
    pub fn new(enemy: EnemyEnum) -> Self {
        let hp = enemy.get_hp();
        let battle_info = CharacterBattleInfo::new_enemy(hp);

        EnemyInBattle {
            enemy,
            battle_info,
        }
    }

    /// Delegate damage to character (keeping this for the i32 -> u32 conversion)
    pub(in crate::battle) fn take_damage(&mut self, damage: i32) -> u32 {
        if damage <= 0 {
            return 0;
        }
        self.battle_info.take_damage(damage as u32)
    }

    /// Get current HP
    pub fn get_current_hp(&self) -> u32 {
        self.battle_info.get_current_hp()
    }

    /// Get weak turns
    pub fn get_weak(&self) -> u32 {
        self.battle_info.get_weak_turns()
    }

    /// Get vulnerable turns
    pub fn get_vulnerable(&self) -> u32 {
        self.battle_info.get_vulnerable_turns()
    }
}