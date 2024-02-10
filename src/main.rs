#[macro_use] extern crate json;
extern crate uuid;
extern crate clap;

use clap::{ Arg, App };
use std::env;

mod server;
mod template;
mod cli;
mod http;

fn main() {
    
    let matches = App::new("JaaS, Anhar Hussain Miah, 2021(c)")
                      .version("0.0.1")
                      .author("Anhar Hussain Miah <anharhussainmiah@gmail.com>")
                      .about("JaaS is an open source \"JavaScript as a Service\" sort of serverless i.e \"function as a service\" application")
                      .arg(Arg::with_name("server")
                               .short("s")
                               .long("server")
                               .help("enable server mode for JaaS"))
                      .subcommand(
                          App::new("deploy")
                              .about("deploy your service to a JaaS server")
                              .arg(
                                  Arg::with_name("target")
                                      .index(1)
                                      .required(true)
                                      .takes_value(true)
                              )
                      )
                      .subcommand(
                            App::new("new")
                                .about("scaffold a JaaS function")
                                .arg(
                                    Arg::with_name("project_name")
                                        .index(1)
                                        .required(true)
                                        .takes_value(true)
                                )
                      )
                      .get_matches();
    
    if matches.is_present("server") {
        server::run();
    } else {
        if matches.is_present("new") {
            if let Some(new_matches) = matches.subcommand_matches("new") {
                let project_name = new_matches.value_of("project_name").expect("no project name specified");
                cli::mknew(&project_name.to_string());
                std::process::exit(0);
            }
        }

        if matches.is_present("deploy") {
            if let Some(deploy_matches) = matches.subcommand_matches("deploy") {
                let target = deploy_matches.value_of("target").expect("no target specified");
                let service = self::get_curret_service();
                cli::deploy(&service, &target.to_string());
                std::process::exit(0);
            }
        }


        // handle delete command
        // jass rm foo prod


        // handle list command
        // jass ls prod
    }
}

fn get_curret_service() -> String {
    // not sure why getting *just* the current direcotry name needs to be so weird
    let mut service = "".to_string();
    let pbuff = env::current_dir().expect("unable to get pathBuff");
    for component in pbuff.components() {
        if let std::path::Component::Normal(_c) = component {
            service = _c.to_os_string().into_string().expect("unable to get os string");
        }
    }
    service
}
