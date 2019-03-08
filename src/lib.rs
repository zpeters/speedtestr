extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod server {

    use std::error;

    #[derive(Debug,Deserialize)]
    pub struct Server {
        url: String,
        lat: String,
        lon: String,
        distance: i32,
        name: String,
        country: String,
        cc: String,
        sponsor: String,
        id: String,
        host: String,
        #[serde(skip)]
        latency: i32,
    }

    pub fn list_servers() -> Result<Vec<Server>, Box<error::Error>> {
        let body: Vec<Server> = reqwest::get("https://www.speedtest.net/api/js/servers?engine=js")?
            .json()?;
        Ok(body)
    }
}
