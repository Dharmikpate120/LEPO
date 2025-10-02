-- -- Function trigger_set_updated_at must be defined before running this script.

-- -- As a style choice, we prefer to avoid plurals in table names, mainly because it makes queries read better.
-- --
-- -- For our user table, quoting the table name is recommended by IntelliJ's tooling because `user` is a keyword.
-- create table "users"
-- (
--     -- Placeholder for the primary key column (assuming UUID is intended)
--     user_id uuid primary key default uuid_generate_v1mc(),
    
--     username text unique not null,

--     -- The CHECK constraint enforces that all emails are stored in lowercase.
--     email text unique not null check (email = lower(email)),

--     -- Field for storing the GitHub access token.
--     github_access_token text unique,

--     bio text not null default '',
--     image text default '',
    
--     -- The Argon2 hashed password string for the user.
--     password_hash text default '',

--     -- Record creation time (cannot change).
--     created_at timestamptz not null default now(),

--     -- Record update time (managed by the trigger).
--     updated_at timestamptz default now()
-- );

-- -- Apply the trigger to the "users" table.
-- -- This trigger executes BEFORE any UPDATE operation and calls the function
-- -- to set the updated_at column value.
-- CREATE TRIGGER set_updated_at
-- BEFORE UPDATE ON "users"
-- FOR EACH ROW
-- EXECUTE FUNCTION trigger_set_updated_at();

-- -- Note on the commented SELECT:
-- -- The original comment "SELECT trigger_updated_at('"user"');" implies a pre-defined
-- -- procedure. Here we are defining the function (trigger_set_updated_at) and
-- -- applying the trigger (set_updated_at) explicitly, which is the standard PostgreSQL way.
