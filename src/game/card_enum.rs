//! Card enum for type-safe card references

use crate::game::card_reward::Rarity;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CardEnum {
    // Ironclad Cards
    Strike,
    Defend,
    Bash,
    BodySlam,
    Clash,
    Carnage,
    Cleave,
    Embrace,
    Flex,
    Inflame,
    Immolate,
    IronWave,
    PommelStrike,
    PowerThrough,
    ShrugItOff,
    TwinStrike,
    Clothesline,
    HeavyBlade,
    PerfectedStrike,
    Thunderclap,
    WildStrike,
    Combust,
    Disarm,
    Dropkick,
    FeelNoPain,
    Entrench,
    Bludgeon,
    Anger,
    SwordBoomerang,
    Hemokinesis,
    Armaments,
    Impervious,
    Brutality,
    Offering,
    Shockwave,
    Uppercut,
    Intimidate,
    SeeingRed,
    GhostlyArmor,
    Havoc,
    Headbutt,
    TrueGrit,
    Warcry,
    Corruption,
    LimitBreak,
    Metallicize,
    FlameBarrier,
    Rage,
    Rampage,
    RecklessCharge,
    SearingBlow,
    SeverSoul,
    SpotWeakness,
    Pummel,
    InfernalBlade,
    Evolve,
    Sentinel,
    Whirlwind,
    DemonForm,
    SecondWind,
    Rupture,
    DualWield,
    DoubleTap,
    Exhume,
    Feed,
    Reaper,
    FiendFire,
    FireBreathing,
    // Status Cards
    Slimed,
    Wound,
    Burn,
    Dazed,

    // Curse Cards
    AscendersCurse,
    Injury,
    Clumsy,
    Regret,
    Writhe,

    // Colorless Cards
    SwiftStrike,
    Finesse,
    FlashOfSteel,
    Blind,
    Trip,
    GoodInstincts,
    BandageUp,
    DeepBreath,
    MasterOfStrategy,
    DarkShackles,
    Impatience,
    PanicButton,
    Panacea,
    DramaticEntrance,
    // Add more cards as needed
}

impl CardEnum {
    /// Get the display name for this card
    pub fn name(&self) -> &'static str {
        match self {
            CardEnum::Strike => "Strike",
            CardEnum::Defend => "Defend",
            CardEnum::Bash => "Bash",
            CardEnum::BodySlam => "Body Slam",
            CardEnum::Clash => "Clash",
            CardEnum::Carnage => "Carnage",
            CardEnum::Cleave => "Cleave",
            CardEnum::Embrace => "Embrace",
            CardEnum::Flex => "Flex",
            CardEnum::Inflame => "Inflame",
            CardEnum::Immolate => "Immolate",
            CardEnum::IronWave => "IronWave",
            CardEnum::PommelStrike => "PommelStrike",
            CardEnum::PowerThrough => "Power Through",
            CardEnum::ShrugItOff => "ShrugItOff",
            CardEnum::TwinStrike => "TwinStrike",
            CardEnum::Clothesline => "Clothesline",
            CardEnum::HeavyBlade => "HeavyBlade",
            CardEnum::PerfectedStrike => "PerfectedStrike",
            CardEnum::Thunderclap => "Thunderclap",
            CardEnum::WildStrike => "WildStrike",
            CardEnum::Combust => "Combust",
            CardEnum::Disarm => "Disarm",
            CardEnum::Dropkick => "Dropkick",
            CardEnum::FeelNoPain => "Feel No Pain",
            CardEnum::Entrench => "Entrench",
            CardEnum::Bludgeon => "Bludgeon",
            CardEnum::Anger => "Anger",
            CardEnum::SwordBoomerang => "Sword Boomerang",
            CardEnum::Hemokinesis => "Hemokinesis",
            CardEnum::Armaments => "Armaments",
            CardEnum::Impervious => "Impervious",
            CardEnum::Brutality => "Brutality",
            CardEnum::Offering => "Offering",
            CardEnum::Shockwave => "Shockwave",
            CardEnum::Uppercut => "Uppercut",
            CardEnum::Intimidate => "Intimidate",
            CardEnum::SeeingRed => "Seeing Red",
            CardEnum::GhostlyArmor => "Ghostly Armor",
            CardEnum::Havoc => "Havoc",
            CardEnum::Headbutt => "Headbutt",
            CardEnum::TrueGrit => "True Grit",
            CardEnum::Warcry => "Warcry",
            CardEnum::Corruption => "Corruption",
            CardEnum::LimitBreak => "Limit Break",
            CardEnum::Metallicize => "Metallicize",
            CardEnum::FlameBarrier => "Flame Barrier",
            CardEnum::Rage => "Rage",
            CardEnum::Rampage => "Rampage",
            CardEnum::RecklessCharge => "Reckless Charge",
            CardEnum::SearingBlow => "Searing Blow",
            CardEnum::SeverSoul => "Sever Soul",
            CardEnum::SpotWeakness => "Spot Weakness",
            CardEnum::Pummel => "Pummel",
            CardEnum::InfernalBlade => "Infernal Blade",
            CardEnum::Evolve => "Evolve",
            CardEnum::Sentinel => "Sentinel",
            CardEnum::Whirlwind => "Whirlwind",
            CardEnum::DemonForm => "Demon Form",
            CardEnum::SecondWind => "Second Wind",
            CardEnum::Rupture => "Rupture",
            CardEnum::DualWield => "Dual Wield",
            CardEnum::DoubleTap => "Double Tap",
            CardEnum::Exhume => "Exhume",
            CardEnum::Feed => "Feed",
            CardEnum::Reaper => "Reaper",
            CardEnum::FiendFire => "Fiend Fire",
            CardEnum::FireBreathing => "Fire Breathing",
            CardEnum::Slimed => "Slimed",
            CardEnum::Wound => "Wound",
            CardEnum::Burn => "Burn",
            CardEnum::Dazed => "Dazed",
            CardEnum::AscendersCurse => "Ascender's Curse",
            CardEnum::Injury => "Injury",
            CardEnum::Clumsy => "Clumsy",
            CardEnum::Regret => "Regret",
            CardEnum::Writhe => "Writhe",
            CardEnum::SwiftStrike => "Swift Strike",
            CardEnum::Finesse => "Finesse",
            CardEnum::FlashOfSteel => "Flash of Steel",
            CardEnum::Blind => "Blind",
            CardEnum::Trip => "Trip",
            CardEnum::GoodInstincts => "Good Instincts",
            CardEnum::BandageUp => "Bandage Up",
            CardEnum::DeepBreath => "Deep Breath",
            CardEnum::MasterOfStrategy => "Master of Strategy",
            CardEnum::DarkShackles => "Dark Shackles",
            CardEnum::Impatience => "Impatience",
            CardEnum::PanicButton => "Panic Button",
            CardEnum::Panacea => "Panacea",
            CardEnum::DramaticEntrance => "Dramatic Entrance",
        }
    }

    /// Get the upgraded name for this card
    pub fn upgraded_name(&self) -> String {
        format!("{}+", self.name())
    }

    /// Get the rarity of this card (if rewardable)
    /// Returns None for basic cards, status cards, and curse cards
    /// This references the actual card definition's CardClass to get the rarity
    pub fn rarity(&self) -> Option<Rarity> {
        let card = self.to_card();
        let card_rarity = card.get_rarity();

        // Convert from card::Rarity to card_reward::Rarity
        // Only return rarity if the card is rewardable (not Basic, Status, or Curse)
        use super::card::Rarity as CardRarity;
        match card_rarity {
            CardRarity::Basic => None,
            CardRarity::Common => Some(Rarity::Common),
            CardRarity::Uncommon => Some(Rarity::Uncommon),
            CardRarity::Rare => Some(Rarity::Rare),
        }
    }

    /// Check if this card can appear in reward pools
    /// Basic cards, status cards, and curse cards should not be rewards
    pub fn is_rewardable(&self) -> bool {
        self.rarity().is_some()
    }

    /// Get all Ironclad Attack cards
    /// Returns a vector of all CardEnums that are Ironclad Attack cards
    pub fn all_ironclad_attacks() -> Vec<CardEnum> {
        use crate::game::card_type::CardType;

        // Get all Ironclad cards
        let all_ironclad = vec![
            CardEnum::Strike, CardEnum::Defend, CardEnum::Bash,
            CardEnum::BodySlam, CardEnum::Clash, CardEnum::Carnage,
            CardEnum::Cleave, CardEnum::Embrace, CardEnum::Flex,
            CardEnum::Inflame, CardEnum::Immolate, CardEnum::IronWave,
            CardEnum::PommelStrike, CardEnum::PowerThrough, CardEnum::ShrugItOff,
            CardEnum::TwinStrike, CardEnum::Clothesline, CardEnum::HeavyBlade,
            CardEnum::PerfectedStrike, CardEnum::Thunderclap, CardEnum::WildStrike,
            CardEnum::Combust, CardEnum::Disarm, CardEnum::Dropkick,
            CardEnum::FeelNoPain, CardEnum::Entrench, CardEnum::Bludgeon,
            CardEnum::Anger, CardEnum::SwordBoomerang, CardEnum::Hemokinesis,
            CardEnum::Armaments, CardEnum::Impervious, CardEnum::Brutality,
            CardEnum::Offering, CardEnum::Shockwave, CardEnum::Uppercut,
            CardEnum::Intimidate, CardEnum::SeeingRed, CardEnum::GhostlyArmor,
            CardEnum::Havoc, CardEnum::Headbutt, CardEnum::TrueGrit,
            CardEnum::Warcry, CardEnum::Corruption, CardEnum::LimitBreak,
            CardEnum::Metallicize, CardEnum::FlameBarrier, CardEnum::Rage,
            CardEnum::Rampage, CardEnum::RecklessCharge, CardEnum::SearingBlow,
            CardEnum::SeverSoul, CardEnum::SpotWeakness, CardEnum::Pummel,
            CardEnum::InfernalBlade, CardEnum::Evolve, CardEnum::Sentinel,
            CardEnum::Whirlwind, CardEnum::DemonForm, CardEnum::SecondWind,
            CardEnum::Rupture, CardEnum::DualWield, CardEnum::DoubleTap,
            CardEnum::Exhume, CardEnum::Feed, CardEnum::Reaper,
            CardEnum::FiendFire, CardEnum::FireBreathing,
        ];

        // Filter to only Attack cards
        all_ironclad.into_iter()
            .filter(|card_enum| {
                let card = card_enum.to_card();
                card.get_card_type() == CardType::Attack
            })
            .collect()
    }

    pub fn all_ironclad_skills() -> Vec<CardEnum> {
        use crate::game::card_type::CardType;

        // Get all Ironclad cards
        let all_ironclad = vec![
            CardEnum::Strike, CardEnum::Defend, CardEnum::Bash,
            CardEnum::BodySlam, CardEnum::Clash, CardEnum::Carnage,
            CardEnum::Cleave, CardEnum::Embrace, CardEnum::Flex,
            CardEnum::Inflame, CardEnum::Immolate, CardEnum::IronWave,
            CardEnum::PommelStrike, CardEnum::PowerThrough, CardEnum::ShrugItOff,
            CardEnum::TwinStrike, CardEnum::Clothesline, CardEnum::HeavyBlade,
            CardEnum::PerfectedStrike, CardEnum::Thunderclap, CardEnum::WildStrike,
            CardEnum::Combust, CardEnum::Disarm, CardEnum::Dropkick,
            CardEnum::FeelNoPain, CardEnum::Entrench, CardEnum::Bludgeon,
            CardEnum::Anger, CardEnum::SwordBoomerang, CardEnum::Hemokinesis,
            CardEnum::Armaments, CardEnum::Impervious, CardEnum::Brutality,
            CardEnum::Offering, CardEnum::Shockwave, CardEnum::Uppercut,
            CardEnum::Intimidate, CardEnum::SeeingRed, CardEnum::GhostlyArmor,
            CardEnum::Havoc, CardEnum::Headbutt, CardEnum::TrueGrit,
            CardEnum::Warcry, CardEnum::Corruption, CardEnum::LimitBreak,
            CardEnum::Metallicize, CardEnum::FlameBarrier, CardEnum::Rage,
            CardEnum::Rampage, CardEnum::RecklessCharge, CardEnum::SearingBlow,
            CardEnum::SeverSoul, CardEnum::SpotWeakness, CardEnum::Pummel,
            CardEnum::InfernalBlade, CardEnum::Evolve, CardEnum::Sentinel,
            CardEnum::Whirlwind, CardEnum::DemonForm, CardEnum::SecondWind,
            CardEnum::Rupture, CardEnum::DualWield, CardEnum::DoubleTap,
            CardEnum::Exhume, CardEnum::Feed, CardEnum::Reaper,
            CardEnum::FiendFire, CardEnum::FireBreathing,
        ];

        // Filter to only Skill cards
        all_ironclad.into_iter()
            .filter(|card_enum| {
                let card = card_enum.to_card();
                card.get_card_type() == CardType::Skill
            })
            .collect()
    }

    /// Create a Card instance from this CardEnum
    /// This provides a centralized way to get card definitions
    pub fn to_card(&self) -> super::card::Card {
        match self {
            CardEnum::Strike => crate::cards::ironclad::strike::strike(),
            CardEnum::Defend => crate::cards::ironclad::defend::defend(),
            CardEnum::Bash => crate::cards::ironclad::bash::bash(),
            CardEnum::BodySlam => crate::cards::ironclad::body_slam::body_slam(),
            CardEnum::Clash => crate::cards::ironclad::clash::clash(),
            CardEnum::Carnage => crate::cards::ironclad::carnage::carnage(),
            CardEnum::Cleave => crate::cards::ironclad::cleave::cleave(),
            CardEnum::Embrace => crate::cards::ironclad::embrace::embrace(),
            CardEnum::Flex => crate::cards::ironclad::flex::flex(),
            CardEnum::Inflame => crate::cards::ironclad::inflame::inflame(),
            CardEnum::Immolate => crate::cards::ironclad::immolate::immolate(),
            CardEnum::IronWave => crate::cards::ironclad::iron_wave::iron_wave(),
            CardEnum::PommelStrike => crate::cards::ironclad::pommel_strike::pommel_strike(),
            CardEnum::PowerThrough => crate::cards::ironclad::power_through::power_through(),
            CardEnum::ShrugItOff => crate::cards::ironclad::shrug_it_off::shrug_it_off(),
            CardEnum::TwinStrike => crate::cards::ironclad::twin_strike::twin_strike(),
            CardEnum::Clothesline => crate::cards::ironclad::clothesline::clothesline(),
            CardEnum::HeavyBlade => crate::cards::ironclad::heavy_blade::heavy_blade(),
            CardEnum::PerfectedStrike => crate::cards::ironclad::perfected_strike::perfected_strike(),
            CardEnum::Thunderclap => crate::cards::ironclad::thunderclap::thunderclap(),
            CardEnum::WildStrike => crate::cards::ironclad::wild_strike::wild_strike(),
            CardEnum::Combust => crate::cards::ironclad::combust::combust(),
            CardEnum::Disarm => crate::cards::ironclad::disarm::disarm(),
            CardEnum::Dropkick => crate::cards::ironclad::dropkick::dropkick(),
            CardEnum::FeelNoPain => crate::cards::ironclad::feel_no_pain::feel_no_pain(),
            CardEnum::Entrench => crate::cards::ironclad::entrench::entrench(),
            CardEnum::Bludgeon => crate::cards::ironclad::bludgeon::bludgeon(),
            CardEnum::Anger => crate::cards::ironclad::anger::anger(),
            CardEnum::SwordBoomerang => crate::cards::ironclad::sword_boomerang::sword_boomerang(),
            CardEnum::Hemokinesis => crate::cards::ironclad::hemokinesis::hemokinesis(),
            CardEnum::Armaments => crate::cards::ironclad::armaments::armaments(),
            CardEnum::Impervious => crate::cards::ironclad::impervious::impervious(),
            CardEnum::Brutality => crate::cards::ironclad::brutality::brutality(),
            CardEnum::Offering => crate::cards::ironclad::offering::offering(),
            CardEnum::Shockwave => crate::cards::ironclad::shockwave::shockwave(),
            CardEnum::Uppercut => crate::cards::ironclad::uppercut::uppercut(),
            CardEnum::Intimidate => crate::cards::ironclad::intimidate::intimidate(),
            CardEnum::SeeingRed => crate::cards::ironclad::seeing_red::seeing_red(),
            CardEnum::GhostlyArmor => crate::cards::ironclad::ghostly_armor::ghostly_armor(),
            CardEnum::Havoc => crate::cards::ironclad::havoc::havoc(),
            CardEnum::Headbutt => crate::cards::ironclad::headbutt::headbutt(),
            CardEnum::TrueGrit => crate::cards::ironclad::true_grit::true_grit(),
            CardEnum::Warcry => crate::cards::ironclad::warcry::warcry(),
            CardEnum::Corruption => crate::cards::ironclad::corruption::corruption(),
            CardEnum::LimitBreak => crate::cards::ironclad::limit_break::limit_break(),
            CardEnum::Metallicize => crate::cards::ironclad::metallicize::metallicize(),
            CardEnum::FlameBarrier => crate::cards::ironclad::flame_barrier::flame_barrier(),
            CardEnum::Rage => crate::cards::ironclad::rage::rage(),
            CardEnum::Rampage => crate::cards::ironclad::rampage::rampage(),
            CardEnum::RecklessCharge => crate::cards::ironclad::reckless_charge::reckless_charge(),
            CardEnum::SearingBlow => crate::cards::ironclad::searing_blow::searing_blow(),
            CardEnum::SeverSoul => crate::cards::ironclad::sever_soul::sever_soul(),
            CardEnum::SpotWeakness => crate::cards::ironclad::spot_weakness::spot_weakness(),
            CardEnum::Pummel => crate::cards::ironclad::pummel::pummel(),
            CardEnum::InfernalBlade => crate::cards::ironclad::infernal_blade::infernal_blade(),
            CardEnum::Evolve => crate::cards::ironclad::evolve::evolve(),
            CardEnum::Sentinel => crate::cards::ironclad::sentinel::sentinel(),
            CardEnum::Whirlwind => crate::cards::ironclad::whirlwind::whirlwind(),
            CardEnum::DemonForm => crate::cards::ironclad::demon_form::demon_form(),
            CardEnum::SecondWind => crate::cards::ironclad::second_wind::second_wind(),
            CardEnum::Rupture => crate::cards::ironclad::rupture::rupture(),
            CardEnum::DualWield => crate::cards::ironclad::dual_wield::dual_wield(),
            CardEnum::DoubleTap => crate::cards::ironclad::double_tap::double_tap(),
            CardEnum::Exhume => crate::cards::ironclad::exhume::exhume(),
            CardEnum::Feed => crate::cards::ironclad::feed::feed(),
            CardEnum::Reaper => crate::cards::ironclad::reaper::reaper(),
            CardEnum::FiendFire => crate::cards::ironclad::fiend_fire::fiend_fire(),
            CardEnum::FireBreathing => crate::cards::ironclad::fire_breathing::fire_breathing(),

            // Status Cards
            CardEnum::Slimed => crate::cards::status::slimed::slimed(),
            CardEnum::Wound => crate::cards::status::wound::wound(),
            CardEnum::Burn => crate::cards::status::burn::burn(),
            CardEnum::Dazed => crate::cards::status::dazed::dazed(),

            // Curse Cards
            CardEnum::AscendersCurse => crate::cards::curse::ascenders_curse(),
            CardEnum::Injury => crate::cards::curse::injury(),
            CardEnum::Clumsy => crate::cards::curse::clumsy(),
            CardEnum::Regret => crate::cards::curse::regret(),
            CardEnum::Writhe => crate::cards::curse::writhe(),

            // Colorless Cards
            CardEnum::SwiftStrike => crate::cards::colorless::swift_strike::swift_strike(),
            CardEnum::Finesse => crate::cards::colorless::finesse::finesse(),
            CardEnum::FlashOfSteel => crate::cards::colorless::flash_of_steel::flash_of_steel(),
            CardEnum::Blind => crate::cards::colorless::blind::blind(),
            CardEnum::Trip => crate::cards::colorless::trip::trip(),
            CardEnum::GoodInstincts => crate::cards::colorless::good_instincts::good_instincts(),
            CardEnum::BandageUp => crate::cards::colorless::bandage_up::bandage_up(),
            CardEnum::DeepBreath => crate::cards::colorless::deep_breath::deep_breath(),
            CardEnum::MasterOfStrategy => crate::cards::colorless::master_of_strategy::master_of_strategy(),
            CardEnum::DarkShackles => crate::cards::colorless::dark_shackles::dark_shackles(),
            CardEnum::Impatience => crate::cards::colorless::impatience::impatience(),
            CardEnum::PanicButton => crate::cards::colorless::panic_button::panic_button(),
            CardEnum::Panacea => crate::cards::colorless::panacea::panacea(),
            CardEnum::DramaticEntrance => crate::cards::colorless::dramatic_entrance::dramatic_entrance(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_ironclad_attacks() {
        let attacks = CardEnum::all_ironclad_attacks();

        // Print for debugging
        println!("\nTotal Ironclad Attack cards: {}", attacks.len());
        for attack in &attacks {
            println!("  - {}", attack.name());
        }

        // Should have significantly more than the old hardcoded list of 17
        assert!(attacks.len() > 17, "Should have more than 17 attack cards, got {}", attacks.len());

        // Verify all returned cards are actually Attacks
        use crate::game::card_type::CardType;
        for attack_enum in &attacks {
            let card = attack_enum.to_card();
            assert_eq!(card.get_card_type(), CardType::Attack,
                      "{} should be an Attack card", attack_enum.name());
        }
    }
}
