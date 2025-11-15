//! Card enum for type-safe card references

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CardEnum {
    // Ironclad Cards
    Strike,
    Defend,
    Bash,
    Carnage,
    Cleave,
    Embrace,
    Flex,
    Inflame,
    IronWave,
    PommelStrike,
    ShrugItOff,
    TwinStrike,
    Clothesline,
    HeavyBlade,
    PerfectedStrike,
    Thunderclap,
    WildStrike,
    Combust,
    Disarm,
    FeelNoPain,
    Entrench,
    // Status Cards
    Slimed,
    Wound,
    // Add more cards as needed
}

impl CardEnum {
    /// Get the display name for this card
    pub fn name(&self) -> &'static str {
        match self {
            CardEnum::Strike => "Strike",
            CardEnum::Defend => "Defend",
            CardEnum::Bash => "Bash",
            CardEnum::Carnage => "Carnage",
            CardEnum::Cleave => "Cleave",
            CardEnum::Embrace => "Embrace",
            CardEnum::Flex => "Flex",
            CardEnum::Inflame => "Inflame",
            CardEnum::IronWave => "IronWave",
            CardEnum::PommelStrike => "PommelStrike",
            CardEnum::ShrugItOff => "ShrugItOff",
            CardEnum::TwinStrike => "TwinStrike",
            CardEnum::Clothesline => "Clothesline",
            CardEnum::HeavyBlade => "HeavyBlade",
            CardEnum::PerfectedStrike => "PerfectedStrike",
            CardEnum::Thunderclap => "Thunderclap",
            CardEnum::WildStrike => "WildStrike",
            CardEnum::Combust => "Combust",
            CardEnum::Disarm => "Disarm",
            CardEnum::FeelNoPain => "Feel No Pain",
            CardEnum::Entrench => "Entrench",
            CardEnum::Slimed => "Slimed",
            CardEnum::Wound => "Wound",
        }
    }
    
    /// Get the upgraded name for this card
    pub fn upgraded_name(&self) -> String {
        format!("{}+", self.name())
    }
}