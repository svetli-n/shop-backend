# Shop backend

Example Shop application using Actix-web and [SQLx](https://github.com/launchbadge/sqlx) with posgres

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

By default application will be available on `http://localhost:5000`. If you wish to change address or port you can do it inside `.env` file
