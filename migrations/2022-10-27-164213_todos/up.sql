-- Your SQL goes here
CREATE TABLE todos (
       id SERIAL PRIMARY KEY,
       title VARCHAR (50) NOT NULL,
       description TEXT NOT NULL,
       status TEXT (10) NOT NULL
)
