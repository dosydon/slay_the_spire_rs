//! Card enum for type-safe card references

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardEnum {
    // Ironclad Cards
    Strike,
    Defend,
    Bash,
    Cleave,
    Flex,
    IronWave,
    PommelStrike,
    // Status Cards
    Slimed,
    // Add more cards as needed
}

impl CardEnum {
    /// Get the display name for this card
    pub fn name(&self) -> &'static str {
        match self {
            CardEnum::Strike => "Strike",
            CardEnum::Defend => "Defend",
            CardEnum::Bash => "Bash",
            CardEnum::Cleave => "Cleave",
            CardEnum::Flex => "Flex",
            CardEnum::IronWave => "IronWave",
            CardEnum::PommelStrike => "PommelStrike",
            CardEnum::Slimed => "Slimed",
        }
    }
    
    /// Get the upgraded name for this card
    pub fn upgraded_name(&self) -> String {
        format!("{}+", self.name())
    }
}