pub mod burning_blood;
pub mod anchor;
pub mod blood_vial;
pub mod bag_of_marbles;
pub mod vajra;
pub mod lantern;
pub mod strawberry;
pub mod pear;
pub mod mercury_hourglass;
pub mod horn_cleat;
pub mod bronze_scales;
pub mod bag_of_preparation;
pub mod oddly_smooth_stone;
pub mod mango;
pub mod centennial_puzzle;
pub mod orichalcum;
pub mod nunchaku;
pub mod akabeko;
pub mod the_boot;
pub mod gremlin_horn;
pub mod happy_flower;
pub mod pen_nib;
pub mod art_of_war;
pub mod ink_bottle;
pub mod kunai;
pub mod letter_opener;

pub use burning_blood::BurningBloodRelic;
pub use anchor::AnchorRelic;
pub use blood_vial::BloodVialRelic;
pub use bag_of_marbles::BagOfMarblesRelic;
pub use vajra::VajraRelic;
pub use lantern::LanternRelic;
pub use strawberry::StrawberryRelic;
pub use pear::PearRelic;
pub use mercury_hourglass::MercuryHourglassRelic;
pub use horn_cleat::HornCleatRelic;
pub use bronze_scales::BronzeScalesRelic;
pub use bag_of_preparation::BagOfPreparationRelic;
pub use oddly_smooth_stone::OddlySmoothStoneRelic;
pub use mango::MangoRelic;
pub use centennial_puzzle::CentennialPuzzleRelic;
pub use orichalcum::OrichalcumRelic;
pub use nunchaku::NunchakuRelic;
pub use akabeko::AkabekoRelic;
pub use the_boot::TheBootRelic;
pub use gremlin_horn::GremlinHornRelic;
pub use happy_flower::HappyFlowerRelic;
pub use pen_nib::PenNibRelic;
pub use art_of_war::ArtOfWarRelic;
pub use ink_bottle::InkBottleRelic;
pub use kunai::KunaiRelic;
pub use letter_opener::LetterOpenerRelic;

#[derive(Debug, Clone, PartialEq)]
pub enum Relic {
    BurningBlood,
    Anchor,
    BloodVial,
    BagOfMarbles,
    Vajra,
    Lantern,
    Strawberry,
    Pear,
    MercuryHourglass,
    HornCleat,
    BronzeScales,
    BagOfPreparation,
    OddlySmoothStone,
    Mango,
    CentennialPuzzle,
    Orichalcum,
    Nunchaku,
    Akabeko,
    TheBoot,
    GremlinHorn,
    HappyFlower,
    PenNib,
    ArtOfWar,
    InkBottle,
    Kunai,
    LetterOpener,
}

impl Relic {
    /// Get the display name for this relic
    pub fn name(&self) -> &'static str {
        match self {
            Relic::BurningBlood => "Burning Blood",
            Relic::Anchor => "Anchor",
            Relic::BloodVial => "Blood Vial",
            Relic::BagOfMarbles => "Bag of Marbles",
            Relic::Vajra => "Vajra",
            Relic::Lantern => "Lantern",
            Relic::Strawberry => "Strawberry",
            Relic::Pear => "Pear",
            Relic::MercuryHourglass => "Mercury Hourglass",
            Relic::HornCleat => "Horn Cleat",
            Relic::BronzeScales => "Bronze Scales",
            Relic::BagOfPreparation => "Bag of Preparation",
            Relic::OddlySmoothStone => "Oddly Smooth Stone",
            Relic::Mango => "Mango",
            Relic::CentennialPuzzle => "Centennial Puzzle",
            Relic::Orichalcum => "Orichalcum",
            Relic::Nunchaku => "Nunchaku",
            Relic::Akabeko => "Akabeko",
            Relic::TheBoot => "The Boot",
            Relic::GremlinHorn => "Gremlin Horn",
            Relic::HappyFlower => "Happy Flower",
            Relic::PenNib => "Pen Nib",
            Relic::ArtOfWar => "Art of War",
            Relic::InkBottle => "Ink Bottle",
            Relic::Kunai => "Kunai",
            Relic::LetterOpener => "Letter Opener",
        }
    }

    /// Convert this relic to a game event listener
    pub fn to_game_event_listener(self) -> Option<Box<dyn crate::game::game_event::GameEventListener>> {
        match self {
            Relic::BurningBlood => Some(Box::new(BurningBloodRelic::new())),
            Relic::Strawberry => Some(Box::new(StrawberryRelic::new())),
            Relic::Pear => Some(Box::new(PearRelic::new())),
            Relic::Mango => Some(Box::new(MangoRelic::new())),
            _ => None,
        }
    }

    /// Convert this relic to a battle event listener
    pub fn to_battle_event_listener(self) -> Option<Box<dyn crate::battle::battle_events::EventListener>> {
        match self {
            Relic::Anchor => Some(Box::new(AnchorRelic::new(crate::battle::target::Entity::Player))),
            Relic::BloodVial => Some(Box::new(BloodVialRelic::new(crate::battle::target::Entity::Player))),
            Relic::BagOfMarbles => Some(Box::new(BagOfMarblesRelic::new(crate::battle::target::Entity::Player))),
            Relic::Vajra => Some(Box::new(VajraRelic::new(crate::battle::target::Entity::Player))),
            Relic::Lantern => Some(Box::new(LanternRelic::new(crate::battle::target::Entity::Player))),
            Relic::MercuryHourglass => Some(Box::new(MercuryHourglassRelic::new(crate::battle::target::Entity::Player))),
            Relic::HornCleat => Some(Box::new(HornCleatRelic::new(crate::battle::target::Entity::Player))),
            Relic::BronzeScales => Some(Box::new(BronzeScalesRelic::new(crate::battle::target::Entity::Player))),
            Relic::BagOfPreparation => Some(Box::new(BagOfPreparationRelic::new(crate::battle::target::Entity::Player))),
            Relic::OddlySmoothStone => Some(Box::new(OddlySmoothStoneRelic::new(crate::battle::target::Entity::Player))),
            Relic::CentennialPuzzle => Some(Box::new(CentennialPuzzleRelic::new(crate::battle::target::Entity::Player))),
            Relic::Orichalcum => Some(Box::new(OrichalcumRelic::new(crate::battle::target::Entity::Player))),
            Relic::Nunchaku => Some(Box::new(NunchakuRelic::new(crate::battle::target::Entity::Player))),
            Relic::Akabeko => Some(Box::new(AkabekoRelic::new(crate::battle::target::Entity::Player))),
            Relic::TheBoot => Some(Box::new(TheBootRelic::new(crate::battle::target::Entity::Player))),
            Relic::GremlinHorn => Some(Box::new(GremlinHornRelic::new(crate::battle::target::Entity::Player))),
            Relic::HappyFlower => Some(Box::new(HappyFlowerRelic::new(crate::battle::target::Entity::Player))),
            Relic::PenNib => Some(Box::new(PenNibRelic::new(crate::battle::target::Entity::Player))),
            Relic::ArtOfWar => Some(Box::new(ArtOfWarRelic::new(crate::battle::target::Entity::Player))),
            Relic::InkBottle => Some(Box::new(InkBottleRelic::new(crate::battle::target::Entity::Player))),
            Relic::Kunai => Some(Box::new(KunaiRelic::new(crate::battle::target::Entity::Player))),
            Relic::LetterOpener => Some(Box::new(LetterOpenerRelic::new(crate::battle::target::Entity::Player))),
            _ => None,
        }
    }

    /// Sample a random relic of the given rarity
    pub fn sample_relic(rarity: crate::game::reward_state::RelicRarity, rng: &mut impl rand::Rng) -> Self {
        use crate::game::reward_state::RelicRarity;

        // Define relic pools by rarity (using arrays instead of vecs)
        const COMMON_RELICS: [Relic; 4] = [
            Relic::Anchor,
            Relic::BagOfPreparation,
            Relic::BloodVial,
            Relic::TheBoot,
        ];

        const UNCOMMON_RELICS: [Relic; 8] = [
            Relic::BagOfMarbles,
            Relic::BronzeScales,
            Relic::HornCleat,
            Relic::Lantern,
            Relic::MercuryHourglass,
            Relic::OddlySmoothStone,
            Relic::PenNib,
            Relic::Vajra,
        ];

        const RARE_RELICS: [Relic; 10] = [
            Relic::Akabeko,
            Relic::ArtOfWar,
            Relic::CentennialPuzzle,
            Relic::GremlinHorn,
            Relic::HappyFlower,
            Relic::InkBottle,
            Relic::Kunai,
            Relic::LetterOpener,
            Relic::Nunchaku,
            Relic::Orichalcum,
        ];

        let relics = match rarity {
            RelicRarity::Common => &COMMON_RELICS[..],
            RelicRarity::Uncommon => &UNCOMMON_RELICS[..],
            RelicRarity::Rare => &RARE_RELICS[..],
        };

        // Sample a random relic from the pool
        relics[rng.random_range(0..relics.len())].clone()
    }
}