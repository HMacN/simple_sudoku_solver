pub mod puzzle_solver;
mod display_unit;
mod sub_set_handler;
mod sub_set_handler_factory;
mod puzzle_board;
mod edit_request;

use crate::puzzle_solver::PuzzleAttempt;

pub fn set_up_new_puzzle_attempt(puzzle_to_solve: [[u16; 9]; 9]) -> PuzzleAttempt
{
    let mut _puzzle1 = PuzzleAttempt::new_puzzle_attempt(puzzle_to_solve);

    return _puzzle1;
}