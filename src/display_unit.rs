use std::borrow::Borrow;

pub (crate) struct DisplayUnit
{
    display_array: [[String; 11]; 11],
    puzzle_array: [[u16; 9]; 9],
    status: String,
}

impl DisplayUnit
{
    pub fn get_display_array(&self) -> [[&str; 11]; 11]
    {
        let mut snapshot_of_current_display_array: [[&str; 11]; 11] = [[" "; 11]; 11];

        for x in 0..11
        {
            for y in 0..11
            {
                let cell_value: &str = self.display_array[x][y].borrow();

                snapshot_of_current_display_array[x][y] = cell_value;
            }
        }

        return snapshot_of_current_display_array;
    }

    pub fn new_display_unit() -> Self
    {
        let starting_display_array: [[String; 11]; 11] = Default::default();
        let starting_puzzle_array: [[u16; 9]; 9] = [[0; 9]; 9];
        let starting_status: String = "Setting up the puzzle...".to_string();

        let mut display_unit: DisplayUnit = DisplayUnit
        {
            display_array: starting_display_array,
            puzzle_array: starting_puzzle_array,
            status: starting_status,
        };

        display_unit.update_array(starting_puzzle_array);

        return display_unit;
    }

    pub fn set_status_processing(&mut self)
    {
        self.status.replace_range(.., "Processing the puzzle...");
    }

    pub fn set_status_invalid_puzzle(&mut self)
    {
        self.status.replace_range(.., "The puzzle could not be solved.");
    }

    pub fn set_status_completed(&mut self)
    {
        self.status.replace_range(.., "The puzzle has been completed.");
    }

    pub fn update_array(&mut self, new_puzzle_array: [[u16; 9]; 9])
    {
        self.puzzle_array = new_puzzle_array;

        self.update_one_square_of_display_array(0, 0, 0, 0);
        self.update_one_square_of_display_array(3, 0, 4, 0);
        self.update_one_square_of_display_array(6, 0, 8, 0);

        self.update_one_square_of_display_array(0, 3, 0, 4);
        self.update_one_square_of_display_array(3, 3, 4, 4);
        self.update_one_square_of_display_array(6, 3, 8, 4);

        self.update_one_square_of_display_array(0, 6, 0, 8);
        self.update_one_square_of_display_array(3, 6, 4, 8);
        self.update_one_square_of_display_array(6, 6, 8, 8);

        self.put_lines_into_display_array();
    }

    pub fn display_to_user(&self)
    {
        DisplayUnit::clear_display_area_and_reset_cursor();

        DisplayUnit::display_title_banner();

        self.display_status();

        self.display_array_to_console();
    }

    fn display_array_to_console(&self)
    {
        for x in 0..11
        {
            for y in 0..11
            {
                if self.display_array[x][y] == "0"
                {
                    print!("  ")
                }
                else
                {
                    print!("{} ", self.display_array[x][y]);
                }

            }

            print!("\n")
        }
    }

    fn clear_display_area_and_reset_cursor()
    {
        print!("\x1B[2J\x1B[1;1H");
    }

    fn display_status(&self)
    {
        println!("{} \n", self.status);
    }

    fn display_title_banner()
    {
        println!("===================================");
        println!("HUGH'S SIMPLE SUDOKU SOLVER IN RUST");
        println!("===================================\n");
    }

    fn update_one_square_of_display_array(&mut self, x_of_top_left_in_puzzle_square: usize, y_of_top_left_in_puzzle_square: usize,
                                          x_of_top_left_in_display_square: usize, y_of_top_left_in_display_square: usize)
    {
        for x in 0..3
        {
            for y in 0..3
            {
                self.display_array[x_of_top_left_in_display_square + x][y_of_top_left_in_display_square + y].replace_range
                (.., self.puzzle_array[x_of_top_left_in_puzzle_square + x][y_of_top_left_in_puzzle_square + y].to_string().as_str());
            }
        }
    }

    fn put_lines_into_display_array(& mut self)
    {
        for x in 0..11
        {
            for y in 0..11
            {
                if (y == 3 || y == 7) && (x != 3 && x != 7)
                {
                    self.display_array[y][x].replace_range(.., "-");
                }

                if (x == 3 || x == 7) && (y != 3 && y != 7)
                {
                    self.display_array[y][x].replace_range(.., "|");
                }

                if (x == 3 || x == 7) && (y == 3 || y ==7)
                {
                    self.display_array[y][x].replace_range(.., "+");
                }
            }
        }
    }
}