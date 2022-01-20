use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum Purpose {
   Play,
   Publish,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    #[serde(rename = "_type")]
    pub _type : String,
    pub id : String,
    pub url : String,
    pub purpose : Purpose,
    pub description : Option<String>,
    pub foreign_data : Option<String>
}

impl Domain {
    pub fn to_json(&self) -> String {
        let json =  serde_json::to_string(&self).unwrap();
        json
    }

    pub fn from_json(json: &String) -> Domain {
        let domain =  serde_json::from_str(&json).unwrap();
        domain
    }
}