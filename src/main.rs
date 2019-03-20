extern crate clap;
extern crate speedtestr;
#[macro_use]
extern crate log;
extern crate simplelog;

use clap::{App, AppSettings, Arg, SubCommand};
use simplelog::*;
use speedtestr::{server, server::Server};

fn main() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Error, Config::default()).unwrap()
    ])
    .unwrap();
    info!("speedtestr init");

    let app = App::new("speedtestr")
        .version("0.0.1")
        .about("Unofficial speedtest cli")
        .author("Zach Peters")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommand(SubCommand::with_name("list").about("Lists available servers"))
        .subcommand(
            SubCommand::with_name("upload")
                .arg(
                    Arg::with_name("bytes")
                        .short("b")
                        .takes_value(true)
                        .help("Number of bytes to upload"),
                )
                .about("Upload test"),
        )
        .subcommand(
            SubCommand::with_name("download")
                .arg(
                    Arg::with_name("bytes")
                        .short("b")
                        .takes_value(true)
                        .help("Number of bytes to download"),
                )
                .about("Download test"),
        )
        .subcommand(
            SubCommand::with_name("ping")
                .about("Pings the best server")
                .arg(
                    Arg::with_name("numservers")
                        .short("n")
                        .takes_value(true)
                        .help("Number of servers to test with"),
                )
                .arg(
                    Arg::with_name("numpings")
                        .short("p")
                        .takes_value(true)
                        .help("Number of pings to test with"),
                )
                .arg(
                    Arg::with_name("server")
                        .short("s")
                        .takes_value(true)
                        .help("specify a server number to ping"),
                ),
        )
        .get_matches();

    info!("Arguments {:#?}", app);

    if app.is_present("list") {
        let resp = server::list_servers();
        match resp {
            Ok(n) => print_servers(n),
            Err(e) => error!("Err: {:?}", e),
        }
    }

    if let Some(app) = app.subcommand_matches("download") {
        let bytes = app.value_of("bytes").unwrap_or("100000024");
        let best = server::best_server("3").unwrap().to_owned();
        let dl = server::download(best.id.as_str(), bytes).unwrap();
        println!("Download Results {:#?} mbps", dl);
    }

    if let Some(app) = app.subcommand_matches("upload") {
        let bytes = app.value_of("bytes").unwrap_or("50000024");
        let best = server::best_server("3").unwrap().to_owned();
        let dl = server::upload(best.id.as_str(), bytes).unwrap();
        println!("Upload Results {:#?} mbps", dl);
    }

    if let Some(app) = app.subcommand_matches("ping") {
        let best;
        let num_best = app.value_of("numservers").unwrap_or("3");
        let num_pings = app
            .value_of("numpings")
            .unwrap_or("3")
            .parse::<u128>()
            .unwrap();
        info!(
            "Ping test (servers: {}/Given? {}, pings: {}/Given? {})",
            num_best,
            app.is_present("numservers"),
            num_pings,
            app.is_present("numpings")
        );

        println!("[ping test]");
        let svr = if app.is_present("server") {
            app.value_of("server").unwrap()
        } else {
            best = server::best_server(num_best).unwrap().to_owned();
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
