create extension if not exists pgcrypto;

-- Tables

create table if not exists sheets (
    id uuid primary key default gen_random_uuid(),
    title varchar(256) not null,
    created timestamptz not null default current_timestamp,
    changed timestamptz not null default current_timestamp,
    tiptap jsonb not null
);

create table if not exists users (
    id serial primary key,
    username varchar(20) not null unique,
    password_hash varchar(255) not null
);

create table if not exists sessions (
    session_id varchar(128) primary key,
    user_id integer not null references users on update cascade on delete cascade,
    expires timestamp not null
);

-- Triggers

create or replace function update_changed_timestamp()
returns trigger as $$
begin
    new.changed = current_timestamp;
    return new;
end;
$$ language 'plpgsql';

create trigger sheet_changed before update on sheets
for each row execute procedure update_changed_timestamp();
