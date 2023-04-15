mod services;
mod model;
mod schema;

use actix_web::{
    web,
    App,
    HttpServer
};

use dotenv::dotenv;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection DB resolved");
            pool
        }
        Err(error) => {
            println!("Failed to connect to the dabase: {:?}", error);
            std::process::exit(1);
        }
    };

    println!("Server started successfully");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}
