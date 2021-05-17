CREATE TABLE IF NOT EXISTS items (
    id          BIGSERIAL PRIMARY KEY,
    description TEXT NOT NULL,
    price        INT NOT NULL
);
