// use crate::item::{Item, ItemRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;
use std::collections::HashMap;
use meilisearch_sdk::client::Client;


#[get("/search/{query}")]
async fn search(query: web::Path<String>) -> impl Responder {
    let host_port = "http://localhost:7700";
    let c = Client::new(host_port, "");
    let index_name = "chemicals";
    let chemicals = c.get_or_create(index_name).await.unwrap();

    let hits: Vec<HashMap<String, String>> = chemicals.search()
        .with_query(query.as_str())
        .execute::<HashMap<String, String>>()
        .await.unwrap().hits
        .iter().map(|res| res.result.clone()).collect();

    HttpResponse::Ok().json(hits.to_owned())
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(search);
}
