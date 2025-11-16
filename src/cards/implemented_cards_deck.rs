//! Test deck containing one of each implemented card for testing purposes

use crate::game::deck::Deck;
use crate::cards::ironclad::*;

/// Creates a test deck containing one copy of each implemented Ironclad card
/// Useful for testing card mechanics and interactions
pub fn create_implemented_cards_deck() -> Deck {
    let cards = vec![
        // Basic Cards
        strike(),
        defend(),
        bash(),

        // Common Attack Cards
        cleave(),
        iron_wave(),
        pommel_strike(),
        clothesline(),
        heavy_blade(),
        perfected_strike(),
        thunderclap(),
        twin_strike(),
        wild_strike(),

        // Common Skill Cards
        flex(),
        shrug_it_off(),

        // Common Power Cards
        combust(),
        disarm(),
        feel_no_pain(),
        entrench(),
        embrace(),

        // Newly Implemented Cards
        anger(),
        armaments(),
        sword_boomerang(),

        // Uncommon Cards
        hemokinesis(),
        inflame(),

        // Rare Cards
        bludgeon(),
        brutality(),
        impervious(),
        offering(),
        shockwave(),
        uppercut(),
    ];

    Deck::new(cards)
}

/// Creates a test deck with only the newly implemented cards
pub fn create_new_cards_deck() -> Deck {
    let cards = vec![
        // Newly implemented cards
        anger(),
        armaments(),
        sword_boomerang(),
        hemokinesis(),
        inflame(),
        bludgeon(),
        brutality(),
        impervious(),
        offering(),
        shockwave(),
        uppercut(),
    ];

    Deck::new(cards)
}

/// Creates a test deck focusing on power cards
pub fn create_power_cards_deck() -> Deck {
    let cards = vec![
        combust(),
        embrace(),
        inflame(),
        brutality(),
        // Include some basic cards for testing
        strike(),
        strike(),
        defend(),
        defend(),
    ];

    Deck::new(cards)
}

/// Creates a test deck focusing on attack cards
pub fn create_attack_cards_deck() -> Deck {
    let cards = vec![
        cleave(),
        iron_wave(),
        pommel_strike(),
        clothesline(),
        heavy_blade(),
        perfected_strike(),
        thunderclap(),
        twin_strike(),
        wild_strike(),
        bludgeon(),
        hemokinesis(),
        shockwave(),
        uppercut(),
        // Include some basic cards
        strike(),
        defend(),
    ];

    Deck::new(cards)
}