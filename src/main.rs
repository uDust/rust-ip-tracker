//this is a work in progress
//esto es un trabajo en progreso
//:)
use std::io;
extern crate reqwest;

#[tokio::main]
async fn main() {
    let mut api = String::from("http://ip-api.com/json/");
    let mut ip = String::new();
    

    //get a user input
    println!("introduce una ip: ");
    let _bytes = io::stdin()
        .read_line(&mut ip)
        .unwrap();
    
    api.push_str(&ip);


    let response = reqwest::get(api)
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);

    
}
