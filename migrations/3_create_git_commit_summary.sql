-- -- Function trigger_set_updated_at must be defined before running this script.

-- -- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
-- --
-- -- For our user table, quoting the table name is recommended by IntelliJ's tooling because `user` is a keyword.
-- create table "commit_summary"
-- (
--     -- Placeholder for the primary key column (assuming UUID is intended)
--     summary_id uuid primary key default uuid_generate_v1mc(),
    
--     user_id uuid not null,
--     FOREIGN KEY (user_id) REFERENCES users(user_id),
--     repo_name text       not null,

--     username text  not null,

    
--     summary text not null default '',
    
--     -- Record creation time (cannot change).
--     created_at timestamptz not null default now(),

--     -- Record update time (managed by the trigger).
--     updated_at timestamptz default now()
-- );

-- -- Apply the trigger to the "users" table.
-- -- This trigger executes BEFORE any UPDATE operation and calls the function
-- -- to set the updated_at column value.
-- CREATE TRIGGER set_updated_at
-- BEFORE UPDATE ON "commit_summary"
-- FOR EACH ROW
-- EXECUTE FUNCTION trigger_set_updated_at();

-- -- Note on the commented SELECT:
-- -- The original comment "SELECT trigger_updated_at('"user"');" implies a pre-defined
-- -- procedure. Here we are defining the function (trigger_set_updated_at) and
-- -- applying the trigger (set_updated_at) explicitly, which is the standard PostgreSQL way.
