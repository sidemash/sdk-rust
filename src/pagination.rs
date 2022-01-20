/*
   Copyright © 2020 Sidemash Cloud Services

   Licensed under the Apache  License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless  required  by  applicable  law  or  agreed to in writing,
   software  distributed  under  the  License  is distributed on an
   "AS IS"  BASIS, WITHOUT  WARRANTIES  OR CONDITIONS OF  ANY KIND,
   either  express  or  implied.  See the License for the  specific
   language governing permissions and limitations under the License.
*/


use crate::utc_date_time::UTCDateTime;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub self_url : String,
    pub prev_url : Option<String>,
    pub next_url : Option<String>,
    pub started_time : UTCDateTime,
    pub nb_items_on_this_page : i32,
    pub page : i32,
    pub nb_items_per_page : i32
}

impl Pagination {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        json
    }
}