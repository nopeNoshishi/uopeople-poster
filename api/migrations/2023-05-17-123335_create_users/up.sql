CREATE TABLE users (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);