extern crate canteen;

use canteen::utils;
use canteen::*;
use json::parse;
use uuid::Uuid;
use std::fs;
use std::io::Write;
use std::process::{ Command, Stdio };
use crate::template;

pub fn run() {
    println!("{}", self::jaas_logo());

    self::init();
    let port = 8080;

    let mut web = Canteen::new();
    web.bind(("0.0.0.0", port));
    web.set_default(utils::err_404);
    web.add_route("/_/deploy", &[Method::Post], deploy);
    web.add_route("/<str:service>", &[Method::Post], execute);
    println!("==> JaaS is running and listening on 0.0.0.0 PORT [{}]", port);
    web.run();
}

fn deploy(req: &Request) -> Response {
    /*
        this API method is straight forward, we:
            * authenticate the server key in the header x-jaas-key
            * we then take the service name
            * make sure the service name is not called deploy and name is alphanumeric with dashes and no more then 50 characters in length
            * create a directory with the same name as the service
            * dump payload data into index.js file for example:
                if we had a service called "hello-world"
                we would end up with the following folder data/hello-world/index.js
            * return any errors
    */
    println!("==> new JaaS service deployment requested...");
    let mut res = Response::new();

    let raw = String::from_utf8_lossy(&req.payload);
    let x_jaas_key = req.get_header("x-jaas-key").expect("Unable to get header key");
    let server_key = self::get_key();

    if x_jaas_key != server_key {
        println!("==> rejecting invalid key ({})", x_jaas_key);
        res.set_status(401);
        res.append("invalid key");
        return res;
    }

    if let Ok(_data) = parse(&raw) {
        let service = _data["service"].to_string();
        println!("==> service name ({})", service);

        // TODO: validate service name!!
        let payload = _data["payload"].to_string();

        if payload == "" {
            res.set_status(401);
            res.append("payload load can not be empty");
            return res;
        }

        // we have the service + payload we just need to deploy/update this service
        self::create_service_folder(&service);
        println!("==> deploying new service ({})", service);
        fs::write(format!("data/{}/service.js", service), payload.to_string());
        // write out JaaS wrapper
        fs::write(format!("data/{}/index.js", service), template::index_js());

        res.set_status(200);
        res.append("{\"status\": \"deployed\"}");
        return res;
    }

    res.set_status(200);
    res.set_content_type("application/json");
    res.append("{'status': 'todo'}".to_string());
    res
}

fn database(req: &Request) -> Response {
    println!("==> internal http database call...");
    let mut res = Response::new();
    res.set_status(200);
    res.set_content_type("application/json");
    let raw = String::from_utf8_lossy(&req.payload);
    res.append("{'status': 'todo'}".to_string());
    res
}

fn execute(req: &Request) -> Response {
    /*
        this API is the juicy part of the whole JaaS server. based on the path we try and find the matching
        service and then we execute that function, giving it access to native calls for both HTTP and database
        we then return whatever this sandboxed function returns.

        we also need to handle in errors here as well, for example service doesn't exist OR executated with an error
    */
    let mut res = Response::new();
    res.set_content_type("application/json");

    let service: String = req.get("service");
    println!("==> new request to execute service [{}]", service);

    // TODO: VALIDATE SERVICE!!

    let raw = String::from_utf8_lossy(&req.payload);
    // our RAW is our input data to be consumed by the service, we need to load the
    // TODO: we need to setup a database token so that our script can make DB calls over internal http


    let source = self::get_service_source(&service);
    let session = Uuid::new_v4();

    let mut output = Command::new("deno")
                         .arg("run")
                         .arg(format!("--allow-write=data/{}/", service))
                         .arg(format!("data/{}/index.js", service))
                         .arg(session.to_string())
                         .arg(service.to_string())
                         .stdin(Stdio::piped())
                         .spawn()
                         .expect("unable to spawn deno sub-process");
    
    {
        let stdin = output.stdin.as_mut().expect("Failed to open stdin");
        stdin.write_all(&raw.as_bytes()).expect("Failed to write to stdin");
    }

    let _output = output.wait_with_output().expect("Failed to read stdout");

    let result = self::get_session_output(&service, &session.to_string());
    self::kill_sesson(&service, &session.to_string());
    if result != "" {
        res.set_status(200);
        res.append(result);
        return res;
    }
    res.set_status(500);
    res.append("FAILED!"); // we need to somehow pass back any script failures
    res
}

fn jaas_logo() -> &'static str {
    let logo = r#"

             ██  █████   █████  ███████ 
             ██ ██   ██ ██   ██ ██      
             ██ ███████ ███████ ███████ 
        ██   ██ ██   ██ ██   ██      ██ 
         █████  ██   ██ ██   ██ ███████ 
                                       
         all functions are belong to us!
          (booting up the JaaS server)
    "#;
    logo
}


fn init() {
    println!("==> initilising the server");
    if !self::is_key_exist() {
        println!("==> new key generated -> {}", self::generate_new_key());
    } else {
        println!("==> JaaS server key has been found");
    }
    self::create_data_folder();
}

fn is_key_exist() -> bool {
    println!("==> checking to see if JaaS server key exists");
    std::path::Path::new("key").exists()
}

fn generate_new_key() -> String {
    println!("==> generating a new JaaS key..");
    let key = Uuid::new_v4();
    fs::write("key", key.to_string());
    key.to_string()
}

fn get_key() -> String {
    if let Ok(data) = fs::read_to_string("key") {
        return data;
    }
    "".to_string()
}

fn create_data_folder() {
    println!("==> settting up the data directory");
    fs::create_dir("data/");
}

fn create_service_folder(service: &String) {
    println!("==> createing service directory ({})", service);
    fs::create_dir(format!("data/{}", service));
}

fn is_valid_service_name(serivce: &String) -> bool {
    // todo (max 50 chars, alpha-numeric with dash only)
    false
}

fn is_service_exist(service: &String) -> bool {
    let path_to_service = format!("data/{}/index.js", service);
    std::path::Path::new(&path_to_service).exists()
}

fn get_service_source(service: &String) -> String {
    println!("==> loading source file for ({})", service);
    if let Ok(data) = fs::read_to_string(format!("data/{}/index.js", service)) {
        return data;
    }
    "".to_string()
}

fn get_session_output(service: &String, session: &String) -> String {
    println!("==> getting session result for ({})", service);
    if let Ok(data) = fs::read_to_string(format!("data/{}/{}", service, session)) {
        return data;
    }
    "".to_string()
}

fn kill_sesson(service: &String, session: &String) {
    println!("==> killing session ({}) for ({})", session, service);
    fs::remove_file(format!("data/{}/{}", service, session));
}