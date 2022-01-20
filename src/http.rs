use base64;
use serde::Deserialize;
use reqwest::{Method, Client};
// use std::time::SystemTime;
use hmac::{Hmac, Mac, NewMac};
use sha2::{Sha256, Sha512, Digest};
use crate::http_request::{HttpRequest, Headers, Body, QueryString};
use crate::auth::Auth;
use crate::rest_collection::RestCollection;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;


pub (crate) async fn list<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, auth: &Auth) -> Result<RestCollection<T>> {
    call(Method::GET, path, headers, &query_string, None, auth).await
}
pub (crate) async fn get<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, auth: &Auth) -> Result<T> {
    call(Method::GET, path, headers, &query_string, None, auth).await
}
pub (crate) async fn post<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, body:Body, auth: &Auth) -> Result<T> {
    call(Method::POST, path, headers, query_string, body, auth).await
}
pub (crate) async fn put<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, body:Body, auth: &Auth) -> Result<T> {
    call(Method::PUT, path, headers, query_string, body, auth).await
}
pub (crate) async fn patch<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, body:Body, auth: &Auth) -> Result<T> {
    call(Method::PATCH, path, headers, query_string, body, auth).await
}
pub (crate) async fn delete<T: for<'de> Deserialize<'de>>(path: &str, headers: &Headers, query_string: &QueryString, body:Body, auth: &Auth) -> Result<T> {
    call(Method::DELETE, path, headers, query_string, body, auth).await
}

pub (crate) static HOST : &'static str = "http://dev-api.sidemash.io";
pub (crate) static VERSION : &'static str = "v1.0";

type HmacSha512 = Hmac<Sha512>;
fn sign(message: String, private_key: &String) -> String {
    let mut hmac= HmacSha512::new_varkey(private_key.as_bytes()).unwrap();
    hmac.update(message.as_bytes());
    base64::encode(hmac.finalize().into_bytes())
}
fn sha256(message: &String) -> String {
    let output = Sha256::digest(message.as_bytes());
    base64::encode(output.as_slice())
}
fn compute_signed_headers<'a>(body: &Option<String>, headers:&Headers, auth: &Auth) -> Headers {
    let mut result : Headers = headers.clone();
    result.push((String::from("Accept"), String::from("application/json")));
    result.push((String::from("User-Agent"), String::from("Sdk Rust ") + VERSION));
    result.push((String::from("Authorization"), String::from("Bearer ") + &auth.token));
    if body.is_some() { result.push( (String::from("Content-Type"), String::from("application/json")))  }
    result
}
async fn call<T: for<'de> Deserialize<'de>>(method:Method, path: &str, headers: &Headers, query_string: &QueryString, body:Body, auth: &Auth) -> Result<T> {
    let url = String::from(HOST) + path;
    let mut signed_headers = compute_signed_headers(&body, headers, auth);
    let body_hash = body.as_ref().map(sha256);
    let sdm_request = HttpRequest::new(&signed_headers, &method.as_str(), path, &query_string, body_hash);
    let signature = sign(sdm_request.to_message(), &auth.secret_key);
    let mut signed_header_value = String::new();
    let mut i = 0;
    for (header_name, _) in &signed_headers {
        if i != 0 { signed_header_value.push_str(", ") };
        signed_header_value.push_str(header_name.as_str());
        i = i + 1;
    }
    let mut request_builder = Client::new().request(method.to_owned(), url.as_str()).query(&query_string);
    let mut all_headers = signed_headers.clone();
    all_headers.append(&mut vec![
        (String::from("X-Sdm-SignedHeaders"), signed_header_value),
        (String::from("X-Sdm-Nonce"), sdm_request.nonce.to_string()),
        (String::from("X-Sdm-Signature"), String::from("SHA512 ") + &signature),
    ]);
    if body.is_some() { request_builder = request_builder.body(body.unwrap())  }
    for (header_name, header_value) in all_headers {
        request_builder = request_builder.header(header_name.as_str(), header_value.as_str())
    }
    let response = request_builder.send().await?;
    let status_code = response.status().as_u16();
    if 200 <= status_code && status_code < 300 {
        let obj:  T = response.json().await?;
        Ok(obj)
    }
    else {
        let mut err_message = String::new();
        err_message.push_str(response.status().as_str() );
        err_message.push_str("  ");
        err_message.push_str(response.text().await?.as_str());
        panic!(err_message)
    }
}