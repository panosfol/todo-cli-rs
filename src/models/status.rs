//An enum to track the status of the 'todo' entries
#[derive(Clone)]
pub enum Status {
    _Abandoned,
    Active,
    _Done,
}

impl Default for Status {
    fn default() -> Self {
        Self::Active
    }
}
