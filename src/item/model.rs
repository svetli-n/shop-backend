use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row, PgPool};

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct ItemRequest {
    pub description: String,
    pub price: i32,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i64,
    pub description: String,
    pub price: i32,
}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler
impl Responder for Item {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

// Implementation for Todo struct, functions for read/write/update and delete item from database
impl Item {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Item>> {
        let mut items = vec![];
        let recs = sqlx::query!(
            r#"
                SELECT id, description, price
                    FROM items
                ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await?;

        for rec in recs {
            items.push(Item {
                id: rec.id,
                description: rec.description,
                price: rec.price,
            });
        }

        Ok(items)
    }

    pub async fn find_by_id(id: i64, pool: &PgPool) -> Result<Item> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM items WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*pool)
        .await?;

        Ok(Item {
            id: rec.id,
            description: rec.description,
            price: rec.price,
        })
    }

    pub async fn create(item: ItemRequest, pool: &PgPool) -> Result<Item> {
        let mut tx = pool.begin().await?;
        let item = sqlx::query("INSERT INTO items (description, price) VALUES ($1, $2) RETURNING id, description, price")
            .bind(&item.description)
            .bind(item.price)
            .map(|row: PgRow| {
                Item {
                    id: row.get(0),
                    description: row.get(1),
                    price: row.get(2)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(item)
    }

    pub async fn update(id: i32, item: ItemRequest, pool: &PgPool) -> Result<Item> {
        let mut tx = pool.begin().await.unwrap();
        let item = sqlx::query("UPDATE items SET description = $1, price = $2 WHERE id = $3 RETURNING id, description, price")
            .bind(&item.description)
            .bind(item.price)
            .bind(id)
            .map(|row: PgRow| {
                Item {
                    id: row.get(0),
                    description: row.get(1),
                    price: row.get(2)
                }
            })
            .fetch_one(&mut tx)
            .await?;

        tx.commit().await.unwrap();
        Ok(item)
    }

    pub async fn delete(id: i32, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;
        let deleted = sqlx::query("DELETE FROM items WHERE id = $1")
            .bind(id)
            .execute(&mut tx)
            .await?;

        tx.commit().await?;
        Ok(deleted)
    }
}
