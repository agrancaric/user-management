use std::fmt::{Display, Formatter, Result};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl Display for SortDirection {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        write!(formatter, "{:?}", self)
    }
}

#[derive(Deserialize)]
pub struct SortProperty {
    pub property: String,
    pub direction: SortDirection,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Page<T> {
    pub total_elements: i64,
    pub content: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(total_elements: i64, content: Vec<T>) -> Page<T> {
        Self {
            total_elements,
            content,
        }
    }
}
