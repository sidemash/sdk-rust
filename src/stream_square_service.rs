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
use crate::create_stream_square_form::CreateStreamSquareForm;
use crate::http::*;
use crate::http;
use crate::list_form::ListForm;
use crate::rest_collection::RestCollection;
use crate::stream_square::StreamSquare;
use crate::update_stream_square_form::UpdateStreamSquareForm;
use serde::{Serialize, Deserialize};


type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StreamSquareService<'a> {
    pub auth : &'a Auth
}

impl StreamSquareService<'_> {
    pub fn new(auth: &Auth) -> StreamSquareService {
        StreamSquareService{auth}
    }

    pub async fn create(&self, form: &CreateStreamSquareForm) -> Result<StreamSquare> {
        post(&*("/".to_owned() + VERSION + "/stream-squares"), &vec![], &vec![], Some(form.to_json()), self.auth).await
    }

    pub async fn get(&self, id: &String) -> Result<StreamSquare> {
        get(&*("/".to_owned() + VERSION + "/stream-squares/" + id), &vec![], &vec![], self.auth).await
    }

    pub async fn list(&self) -> Result<RestCollection<StreamSquare>> {
        http::list(&*("/".to_owned() + VERSION + "/stream-squares"), &vec![], &vec![], self.auth).await
    }

    pub async fn list_with(&self, form: &ListForm<'_>) -> Result<RestCollection<StreamSquare>> {
        http::list(&*("/".to_owned() + VERSION + "/stream-squares"), &vec![], &form.to_query_string(), self.auth).await
    }

    pub async fn update(&self, form: &UpdateStreamSquareForm) -> Result<StreamSquare> {
        patch(&*("/".to_owned() + VERSION + "/stream-squares/" + form.id.as_str()), &vec![], &vec![], Some(form.to_json()), self.auth).await
    }

    pub async fn delete(&self, id: &String) -> Result<()> {
        delete(&*("/".to_owned() + VERSION + "/stream-squares/" + id), &vec![], &vec![], None, self.auth).await
    }
}