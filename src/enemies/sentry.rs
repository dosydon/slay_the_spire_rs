use crate::{game::{effect::Effect, enemy::EnemyTrait, global_info::GlobalInfo}};

#[derive(Copy, Debug, Clone, PartialEq)]
pub enum SentryMove {
    Bolt,  // Debuff intent - adds Dazed cards
    Beam,  // Attack intent - deals damage
}

#[derive(Clone, Debug)]
pub struct Sentry {
    hp: u32,
    ascension: u32,
    turn_count: u32,
    starts_with_bolt: bool,  // true = Bolt first (outer sentries), false = Beam first (middle sentry)
}

impl Sentry {
    /// Create a new Sentry
    pub fn new(hp: u32, ascension: u32, starts_with_bolt: bool) -> Self {
        Sentry {
            hp,
            ascension,
            turn_count: 0,
            starts_with_bolt,
        }
    }

    /// Calculate HP range based on ascension
    pub fn calculate_hp_range(ascension: u32) -> (u32, u32) {
        if ascension >= 8 {
            (39, 45)  // Ascension 8+: 39-45 HP
        } else {
            (38, 42)  // Base: 38-42 HP
        }
    }

    /// Calculate Bolt dazed card count based on ascension
    pub fn calculate_bolt_dazed_count(ascension: u32) -> u32 {
        if ascension >= 18 {
            3  // Ascension 18+: 3 Dazed cards
        } else {
            2  // Base: 2 Dazed cards
        }
    }

    /// Calculate Beam damage based on ascension
    pub fn calculate_beam_damage(ascension: u32) -> u32 {
        if ascension >= 3 {
            10  // Ascension 3+: 10 damage
        } else {
            9   // Base: 9 damage
        }
    }

    /// Get current turn move based on starting pattern and turn count
    pub fn get_current_move(&self) -> SentryMove {
        if self.starts_with_bolt {
            // Starts with Bolt: Bolt → Beam → Bolt (repeating)
            if self.turn_count % 2 == 0 {
                SentryMove::Bolt
            } else {
                SentryMove::Beam
            }
        } else {
            // Starts with Beam: Beam → Bolt → Beam (repeating)
            if self.turn_count % 2 == 0 {
                SentryMove::Beam
            } else {
                SentryMove::Bolt
            }
        }
    }

    /// Get the effects for a specific move
    pub fn get_move_effects(&self, move_type: SentryMove) -> Vec<Effect> {
        match move_type {
            SentryMove::Bolt => {
                // Add Dazed cards to discard pile
                let dazed_count = Self::calculate_bolt_dazed_count(self.ascension);
                vec![Effect::AddStatusToDiscard {
                    status_card: crate::game::card_enum::CardEnum::Dazed
                }; dazed_count as usize]
            }
            SentryMove::Beam => {
                // Deal damage to player
                let damage = Self::calculate_beam_damage(self.ascension);
                vec![Effect::AttackToTarget {
                    amount: damage,
                    num_attacks: 1,
                    strength_multiplier: 0,
                }]
            }
        }
    }

    /// Increment turn count and get current move
    pub fn next_turn(&mut self) -> SentryMove {
        let current_move = self.get_current_move();
        self.turn_count += 1;
        current_move
    }

    /// Get valid moves for current turn
    pub fn get_valid_moves(&self) -> Vec<SentryMove> {
        vec![self.get_current_move()]
    }

    /// Create multiple Sentries with proper position patterns
    /// Returns 3 Sentries: Left (Bolt first), Middle (Beam first), Right (Bolt first)
    pub fn create_sentry_group(rng: &mut impl rand::Rng, ascension: u32) -> Vec<Self> {
        let (min_hp, max_hp) = Self::calculate_hp_range(ascension);

        // Create 3 Sentries with alternating patterns
        let hp1 = min_hp + rng.random_range(0..=(max_hp - min_hp));
        let hp2 = min_hp + rng.random_range(0..=(max_hp - min_hp));
        let hp3 = min_hp + rng.random_range(0..=(max_hp - min_hp));

        let sentry1 = Self::new(hp1, ascension, true);   // Left: Bolt first
        let sentry2 = Self::new(hp2, ascension, false);  // Middle: Beam first
        let sentry3 = Self::new(hp3, ascension, true);   // Right: Bolt first

        vec![sentry1, sentry2, sentry3]
    }
}

impl EnemyTrait for Sentry {
    type MoveType = SentryMove;

    fn instantiate(rng: &mut impl rand::Rng, global_info: &GlobalInfo) -> Self {
        let ascension = global_info.ascention;

        // Randomly decide if this Sentry starts with Bolt or Beam
        // In practice, Sentries usually appear in groups of 3 with specific patterns,
        // but for single instantiation, we randomize
        let starts_with_bolt = rng.random_range(0..2) == 0;

        let (min_hp, max_hp) = Self::calculate_hp_range(ascension);
        let hp = min_hp + rng.random_range(0..=(max_hp - min_hp));

        Self::new(hp, ascension, starts_with_bolt)
    }

    fn get_name() -> String {
        "Sentry".to_string()
    }

    fn get_hp(&self) -> u32 {
        self.hp
    }

    fn choose_move_and_effects(&mut self, _global_info: &GlobalInfo, _rng: &mut impl rand::Rng) -> (Self::MoveType, Vec<Effect>) {
        let move_type = self.next_turn();
        let effects = self.get_move_effects(move_type);
        (move_type, effects)
    }
}