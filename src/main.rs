//#![allow(unused_imports)]

use dotenvy::dotenv;
use actix_web::{web, App, HttpServer};
use actix_files::Files;
mod database;
mod services;
use services::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    println!("Connecting to Database ...");
    let pool = match database::connect().await {
        Ok(dbpool) => dbpool,
        Err(e) => panic!("Error connecting to Database: {:?}", e)
    };

    let port:u16 = std::env::var("PORT").unwrap_or_else(|_| {
            eprintln!("PORT environment-variable not set, reverting to 8080");
            "8080".to_string()
            }).parse().unwrap_or_else(|e| {
                eprintln!("Could not parse PORT, reverting to 8080. ({:?})", e);
                8080
            });
    let ip: std::net::IpAddr = std::env::var("IP").unwrap_or_else(|_| {
            eprintln!("IP not set in environment, reverting to 0.0.0.0");
            "0.0.0.0".to_string()
    }).parse().unwrap_or_else(|e| {
            eprintln!("Could not parse IP, Reverting to 0.0.0.0 {:?}", e);
            std::net::IpAddr::from([0,0,0,0])
    });

    println!("Starting Server at {}:{}", ip, port);
    HttpServer::new( move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(Files::new("/static", "static").show_files_listing())
            .service(hello)
            .service(echo)
            .service(plants)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind((ip, port))?
    .run()
    .await
}


