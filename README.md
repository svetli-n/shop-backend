# Shop backend

Example Shop application using Actix-web and [SQLx](https://github.com/launchbadge/sqlx) with Postgres

# Usage

## Change into the project directory

All instructions assume you have changed into this folder:

```bash
cd shop-backend 
```

## Set up the database

* Create new database using `schema.sql`
* Adjust DATABASE_URL to match your Postgres address, username and password 

## Run the application

To run the application execute:

```bash
cargo run
```

## Available routes

```
GET     /items -> list of all items
POST    /item -> create new item, example: { "description": "actix rulez", "price": 3 }
GET     /item/{id} -> show one item with requested id
PUT     /item/{id} -> update item with requested id, example: { "description": "actix rulez", "price": 3 }
DELETE  /item/{id} -> delete item with requested id
```

## Example requests 

```
curl -d '{ "description": "rust rulez", "price": 4 }' -H "Content-Type: application/json" -X POST http://localhost:5000/item
```