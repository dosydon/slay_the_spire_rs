use super::character_battle_info::CharacterBattleInfo;

#[derive(Debug)]
pub struct Player {
    pub battle_info: CharacterBattleInfo,
}

impl Player {
    pub fn new(current_hp: u32, max_hp: u32, energy: u32) -> Self {
        Player {
            battle_info: CharacterBattleInfo::new(current_hp, max_hp, energy),
        }
    }

    pub(in crate::battle) fn spend_energy(&mut self, amount: u32) -> bool {
        self.battle_info.spend_energy(amount)
    }

    pub fn get_energy(&self) -> u32 {
        self.battle_info.get_energy()
    }

    pub fn get_block(&self) -> u32 {
        self.battle_info.get_block()
    }

    pub fn gain_block(&mut self, amount: u32) {
        self.battle_info.gain_block(amount);
    }

    pub(in crate::battle) fn apply_vulnerable(&mut self, turns: u32) {
        self.battle_info.apply_vulnerable(turns);
    }

    pub(in crate::battle) fn is_vulnerable(&self) -> bool {
        self.battle_info.is_vulnerable()
    }
    
    pub fn at_start_of_turn(&mut self) {
        self.battle_info.at_start_of_turn();
        self.battle_info.energy = 3;
    }

    pub fn is_alive(&self) -> bool {
        self.battle_info.is_alive()
    }
}