use json::parse;
use json::JsonValue;
use std::fs;
use crate::http;
use std::collections::HashMap;


pub fn mknew(service: &String) {
    println!("==> attempting to create new project called ({})", service);
}

pub fn deploy(service: &String, target: &String) {
    println!("==> attempting to deploy service ({}) to [{}] JaaS server...", service, target);
    match self::get_key(target) {
        Ok(config) => {
            println!("==> using config {:?}", config);
            let source = self::get_service_source();
            if source == "" {
                println!("==> ERROR: could not load src/index.js!");
                std::process::exit(1);
            }

            let data = object!{
                service: service.to_string(),
                payload: source.to_string()
            };

            let mut headers =  HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("x-jaas-key".to_string(), config.0);

            let response = http::post(&format!("{}/_/deploy", config.1), &data.dump(), headers);
            match response {
                Ok(r) => {
                    println!("==> success! \n{}", r);
                },
                Err(e) => {
                    println!("==> deployment FAILED: \n{}", e);
                    std::process::exit(1);        
                }
            }
        },
        Err(e) => {
            println!("==> Error: {}", e);
            std::process::exit(1);
        }
    }
}

pub fn remove(service: &String, target: &String) {
    // todo (add user challenge to stop accidental service removal)
}

pub fn list(target: &String) {
    // todo
}

fn get_key(target: &String) -> Result<(String, String), String> {
    println!("==> extracting key for ({}) target...", target);
    if let Ok(data) = fs::read_to_string("jaas.config.json") {
        match parse(&data) {
            Ok(_json) => {
                if let JsonValue::Object(server) = &_json[target] {
                    if let Some(key) = server["key"].as_str() {
                        if let Some(remote_server) = server["server"].as_str() {
                            return Ok((key.to_string(), remote_server.to_string()));
                        } else {
                            return Err("unable to find \"server\" in jaas.config.json".to_string());    
                        }
                    } else {
                        return Err("unable to find key in jaas.config.json".to_string());
                    }
                } else {
                    return Err(format!("unable to find target ({}) in jaas.config.json", target));
                }
            },
            Err(e) => {
                return Err(format!("Error, unable to parse jaas.config.json: {}", e));
            }
        }
    } else {
        return Err("unable to read the jaas.config.json file".to_string());
    }
}

fn get_service_source() -> String {
    println!("==> loading source file...");
    if let Ok(data) = fs::read_to_string("src/index.js") {
        return data;
    }
    "".to_string()
}