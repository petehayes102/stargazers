PRAGMA foreign_keys = ON;

CREATE TABLE user (
    id INTEGER PRIMARY KEY,
    username VARCHAR UNIQUE,
    name VARCHAR,
    email VARCHAR
);

CREATE TABLE repository (
    id INTEGER PRIMARY KEY,
    full_name VARCHAR UNIQUE,
    stargazers INTEGER NOT NULL
);

CREATE TABLE user_repos (
    user INTEGER NOT NULL,
    repository INTEGER NOT NULL,
    type VARCHAR NOT NULL,
    date TEXT,
    FOREIGN KEY (user) REFERENCES user (id),
    FOREIGN KEY (repository) REFERENCES repository (id),
    UNIQUE(user, repository, type)
);

CREATE TABLE user_users (
    subject INTEGER NOT NULL,
    linked INTEGER NOT NULL,
    type VARCHAR NOT NULL,
    FOREIGN KEY (subject) REFERENCES user (id),
    FOREIGN KEY (linked) REFERENCES user (id),
    UNIQUE(subject, linked, type)
);
