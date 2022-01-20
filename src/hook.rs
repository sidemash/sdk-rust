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


use crate::http_method::HttpMethod;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase", tag = "_type")]
pub enum Hook {
    HttpCall{method : HttpMethod, url : String},
    WsCall{method : HttpMethod, url : String},
}