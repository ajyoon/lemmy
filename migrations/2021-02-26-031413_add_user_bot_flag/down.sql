alter table user_ drop column bot;

-- Reset self-join alias views for new reference table schema
create or replace view user_alias_1 as select * from user_;
create or replace view user_alias_2 as select * from user_;
