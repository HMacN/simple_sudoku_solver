pub (crate) struct EditRequest
{
    row_number: usize,
    col_number: usize,
    new_value: u16,
    make_this_edit: bool,
}

impl EditRequest
{
    pub fn new_edit_request(row_number: usize, col_number: usize, new_value: u16, make_this_edit: bool) -> EditRequest
    {
        let new_edit_request: EditRequest = EditRequest
        {
            row_number,
            col_number,
            new_value,
            make_this_edit,
        };

        return new_edit_request;
    }

    pub fn get_row_number(&self) -> usize
    {
        return self.row_number;
    }

    pub fn get_col_number(&self) -> usize
    {
        return self.col_number;
    }

    pub fn get_new_value(&self) -> u16
    {
        return self.new_value;
    }

    pub fn get_make_this_edit(&self) -> bool
    {
        return self.make_this_edit;
    }
}

impl Default for EditRequest
{
    fn default() -> Self
    {
        let new_edit_request: EditRequest = EditRequest
        {
            row_number: 0,
            col_number: 0,
            new_value: 0,
            make_this_edit: false
        };

        return new_edit_request;
    }
}