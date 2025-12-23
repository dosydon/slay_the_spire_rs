use crate::game::card_type::CardType;
use crate::game::card_enum::CardEnum;
use crate::game::effect::{Effect, Condition};

/// Card rarity for classification and reward generation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rarity {
    Basic,      // Strike, Defend - never in reward pools
    Common,     // Most frequent rewards (~75% of pool)
    Uncommon,   // Less frequent rewards (~20% of pool)
    Rare,       // Rare rewards (~5% of pool)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardClass {
    IronClad(Rarity, CardType),
    Colorless(Rarity, CardType),
    Status,
    Curse,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    card_enum: CardEnum,
    cost: u32,
    card_class: CardClass, // Combines card type, rarity, and character class
    effects: Vec<Effect>,
    upgrade_level: u32, // 0 = not upgraded, 1+ = upgraded level
    play_condition: Condition,
    ethereal: bool,
    on_exhaust: Option<Vec<Effect>>, // Effects that trigger when this card is exhausted
    end_of_turn: Option<Vec<Effect>>, // Effects that trigger at end of turn
    is_removable: bool, // Whether this card can be removed from the deck (false for Ascender's Bane, Curse of the Bell, Necronomicurse)
    is_innate: bool, // Whether this card is innate (starts in every hand and returns to hand after played)
}

impl Card {

    pub fn new(card_enum: CardEnum, cost: u32, card_class: CardClass, effects: Vec<Effect>) -> Self {
        Card {
            card_enum,
            cost,
            card_class,
            effects,
            upgrade_level: 0, // Default to not upgraded
            play_condition: Condition::True, // Default to playable
            ethereal: false,
            on_exhaust: None,
            end_of_turn: None,
            is_removable: true, // Default to removable
            is_innate: false, // Default to not innate
        }
    }

    /// Builder pattern method to set the upgrade level
    pub fn set_upgrade_level(mut self, upgrade_level: u32) -> Self {
        self.upgrade_level = upgrade_level;
        self
    }

    /// Builder pattern method to set whether the card is ethereal
    pub fn set_ethereal(mut self, ethereal: bool) -> Self {
        self.ethereal = ethereal;
        self
    }

    /// Builder pattern method to set the play condition
    pub fn set_play_condition(mut self, play_condition: Condition) -> Self {
        self.play_condition = play_condition;
        self
    }

    /// Builder pattern method to set on-exhaust effects
    pub fn set_on_exhaust(mut self, on_exhaust: Vec<Effect>) -> Self {
        self.on_exhaust = Some(on_exhaust);
        self
    }

    /// Builder pattern method to set end-of-turn effects
    pub fn set_end_of_turn(mut self, end_of_turn: Vec<Effect>) -> Self {
        self.end_of_turn = Some(end_of_turn);
        self
    }

    /// Convenience method to set upgradable from boolean
    pub fn set_upgraded(mut self, upgraded: bool) -> Self {
        if upgraded {
            self.upgrade_level = 1;
        } else {
            self.upgrade_level = 0;
        }
        self
    }

    /// Convenience method to set playable from boolean
    pub fn set_playable(mut self, playable: bool) -> Self {
        self.play_condition = if playable { Condition::True } else { Condition::False };
        self
    }

    /// Builder pattern method to set whether the card can be removed from the deck
    pub fn set_removable(mut self, is_removable: bool) -> Self {
        self.is_removable = is_removable;
        self
    }

    /// Builder pattern method to set whether the card is innate
    pub fn set_innate(mut self, is_innate: bool) -> Self {
        self.is_innate = is_innate;
        self
    }

    pub fn get_name(&self) -> String {
        if self.upgrade_level > 0 {
            self.card_enum.upgraded_name()
        } else {
            self.card_enum.name().to_string()
        }
    }
    
    pub fn get_card_enum(&self) -> CardEnum {
        self.card_enum
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    pub fn get_card_type(&self) -> CardType {
        match &self.card_class {
            CardClass::IronClad(_, card_type) => *card_type,
            CardClass::Colorless(_, card_type) => *card_type,
            CardClass::Status => CardType::Status,
            CardClass::Curse => CardType::Curse,
        }
    }

    pub fn get_card_class(&self) -> &CardClass {
        &self.card_class
    }

    pub fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn get_on_exhaust(&self) -> Option<&Vec<Effect>> {
        self.on_exhaust.as_ref()
    }

    pub fn get_end_of_turn(&self) -> Option<&Vec<Effect>> {
        self.end_of_turn.as_ref()
    }

    pub fn get_rarity(&self) -> Rarity {
        match &self.card_class {
            CardClass::IronClad(rarity, _) => *rarity,
            CardClass::Colorless(rarity, _) => *rarity,
            CardClass::Status => Rarity::Basic, // Status cards don't have traditional rarity
            CardClass::Curse => Rarity::Basic,  // Curse cards don't have traditional rarity
        }
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }
    
    /// Upgrades this card to its improved version
    /// Returns a new Card instance with upgraded properties
    pub fn upgrade(self) -> Card {
        if self.upgrade_level > 0 {
            return self; // Already upgraded
        }


        // Delegate to individual card upgrade functions and preserve rarity
        let upgraded_card = match self.card_enum {
            CardEnum::Strike => crate::cards::ironclad::strike::strike_upgraded(),
            CardEnum::Defend => crate::cards::ironclad::defend::defend_upgraded(),
            CardEnum::Bash => crate::cards::ironclad::bash::bash_upgraded(),
            CardEnum::Carnage => crate::cards::ironclad::carnage::carnage_upgraded(),
            CardEnum::Cleave => crate::cards::ironclad::cleave::cleave_upgraded(),
            CardEnum::Embrace => crate::cards::ironclad::embrace::embrace_upgraded(),
            CardEnum::Flex => crate::cards::ironclad::flex::flex_upgraded(),
            CardEnum::Inflame => crate::cards::ironclad::inflame::inflame_upgraded(),
            CardEnum::Immolate => crate::cards::ironclad::immolate::immolate_upgraded(),
            CardEnum::IronWave => crate::cards::ironclad::iron_wave::iron_wave_upgraded(),
            CardEnum::PommelStrike => crate::cards::ironclad::pommel_strike::pommel_strike_upgraded(),
            CardEnum::ShrugItOff => crate::cards::ironclad::shrug_it_off::shrug_it_off_upgraded(),
            CardEnum::TwinStrike => crate::cards::ironclad::twin_strike::twin_strike_upgraded(),
            CardEnum::Clothesline => crate::cards::ironclad::clothesline::clothesline_upgraded(),
            CardEnum::HeavyBlade => crate::cards::ironclad::heavy_blade::heavy_blade_upgraded(),
            CardEnum::PerfectedStrike => crate::cards::ironclad::perfected_strike::perfected_strike_upgraded(),
            CardEnum::Thunderclap => crate::cards::ironclad::thunderclap::thunderclap_upgraded(),
            CardEnum::WildStrike => crate::cards::ironclad::wild_strike::wild_strike_upgraded(),
            CardEnum::Combust => crate::cards::ironclad::combust::combust_upgraded(),
            CardEnum::Disarm => crate::cards::ironclad::disarm::disarm_upgraded(),
            CardEnum::Dropkick => crate::cards::ironclad::dropkick::dropkick_upgraded(),
            CardEnum::FeelNoPain => crate::cards::ironclad::feel_no_pain::feel_no_pain_upgraded(),
            CardEnum::Entrench => crate::cards::ironclad::entrench::entrench_upgraded(),
            CardEnum::Bludgeon => crate::cards::ironclad::bludgeon::bludgeon_upgraded(),
            CardEnum::Anger => crate::cards::ironclad::anger::anger_upgraded(),
            CardEnum::SwordBoomerang => crate::cards::ironclad::sword_boomerang::sword_boomerang_upgraded(),
            CardEnum::Hemokinesis => crate::cards::ironclad::hemokinesis::hemokinesis_upgraded(),
            CardEnum::Armaments => crate::cards::ironclad::armaments::armaments_upgraded(),
            CardEnum::Impervious => crate::cards::ironclad::impervious::impervious_upgraded(),
            CardEnum::Brutality => crate::cards::ironclad::brutality::brutality_upgraded(),
            CardEnum::Offering => crate::cards::ironclad::offering::offering_upgraded(),
            CardEnum::PowerThrough => crate::cards::ironclad::power_through::power_through_upgraded(),
            CardEnum::Shockwave => crate::cards::ironclad::shockwave::shockwave_upgraded(),
            CardEnum::Uppercut => crate::cards::ironclad::uppercut::uppercut_upgraded(),
            CardEnum::Intimidate => crate::cards::ironclad::intimidate::intimidate_upgraded(),
            CardEnum::SeeingRed => crate::cards::ironclad::seeing_red::seeing_red_upgraded(),
            CardEnum::GhostlyArmor => crate::cards::ironclad::ghostly_armor::ghostly_armor_upgraded(),
            CardEnum::Havoc => crate::cards::ironclad::havoc::havoc_upgraded(),
            CardEnum::Headbutt => crate::cards::ironclad::headbutt::headbutt_upgraded(),
            CardEnum::TrueGrit => crate::cards::ironclad::true_grit::true_grit_upgraded(),
            CardEnum::Warcry => crate::cards::ironclad::warcry::warcry_upgraded(),
            CardEnum::BodySlam => crate::cards::ironclad::body_slam::body_slam_upgraded(),
            CardEnum::Clash => crate::cards::ironclad::clash::clash_upgraded(),
            CardEnum::Corruption => crate::cards::ironclad::corruption::corruption_upgraded(),
            CardEnum::LimitBreak => crate::cards::ironclad::limit_break::limit_break_upgraded(),
            CardEnum::Metallicize => crate::cards::ironclad::metallicize::metallicize_upgraded(),
            CardEnum::FlameBarrier => crate::cards::ironclad::flame_barrier::flame_barrier_upgraded(),
            CardEnum::Rage => crate::cards::ironclad::rage::rage_upgraded(),
            CardEnum::Rampage => self, // TODO: Implement rampage_upgraded()
            CardEnum::RecklessCharge => crate::cards::ironclad::reckless_charge::reckless_charge_upgraded(),
            CardEnum::SearingBlow => self.upgrade_searing_blow(),
            CardEnum::SeverSoul => crate::cards::ironclad::sever_soul::sever_soul_upgraded(),
            CardEnum::SpotWeakness => crate::cards::ironclad::spot_weakness::spot_weakness_upgraded(),
            CardEnum::Pummel => crate::cards::ironclad::pummel::pummel_upgraded(),
            CardEnum::InfernalBlade => crate::cards::ironclad::infernal_blade::infernal_blade_upgraded(),
            CardEnum::Evolve => crate::cards::ironclad::evolve::evolve_upgraded(),
            CardEnum::Sentinel => self, // TODO: Implement sentinel_upgraded()
            CardEnum::Whirlwind => crate::cards::ironclad::whirlwind::whirlwind_upgraded(),
            CardEnum::DemonForm => crate::cards::ironclad::demon_form::demon_form_upgraded(),
            CardEnum::SecondWind => crate::cards::ironclad::second_wind::second_wind_upgraded(),
            CardEnum::Rupture => crate::cards::ironclad::rupture::rupture_upgraded(),
            CardEnum::DualWield => crate::cards::ironclad::dual_wield::dual_wield_upgraded(),
            CardEnum::DoubleTap => crate::cards::ironclad::double_tap::double_tap_upgraded(),
            CardEnum::Exhume => crate::cards::ironclad::exhume::exhume_upgraded(),
            CardEnum::Feed => crate::cards::ironclad::feed::feed_upgraded(),
            CardEnum::Reaper => crate::cards::ironclad::reaper::reaper_upgraded(),
            CardEnum::FiendFire => crate::cards::ironclad::fiend_fire::fiend_fire_upgraded(),
            CardEnum::FireBreathing => crate::cards::ironclad::fire_breathing::fire_breathing_upgraded(),
            CardEnum::Slimed => self, // Status cards don't upgrade
            CardEnum::Wound => self, // Status cards don't upgrade
            CardEnum::Burn => self, // Status cards don't upgrade
            CardEnum::Dazed => self, // Status cards don't upgrade

            // Colorless cards
            CardEnum::SwiftStrike => crate::cards::colorless::swift_strike::swift_strike_upgraded(),
            CardEnum::Finesse => crate::cards::colorless::finesse::finesse_upgraded(),
            CardEnum::FlashOfSteel => crate::cards::colorless::flash_of_steel::flash_of_steel_upgraded(),
            CardEnum::Blind => crate::cards::colorless::blind::blind_upgraded(),
            CardEnum::Trip => crate::cards::colorless::trip::trip_upgraded(),
            CardEnum::GoodInstincts => crate::cards::colorless::good_instincts::good_instincts_upgraded(),
            CardEnum::BandageUp => crate::cards::colorless::bandage_up::bandage_up_upgraded(),
            CardEnum::DeepBreath => crate::cards::colorless::deep_breath::deep_breath_upgraded(),
            CardEnum::MasterOfStrategy => crate::cards::colorless::master_of_strategy::master_of_strategy_upgraded(),
            CardEnum::DarkShackles => crate::cards::colorless::dark_shackles::dark_shackles_upgraded(),
            CardEnum::Impatience => crate::cards::colorless::impatience::impatience_upgraded(),
            CardEnum::AscendersCurse => crate::cards::curse::ascenders_curse(), // Curse cards don't have upgrades
            CardEnum::Injury => crate::cards::curse::injury(), // Curse cards don't have upgrades
            CardEnum::Clumsy => crate::cards::curse::clumsy(), // Curse cards don't have upgrades
            CardEnum::Regret => crate::cards::curse::regret(), // Curse cards don't have upgrades
            CardEnum::Writhe => crate::cards::curse::writhe(), // Curse cards don't have upgrades
        };

        upgraded_card
    }

    /// Special upgrade method for Searing Blow that supports multiple upgrade levels
    /// Uses quadratic progression: damage = n(n+7)/2+12 where n is upgrade level
    fn upgrade_searing_blow(self) -> Card {
        // Use the Searing Blow special upgrade system that tracks upgrade levels
        crate::cards::ironclad::searing_blow::upgrade_searing_blow_to_next_level(self)
    }

    /// Checks if this card is already upgraded
    pub fn is_upgraded(&self) -> bool {
        self.upgrade_level > 0
    }

    /// Gets the upgrade level of this card
    pub fn get_upgrade_level(&self) -> u32 {
        self.upgrade_level
    }

    /// Creates a new card with a specific upgrade level
    pub fn with_upgrade_level(self, upgrade_level: u32) -> Card {
        Card {
            card_enum: self.card_enum,
            cost: self.cost,
            card_class: self.card_class,
            effects: self.effects,
            upgrade_level,
            play_condition: self.play_condition,
            ethereal: self.ethereal,
            on_exhaust: self.on_exhaust,
            end_of_turn: self.end_of_turn,
            is_removable: self.is_removable,
            is_innate: self.is_innate,
        }
    }

    /// Gets the play condition for this card
    pub fn get_play_condition(&self) -> Condition {
        self.play_condition
    }

    /// Checks if this card is playable (for backward compatibility - always returns true for now since playability depends on context)
    pub fn is_playable(&self) -> bool {
        match self.play_condition {
            Condition::False => false,
            _ => true, // True, HandAllAttacks, etc. depend on context
        }
    }

    /// Checks if this card is ethereal (exhausts at end of turn if not played)
    pub fn is_ethereal(&self) -> bool {
        self.ethereal
    }

    /// Checks if this card can be removed from the deck
    /// Returns false for non-removable curses like Ascender's Bane, Curse of the Bell, and Necronomicurse
    pub fn is_removable(&self) -> bool {
        self.is_removable
    }

    /// Checks if this card is innate
    /// Innate cards start in every hand and return to hand after being played
    pub fn is_innate(&self) -> bool {
        self.is_innate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }]);
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(matches!(card.get_card_type(), CardType::Attack), true);
        assert!(card.is_playable());
    }

    #[test]
    fn test_strike_upgrade() {
        let strike = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }]);
        let upgraded = strike.upgrade();

        assert_eq!(upgraded.get_name(), "Strike+");
        assert_eq!(upgraded.get_cost(), 1); // Cost stays same
        assert_eq!(upgraded.get_card_type(), CardType::Attack);
        assert!(upgraded.is_upgraded());

        // Check damage increased to 9
        match &upgraded.get_effects()[0] {
            Effect::AttackToTarget { amount, num_attacks, strength_multiplier } => {
                assert_eq!(*amount, 9);
                assert_eq!(*num_attacks, 1);
                assert_eq!(*strength_multiplier, 1);
            }
            _ => panic!("Expected AttackToTarget effect"),
        }
    }

    #[test]
    fn test_is_upgraded() {
        let basic = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }]);
        let upgraded = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 }]).set_upgraded(true);

        assert!(!basic.is_upgraded());
        assert!(upgraded.is_upgraded());
    }

    #[test]
    fn test_card_removable_default() {
        let card = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }]);
        assert!(card.is_removable()); // Default should be removable
    }

    #[test]
    fn test_card_set_non_removable() {
        let card = Card::new(CardEnum::Strike, 1, CardClass::Curse, vec![])
            .set_removable(false);
        assert!(!card.is_removable()); // Should be non-removable
    }

    #[test]
    fn test_card_removable_preserved_in_upgrade() {
        let non_removable_card = Card::new(CardEnum::Strike, 1, CardClass::Curse, vec![])
            .set_removable(false);

        let upgraded = non_removable_card.with_upgrade_level(1);
        assert!(!upgraded.is_removable()); // Should preserve non-removable status
    }

    #[test]
    fn test_card_innate_default() {
        let card = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }]);
        assert!(!card.is_innate()); // Default should be not innate
    }

    #[test]
    fn test_card_set_innate() {
        let card = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }])
            .set_innate(true);
        assert!(card.is_innate()); // Should be innate
    }

    #[test]
    fn test_card_innate_preserved_in_upgrade() {
        let innate_card = Card::new(CardEnum::Strike, 1, CardClass::IronClad(Rarity::Basic, CardType::Attack), vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }])
            .set_innate(true);

        let upgraded = innate_card.with_upgrade_level(1);
        assert!(upgraded.is_innate()); // Should preserve innate status
    }
}