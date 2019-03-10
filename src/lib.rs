extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use] extern crate log;
extern crate simplelog;

pub mod server {

    use std::error;

    #[derive(Clone,Debug,Deserialize)]
    pub struct Server {
        url: String,
        lat: String,
        lon: String,
        pub distance: i32,
        pub name: String,
        country: String,
        pub cc: String,
        pub sponsor: String,
        pub id: String,
        pub host: String,
        #[serde(skip)]
        pub latency: u128,
    }

    pub fn download(server: &str) -> Result<f64, Box<error::Error>>{
        use std::net::TcpStream;
        use std::time::{Instant};
        use std::io::{BufReader, BufRead, Write};

        let dlsize: &str = "100000024";
        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        let s = all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or(format!("Can't find server '{}'", server))?;
        let serv = s.clone();

        println!("Reading {} bytes", dlsize);
        let conn = TcpStream::connect(&serv.host);
        match conn {
            Ok(mut stream) => {
                let now = Instant::now();
                let dlstring = format!("DOWNLOAD {}\r\n", dlsize);
                stream.write(dlstring.as_bytes()).unwrap();
                let mut line = String::new();
                let mut reader = BufReader::new(stream);
                let _resp = reader.read_line(&mut line);
                let elapsed = now.elapsed().as_millis();
                println!("Download took {} ms", elapsed);
                let bms = dlsize.parse::<u128>().unwrap() / elapsed;
                let mbps = bms as f64 * 0.008;
                Ok(mbps)
            },
            Err(e) => {error!("Failed to connect to server: Error: '{}'", e); panic!();},
        }
    }

    pub fn ping_server(server: &str, num_pings: u128) -> Result<u128, Box<error::Error>> {
        use std::net::TcpStream;
        use std::io::{BufReader, BufRead, Write};
        use std::time::{Instant};

        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        let s = all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or(format!("Can't find server '{}'", server))?;
        let serv = s.clone();

        let mut acc: u128 = 0;
        for _x in 0..num_pings {
            info!("Pinging {}", &serv.host);
            let conn = TcpStream::connect(&serv.host);
            match conn {
                Ok(mut stream) => {
                    let now = Instant::now();
                    stream.write(b"HI\r\n").unwrap();
                    let mut line = String::new();
                    let mut reader = BufReader::new(stream);
                    let resp = reader.read_line(&mut line);
                    match resp {
                        Ok(_n) => {
                            let elapsed = now.elapsed().as_millis();
                            acc = acc + elapsed;
                            info!("Ping {} ms", elapsed);
                        },
                            Err(e) => {error!("Failed to ping server: Error: '{}'", e); panic!();},
                    }
                },
                Err(e) => {error!("Failed to connect to server: Error: '{}'", e); panic!();},
            }
        }
        Ok(acc / num_pings)
    }

    pub fn list_servers() -> Result<Vec<Server>, Box<error::Error>> {
        let body: Vec<Server> = reqwest::get("https://www.speedtest.net/api/js/servers?engine=js")?
            .json()?;
        Ok(body)
    }

    pub fn best_server(num_test: &str) -> Result<Server, Box<error::Error>> {
        use std::io;
        use std::io::Write;

        print!("[finding best server ");
        io::stdout().flush().unwrap();
        let mut servers = match list_servers() {
            Ok(s) => s,
            Err(e) => {error!("List servers failed: Error: '{}'", e); panic!();},
        };
        servers.sort_by_key(|s| s.distance);
        servers.truncate(num_test.parse::<usize>().unwrap());
        servers.iter_mut().for_each(|s| {
            print!(".");
            io::stdout().flush().unwrap();
            s.latency = ping_server(&s.id, 1).unwrap();
        });
        println!("]");
        io::stdout().flush().unwrap();
        servers.sort_by_key(|s| s.latency);
        let best = servers[0].clone();
        Ok(best)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
