use crate::http_request::QueryString;
use serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListForm <'a>{
    pub order_by: &'a Option<&'a String>,
    pub limit : Option<u16>,
    pub _where : &'a Option<&'a String>
}

impl ListForm<'_> {

    pub fn to_query_string(&self) -> QueryString  {
        let mut qs = vec![];
        if self.order_by.is_some() { qs.push((String::from("orderBy"), self.order_by.as_ref().unwrap().to_string()) ) }
        if self.limit.is_some() { qs.push((String::from("limit"), self.limit.as_ref().unwrap().to_string()) ) }
        if self._where.is_some() { qs.push((String::from("where"), self._where.as_ref().unwrap().to_string()) ) }
        qs
    }
}