use std::fmt::{Display, Formatter, Result};

use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::Deserialize;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
