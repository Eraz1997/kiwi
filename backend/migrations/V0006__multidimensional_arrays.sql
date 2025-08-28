alter table services
alter column environment_variables type jsonb using to_jsonb(environment_variables),
alter column secrets type jsonb using to_jsonb(secrets);
