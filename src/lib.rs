extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate rand;
extern crate serde_json;

pub mod server {

    use std::error;

    use failure::{err_msg, Error, Fail};
    #[derive(Debug, Fail)]
    #[fail(display = "list error")]
    pub struct ListServersError(#[fail(cause)] Error);

    #[derive(Clone, Debug, Deserialize)]
    pub struct Server {
        pub distance: i32,
        pub name: String,
        pub cc: String,
        pub sponsor: String,
        pub id: String,
        pub host: String,
        #[serde(skip)]
        pub latency: u128,
    }

    pub fn upload(server: &str, bytes: &str) -> Result<f64, Box<error::Error>> {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpStream;
        use std::time::Instant;

        println!("Writing {} bytes", bytes);

        let serv = find_server(server);

        let conn = TcpStream::connect(&serv.host);
        match conn {
            Ok(mut stream) => {
                // tell the server how much we are sending
                let ulstring = format!("UPLOAD {} 0\r\n", bytes);
                stream.write_all(ulstring.as_bytes()).unwrap();

                // send the bytes
                println!("generating random bytes");
                let randnow = Instant::now();
                let randstring: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(bytes.parse::<usize>().unwrap())
                    .collect();
                let randelapsed = randnow.elapsed().as_millis();
                println!("Random bytes took {} ms", randelapsed);

                println!("uploading...");
                let now = Instant::now();
                stream.write_all(randstring.as_bytes()).unwrap();

                let mut line = String::new();
                let mut reader = BufReader::new(stream);
                let _resp = reader.read_line(&mut line);
                let elapsed = now.elapsed().as_millis();
                println!("Upload took {} ms", elapsed);
                let bms = bytes.parse::<u128>().unwrap() / elapsed;
                let mbps = bms as f64 * 0.008;
                Ok(mbps)
            }
            Err(e) => {
                println!("Failed to connect to server: Error: '{}'", e);
                panic!();
            }
        }
    }

    pub fn download(server: &str, bytes: &str) -> Result<f64, Box<error::Error>> {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpStream;
        use std::time::Instant;

        println!("Reading {} bytes", bytes);

        let serv = find_server(server);

        let conn = TcpStream::connect(&serv.host);
        match conn {
            Ok(mut stream) => {
                let dlstring = format!("DOWNLOAD {}\r\n", bytes);
                stream.write_all(dlstring.as_bytes()).unwrap();
                let mut line = String::new();
                let mut reader = BufReader::new(stream);
                let now = Instant::now();
                let _resp = reader.read_line(&mut line);
                let elapsed = now.elapsed().as_millis();
                println!("Download took {} ms", elapsed);
                let bms = bytes.parse::<u128>().unwrap() / elapsed;
                let mbps = bms as f64 * 0.008;
                Ok(mbps)
            }
            Err(e) => {
                println!("Failed to connect to server: Error: '{}'", e);
                panic!();
            }
        }
    }

    pub fn ping_server(server: &str, num_pings: u128) -> Result<u128, Box<error::Error>> {
        use std::io::{BufRead, BufReader, Write};
        use std::net::TcpStream;
        use std::time::Instant;

        let serv = find_server(server);

        let mut acc: u128 = 0;
        for _x in 0..num_pings {
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
                        }
                        Err(e) => {
                            println!("Failed to ping server: Error: '{}'", e);
                            panic!();
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to connect to server: Error: '{}'", e);
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
                    Err(e) => Err(ListServersError(err_msg(format!(
                        "Error parsing list {}",
                        e
                    )))),
                }
            }
            Err(e) => Err(ListServersError(err_msg(format!(
                "Error retrieving list {}",
                e
            )))),
        }
    }

    pub fn best_server(num_test: &str) -> Result<Server, Box<error::Error>> {
        println!("Finding best server...");
        let mut servers = match list_servers() {
            Ok(s) => s,
            Err(e) => {
                println!("List servers failed: Error: '{}'", e);
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

    fn find_server(server: &str) -> Server {
        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or_else(|| format!("Can't find server '{}'", server))
            .unwrap()
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
