# Shop backend

Shop API using [Actix-web](https://github.com/actix/actix-web) 
with [MeiliSearch](https://www.meilisearch.com/).

## Usage

To run the application execute:

```bash
cargo run
```

Available routes

```
GET /search/{query} -> get all items for query
```

Example request 

```
curl http://localhost:5000/search/in
```