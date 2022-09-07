create table sources
(
    id         INTEGER not null
        primary key autoincrement
        unique,
    url        TEXT    not null,
    fetch_date INTEGER,
    source     TEXT    not null
);