#[derive(Debug, Clone, PartialEq)]
pub enum BattleState {
    PlayerTurn,
    SelectCardInHand (CardInHandTo),
    SelectCardInDiscard,
    SelectCardInExhaust,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardInHandTo {
    PutOnDeck,
    Upgrade,
    Duplicate { copies: u32 },
}