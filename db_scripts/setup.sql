create extension if not exists pgcrypto;

create or replace function update_changed_timestamp()
returns trigger as $$
begin
    new.changed = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create table if not exists sheets (
    id uuid primary key default gen_random_uuid(),
    title varchar(256) not null,
    created timestamptz not null default current_timestamp,
    changed timestamptz not null default current_timestamp,
    tiptap jsonb not null
);

create trigger sheet_changed before update on sheets
for each row execute procedure update_changed_timestamp();
