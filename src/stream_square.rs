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
use crate::instance_status::InstanceStatus;
use crate::publish::Publish;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "PascalCase", tag = "_type")]
pub enum Size {
    S,
    M,
    L,
    XL,
    XXL
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamSquare {
    pub id : String,
    pub url : String,
    pub status : InstanceStatus,
    pub is_elastic : bool,
    pub size : Size,
    pub play_domain_name : Option<String>,
    pub publish_domain_name : Option<String>,
    pub publish : Publish,
    pub hook : Hook,
    pub description : Option<String>,
    pub foreign_data : Option<String>
}

impl StreamSquare {
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        json
    }
}