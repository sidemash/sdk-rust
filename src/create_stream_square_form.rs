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


use crate::hook::Hook;
use crate::stream_square::Size;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateStreamSquareForm {
    pub is_elastic : bool,
    pub size : Size,
    pub hook : Hook,
    pub description : Option<String>,
    pub foreign_data : Option<String>,
    pub play_domain_name : Option<String>,
    pub publish_domain_name : Option<String>
}

impl CreateStreamSquareForm {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        json
    }
}