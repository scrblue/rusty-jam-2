use rgj_shared::protocol::notifications::WhoseTurn;

pub struct TurnTracker {
    pub whose_turn: WhoseTurn,
}

impl TurnTracker {
    pub fn new(wt: &WhoseTurn) -> TurnTracker {
        TurnTracker {
            whose_turn: wt.clone(),
        }
    }
}
