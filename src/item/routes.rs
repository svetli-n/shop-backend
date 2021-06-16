use crate::item::{Item, ItemRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/items")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Item::find_all(db_pool.get_ref()).await;
    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(e) => HttpResponse::BadRequest()
            .body(format!("Error trying to read all items from database: {}", e)),
    }
}

#[get("/item/{id}")]
async fn find(id: web::Path<i64>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Item::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        _ => HttpResponse::BadRequest().body("Item not found"),
    }
}

#[post("/item")]
async fn create(
    item: web::Json<ItemRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Item::create(item.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(e) => {
            println!("hahah");
            HttpResponse::BadRequest().body(e.to_string())
        },
    }
}

#[put("/item/{id}")]
async fn update(
    id: web::Path<i32>,
    item: web::Json<ItemRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result =
        Item::update(id.into_inner(), item.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        _ => HttpResponse::BadRequest().body("Item not found"),
    }
}

#[delete("/item/{id}")]
async fn delete(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Item::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows) => {
            if rows > 0 {
                HttpResponse::Ok()
                    .body(format!("Successfully deleted {} record(s)", rows))
            } else {
                HttpResponse::BadRequest().body("Item not found")
            }
        }
        _ => HttpResponse::BadRequest().body("Item not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
