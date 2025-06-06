create table if not exists users (
    id bigint primary key generated always as identity,
    username varchar(256) not null,
    password_hash text not null
);
