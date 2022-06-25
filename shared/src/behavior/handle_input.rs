use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ClaimTileError {
    #[error("it must be your turn to claim a tile")]
    ItIsNotYourTurn,
    #[error("claimed tiles must be adjacent to already claimed tiles")]
    ClaimedTileNotAdjacent,
}
