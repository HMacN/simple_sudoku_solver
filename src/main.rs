use simple_sudoku_solver::set_up_new_puzzle_attempt;
use std::{thread, time};

fn main()
{
    let test_array_one: [[u16; 9]; 9] =
        [
            [0, 0, 0,    0, 1, 0,    0, 0, 0],
            [0, 0, 8,    6, 0, 3,    2, 0, 0],
            [0, 9, 0,    0, 5, 0,    0, 6, 0],

            [0, 6, 0,    7, 0, 5,    0, 3, 0],
            [9, 0, 2,    0, 0, 0,    1, 0, 7],
            [0, 8, 0,    3, 0, 1,    0, 2, 0],

            [0, 2, 0,    0, 8, 0,    0, 9, 0],
            [0, 0, 6,    1, 0, 9,    8, 0, 0],
            [0, 0, 0,    0, 7, 0,    0, 0, 0],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array_one);
    let delay_between_displays = time::Duration::from_millis(50);

    attempt.display_current_attempt();

    while attempt.attempting()
    {
        attempt.guess_value();
        attempt.display_current_attempt();
        thread::sleep(delay_between_displays);
    }

    attempt.display_current_attempt();
}
