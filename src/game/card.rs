use crate::game::card_type::CardType;
use crate::game::card_enum::CardEnum;
use crate::game::effect::{Effect, Condition};

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    card_enum: CardEnum,
    cost: u32,
    card_type: CardType,
    effects: Vec<Effect>,
    upgraded: bool,
    play_condition: Condition,
    ethereal: bool,
}

impl Card {
    pub fn new(card_enum: CardEnum, cost: u32, card_type: CardType, effects: Vec<Effect>, upgraded: bool, playable: bool) -> Self {
        let play_condition = if playable { Condition::True } else { Condition::False };
        Card {
            card_enum,
            cost,
            card_type,
            effects,
            upgraded,
            play_condition,
            ethereal: false,
        }
    }

    pub fn new_with_ethereal(card_enum: CardEnum, cost: u32, card_type: CardType, effects: Vec<Effect>, upgraded: bool, playable: bool, ethereal: bool) -> Self {
        let play_condition = if playable { Condition::True } else { Condition::False };
        Card {
            card_enum,
            cost,
            card_type,
            effects,
            upgraded,
            play_condition,
            ethereal,
        }
    }

    pub fn new_with_condition(card_enum: CardEnum, cost: u32, card_type: CardType, effects: Vec<Effect>, upgraded: bool, play_condition: Condition) -> Self {
        Card {
            card_enum,
            cost,
            card_type,
            effects,
            upgraded,
            play_condition,
            ethereal: false,
        }
    }

    pub fn get_name(&self) -> String {
        if self.upgraded {
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

    pub fn get_card_type(&self) -> &CardType {
        &self.card_type
    }

    pub fn get_effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }
    
    /// Upgrades this card to its improved version
    /// Returns a new Card instance with upgraded properties
    pub fn upgrade(self) -> Card {
        if self.upgraded {
            return self; // Already upgraded
        }
        
        // Delegate to individual card upgrade functions
        match self.card_enum {
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
            CardEnum::InfernalBlade => crate::cards::ironclad::infernal_blade::infernal_blade_upgraded(),
            CardEnum::Evolve => crate::cards::ironclad::evolve::evolve_upgraded(),
            CardEnum::Sentinel => self, // TODO: Implement sentinel_upgraded()
            CardEnum::Whirlwind => crate::cards::ironclad::whirlwind::whirlwind_upgraded(),
            CardEnum::DemonForm => crate::cards::ironclad::demon_form::demon_form_upgraded(),
            CardEnum::SecondWind => crate::cards::ironclad::second_wind::second_wind_upgraded(),
            CardEnum::Rupture => crate::cards::ironclad::rupture::rupture_upgraded(),
            CardEnum::Slimed => self, // Status cards don't upgrade
            CardEnum::Wound => self, // Status cards don't upgrade
            CardEnum::Burn => self, // Status cards don't upgrade
        }
    }
    
    /// Checks if this card is already upgraded
    pub fn is_upgraded(&self) -> bool {
        self.upgraded
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_creation() {
        let card = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }], false, true);
        assert_eq!(card.get_name(), "Strike");
        assert_eq!(card.get_cost(), 1);
        assert_eq!(matches!(card.get_card_type(), CardType::Attack), true);
        assert!(card.is_playable());
    }

    #[test]
    fn test_strike_upgrade() {
        let strike = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }], false, true);
        let upgraded = strike.upgrade();
        
        assert_eq!(upgraded.get_name(), "Strike+");
        assert_eq!(upgraded.get_cost(), 1); // Cost stays same
        assert_eq!(upgraded.get_card_type(), &CardType::Attack);
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
        let basic = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 6, num_attacks: 1, strength_multiplier: 1 }], false, true);
        let upgraded = Card::new(CardEnum::Strike, 1, CardType::Attack, vec![Effect::AttackToTarget { amount: 9, num_attacks: 1, strength_multiplier: 1 }], true, true);
        
        assert!(!basic.is_upgraded());
        assert!(upgraded.is_upgraded());
    }
}