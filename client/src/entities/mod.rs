pub mod todo;

use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub count: usize,
    pub total: usize,
    pub page: usize,
    pub page_count: usize,
}
