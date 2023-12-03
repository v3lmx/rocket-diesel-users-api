create database user_db;
create role dev_user with PASSWORD 'dev_password' LOGIN;
alter database user_db owner to dev_user;
