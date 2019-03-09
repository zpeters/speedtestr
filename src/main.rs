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
        .subcommand(SubCommand::with_name("best")
                    .about("Best server")
                    .arg(Arg::with_name("numclosest")
                         .short("n")
                         .default_value("3")
                         .takes_value(true)
                         .help("number of closest servers to test")))
        .subcommand(SubCommand::with_name("ping")
                    .about("Pings the best server")
                    .arg(Arg::with_name("server")
                         .short("s")
                         .takes_value(true)
                         .help("specify a server number to ping")))
        .get_matches();

    if let Some(app) = app.subcommand_matches("best") {
        let best = server::best_server(app.value_of("numclosest").unwrap());
        println!("Best Server: {:#?}", best)
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
