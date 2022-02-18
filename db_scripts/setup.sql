create extension if not exists pgcrypto;

create table if not exists sheets (                                                                   
    id uuid primary key default gen_random_uuid(),
    title varchar(256) not null,
    tiptap jsonb not null
);
