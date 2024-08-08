use super::state::game_loop::GameLoop;

pub trait PlayerDriver {
    fn determine_move(game: &GameLoop) -> Move;
}

#[derive(Debug)]
pub enum Move {
    Fold,
    Call,
    Raise { amount: u32 },
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
