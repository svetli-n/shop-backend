#[macro_use]
extern crate log;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use listenfd::ListenFd;
use sqlx::postgres::PgPool;
use std::env;

// import item module (routes and model)
mod item;

// default / handler
async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Available routes:
        GET /items -> list of all items
        POST /item -> create new item, example: { "description": "learn actix and sqlx", "done": false }
        GET /item/{id} -> show one item with requested id
        PUT /item/{id} -> update item with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /item/{id} -> delete item with requested id
    "#
    )
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
            .configure(item::init) // init item routes
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;

    Ok(())
}
