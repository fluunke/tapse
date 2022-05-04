-- Add migration script here

DROP TABLE IF EXISTS messages;
DROP TABLE IF EXISTS rooms;
DROP TABLE IF EXISTS files;

CREATE TABLE rooms (
        name            VARCHAR(80) PRIMARY KEY NOT NULL,
        creation_date   TIMESTAMP NOT NULL
);

CREATE TABLE messages (
        id              INTEGER PRIMARY KEY NOT NULL,
        author          VARCHAR(80) DEFAULT 'Anonymous' NOT NULL,
        room            VARCHAR(80) REFERENCES rooms(name) NOT NULL,
        content         VARCHAR(200) NOT NULL,
        creation_date   TIMESTAMP NOT NULL
);

CREATE TABLE files (
        room            VARCHAR(80) REFERENCES rooms(name) NOT NULL,
        id              VARCHAR(32) PRIMARY KEY NOT NULL,
        name            VARCHAR(300) NOT NULL,
        file            BLOB NOT NULL,
        upload_date     TIMESTAMP NOT NULL

);

-- Default room
insert into rooms (name, creation_date) values ('general', datetime('now'))