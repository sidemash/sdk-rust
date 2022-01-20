use crate::pagination::Pagination;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RestCollection<T> {
    pub url : String,
    pub pagination : Pagination,
    pub items : Vec<T>
}