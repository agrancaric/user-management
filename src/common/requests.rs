use serde::Deserialize;
use validator::Validate;

use super::models::SortProperty;

#[derive(Deserialize, Validate)]
pub struct PageRequest {
    #[validate(range(min = 1, max = 1000))]
    pub limit: i64,
    #[validate(range(min = 0))]
    pub offset: i64,
    pub sort_properties: Option<Vec<SortProperty>>,
}
