use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    
    println!("{}",env::var("MYENVVAR").unwrap().to_string());
}
