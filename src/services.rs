use actix_web::{self, web, get, post, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use crate::database;

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/plants")]
pub async fn plants(db: web::Data<Pool<Postgres>>) -> impl Responder {
    let plants = database::fetch_plants(&db).await.unwrap();
    let mut resp = String::new();
    for plant in plants {
        resp += &plant.name;
        resp += " ";
    }
    HttpResponse::Ok().body(resp)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

