extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
extern crate simplelog;

pub mod server {

    use std::error;

    use failure::{err_msg, Error, Fail};
    #[derive(Debug, Fail)]
    #[fail(display = "list error")]
    pub struct ListServersError(#[fail(cause)] Error);

    #[derive(Clone, Debug, Deserialize)]
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

    pub fn download(server: &str, bytes: &str) -> Result<f64, Box<error::Error>> {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpStream;
        use std::time::Instant;

        println!("Reading {} bytes", bytes);

        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        let s = all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or_else(|| format!("Can't find server '{}'", server))?;
        let serv = s.clone();

        let conn = TcpStream::connect(&serv.host);
        match conn {
            Ok(mut stream) => {
                let now = Instant::now();
                let dlstring = format!("DOWNLOAD {}\r\n", bytes);
                stream.write_all(dlstring.as_bytes()).unwrap();
                let mut line = String::new();
                let mut reader = BufReader::new(stream);
                let _resp = reader.read_line(&mut line);
                let elapsed = now.elapsed().as_millis();
                println!("Download took {} ms", elapsed);
                let bms = bytes.parse::<u128>().unwrap() / elapsed;
                let mbps = bms as f64 * 0.008;
                Ok(mbps)
            }
            Err(e) => {
                error!("Failed to connect to server: Error: '{}'", e);
                panic!();
            }
        }
    }

    pub fn ping_server(server: &str, num_pings: u128) -> Result<u128, Box<error::Error>> {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpStream;
        use std::time::Instant;

        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        let s = all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or_else(|| format!("Can't find server '{}'", server))?;
        let serv = s.clone();

        let mut acc: u128 = 0;
        for _x in 0..num_pings {
            info!("Pinging {}", &serv.host);
            let conn = TcpStream::connect(&serv.host);
            match conn {
                Ok(mut stream) => {
                    let now = Instant::now();
                    stream.write_all(b"HI\r\n").unwrap();
                    let mut line = String::new();
                    let mut reader = BufReader::new(stream);
                    let resp = reader.read_line(&mut line);
                    match resp {
                        Ok(_n) => {
                            let elapsed = now.elapsed().as_millis();
                            acc += elapsed;
                            info!("Ping {} ms", elapsed);
                        }
                        Err(e) => {
                            error!("Failed to ping server: Error: '{}'", e);
                            panic!();
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to connect to server: Error: '{}'", e);
                    panic!();
                }
            }
        }
        Ok(acc / num_pings)
    }

    pub fn list_servers() -> Result<Vec<Server>, ListServersError> {
        let resp = reqwest::get("https://speedtest.net/api/js/servers?engine=js");
        match resp {
            Ok(mut r) => {
                let body = r.json();
                match body {
                    Ok(b) => {
                        let newb: Vec<Server> = b;
                        Ok(newb)
                    }
                    Err(e) => {
                        return Err(ListServersError(err_msg(format!(
                            "Error parsing list {}",
                            e
                        ))));
                    }
                }
            }
            Err(e) => {
                return Err(ListServersError(err_msg(format!(
                    "Error retrieving list {}",
                    e
                ))));
            }
        }
    }

    pub fn best_server(num_test: &str) -> Result<Server, Box<error::Error>> {
        println!("Finding best server...");
        let mut servers = match list_servers() {
            Ok(s) => s,
            Err(e) => {
                error!("List servers failed: Error: '{}'", e);
                panic!();
            }
        };
        servers.sort_by_key(|s| s.distance);
        servers.truncate(num_test.parse::<usize>().unwrap());
        servers.iter_mut().for_each(|s| {
            s.latency = ping_server(&s.id, 1).unwrap();
        });
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
