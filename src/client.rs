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


use crate::auth::Auth;
use crate::stream_square_service::StreamSquareService;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Client<'a> {
    pub auth : Auth,
    pub stream_square : StreamSquareService
}

impl Client<'_> {
    pub fn new(auth: &Auth) -> Client {
        Client {
            auth,
            stream_square : StreamSquareService::new(auth)
        }
    }
}