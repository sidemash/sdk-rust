use std::time::SystemTime;

pub(crate) type Headers = Vec<(String, String)>;
pub type QueryString = Vec<(String, String)>;
pub(crate) type Body = Option<String>;

pub struct HttpRequest<'a> {
    pub (crate) nonce : u128,
    pub (crate) signed_headers : &'a Headers,
    pub (crate) method : &'a str,
    pub (crate) path : &'a str,
    pub (crate) query_string : &'a QueryString,
    pub (crate) body_hash : Option<String>,
}

impl HttpRequest<'_> {
    pub(crate) fn new<'a>(signed_headers : &'a Headers, method : &'a str, path : &'a str,  query_string : &'a QueryString, body_hash : Option<String>) -> HttpRequest <'a> {
        HttpRequest {
            nonce : SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis(),
            signed_headers,
            method,
            path,
            query_string,
            body_hash
        }
    }

    pub(crate) fn to_message(&self) -> String {
        let mut s = self.nonce.to_string();
        for (header_name, header_value) in self.signed_headers {
            s.push_str(header_name);
            s.push_str(":");
            s.push_str(header_value);
        }
        s.push_str(self.method);
        s.push_str(self.path);
        if !self.query_string.is_empty() { s.push_str("?"); }
        let mut i = 0;
        for (key, value) in self.query_string {
            if i != 0 { s.push_str("&")}
            s.push_str(key);
            s.push_str("=");
            s.push_str(value);
            i = i + 1;
        }
        if self.body_hash.is_some() { s.push_str(self.body_hash.as_ref().unwrap().as_str()); }
        s
    }
}