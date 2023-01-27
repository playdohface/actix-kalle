//! Module for all database-related logic

use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, Postgres, Pool, Error};
use serde::Serialize;
use std::env;


#[derive(Debug, Serialize, FromRow)]
pub struct Plant {
    pub name: String,
    pub age: Option<i32>,
    pub location: Option<String>,
}

pub async fn fetch_plants(db: &Pool<Postgres>) -> Result<Vec<Plant>, Error>{
    let plants = sqlx::query_as!(Plant, "
    SELECT name, age, location FROM plants;
    ").fetch_all(db).await?;
    Ok(plants)
}

///Returns a connection-pool to a PostgreSQL Database. Needs DATABASE_URL to be set. 
/// 
///Example: `DATABASE_URL=postgres://<username>:<password>@localhost:5432/<database_name>`
pub async fn connect() -> Result<Pool<Postgres>, Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await
}
    //sqlx::query("INSERT INTO plants (name, age, location) VALUES ('Basilikum', 1, 'kitchen')")
    //    .execute(&pool).await?;

    // let plants = sqlx::query_as!(Plant, "
    //     SELECT name, age, location FROM plants ORDER BY age;
    // ").fetch_all(&pool).await?;

    // for plant in plants {
    //     println!("{:?}", plant);
    // }

 
