use crate::sub_set_handler::SubSetHandler;

pub(crate) struct SubSetHandlerFactory
{

}

impl SubSetHandlerFactory
{
    pub fn create_starting_subsets() -> [SubSetHandler; 27]
    {
        let mut subsets_to_return: [SubSetHandler; 27] = Default::default();
        let mut entry_counter: usize = 0;

        for x in 0..9
        {
            subsets_to_return[entry_counter] = SubSetHandlerFactory::create_row_subset(x);
            entry_counter = entry_counter + 1;
        }

        for x in 0..9
        {
            subsets_to_return[entry_counter] = SubSetHandlerFactory::create_column_subset(x);
            entry_counter = entry_counter + 1;
        }

        for x in 0..3
        {
            for y in 0..3
            {
                subsets_to_return[entry_counter] = SubSetHandlerFactory::create_square_subset(x * 3, y * 3);
                entry_counter = entry_counter + 1;
            }
        }

        return subsets_to_return;
    }

    fn create_row_subset(row_num: usize) -> SubSetHandler
    {
        let x_y_coords_of_cells: [[usize; 2]; 9] =
            [[row_num, 0], [row_num, 1], [row_num, 2], [row_num, 3], [row_num, 4], [row_num, 5], [row_num, 6], [row_num, 7], [row_num, 8]];

        return SubSetHandler::create_new_subset(x_y_coords_of_cells);
    }

    fn create_column_subset(col_num: usize) -> SubSetHandler
    {
        let x_y_coords_of_cells: [[usize; 2]; 9] =
            [[0, col_num], [1, col_num], [2, col_num], [3, col_num], [4, col_num], [5, col_num], [6, col_num], [7, col_num], [8, col_num]];

        return SubSetHandler::create_new_subset(x_y_coords_of_cells);
    }

    fn create_square_subset(tp_lft_cell_row_num: usize, tp_lft_cell_col_num: usize) -> SubSetHandler
    {
        let x_y_coords_of_cells: [[usize; 2]; 9] =
            [[tp_lft_cell_row_num, tp_lft_cell_col_num],     [tp_lft_cell_row_num + 1, tp_lft_cell_col_num],     [tp_lft_cell_row_num + 2, tp_lft_cell_col_num],
         [tp_lft_cell_row_num, tp_lft_cell_col_num + 1], [tp_lft_cell_row_num + 1, tp_lft_cell_col_num + 1], [tp_lft_cell_row_num + 2, tp_lft_cell_col_num + 1],
         [tp_lft_cell_row_num, tp_lft_cell_col_num + 2], [tp_lft_cell_row_num + 1, tp_lft_cell_col_num + 2], [tp_lft_cell_row_num + 2, tp_lft_cell_col_num + 2]];

        return SubSetHandler::create_new_subset(x_y_coords_of_cells);
    }
}