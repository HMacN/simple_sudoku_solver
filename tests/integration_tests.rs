extern crate simple_sudoku_solver;

pub use simple_sudoku_solver::set_up_new_puzzle_attempt;

#[test]
fn new_attempt_contains_correct_array()
{
    let zero_array: [[u16; 9]; 9] = [[0; 9]; 9];

    let attempt = set_up_new_puzzle_attempt(zero_array);

    assert_eq!(attempt.get_puzzle_array(), zero_array);
}

#[test]
fn new_attempt_can_contain_non_zero_array()
{
    let test_array: [[u16; 9]; 9] =
    [
        [1, 2, 3, 4, 5, 6, 7, 8, 9],
        [0, 0, 0, 0, 0, 0, 0, 0, 8],
        [0, 0, 0, 0, 0, 0, 0, 0, 7],
        [0, 0, 0, 0, 0, 0, 0, 0, 6],
        [0, 0, 0, 0, 0, 0, 0, 0, 5],
        [0, 0, 0, 0, 0, 0, 0, 0, 4],
        [0, 0, 0, 0, 0, 0, 0, 0, 3],
        [0, 0, 0, 0, 0, 0, 0, 0, 2],
        [0, 0, 0, 0, 0, 0, 0, 0, 1],
    ];

    let attempt = set_up_new_puzzle_attempt(test_array);

    assert_eq!(attempt.get_puzzle_array(), test_array);
}

#[test]
fn new_attempt_corrects_numbers_over_nine_to_nine()
{
    let test_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 4, 5, 6, 7, 8, 10],
            [0, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 0, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 0, 0, 0, 0, 0, 0, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 5],
            [0, 0, 0, 0, 0, 0, 0, 0, 4],
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 0, 0, 0, 2],
            [100, 0, 0, 0, 0, 0, 0, 0, 1],
        ];

    let expected_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 4, 5, 6, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 8],
            [0, 0, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 0, 0, 0, 0, 0, 0, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 5],
            [0, 0, 0, 0, 0, 0, 0, 0, 4],
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 0, 0, 0, 2],
            [9, 0, 0, 0, 0, 0, 0, 0, 1],
        ];

    let attempt = set_up_new_puzzle_attempt(test_array);

    assert_eq!(attempt.get_puzzle_array(), expected_array);
}

#[test]
fn returns_true_if_valid_sudoku_array()
{
    let valid_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let attempt = set_up_new_puzzle_attempt(valid_array);

    assert!(attempt.contains_valid_puzzle())
}

#[test]
fn returns_true_if_valid_sudoku_array_with_zeros()
{
    let valid_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 0],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [0, 0, 0, 7, 4, 8, 2, 3, 1],
            [0, 0, 0, 2, 9, 6, 7, 4, 5],
            [0, 0, 0, 0, 5, 1, 6, 9, 8],
        ];

    let attempt = set_up_new_puzzle_attempt(valid_array);

    assert!(attempt.contains_valid_puzzle())
}

#[test]
fn returns_false_if_invalid_row()
{
    let expected_return_false: bool = false;

    let invalid_row_array: [[u16; 9]; 9] =
        [
            [1, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [9, 9, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let attempt = set_up_new_puzzle_attempt(invalid_row_array);

    assert_eq!(attempt.contains_valid_puzzle(), expected_return_false)
}

#[test]
fn returns_false_if_invalid_column()
{
    let expected_return_false: bool = false;

    let invalid_column_array: [[u16; 9]; 9] =
        [
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 9, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0, 9, 0],
        ];

    let attempt = set_up_new_puzzle_attempt(invalid_column_array);

    assert_eq!(attempt.contains_valid_puzzle(), expected_return_false)
}

#[test]
fn returns_false_if_invalid_square()
{
    let expected_return_false: bool = false;

    let invalid_square_array: [[u16; 9]; 9] =
        [
            [1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 3, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 3, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 9, 0],
        ];

    let attempt = set_up_new_puzzle_attempt(invalid_square_array);

    assert_eq!(attempt.contains_valid_puzzle(), expected_return_false)
}

#[test]
fn returns_true_if_complete()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert!(attempt.check_if_complete())
}

#[test]
fn returns_false_if_puzzle_not_complete()
{
    let test_array: [[u16; 9]; 9] =
        [
            [0, 0, 9, 5, 7, 4, 1, 8, 2],
            [0, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert!(!attempt.check_if_complete())
}

#[test]
fn returns_current_display_array()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9,   5, 7, 4,   1, 8, 2],
            [5, 4, 1,   8, 2, 9,   3, 7, 6],
            [7, 8, 2,   6, 1, 3,   9, 5, 4],

            [1, 9, 8,   4, 6, 7,   5, 2, 3],
            [3, 6, 5,   9, 8, 2,   4, 1, 7],
            [4, 2, 7,   1, 3, 5,   8, 6, 9],

            [9, 5, 6,   7, 4, 8,   2, 3, 1],
            [8, 1, 3,   2, 9, 6,   7, 4, 5],
            [2, 7, 4,   3, 5, 1,   6, 9, 8],
        ];

    let expected_display_array: [[&str; 11]; 11] =
        [
            ["6", "3", "9", "|", "5", "7", "4", "|", "1", "8", "2"],
            ["5", "4", "1", "|", "8", "2", "9", "|", "3", "7", "6"],
            ["7", "8", "2", "|", "6", "1", "3", "|", "9", "5", "4"],
            ["-", "-", "-", "+", "-", "-", "-", "+", "-", "-", "-"],
            ["1", "9", "8", "|", "4", "6", "7", "|", "5", "2", "3"],
            ["3", "6", "5", "|", "9", "8", "2", "|", "4", "1", "7"],
            ["4", "2", "7", "|", "1", "3", "5", "|", "8", "6", "9"],
            ["-", "-", "-", "+", "-", "-", "-", "+", "-", "-", "-"],
            ["9", "5", "6", "|", "7", "4", "8", "|", "2", "3", "1"],
            ["8", "1", "3", "|", "2", "9", "6", "|", "7", "4", "5"],
            ["2", "7", "4", "|", "3", "5", "1", "|", "6", "9", "8"],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert_eq!(attempt.return_display_array(), expected_display_array)
}

#[test]
fn show_user_current_puzzle_progress()
{
    let test_array: [[u16; 9]; 9] =
        [
            [0, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert!(attempt.display_current_attempt())
}

#[test]
fn get_value_of_particular_cell()
{
    let test_array: [[u16; 9]; 9] =
        [
            [0, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let attempt = set_up_new_puzzle_attempt(test_array);

    assert_eq!(attempt.get_cell_at(6, 0), 9);
}

#[test]
fn edit_value_of_particular_cell()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 0, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let expected_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    attempt.edit_cell_at(3, 1, 9);

    assert_eq!(attempt.get_puzzle_array(), expected_array);
}

#[test]
fn does_not_do_invalid_edits()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 0, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let expected_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 0, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    attempt.edit_cell_at(3, 1, 7);

    assert_eq!(attempt.get_puzzle_array(), expected_array);
}

#[test]
fn returns_true_on_successful_edit()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 0, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert!(attempt.edit_cell_at(3, 1, 9));
}

#[test]
fn returns_false_on_invalid_edit()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 0, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    assert!(!attempt.edit_cell_at(3, 1, 3));
}

#[test]
fn make_lowest_valid_guess_for_cell_in_most_complete_subset()
{
    let test_array: [[u16; 9]; 9] =
        [
            [0, 0, 0, 0, 7, 4, 1, 8, 2],
            [0, 0, 0, 0, 2, 9, 3, 7, 6],
            [0, 0, 0, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let expected_array: [[u16; 9]; 9] =
        [
            [0, 0, 0, 5, 7, 4, 1, 8, 2],
            [0, 0, 0, 0, 2, 9, 3, 7, 6],
            [0, 0, 0, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    attempt.guess_value();

    assert_eq!(attempt.get_puzzle_array(), expected_array);
}

#[test]
fn for_cell_with_no_valid_guess_do_not_make_edit()
{
    let test_array: [[u16; 9]; 9] =
        [
            [0, 7, 4, 3, 5, 2, 6, 9, 8],
            [0, 0, 0, 0, 0, 0, 2, 3, 1],
            [1, 0, 0, 0, 0, 0, 7, 4, 5],
            [0, 0, 0, 0, 0, 0, 0, 0, 2],
            [0, 0, 0, 0, 0, 0, 0, 0, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 4],
            [0, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 0, 0, 0, 0, 0, 0, 0, 7],
            [0, 0, 0, 0, 0, 0, 0, 0, 9],

        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    attempt.guess_value();

    assert_eq!(attempt.get_puzzle_array(), test_array);
}

#[test]
fn on_reaching_cell_with_no_valid_guesses_go_back_to_last_valid_guess_and_pick_next_valid_number()
{
    let test_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 0, 0, 0, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let expected_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 6, 0, 0, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    attempt.guess_value();

    attempt.guess_value();

    attempt.guess_value();

    assert_eq!(attempt.get_puzzle_array(), expected_array);
}

#[test]
fn on_running_out_of_viable_guesses_raise_stuck_flag()
{
    let test_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 0, 0, 0, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    for _x in 0..5
    {
        attempt.guess_value();
    }

    assert!(attempt.stuck());
}

#[test]
fn attempting_true_if_cells_remaining_and_not_stuck()
{
    let test_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 0, 0, 0, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    for _x in 0..3
    {
        attempt.guess_value();
    }

    assert!(attempt.attempting());
}

#[test]
fn attempting_false_if_stuck()
{
    let test_array: [[u16; 9]; 9] =
        [
            [1, 2, 3, 0, 0, 0, 7, 8, 9],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 4, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    for _x in 0..6
    {
        attempt.guess_value();
    }

    assert_eq!(attempt.attempting(), false);
}

#[test]
fn attempting_false_if_completed()
{
    let test_array: [[u16; 9]; 9] =
        [
            [6, 3, 9, 5, 7, 4, 1, 8, 2],
            [5, 4, 1, 8, 2, 9, 3, 7, 6],
            [7, 8, 2, 6, 1, 3, 9, 5, 4],
            [1, 9, 8, 4, 6, 7, 5, 2, 3],
            [3, 6, 5, 9, 8, 2, 4, 1, 7],
            [4, 2, 7, 1, 3, 5, 8, 6, 9],
            [9, 5, 6, 7, 4, 8, 2, 3, 1],
            [8, 1, 3, 2, 9, 6, 7, 4, 5],
            [2, 7, 4, 3, 5, 1, 6, 9, 8],
        ];

    let mut attempt = set_up_new_puzzle_attempt(test_array);

    for _x in 0..6
    {
        attempt.guess_value();
    }

    assert_eq!(attempt.attempting(), false);
}

