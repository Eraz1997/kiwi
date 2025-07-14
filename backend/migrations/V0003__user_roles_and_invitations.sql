create type user_role as enum ('Admin', 'Customer');

alter table users add column role user_role not null default 'Customer';

create table if not exists user_invitations (
    id uuid default gen_random_uuid() primary key,
    role user_role not null
);
