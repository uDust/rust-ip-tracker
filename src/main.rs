/*    crates and libraries    */ 
use std::io;
use std::process;
extern crate serde;
extern crate reqwest;

/*  main function  */
#[tokio::main]
async fn main() {
    //variables
    let mut api = String::from("http://ip-api.com/json/");
    let mut ip = String::new();
   
    //structure from ip information
    #[derive(serde::Deserialize)]
    struct IpInf {
        query: String,
        status: String,
        country: String,
        countryCode: String,
        region: String,
        regionName: String,
        city: String,
        zip: String,
        lat: f32,
        lon: f32,
        timezone: String,
        isp: String,
        org: String,
    }

    //get a user input
    println!("introduce una ip: ");
    let _bytes = io::stdin()
        .read_line(&mut ip)
        .unwrap();
    
    api.push_str(&ip);

    //request
    let info = reqwest::get(api)
        .await
        .unwrap()
        .json::<IpInf>()
        .await
        .unwrap();
    
    //print info 
    println!("\n\n-----------------------------------------");
    println!("target: {:#?}", info.query);
    println!("country: {:#?}", info.country);
    println!("country code: {:#?}", info.countryCode);
    println!("region: {:#?}", info.region);
    println!("region name: {:#?}", info.regionName);
    println!("city: {:#?}", info.city);
    println!("zip code: {:#?}", info.zip);
    println!("latitude: {:#?}", info.lat);
    println!("longitude: {:#?}", info.lon);
    println!("timezone: {:#?}", info.timezone);
    println!("isp: {:#?}", info.isp);
    println!("org: {:#?}", info.org);
} 
