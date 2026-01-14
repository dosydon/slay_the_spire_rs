use crate::game::{game::Game, action::GameAction, game_state::GameState};

impl Game {
    pub(super) fn list_available_actions(&self) -> Vec<GameAction> {
        match self.get_game_state() {
            GameState::BossBeaten => {
                // Game is over - no actions available
                Vec::new()
            }
            GameState::OnMap => {
                // Get available nodes to move to
                if let Some(_current_node) = self.get_current_node() {
                    self.map.get_neighbors(self.current_node_position)
                        .iter()
                        .enumerate()
                        .map(|(idx, _)| GameAction::ChoosePath(idx))
                        .collect()
                } else {
                    Vec::new()
                }
            }
            GameState::InBattle => {
                // Delegate to battle's available actions
                if let Some(battle) = self.get_battle() {
                    battle.list_available_actions()
                        .into_iter()
                        .map(GameAction::Battle)
                        .collect()
                } else {
                    Vec::new()
                }
            }
            GameState::Reward(reward_state) => {
                let mut actions = Vec::new();

                // Can claim gold if not claimed
                if !reward_state.gold_claimed {
                    actions.push(GameAction::ClaimGold);
                }

                // Can claim potion if available and not claimed and inventory not full
                if reward_state.potion_reward.is_some()
                    && !reward_state.potion_claimed
                    && !self.potions.is_full() {
                    actions.push(GameAction::ClaimPotion);
                }

                // Can claim relic if available and not claimed
                if reward_state.relic_reward.is_some()
                    && !reward_state.relic_claimed {
                    actions.push(GameAction::ClaimRelic);
                }

                // Can skip
                actions.push(GameAction::Skip);

                actions
            }
            GameState::CardRewardSelection(cards) => {
                // Can select any of the card rewards
                (0..cards.len())
                    .map(|i| GameAction::SelectCardReward(i))
                    .collect()
            }
            GameState::InEvent(_, choices) => {
                // Can choose any event option
                (0..choices.len())
                    .map(|i| GameAction::ChooseEvent(i))
                    .collect()
            }
            GameState::RestSite => {
                // Rest site actions
                vec![
                ]
            }
            GameState::SelectingCardFromDeck(_) => {
                // Select from deck
                self.get_upgradeable_cards()
                    .iter()
                    .map(|(idx, _)| GameAction::SelectCardFromDeck(*idx))
                    .collect()
            }
            GameState::Shop(shop_state) => {
                let mut actions = Vec::new();

                // Can buy any card for sale
                for i in 0..shop_state.card_count() {
                    if let Some(price) = shop_state.get_card_price(i) {
                        if self.gold >= price {
                            actions.push(GameAction::BuyCard(i));
                        }
                    }
                }

                // Can skip shop
                actions.push(GameAction::Skip);

                actions
            }
        }
    }
}
