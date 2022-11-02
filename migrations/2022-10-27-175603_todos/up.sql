CREATE TABLE todos (
       id INTEGER AUTO_INCREMENT PRIMARY KEY,
       title VARCHAR (50) NOT NULL UNIQUE,
       description TEXT NOT NULL,
       status TEXT NOT NULL
)
