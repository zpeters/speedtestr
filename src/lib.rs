extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

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

    pub fn ping_server(server: &str) -> Result<u128, Box<error::Error>> {
        use std::net::TcpStream;
        use std::io::{BufReader, BufRead, Write};
        use std::time::{Instant};

        println!("Pinging server: {}", server);

        let all_servers = match list_servers() {
            Ok(n) => n,
            Err(_e) => Vec::<Server>::new(),
        };

        let s = all_servers
            .into_iter()
            .find(|s| s.id == server)
            .ok_or(format!("Can't find server '{}'", server))?;

        let conn = TcpStream::connect(s.host);
        match conn {
            Ok(mut stream) => {
                let now = Instant::now();
                stream.write(b"HI\r\n").unwrap();
                let mut line = String::new();
                let mut reader = BufReader::new(stream);
                let resp = reader.read_line(&mut line);
                match resp {
                    Ok(_n) => {
                        Ok(now.elapsed().as_millis())
                    },
                    Err(e) => {
                        println!("Failed to get response: {}", e);
                        Ok(0)
                    }
                }
            },
            Err(e) => {
                println!("Failed to connect: {}", e);
                Ok(0)
            },
        }
    }

    pub fn list_servers() -> Result<Vec<Server>, Box<error::Error>> {
        let body: Vec<Server> = reqwest::get("https://www.speedtest.net/api/js/servers?engine=js")?
            .json()?;
        Ok(body)
    }
}
