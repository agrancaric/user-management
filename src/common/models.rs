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

impl SortProperty {
    pub fn new<S: Into<String>>(property: S, direction: SortDirection) -> Self {
        Self {
            property: property.into(),
            direction,
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Page<T> {
    pub total_elements: i64,
    pub content: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(total_elements: i64, content: Vec<T>) -> Self {
        Self {
            total_elements,
            content,
        }
    }
}
