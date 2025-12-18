use crate::{enemies::enemy_enum::EnemyEnum, battle::character_battle_info::CharacterBattleInfo};

#[derive(Clone)]
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

    /// Get enemy name
    pub fn get_name(&self) -> String {
        match &self.enemy {
            EnemyEnum::RedLouse(_) => "Red Louse".to_string(),
            EnemyEnum::GreenLouse(_) => "Green Louse".to_string(),
            EnemyEnum::JawWorm(_) => "Jaw Worm".to_string(),
            EnemyEnum::Cultist(_) => "Cultist".to_string(),
            EnemyEnum::SpikeSlimeS(_) => "Spike Slime (S)".to_string(),
            EnemyEnum::SpikeSlimeM(_) => "Spike Slime (M)".to_string(),
            EnemyEnum::SpikeSlimeL(_) => "Spike Slime (L)".to_string(),
            EnemyEnum::AcidSlimeS(_) => "Acid Slime (S)".to_string(),
            EnemyEnum::AcidSlimeM(_) => "Acid Slime (M)".to_string(),
            EnemyEnum::AcidSlimeL(_) => "Acid Slime (L)".to_string(),
            EnemyEnum::GremlinNob(_) => "Gremlin Nob".to_string(),
            EnemyEnum::Lagavulin(_) => "Lagavulin".to_string(),
            EnemyEnum::Sentry(_) => "Sentry".to_string(),
            EnemyEnum::FatGremlin(_) => "Fat Gremlin".to_string(),
            EnemyEnum::SneakyGremlin(_) => "Sneaky Gremlin".to_string(),
            EnemyEnum::MadGremlin(_) => "Mad Gremlin".to_string(),
            EnemyEnum::ShieldGremlin(_) => "Shield Gremlin".to_string(),
            EnemyEnum::GremlinWizard(_) => "Gremlin Wizard".to_string(),
            EnemyEnum::Looter(_) => "Looter".to_string(),
            EnemyEnum::FungiBeast(_) => "Fungi Beast".to_string(),
            EnemyEnum::BlueSlaver(_) => "Blue Slaver".to_string(),
            EnemyEnum::RedSlaver(_) => "Red Slaver".to_string(),
        }
    }
}