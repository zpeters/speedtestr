extern crate clap;
extern crate speedtestr;
use clap::{App, AppSettings, Arg, SubCommand};
use speedtestr::{server, server::Server};

fn main() {
    let app = App::new("speedtestr")
        .version("0.0.1")
        .about("Unofficial speedtest cli")
        .author("Zach Peters")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("List").about("Lists available servers"))
        .subcommand(
            SubCommand::with_name("Upload")
                .arg (
                    Arg::with_name("bytes")
                        .short("b")
                        .takes_value(true)
                        .help("Number of bytes to upload"),
                )
                .arg(
                    Arg::with_name("Server")
                        .short("s")
                        .takes_value(true)
                        .help("specify a server number to ping"),
                )
                .about("Upload test"),
        )
        .subcommand(
            SubCommand::with_name("Download")
                .arg (
                    Arg::with_name("bytes")
                        .short("b")
                        .takes_value(true)
                        .help("Number of bytes to download"),
                )
                .about("Download test"),
        )
        .subcommand(
            SubCommand::with_name("Ping")
                .about("Pings the best server")
                .arg (
                    Arg::with_name("numpings")
                        .short("p")
                        .takes_value(true)
                        .help("Number of pings to test with"),
                )
                .arg (
                    Arg::with_name("Server")
                        .short("s")
                        .takes_value(true)
                        .help("specify a server number to ping"),
                ),
        )
        .get_matches();

    if app.is_present("List") {
        let resp = server::list_servers();
        match resp {
            Ok(n) => print_servers(n),
            Err(e) => println!("Err: {:?}", e),
        }
    }

    if let Some(app) = app.subcommand_matches("Download") {
        let best = server::best_server("3").unwrap().to_owned();
        let bytes = app.value_of("bytes").unwrap_or("100000024");
        let svr = if app.is_present("Server") {
            app.value_of("Server").unwrap()
        } else {
            best.id.as_str()
        };
        let dl = server::download(best.id.as_str(), bytes).unwrap();
        println!("Download Results {:#?} mbps", dl);
    }

    if let Some(app) = app.subcommand_matches("Upload") {
        let best = server::best_server("3").unwrap().to_owned();
        let bytes = app.value_of("bytes").unwrap_or("50000024");
        let svr = if app.is_present("Server") {
            app.value_of("Server").unwrap()
        } else {
            best.id.as_str()
        };
        let dl = server::upload(best.id.as_str(), bytes).unwrap();
        println!("Upload Results {:#?} mbps", dl);
    }

    if let Some(app) = app.subcommand_matches("Ping") {
        let best = server::best_server("3").unwrap().to_owned();
        let num_pings = app
            .value_of("numpings")
            .unwrap_or("3")
            .parse::<u128>()
            .unwrap();
        ;

        println!("[ping test]");
        let svr = if app.is_present("Server") {
            app.value_of("Server").unwrap()
        } else {
            best.id.as_str()
        };

        let resp = server::ping_server(svr, num_pings);
        println!("Avg ms: {}", resp.unwrap());
    }

    fn print_servers(servers: Vec<Server>) {
        for s in servers {
            println!(
                "{} - [distance {}] - ({}) {} {}",
                s.id, s.distance, s.sponsor, s.name, s.cc
            );
        }
    }
}
