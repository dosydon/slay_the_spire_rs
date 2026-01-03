# Ironclad Cards Implementation Status

This document tracks the implementation status of all Ironclad cards in the Slay the Spire Rust implementation.

## Summary

- ✅ **69 cards implemented** (3 Basic + 29 Common + 14 Rare + 23 Uncommon)
- ❌ **0 Ironclad cards not yet implemented**
- 🎯 **Implementation Progress: 100%** of Ironclad cards

## Card Implementation Status

### Basic Cards

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|
| ✅ Strike | Attack | 1 | 1 | Yes | `src/cards/ironclad/strike.rs` | Deal 6 damage | Deal 9 damage |
| ✅ Defend | Skill | 1 | 1 | Yes | `src/cards/ironclad/defend.rs` | Gain 5 Block | Gain 8 Block |
| ✅ Bash | Attack | 2 | 2 | Yes | `src/cards/ironclad/bash.rs` | Deal 8 damage + Apply 2 Vulnerable | Deal 10 damage + Apply 3 Vulnerable |

### Common Cards

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|
| ✅ Anger | Attack | 0 | 0 | Yes | `src/cards/ironclad/anger.rs` | Deal 6 damage. Add copy to discard | Deal 8 damage. Add copy to discard |
| ✅ Armaments | Skill | 1 | 1 | Yes | `src/cards/ironclad/armaments.rs` | Gain 5 Block. Upgrade a card in hand | Gain 5 Block. Upgrade ALL cards in hand |
| ✅ Body Slam | Attack | 1 | 0 | Yes | `src/cards/ironclad/body_slam.rs` | Deal damage equal to your Block | Deal damage equal to your Block |
| ✅ Carnage | Attack | 2 | 2 | Yes | `src/cards/ironclad/carnage.rs` | Deal 20 damage. Ethereal | Deal 28 damage. Ethereal |
| ✅ Clash | Attack | 0 | 0 | Yes | `src/cards/ironclad/clash.rs` | Deal 14 damage. **Condition**: Hand must be all Attacks | Deal 18 damage. **Condition**: Hand must be all Attacks |
| ✅ Cleave | Attack | 1 | 1 | Yes | `src/cards/ironclad/cleave.rs` | Deal 8 damage to ALL enemies | Deal 11 damage to ALL enemies |
| ✅ Clothesline | Attack | 2 | 2 | Yes | `src/cards/ironclad/clothesline.rs` | Deal 12 damage + Apply 2 Weak | Deal 14 damage + Apply 3 Weak |
| ✅ Flex | Skill | 0 | 0 | Yes | `src/cards/ironclad/flex.rs` | Gain 2 Strength. Lose 2 at end of turn | Gain 4 Strength. Lose 4 at end of turn |
| ✅ Havoc | Skill | 1 | 0 | Yes | `src/cards/ironclad/havoc.rs` | Play top card of draw pile. Exhaust it | Play top card of draw pile. Exhaust it |
| ✅ Heavy Blade | Attack | 2 | 2 | Yes | `src/cards/ironclad/heavy_blade.rs` | Deal 14 damage (3× Strength) | Deal 22 damage (5× Strength) |
| ✅ Headbutt | Attack | 1 | 1 | Yes | `src/cards/ironclad/headbutt.rs` | Deal 9 damage. Put discard card on top of draw pile | Deal 12 damage. Put discard card on top of draw pile |
| ✅ Iron Wave | Attack | 1 | 1 | Yes | `src/cards/ironclad/iron_wave.rs` | Gain 5 Block + Deal 5 damage | Gain 8 Block + Deal 8 damage |
| ✅ Perfected Strike | Attack | 2 | 2 | Yes | `src/cards/ironclad/perfected_strike.rs` | Deal 6 damage (+2 per Strike in deck) | Deal 10 damage (+3 per Strike in deck) |
| ✅ Pommel Strike | Attack | 1 | 1 | Yes | `src/cards/ironclad/pommel_strike.rs` | Deal 9 damage + Draw 1 card | Deal 10 damage + Draw 2 cards |
| ✅ Shrug It Off | Skill | 1 | 1 | Yes | `src/cards/ironclad/shrug_it_off.rs` | Gain 8 Block + Draw 1 card | Gain 11 Block + Draw 1 card |
| ✅ Sword Boomerang | Attack | 1 | 1 | Yes | `src/cards/ironclad/sword_boomerang.rs` | Deal 3 damage 3 times | Deal 4 damage 3 times |
| ✅ Thunderclap | Attack | 1 | 1 | Yes | `src/cards/ironclad/thunderclap.rs` | Deal 4 damage to ALL + Apply 1 Vulnerable to ALL | Deal 7 damage to ALL + Apply 1 Vulnerable to ALL |
| ✅ True Grit | Skill | 1 | 1 | Yes | `src/cards/ironclad/true_grit.rs` | Gain 7 Block. Exhaust 1 card from hand | Gain 9 Block. Exhaust 1 card (choose) from hand |
| ✅ Twin Strike | Attack | 1 | 1 | Yes | `src/cards/ironclad/twin_strike.rs` | Deal 5 damage twice | Deal 7 damage twice |
| ✅ Warcry | Skill | 0 | 0 | Yes | `src/cards/ironclad/warcry.rs` | Draw 2 cards. Put 1 card on top of draw pile | Draw 2 cards. Put 1 card (choose) on top of draw pile |
| ✅ Wild Strike | Attack | 1 | 1 | Yes | `src/cards/ironclad/wild_strike.rs` | Deal 12 damage + Add Wound to draw pile | Deal 17 damage + Add Wound to draw pile |
| ✅ Reckless Charge | Attack | 0 | 0 | Yes | `src/cards/ironclad/reckless_charge.rs` | Deal 7 damage + Add Dazed to discard | Deal 10 damage + Add Dazed to discard |

### Rare Cards

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|
| ✅ Bludgeon | Attack | 3 | 3 | Yes | `src/cards/ironclad/bludgeon.rs` | Deal 32 damage | Deal 42 damage |
| ✅ Brutality | Power | 0 | 0 | Yes | `src/cards/ironclad/brutality.rs` | At turn start: lose 1 HP, draw 1 card | At turn start: lose 1 HP, draw 1 card |
| ✅ Corruption | Power | 3 | 2 | Yes | `src/cards/ironclad/corruption.rs` | Skills cost 0. Whenever you play a Skill, Exhaust it | Skills cost 0. Whenever you play a Skill, Exhaust it |
| ✅ Demon Form | Power | 3 | 3 | Yes | `src/cards/ironclad/demon_form.rs` | At turn start, gain 2 Strength | At turn start, gain 3 Strength |
| ✅ Double Tap | Skill | 1 | 1 | Yes | `src/cards/ironclad/double_tap.rs` | This turn, next Attack is played twice. Exhaust | This turn, next 2 Attacks are played twice. Exhaust |
| ✅ Exhume | Skill | 1 | 0 | Yes | `src/cards/ironclad/exhume.rs` | Put Exhaust pile card into hand. Exhaust | Put Exhaust pile card into hand. Exhaust |
| ✅ Feed | Attack | 1 | 1 | Yes | `src/cards/ironclad/feed.rs` | Deal 10 damage. Heal 3 HP if enemy dies. Exhaust | Deal 12 damage. Heal 4 HP if enemy dies. Exhaust |
| ✅ Fiend Fire | Attack | 2 | 2 | Yes | `src/cards/ironclad/fiend_fire.rs` | Exhaust hand. Deal 7 damage per card exhausted. Exhaust | Exhaust hand. Deal 10 damage per card exhausted. Exhaust |
| ✅ Immolate | Attack | 2 | 2 | Yes | `src/cards/ironclad/immolate.rs` | Deal 21 damage to ALL. Add Burn to discard | Deal 28 damage to ALL. Add Burn to discard |
| ✅ Impervious | Skill | 2 | 2 | Yes | `src/cards/ironclad/impervious.rs` | Gain 30 Block. Exhaust | Gain 40 Block. Exhaust |
| ✅ Juggernaut | Power | 2 | 2 | Yes | `src/cards/ironclad/juggernaut.rs` | Whenever you gain Block, deal 5 damage to random enemy | Whenever you gain Block, deal 7 damage to random enemy |
| ✅ Limit Break | Skill | 1 | 0 | Yes | `src/cards/ironclad/limit_break.rs` | Double your Strength. Exhaust | Double your Strength |
| ✅ Offering | Skill | 0 | 0 | Yes | `src/cards/ironclad/offering.rs` | Lose 6 HP. Gain 2 Energy. Draw 3 cards. Exhaust | Lose 4 HP. Gain 2 Energy. Draw 3 cards. Exhaust |
| ✅ Reaper | Attack | 2 | 2 | Yes | `src/cards/ironclad/reaper.rs` | Deal 4 damage to ALL. Heal for unblocked damage | Deal 5 damage to ALL. Heal for unblocked damage |
| ✅ Shockwave | Skill | 2 | 2 | Yes | `src/cards/ironclad/shockwave.rs` | Apply 3 Weak and 3 Vulnerable to ALL | Apply 5 Weak and 5 Vulnerable to ALL |
| ✅ Uppercut | Attack | 2 | 2 | Yes | `src/cards/ironclad/uppercut.rs` | Deal 13 damage. Apply 1 Weak, 1 Vulnerable | Deal 13 damage. Apply 2 Weak, 2 Vulnerable |


### Uncommon Cards

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|
| ✅ Combust | Power | 1 | 1 | Yes | `src/cards/ironclad/combust.rs` | At turn end: lose 1 HP, deal 5 damage to ALL | At turn end: lose 1 HP, deal 7 damage to ALL |
| ✅ Dark Embrace | Power | 2 | 1 | Yes | `src/cards/ironclad/embrace.rs` | Whenever you Exhaust a card, draw 1 card | Whenever you Exhaust a card, draw 1 card |
| ✅ Disarm | Skill | 1 | 1 | Yes | `src/cards/ironclad/disarm.rs` | Enemy loses 2 Strength. Exhaust | Enemy loses 3 Strength. Exhaust |
| ✅ Dropkick | Attack | 1 | 1 | Yes | `src/cards/ironclad/dropkick.rs` | Deal 5 damage. If enemy Vulnerable: gain 1 Energy, draw 1 | Deal 8 damage. If enemy Vulnerable: gain 1 Energy, draw 1 |
| ✅ Dual Wield | Skill | 1 | 1 | Yes | `src/cards/ironclad/dual_wield.rs` | Duplicate a card to discard pile | Duplicate a card twice to discard pile |
| ✅ Entrench | Skill | 2 | 1 | Yes | `src/cards/ironclad/entrench.rs` | Double your current Block | Double your current Block |
| ✅ Evolve | Power | 1 | 1 | Yes | `src/cards/ironclad/evolve.rs` | Whenever you draw Status, draw 1 card | Whenever you draw Status, draw 2 cards |
| ✅ Feel No Pain | Power | 1 | 1 | Yes | `src/cards/ironclad/feel_no_pain.rs` | Whenever you Exhaust, gain 3 Block | Whenever you Exhaust, gain 4 Block |
| ✅ Fire Breathing | Power | 1 | 1 | Yes | `src/cards/ironclad/fire_breathing.rs` | When you draw Status/Curse, deal 6 damage to ALL | When you draw Status/Curse, deal 10 damage to ALL |
| ✅ Flame Barrier | Skill | 2 | 2 | Yes | `src/cards/ironclad/flame_barrier.rs` | Gain 12 Block. This turn: attacked → deal 4 to attacker | Gain 16 Block. This turn: attacked → deal 6 to attacker |
| ✅ Ghostly Armor | Skill | 1 | 1 | Yes | `src/cards/ironclad/ghostly_armor.rs` | Gain 10 Block. Ethereal | Gain 13 Block. Ethereal |
| ✅ Hemokinesis | Attack | 1 | 1 | Yes | `src/cards/ironclad/hemokinesis.rs` | Lose 2 HP. Deal 15 damage | Lose 2 HP. Deal 22 damage |
| ✅ Inflame | Power | 1 | 1 | Yes | `src/cards/ironclad/inflame.rs` | Gain 2 Strength | Gain 3 Strength |
| ✅ Infernal Blade | Skill | 1 | 0 | Yes | `src/cards/ironclad/infernal_blade.rs` | Add random Attack to hand. Exhaust | Add random Attack to hand. Exhaust |
| ✅ Intimidate | Skill | 0 | 0 | Yes | `src/cards/ironclad/intimidate.rs` | Apply 1 Weak to ALL enemies. Exhaust | Apply 2 Weak to ALL enemies. Exhaust |
| ✅ Metallicize | Power | 1 | 1 | Yes | `src/cards/ironclad/metallicize.rs` | At turn end, gain 3 Block | At turn end, gain 4 Block |
| ✅ Power Through | Skill | 1 | 1 | Yes | `src/cards/ironclad/power_through.rs` | Add 2 Wounds to hand. Gain 15 Block | Add 2 Wounds to hand. Gain 20 Block |
| ✅ Pummel | Attack | 1 | 1 | Yes | `src/cards/ironclad/pummel.rs` | Deal 2 damage 4 times. Exhaust | Deal 2 damage 5 times. Exhaust |
| ✅ Rage | Skill | 0 | 0 | Yes | `src/cards/ironclad/rage.rs` | Whenever you play Attack, gain 3 Block | Whenever you play Attack, gain 4 Block |
| ✅ Rampage | Attack | 1 | 1 | Yes | `src/cards/ironclad/rampage.rs` | Deal 8 damage. Increases by 5 each use | Deal 8 damage. Increases by 8 each use |
| ✅ Reckless Charge | Attack | 0 | 0 | Yes | `src/cards/ironclad/reckless_charge.rs` | Deal 7 damage + Add Dazed to discard | Deal 10 damage + Add Dazed to discard |
| ✅ Rupture | Power | 1 | 1 | Yes | `src/cards/ironclad/rupture.rs` | When you lose HP from cards: gain 1 Strength | When you lose HP from cards: gain 1 Strength |
| ✅ Searing Blow | Attack | 2 | 2 | Yes | `src/cards/ironclad/searing_blow.rs` | Deal 12 damage + Can upgrade infinitely | Deal 16 damage + Can upgrade infinitely |
| ✅ Second Wind | Skill | 1 | 1 | Yes | `src/cards/ironclad/second_wind.rs` | Exhaust non-Attacks. Gain 5 Block per card | Exhaust non-Attacks. Gain 8 Block per card |
| ✅ Seeing Red | Skill | 1 | 0 | Yes | `src/cards/ironclad/seeing_red.rs` | Gain 2 Energy. Exhaust | Gain 2 Energy. Exhaust |
| ✅ Sentinel | Skill | 1 | 0 | Yes | `src/cards/ironclad/sentinel.rs` | Gain 5 Block. If no Block: gain 2 Energy | Gain 8 Block. If no Block: gain 3 Energy |
| ✅ Sever Soul | Attack | 2 | 2 | Yes | `src/cards/ironclad/sever_soul.rs` | Deal 16 damage + Exhaust non-Attacks | Deal 22 damage + Exhaust non-Attacks |
| ✅ Spot Weakness | Skill | 1 | 1 | Yes | `src/cards/ironclad/spot_weakness.rs` | If enemy attacking: gain 3 Strength + Exhaust | If enemy attacking: gain 4 Strength + Exhaust |
| ✅ Whirlwind | Attack | 1 | 1 | Yes | `src/cards/ironclad/whirlwind.rs` | Deal 5 damage to ALL | Deal 8 damage to ALL |

### Status Cards

| Card Name | Type | Cost | Cost+ | Implemented | File Location | Base Effects | Upgraded Effects |
|-----------|------|------|-------|-------------|---------------|--------------|------------------|
| ✅ Dazed | Status | - | - | Yes | `src/cards/status/dazed.rs` | Unplayable. Ethereal | Unplayable. Ethereal |
| ✅ Slimed | Status | 1 | 1 | Yes | `src/cards/status/slimed.rs` | Do nothing. Exhaust | Do nothing. Exhaust |
| ✅ Wound | Status | - | - | Yes | `src/cards/status/wound.rs` | Unplayable | Unplayable |
