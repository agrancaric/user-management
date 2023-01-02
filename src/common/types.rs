use std::fmt::{Display, Formatter, Result};

use serde::Deserialize;

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
