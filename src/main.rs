extern crate clap;
extern crate speedtestr;

use clap::{Arg, App, AppSettings, SubCommand};
use speedtestr::{server, server::Server};

fn main() {
    let app = App::new("speedtestr")
        .version("0.0.1")
        .about("Unofficial speedtest cli")
        .author("Zach Peters")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("list").about("Lists available servers"))
        .subcommand(SubCommand::with_name("best").about("Best server"))
        .subcommand(SubCommand::with_name("ping")
                    .about("Pings the best server")
                    .arg(Arg::with_name("server")
                         .short("s")
                         .takes_value(true)
                         .help("specify a server number to ping")))
        .get_matches();

    if app.is_present("best") {
        let mut servers = server::list_servers().unwrap();
        servers.sort_by_key(|s| s.distance);
        servers.truncate(3);
        servers.iter_mut().for_each(|s| s.latency = server::ping_server(&s.id).unwrap());
        servers.sort_by_key(|s| s.latency);
        println!("Sorted Servers: {:#?}", servers);
        println!("Best server: {:#?}", servers[0]);
    }

    if app.is_present("list") {
        let resp = server::list_servers();
        match resp {
            Ok(n) => print_servers(n),
            Err(n) => println!("Err: {}", n),
        }
    }

    if let Some(app) = app.subcommand_matches("ping") {
        println!("Pinging...");
        if app.is_present("server") {
            let server = app.value_of("server").unwrap();
            let resp = server::ping_server(server);
            match resp {
                Ok(ms) => println!("Ping {} took {} ms", server, ms),
                Err(e) => println!("[Error] {}", e),
            }
        } else {
            println!("Ping but no server specified")
        };
    }

    fn print_servers(servers: Vec<Server>) {
        for s in servers {
            println!("{} - [distance {}] - ({}) {} {}", s.id, s.distance, s.sponsor, s.name, s.cc);
        };
    }

}
