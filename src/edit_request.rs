pub (crate) struct EditRequest
{
    row_number: usize,
    col_number: usize,
    new_value: u16,
}

impl EditRequest
{
    pub fn new_edit_request(row_number: usize, col_number: usize, new_value: u16) -> EditRequest
    {
        let new_edit_request: EditRequest = EditRequest
        {
            row_number,
            col_number,
            new_value,
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
        };

        return new_edit_request;
    }
}