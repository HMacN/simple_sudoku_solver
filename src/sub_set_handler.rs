use crate::edit_request::EditRequest;

pub (crate) struct SubSetHandler
{
    coords_of_cells: [[usize; 2]; 9],
}

impl SubSetHandler
{
    pub fn create_new_subset(coords: [[usize; 2]; 9]) -> SubSetHandler
    {
        let new_subset: SubSetHandler = SubSetHandler
        {
            coords_of_cells: coords,
        };

        return new_subset;
    }

    pub fn is_this_subset_valid(&self, puzzle_array: [[u16; 9]; 9]) -> bool
    {
        for x in 1..10
        {
            if self.count_instances_of_non_zero_number_in_subset(x, puzzle_array) > 1
            {
                return false;
            }
        }

        return true;
    }

    pub fn infer_final_value(&self, puzzle_array: [[u16; 9]; 9]) -> EditRequest
    {
        let mut edit_request_to_return: EditRequest = EditRequest::new_edit_request(0, 0, 0, false);

        if self.only_one_empty_cell(puzzle_array)
        {
            let missing_cell_index: usize = self.find_first_empty_cell_index(puzzle_array);
            let missing_cell_value: u16 = self.find_lowest_missing_value(puzzle_array);

            edit_request_to_return = EditRequest::new_edit_request(
                self.coords_of_cells[missing_cell_index][0],
                self.coords_of_cells[missing_cell_index][1],
                missing_cell_value, true);
        };

        return edit_request_to_return;
    }

    pub fn get_empty_cell_count(&self, puzzle_array: [[u16; 9]; 9]) -> u16
    {
        let mut empty_cell_count: u16 = 0;

        for cell in 0..9
        {
            if self.get_contents_of_subset_cell(puzzle_array, cell) == 0
            {
                empty_cell_count = empty_cell_count + 1;
            }
        }

        return empty_cell_count;
    }

    pub fn get_guess_cell(&self, puzzle_array: [[u16; 9]; 9]) -> EditRequest
    {
        let index_of_target_cell: usize = self.find_first_empty_cell_index(puzzle_array);
        let value_of_guess: u16 = self.find_lowest_missing_value(puzzle_array);
        let edit_request_containing_target_cell: EditRequest = EditRequest::new_edit_request(
            self.coords_of_cells[index_of_target_cell][0],
            self.coords_of_cells[index_of_target_cell][1],
            value_of_guess, false);

        return edit_request_containing_target_cell;
    }
}

impl SubSetHandler
{
    fn count_instances_of_non_zero_number_in_subset(&self, number_to_search_for: u16, puzzle_array: [[u16; 9]; 9]) -> u16
    {
        let mut instances_of_number: u16 = 0;

        for x in 0..9
        {
            if self.get_contents_of_subset_cell(puzzle_array, x) == number_to_search_for
            {
                instances_of_number = instances_of_number + 1;
            }
        }

        return instances_of_number;
    }

    fn get_contents_of_subset_cell(&self, puzzle_array: [[u16; 9]; 9], set_cell_num: usize) -> u16
    {
        return puzzle_array[self.coords_of_cells[set_cell_num][0]][self.coords_of_cells[set_cell_num][1]];
    }

    fn only_one_empty_cell(&self, puzzle_array: [[u16; 9]; 9]) -> bool
    {
        if self.get_empty_cell_count(puzzle_array) == 1
        {
            return true;
        }

        return false;
    }

    fn find_first_empty_cell_index(&self, puzzle_array: [[u16; 9]; 9]) -> usize
    {
        for cell_index in 0..9
        {
            if self.get_contents_of_subset_cell(puzzle_array, cell_index) == 0
            {
                return cell_index;
            }
        };

        println!("SubSetHandler.find_empty_cell_index did not find an empty cell index!");
        return 20;
    }

    fn find_lowest_missing_value(&self, puzzle_array: [[u16; 9]; 9]) -> u16
    {
        let mut numbers_checklist: [bool; 10] = [false; 10];

        for number in 1..10
        {
            numbers_checklist[self.get_contents_of_subset_cell(puzzle_array, number - 1) as usize] = true;
        };

        for missing_cell_index in 1..10
        {
            if numbers_checklist[missing_cell_index] == false
            {
                return missing_cell_index as u16;
            }
        }

        println!("SubSetHandler.find_missing_value did not find a missing value!");
        return 20;
    }
}

impl Default for SubSetHandler
{
    fn default() -> Self
    {
        let default_subset: SubSetHandler = SubSetHandler
        {
            coords_of_cells: [[0; 2]; 9],
        };

        return default_subset;
    }
}