use crate::battle::{battle_events::{BattleEvent, EventListener}, target::Entity};
use crate::game::effect::BattleEffect;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};

// Relic imports
use crate::relics::{
    akabeko::AkabekoRelic,
    anchor::AnchorRelic,
    art_of_war::ArtOfWarRelic,
    bag_of_marbles::BagOfMarblesRelic,
    bag_of_preparation::BagOfPreparationRelic,
    blood_vial::BloodVialRelic,
    bronze_scales::BronzeScalesRelic,
    centennial_puzzle::CentennialPuzzleRelic,
    gremlin_horn::GremlinHornRelic,
    happy_flower::HappyFlowerRelic,
    horn_cleat::HornCleatRelic,
    ink_bottle::InkBottleRelic,
    kunai::KunaiRelic,
    lantern::LanternRelic,
    letter_opener::LetterOpenerRelic,
    mercury_hourglass::MercuryHourglassRelic,
    nunchaku::NunchakuRelic,
    oddly_smooth_stone::OddlySmoothStoneRelic,
    orichalcum::OrichalcumRelic,
    ornamental_fan::OrnamentalFanRelic,
    pen_nib::PenNibRelic,
    red_mask::RedMaskRelic,
    shuriken::ShurikenRelic,
    the_boot::TheBootRelic,
    vajra::VajraRelic,
};

// Power card imports
use crate::cards::ironclad::{
    brutality::BrutalityListener,
    combust::CombustListener,
    demon_form::DemonFormListener,
    double_tap::DoubleTapListener,
    embrace::EmbraceListener,
    feel_no_pain::FeelNoPainListener,
    fire_breathing::FireBreathingListener,
    flame_barrier::FlameBarrierListener,
    flex::LoseStrengthListener,
    metallicize::MetallicizeListener,
    rage::RageListener,
    rupture::RuptureListener,
};

// Enemy imports
use crate::enemies::{
    acid_slime_l::AcidSlimeLSplitListener,
    cultist::GrantRitualNextTurnListener,
    fungi_beast::SporeCloudListener,
    gremlin_nob::EnrageListener,
    lagavulin::LagavulinListener,
    mad_gremlin::AngryListener,
    red_louse::CurlUpListener,
    sentry::SentryListener,
    spike_slime_l::SpikeSlimeLSplitListener,
};

// Battle listener imports
use crate::battle::listeners::regen::RegenListener;

/// Unified enum for all EventListener implementations
/// This allows Clone and Hash to be derived automatically
#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventListenerEnum {
    // Relics (25)
    Akabeko(AkabekoRelic),
    Anchor(AnchorRelic),
    ArtOfWar(ArtOfWarRelic),
    BagOfMarbles(BagOfMarblesRelic),
    BagOfPreparation(BagOfPreparationRelic),
    BloodVial(BloodVialRelic),
    BronzeScales(BronzeScalesRelic),
    CentennialPuzzle(CentennialPuzzleRelic),
    GremlinHorn(GremlinHornRelic),
    HappyFlower(HappyFlowerRelic),
    HornCleat(HornCleatRelic),
    InkBottle(InkBottleRelic),
    Kunai(KunaiRelic),
    Lantern(LanternRelic),
    LetterOpener(LetterOpenerRelic),
    MercuryHourglass(MercuryHourglassRelic),
    Nunchaku(NunchakuRelic),
    OddlySmoothStone(OddlySmoothStoneRelic),
    Orichalcum(OrichalcumRelic),
    OrnamentalFan(OrnamentalFanRelic),
    PenNib(PenNibRelic),
    RedMask(RedMaskRelic),
    Shuriken(ShurikenRelic),
    TheBoot(TheBootRelic),
    Vajra(VajraRelic),

    // Powers (12)
    Brutality(BrutalityListener),
    Combust(CombustListener),
    DemonForm(DemonFormListener),
    DoubleTap(DoubleTapListener),
    Embrace(EmbraceListener),
    FeelNoPain(FeelNoPainListener),
    FireBreathing(FireBreathingListener),
    FlameBarrier(FlameBarrierListener),
    LoseStrength(LoseStrengthListener),
    Metallicize(MetallicizeListener),
    Rage(RageListener),
    Rupture(RuptureListener),

    // Enemy abilities (9)
    AcidSlimeLSplit(AcidSlimeLSplitListener),
    GrantRitualNextTurn(GrantRitualNextTurnListener),
    SporeCloud(SporeCloudListener),
    Enrage(EnrageListener),
    Lagavulin(LagavulinListener),
    Angry(AngryListener),
    CurlUp(CurlUpListener),
    Sentry(SentryListener),
    SpikeSlimeLSplit(SpikeSlimeLSplitListener),

    // Battle listeners (1)
    Regen(RegenListener),
}

impl EventListener for EventListenerEnum {
    fn on_event(&mut self, event: &BattleEvent) -> Vec<BattleEffect> {
        match self {
            // Relics
            EventListenerEnum::Akabeko(l) => l.on_event(event),
            EventListenerEnum::Anchor(l) => l.on_event(event),
            EventListenerEnum::ArtOfWar(l) => l.on_event(event),
            EventListenerEnum::BagOfMarbles(l) => l.on_event(event),
            EventListenerEnum::BagOfPreparation(l) => l.on_event(event),
            EventListenerEnum::BloodVial(l) => l.on_event(event),
            EventListenerEnum::BronzeScales(l) => l.on_event(event),
            EventListenerEnum::CentennialPuzzle(l) => l.on_event(event),
            EventListenerEnum::GremlinHorn(l) => l.on_event(event),
            EventListenerEnum::HappyFlower(l) => l.on_event(event),
            EventListenerEnum::HornCleat(l) => l.on_event(event),
            EventListenerEnum::InkBottle(l) => l.on_event(event),
            EventListenerEnum::Kunai(l) => l.on_event(event),
            EventListenerEnum::Lantern(l) => l.on_event(event),
            EventListenerEnum::LetterOpener(l) => l.on_event(event),
            EventListenerEnum::MercuryHourglass(l) => l.on_event(event),
            EventListenerEnum::Nunchaku(l) => l.on_event(event),
            EventListenerEnum::OddlySmoothStone(l) => l.on_event(event),
            EventListenerEnum::Orichalcum(l) => l.on_event(event),
            EventListenerEnum::OrnamentalFan(l) => l.on_event(event),
            EventListenerEnum::PenNib(l) => l.on_event(event),
            EventListenerEnum::RedMask(l) => l.on_event(event),
            EventListenerEnum::Shuriken(l) => l.on_event(event),
            EventListenerEnum::TheBoot(l) => l.on_event(event),
            EventListenerEnum::Vajra(l) => l.on_event(event),

            // Powers
            EventListenerEnum::Brutality(l) => l.on_event(event),
            EventListenerEnum::Combust(l) => l.on_event(event),
            EventListenerEnum::DemonForm(l) => l.on_event(event),
            EventListenerEnum::DoubleTap(l) => l.on_event(event),
            EventListenerEnum::Embrace(l) => l.on_event(event),
            EventListenerEnum::FeelNoPain(l) => l.on_event(event),
            EventListenerEnum::FireBreathing(l) => l.on_event(event),
            EventListenerEnum::FlameBarrier(l) => l.on_event(event),
            EventListenerEnum::LoseStrength(l) => l.on_event(event),
            EventListenerEnum::Metallicize(l) => l.on_event(event),
            EventListenerEnum::Rage(l) => l.on_event(event),
            EventListenerEnum::Rupture(l) => l.on_event(event),

            // Enemy abilities
            EventListenerEnum::AcidSlimeLSplit(l) => l.on_event(event),
            EventListenerEnum::GrantRitualNextTurn(l) => l.on_event(event),
            EventListenerEnum::SporeCloud(l) => l.on_event(event),
            EventListenerEnum::Angry(l) => l.on_event(event),
            EventListenerEnum::Lagavulin(l) => l.on_event(event),
            EventListenerEnum::Enrage(l) => l.on_event(event),
            EventListenerEnum::CurlUp(l) => l.on_event(event),
            EventListenerEnum::Sentry(l) => l.on_event(event),
            EventListenerEnum::SpikeSlimeLSplit(l) => l.on_event(event),

            // Battle listeners
            EventListenerEnum::Regen(l) => l.on_event(event),
        }
    }

    fn is_active(&self) -> bool {
        match self {
            // Relics
            EventListenerEnum::Akabeko(l) => l.is_active(),
            EventListenerEnum::Anchor(l) => l.is_active(),
            EventListenerEnum::ArtOfWar(l) => l.is_active(),
            EventListenerEnum::BagOfMarbles(l) => l.is_active(),
            EventListenerEnum::BagOfPreparation(l) => l.is_active(),
            EventListenerEnum::BloodVial(l) => l.is_active(),
            EventListenerEnum::BronzeScales(l) => l.is_active(),
            EventListenerEnum::CentennialPuzzle(l) => l.is_active(),
            EventListenerEnum::GremlinHorn(l) => l.is_active(),
            EventListenerEnum::HappyFlower(l) => l.is_active(),
            EventListenerEnum::HornCleat(l) => l.is_active(),
            EventListenerEnum::InkBottle(l) => l.is_active(),
            EventListenerEnum::Kunai(l) => l.is_active(),
            EventListenerEnum::Lantern(l) => l.is_active(),
            EventListenerEnum::LetterOpener(l) => l.is_active(),
            EventListenerEnum::MercuryHourglass(l) => l.is_active(),
            EventListenerEnum::Nunchaku(l) => l.is_active(),
            EventListenerEnum::OddlySmoothStone(l) => l.is_active(),
            EventListenerEnum::Orichalcum(l) => l.is_active(),
            EventListenerEnum::OrnamentalFan(l) => l.is_active(),
            EventListenerEnum::PenNib(l) => l.is_active(),
            EventListenerEnum::RedMask(l) => l.is_active(),
            EventListenerEnum::Shuriken(l) => l.is_active(),
            EventListenerEnum::TheBoot(l) => l.is_active(),
            EventListenerEnum::Vajra(l) => l.is_active(),

            // Powers
            EventListenerEnum::Brutality(l) => l.is_active(),
            EventListenerEnum::Combust(l) => l.is_active(),
            EventListenerEnum::DemonForm(l) => l.is_active(),
            EventListenerEnum::DoubleTap(l) => l.is_active(),
            EventListenerEnum::Embrace(l) => l.is_active(),
            EventListenerEnum::FeelNoPain(l) => l.is_active(),
            EventListenerEnum::FireBreathing(l) => l.is_active(),
            EventListenerEnum::FlameBarrier(l) => l.is_active(),
            EventListenerEnum::LoseStrength(l) => l.is_active(),
            EventListenerEnum::Metallicize(l) => l.is_active(),
            EventListenerEnum::Rage(l) => l.is_active(),
            EventListenerEnum::Rupture(l) => l.is_active(),

            // Enemy abilities
            EventListenerEnum::AcidSlimeLSplit(l) => l.is_active(),
            EventListenerEnum::GrantRitualNextTurn(l) => l.is_active(),
            EventListenerEnum::SporeCloud(l) => l.is_active(),
            EventListenerEnum::Angry(l) => l.is_active(),
            EventListenerEnum::Lagavulin(l) => l.is_active(),
            EventListenerEnum::Enrage(l) => l.is_active(),
            EventListenerEnum::CurlUp(l) => l.is_active(),
            EventListenerEnum::Sentry(l) => l.is_active(),
            EventListenerEnum::SpikeSlimeLSplit(l) => l.is_active(),

            // Battle listeners
            EventListenerEnum::Regen(l) => l.is_active(),
        }
    }

    fn get_owner(&self) -> Entity {
        match self {
            // Relics
            EventListenerEnum::Akabeko(l) => l.get_owner(),
            EventListenerEnum::Anchor(l) => l.get_owner(),
            EventListenerEnum::ArtOfWar(l) => l.get_owner(),
            EventListenerEnum::BagOfMarbles(l) => l.get_owner(),
            EventListenerEnum::BagOfPreparation(l) => l.get_owner(),
            EventListenerEnum::BloodVial(l) => l.get_owner(),
            EventListenerEnum::BronzeScales(l) => l.get_owner(),
            EventListenerEnum::CentennialPuzzle(l) => l.get_owner(),
            EventListenerEnum::GremlinHorn(l) => l.get_owner(),
            EventListenerEnum::HappyFlower(l) => l.get_owner(),
            EventListenerEnum::HornCleat(l) => l.get_owner(),
            EventListenerEnum::InkBottle(l) => l.get_owner(),
            EventListenerEnum::Kunai(l) => l.get_owner(),
            EventListenerEnum::Lantern(l) => l.get_owner(),
            EventListenerEnum::LetterOpener(l) => l.get_owner(),
            EventListenerEnum::MercuryHourglass(l) => l.get_owner(),
            EventListenerEnum::Nunchaku(l) => l.get_owner(),
            EventListenerEnum::OddlySmoothStone(l) => l.get_owner(),
            EventListenerEnum::Orichalcum(l) => l.get_owner(),
            EventListenerEnum::OrnamentalFan(l) => l.get_owner(),
            EventListenerEnum::PenNib(l) => l.get_owner(),
            EventListenerEnum::RedMask(l) => l.get_owner(),
            EventListenerEnum::Shuriken(l) => l.get_owner(),
            EventListenerEnum::TheBoot(l) => l.get_owner(),
            EventListenerEnum::Vajra(l) => l.get_owner(),

            // Powers
            EventListenerEnum::Brutality(l) => l.get_owner(),
            EventListenerEnum::Combust(l) => l.get_owner(),
            EventListenerEnum::DemonForm(l) => l.get_owner(),
            EventListenerEnum::DoubleTap(l) => l.get_owner(),
            EventListenerEnum::Embrace(l) => l.get_owner(),
            EventListenerEnum::FeelNoPain(l) => l.get_owner(),
            EventListenerEnum::FireBreathing(l) => l.get_owner(),
            EventListenerEnum::FlameBarrier(l) => l.get_owner(),
            EventListenerEnum::LoseStrength(l) => l.get_owner(),
            EventListenerEnum::Metallicize(l) => l.get_owner(),
            EventListenerEnum::Rage(l) => l.get_owner(),
            EventListenerEnum::Rupture(l) => l.get_owner(),

            // Enemy abilities
            EventListenerEnum::AcidSlimeLSplit(l) => l.get_owner(),
            EventListenerEnum::GrantRitualNextTurn(l) => l.get_owner(),
            EventListenerEnum::SporeCloud(l) => l.get_owner(),
            EventListenerEnum::Angry(l) => l.get_owner(),
            EventListenerEnum::Lagavulin(l) => l.get_owner(),
            EventListenerEnum::Enrage(l) => l.get_owner(),
            EventListenerEnum::CurlUp(l) => l.get_owner(),
            EventListenerEnum::Sentry(l) => l.get_owner(),
            EventListenerEnum::SpikeSlimeLSplit(l) => l.get_owner(),

            // Battle listeners
            EventListenerEnum::Regen(l) => l.get_owner(),
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        // This method is no longer needed with the enum approach,
        // but we keep it for backward compatibility
        self
    }

    fn hash_to(&self, state: &mut std::collections::hash_map::DefaultHasher) {
        // Delegate to the derived Hash implementation
        self.hash(state);
    }
}
