use color_eyre::{ Result};
use structopt::StructOpt;
use serde::Deserialize;
use inflector::Inflector;


#[derive(StructOpt)]
struct Aqi{
    #[structopt(short = "t", long = "token", env = "AQI_TOKEN")]
    api_token: String,

    #[structopt(subcommand)]
    command: Opt,
}

#[derive(StructOpt)]
enum Opt{
    Info { url: String},
    Search { keyword: String},
}

#[derive(Deserialize)]
struct InfoResponce {
    data: InfoObject
}

#[derive(Deserialize)]
struct InfoObject {
    aqi: u16, 
    city: City,
    // attributions: serde_json::Value,
    // forecast: serde_json::Value,
    // iaqi: serde_json::Value,
}

#[derive(Deserialize)]
struct City {
    name: String
}

#[derive(Deserialize)]
struct SearchResponce {
    data: Vec<AqiObject>
}

#[derive(Deserialize)]
struct AqiObject {
    aqi: String,
    station: Station,
}

#[derive(Deserialize)]
struct Station {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Aqi::from_args();
    let client = reqwest::Client::new();


    match args.command {
        Opt::Info {url} => {
            let res = client
            .get(format!("https://api.waqi.info/feed/{}/", url
                .trim_start_matches("/")
                .trim_end_matches("/")
                .to_kebab_case()))
            .query(&[("token",  args.api_token)])
            .send()
            .await?
            .json::<InfoResponce>()
            .await?;
            
            println!("{:2}   {}\n", res.data.aqi, res.data.city.name);
        },
        Opt::Search {keyword}=> {
            let res = client
            .get("https://api.waqi.info/search/")
            .query(&[("token", args.api_token),("keyword", keyword)])
            .send()
            .await?
            .json::<SearchResponce>()
            .await?;
             
            for data in res.data {
                println!("{:2} {} \n {}\n", data.aqi, data.station.name, data.station.url);
            }
        },
    }


    Ok(())
}
