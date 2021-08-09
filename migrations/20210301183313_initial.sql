-- Add migration script here

DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS files;

CREATE TABLE rooms (
        id              INTEGER PRIMARY KEY NOT NULL,
        name            VARCHAR(80) UNIQUE NOT NULL,
        creation_date   TIMESTAMP NOT NULL
);

CREATE TABLE messages (
        id              INTEGER PRIMARY KEY NOT NULL,
        author          VARCHAR(80) DEFAULT 'Anonymous' NOT NULL,
        room            INTEGER REFERENCES rooms(id) NOT NULL,
        content         VARCHAR(200) NOT NULL,
        creation_date   TIMESTAMP NOT NULL
);

CREATE TABLE files (
        room            INTEGER REFERENCES rooms(id) NOT NULL,
        id              VARCHAR(32) NOT NULL PRIMARY KEY,
        name            VARCHAR(300) NOT NULL,
        file            BLOB NOT NULL,
        upload_date     TIMESTAMP NOT NULL

);

-- Default room
insert into rooms (name, creation_date) values ('general', datetime('now'))