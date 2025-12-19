#[derive(Debug, Clone, PartialEq)]
pub enum BattleState {
    PlayerTurn,
    SelectCardInHand,
    SelectCardInDiscard,
    SelectCardInHandToPutOnDeck,
    SelectCardToDuplicate { copies: u32 },
    SelectCardInExhaust,
}
