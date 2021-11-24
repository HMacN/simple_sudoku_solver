pub (crate) struct PuzzleBoard
{
    puzzle_array: [[u16; 9]; 9],
    row_of_current_guess_cell: usize,
    col_of_current_guess_cell: usize,
    value_of_current_guess: u16,
}

impl PuzzleBoard
{
    pub fn new_puzzle_board(puzzle_array: [[u16; 9]; 9], row_of_current_guess_cell: usize, col_of_current_guess_cell: usize, value_of_current_guess: u16) -> PuzzleBoard
    {
        let mut new_puzzle_board: PuzzleBoard = PuzzleBoard
        {
            puzzle_array,
            row_of_current_guess_cell,
            col_of_current_guess_cell,
            value_of_current_guess,
        };

        new_puzzle_board.correct_all_values_to_be_less_than_ten();

        return new_puzzle_board;
    }

    pub fn get_puzzle_array(&self) -> [[u16; 9]; 9]
    {
        return self.puzzle_array;
    }

    pub fn edit_cell_at(&mut self, row: usize, col: usize, new_value: u16)
    {
        self.puzzle_array[row][col] = new_value;
    }

    pub fn get_cell_at(&self, row: usize, col: usize) -> u16
    {
        return self.puzzle_array[row][col];
    }

    pub fn cell_is_empty(&self, row: usize, col: usize) -> bool
    {
        return if self.puzzle_array[col][row] == 0
        {
            true
        }
        else
        {
            false
        }
    }

    pub fn get_guess_cell_row(&self) -> usize
    {
        return self.row_of_current_guess_cell;
    }

    pub fn get_guess_cell_col(&self) -> usize
    {
        return self.col_of_current_guess_cell;
    }

    pub fn get_guess_value(&self) -> u16
    {
        return self.value_of_current_guess;
    }

    pub fn increment_guess_number(&mut self) -> bool
    {
        self.value_of_current_guess = self.value_of_current_guess + 1;

        if self.value_of_current_guess > 9
        {
            return false;
        }

        return true;
    }
}

impl PuzzleBoard
{
    fn correct_all_values_to_be_less_than_ten(&mut self)
    {
        for row_x_coord in 0..9
        {
            for col_y_coord in 0..9
            {
                self.correct_value_above_nine_to_nine(row_x_coord, col_y_coord);
            }
        }
    }

    fn correct_value_above_nine_to_nine(&mut self, row_x_coord: usize, col_y_coord: usize)
    {
        if self.puzzle_array[col_y_coord][row_x_coord] > 9
        {
            self.puzzle_array[col_y_coord][row_x_coord] = 9;
        }
    }
}