create table if not exists kiwi_admin.users (
    id bigint primary key generated always as identity,
    username varchar(256) not null,
    password_hash text not null,
);
