-- Your SQL goes here
create table users (
  uuid uuid primary key,
  first_name varchar(255) not null,
  last_name varchar(255) not null,
  email varchar(255) unique not null,
  password_hash varchar(255) not null,
  role varchar not null default 'user',
  created_at timestamp not null default now(),
  updated_at timestamp not null default now()
);
