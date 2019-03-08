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
        .subcommand(SubCommand::with_name("ping")
                    .about("Pings the best server")
                    .arg(Arg::with_name("server")
                         .short("s")
                         .takes_value(true)
                         .help("specify a server number to ping")))
        .get_matches();

    if app.is_present("list") {
        let resp = server::list_servers();
        match resp {
            Ok(n) => print_servers(n),
            Err(n) => println!("Err: {}", n),
        }
    }

    if let Some(app) = app.subcommand_matches("ping") {
        println!("You chose ping");
        println!("Server? {:#?}", app.is_present("server"));
        println!("Server val {:#?}", app.value_of("server"));
    }

    fn print_servers(servers: Vec<Server>) {
        for s in servers {
            println!("{} - ({}) {} {}", s.id, s.sponsor, s.name, s.cc);
        };
    }

}
