CREATE TABLE books (
    id         BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMP(0),       
    updated_at TIMESTAMP(0),       
    deleted_at TIMESTAMP(0) DEFAULT NULL,       
    uuid       TEXT UNIQUE NOT NULL,
    title      TEXT NOT NULL,
    author     TEXT NOT NULL,
    quantity   INT NOT NULL
);