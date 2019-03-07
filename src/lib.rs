extern crate reqwest;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

pub mod server {

    use std::error;

    #[derive(Deserialize)]
    struct Ip {
        origin: String,
    }

    pub fn list_servers() -> Result<String, Box<error::Error>> {
        println!("Listing servers");
        let body = reqwest::get("https://www.speedtest.net/api/js/servers?engine=js")?
//        let body = reqwest::get("https://www.rust-lang.org")?
            .text()?;

        println!("body = {:?}", body);

        return Ok("OK".to_string());
    }
}
