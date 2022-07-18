use clap::{value_parser, App, AppSettings, Arg, Command};
use speedtestr::{server, server::Server};

/// Number of servers to test to determine the "best" servers
/// this is currently not configurable
const NUM_SERVERS_BEST_SERVER: &str = "3";
/// Default number of pings to use to test latency
/// this is configurable with the *-p* option
const NUM_PINGS_DEFAULT: &str = "3";

fn main() {
    let app = App::new("speedtestr")
        .version("0.0.1")
        .about("Unofficial speedtest cli")
        .author("Zach Peters")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(Command::new("list").about("Lists available servers"))
        .subcommand(
            Command::new("upload").about("Upload test").arg(
                Arg::new("bytes")
                    .short('b')
                    .takes_value(true)
                    .help("Number of bytes to upload"),
            ),
        )
        .subcommand(
            Command::new("download").about("Download test").arg(
                Arg::new("bytes")
                    .short('b')
                    .takes_value(true)
                    .help("Number of bytes to download"),
            ),
        )
        .subcommand(
            Command::new("ping")
                .about("Pings the best server")
                .arg(
                    Arg::new("numpings")
                        .short('p')
                        .takes_value(true)
                        .value_parser(value_parser!(u128))
                        .default_value(NUM_PINGS_DEFAULT)
                        .help("Number of pings to test with"),
                )
                .arg(
                    Arg::new("server")
                        .short('s')
                        .takes_value(true)
                        .help("Specify a server number to ping"),
                ),
        )
        .get_matches();

    match app.subcommand() {
        Some(("list", _)) => {
            let resp = server::list_servers();
            match resp {
                Ok(n) => print_servers(n),
                Err(e) => println!("Err: {:?}", e),
            }
        }
        // TODO refactor
        // - dedeup if
        // - send int instead of string, use underscores for legibility
        Some(("upload", sub_matches)) => {
            let bytes = sub_matches.get_one::<String>("bytes");
            if let Some(b) = bytes {
                println!("Upload bytes: {:?}", b);
                let best = server::best_server(NUM_SERVERS_BEST_SERVER).unwrap();
                let dl = server::upload(best.id.as_str(), b).unwrap();
                println!("Upload Results {:#?} mbps", dl);
            } else {
                let bytes = "50000024";
                println!("Upload Bytes: {} (default)", bytes);
                let best = server::best_server(NUM_SERVERS_BEST_SERVER).unwrap();
                let dl = server::upload(best.id.as_str(), bytes).unwrap();
                println!("Upload Results {:#?} mbps", dl);
            }
        }
        // TODO refactor
        // - dedeup if
        // - send int instead of string, use underscores for legibility
        Some(("download", sub_matches)) => {
            let bytes = sub_matches.get_one::<String>("bytes");
            if let Some(b) = bytes {
                println!("Download bytes: {:?}", b);
                let best = server::best_server(NUM_SERVERS_BEST_SERVER).unwrap();
                let dl = server::download(best.id.as_str(), b).unwrap();
                println!("Download Results {:#?} mbps", dl);
            } else {
                let bytes = "100000024";
                println!("Download Bytes: {} (default)", bytes);
                let best = server::best_server(NUM_SERVERS_BEST_SERVER).unwrap();
                let dl = server::download(best.id.as_str(), bytes).unwrap();
                println!("Download Results {:#?} mbps", dl);
            }
        }
        // TODO refactor
        // - clean up ref/deref, it looks gross
        // - deduplicate the if let statement
        Some(("ping", sub_matches)) => {
            let n = sub_matches.get_one::<u128>("numpings").unwrap();
            println!("num pings: {:?}", n);

            let svr = sub_matches.get_one::<String>("server");
            if let None = svr {
                let best = server::best_server(NUM_SERVERS_BEST_SERVER).unwrap();
                let resp = server::ping_server(&best.id, *n);
                println!("Avg ms: {}", resp.unwrap());
            } else {
                let resp = server::ping_server(svr.unwrap(), *n);
                println!("Avg ms: {}", resp.unwrap());
            }
        }
        _ => unreachable!("Exhaused list of subsommands"),
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
