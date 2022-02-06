use crate::display_unit::DisplayUnit;
use crate::sub_set_handler::SubSetHandler;
use crate::sub_set_handler_factory::SubSetHandlerFactory;
use crate::puzzle_board::PuzzleBoard;
use crate::edit_request::EditRequest;

pub struct PuzzleAttempt
{
    puzzle_boards: Vec<PuzzleBoard>,
    subset_handlers: [SubSetHandler; 27],
    display_unit: DisplayUnit,
    stuck_flag: bool,
}

impl PuzzleAttempt
{
    pub fn get_puzzle_array(&self) -> [[u16; 9]; 9]
    {
        return self.puzzle_boards[0].get_puzzle_array();
    }

    pub fn return_display_array(&mut self) -> [[&str; 11]; 11]
    {
        self.display_unit.update_array(self.get_puzzle_array());

        return self.display_unit.get_display_array();
    }

    pub fn new_puzzle_attempt(starting_puzzle_array: [[u16; 9]; 9]) -> PuzzleAttempt
    {
        let new_subset_handlers: [SubSetHandler; 27] = SubSetHandlerFactory::create_starting_subsets();
        let new_display_unit: DisplayUnit = DisplayUnit::new_display_unit();
        let new_puzzle_board: PuzzleBoard = PuzzleBoard::new_puzzle_board(starting_puzzle_array, 0, 0, 10);
        let mut new_puzzle_board_vec: Vec<PuzzleBoard> = Vec::new();

        new_puzzle_board_vec.insert(0, new_puzzle_board);

        let puzzle_attempt_to_return = PuzzleAttempt
        {
            puzzle_boards: new_puzzle_board_vec,
            subset_handlers: new_subset_handlers,
            display_unit: new_display_unit,
            stuck_flag: false,
        };

        return puzzle_attempt_to_return;
    }

    pub fn contains_valid_puzzle(&self) -> bool
    {
        for x in 0..27
        {
            if self.subset_handlers[x].is_this_subset_valid(self.puzzle_boards[0].get_puzzle_array()) == false
            {
                return false;
            }
        }

        return true;
    }

    pub fn check_if_complete(&mut self) -> bool
    {
        if self.puzzle_has_empty_cells()
        {
            return false;
        }

        self.display_unit.set_status_completed();

        return true;
    }

    pub fn display_current_attempt(&mut self) -> bool
    {
        self.check_if_complete();

        self.display_unit.update_array(self.puzzle_boards[0].get_puzzle_array());
        self.display_unit.display_to_user();

        return true;
    }

    pub fn get_cell_at(&self, row: usize, col: usize) -> u16
    {
        return self.puzzle_boards[0].get_cell_at(row, col);
    }

    pub fn edit_cell_at(&mut self, row: usize, col: usize, new_value: u16) -> bool
    {
        let old_value: u16 = self.puzzle_boards[0].get_cell_at(row, col);
        let mut new_value: u16 = new_value;

        if new_value > 9
        {
            new_value = 9;
        }

        self.puzzle_boards[0].edit_cell_at(row, col, new_value);

        let valid_edit = self.contains_valid_puzzle();

        if !valid_edit
        {
            self.puzzle_boards[0].edit_cell_at(row, col, old_value);
        }

        return valid_edit;
    }

    pub fn guess_value(&mut self)
    {
        if self.check_if_complete()
        {
            return;
        }

        let index_of_subset_with_fewest_empty_cells: usize = self.find_index_of_subset_with_fewest_empty_cells();
        let cell_to_guess_in: EditRequest = self.subset_handlers[index_of_subset_with_fewest_empty_cells].get_guess_cell(self.get_puzzle_array());
        let target_cell_row: usize = cell_to_guess_in.get_row_number();
        let target_cell_col: usize = cell_to_guess_in.get_col_number();
        let guess_value: u16 = cell_to_guess_in.get_new_value();

        self.create_next_puzzle_board(target_cell_row, target_cell_col, guess_value);

        self.make_a_guess_on_next_valid_board();
    }

    pub fn stuck(&self) -> bool
    {
        return self.stuck_flag;
    }

    pub fn attempting(&mut self) -> bool
    {
        return if (self.stuck_flag == false) && (self.check_if_complete() == false)
        {
            self.display_unit.set_status_processing();
            true
        }
        else
        {
            false
        }
    }
}

impl PuzzleAttempt
{
    fn puzzle_has_empty_cells(&self) -> bool
    {
        for row_x_coord in 0..9
        {
            for col_y_coord in 0..9
            {
                if self.puzzle_boards[0].cell_is_empty(row_x_coord, col_y_coord)
                {
                    return true;
                }
            }
        }

        return false;
    }

    fn infer_for_single_subset(&mut self, subset: usize) -> bool
    {
        let edit_req: EditRequest = self.subset_handlers[subset].infer_final_value(self.get_puzzle_array());

        if edit_req.get_make_this_edit()
        {
            if self.edit_cell_at(edit_req.get_row_number(), edit_req.get_col_number(), edit_req.get_new_value())
            {
                return true;
            }

            println!("PuzzleAttempt.infer_for_single_subset failed to make a successful edit!");
            return false;
        };

        return false;
    }

    fn find_index_of_subset_with_fewest_empty_cells(&self) -> usize
    {
        let mut index_of_subset_with_fewest_empty_cells: usize = 0;
        let mut lowest_number_of_empty_cells_found: u16 = 9;

        for index in 0..27
        {
            let empty_cells: u16 = self.subset_handlers[index].get_empty_cell_count(self.get_puzzle_array());

            if (empty_cells < lowest_number_of_empty_cells_found) && (empty_cells != 0)
            {
                lowest_number_of_empty_cells_found = empty_cells;
                index_of_subset_with_fewest_empty_cells = index;
            }
        }

        return index_of_subset_with_fewest_empty_cells;
    }

    fn make_valid_guess_on_current_board(&mut self) -> bool
    {
        let mut valid_guess_made: bool = false;

        let next_guess_value: u16 = self.puzzle_boards[0].get_guess_value();
        let target_cell_row: usize = self.puzzle_boards[0].get_guess_cell_row();
        let target_cell_col: usize = self.puzzle_boards[0].get_guess_cell_col();

        if self.edit_cell_at(target_cell_row, target_cell_col, next_guess_value)
        {
            valid_guess_made = true;
        }

        return valid_guess_made;
    }

    fn create_next_puzzle_board(&mut self, target_cell_row: usize, target_cell_col: usize, guess_value: u16)
    {
        let new_puzzle_board: PuzzleBoard = PuzzleBoard::new_puzzle_board
            (self.get_puzzle_array(),
            target_cell_row,
            target_cell_col,
            guess_value);

        self.puzzle_boards.insert(0, new_puzzle_board);
    }

    fn make_a_guess_on_next_valid_board(&mut self)
    {
        let mut guess_made_on_a_board: bool = false;

        while !guess_made_on_a_board && !self.stuck_flag
        {
            if self.make_valid_guess_on_current_board() == false
            {
                self.increment_guess_number_and_delete_board_if_that_is_not_valid()
            }
            else
            {
                guess_made_on_a_board = true;
            }
        }
    }

    fn increment_guess_number_and_delete_board_if_that_is_not_valid(&mut self)
    {
        if (self.puzzle_boards[0].increment_guess_number() == false) && (self.puzzle_boards.len() > 1)
        {
            self.remove_board_safely();

            self.increment_guess_number_and_delete_board_if_that_is_not_valid();
        }

    }

    fn remove_board_safely(&mut self)
    {
        if !self.raise_stuck_flag_if_back_to_starting_board()
        {
            self.puzzle_boards.remove(0);
        }

        self.raise_stuck_flag_if_back_to_starting_board();
    }

    fn raise_stuck_flag_if_back_to_starting_board(&mut self) -> bool
    {
        if self.puzzle_boards.len() == 1
        {
            self.stuck_flag = true;
            self.display_unit.set_status_invalid_puzzle();

            return true;
        };

        return false;
    }
}