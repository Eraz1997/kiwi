create table if not exists services (
    name text primary key,
    image_name text not null,
    image_sha text not null,
    exposed_ports integer[][] not null,
    environment_variables text[][] not null,
    secrets text[][] not null,
    stateful_volume_paths text[] not null,
    postgres_username text not null,
    postgres_password text not null,
    redis_username text not null,
    redis_password text not null,
    created_at timestamp not null default now(),
    last_modified_at timestamp not null default now(),
    last_deployed_at timestamp not null default now()
);
