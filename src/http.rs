use reqwest::header::{HeaderMap, HeaderValue, HeaderName};
use std::collections::HashMap;
use reqwest::StatusCode;
use std::str::FromStr;
use std::io::Read;
use reqwest;


pub fn post(url: &String, data: &String, headers: HashMap<String, String>) -> Result<String, String> {
    println!("==> http post request to {}", url);
    let client   = reqwest::Client::new();
    let response = client.post(url)
                         .headers(self::to_headermap(headers))
                         .body(data.to_string())
                         .send();
    
    match response {
        Ok(mut res) => {
            if res.status() != 200 {
                if let Ok(msg) = self::unwrap_response(&mut res) {
                    return Err(msg);
                }
                return Err("remote server did not return HTTP 200".to_string());
            }
            return self::unwrap_response(&mut res);
        },
        Err(x) => Err(x.to_string()) // who even knows anymore
    }
}

fn to_headermap(headers: HashMap<String, String>) -> HeaderMap {
    let mut map = HeaderMap::new();

    for i in headers.keys() {
        if let Some(v) = headers.get(i) {
            if let Ok(x) = HeaderValue::from_str(v) {
                if let Ok(n) = HeaderName::from_str(&i) {
                    map.insert(n, x);
                }
            }
        }
    }
    map
}

fn unwrap_response(res: &mut reqwest::Response) -> Result<String, String> {
    let mut buff = String::new();
    let len      = res.read_to_string(&mut buff);
    match len {
        Ok(_) => Ok(buff),
        Err(x) => Err(x.to_string())
    }
}