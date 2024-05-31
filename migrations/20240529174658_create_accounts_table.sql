-- Create accounts table
CREATE TABLE IF NOT EXISTS accounts(
                    id INTEGER PRIMARY KEY NOT NULL,
                    email TEXT NOT NULL UNIQUE,
                    access_token TEXT NOT NULL,
                    refresh_token TEXT NOT NULL,
                    expires_at TEXT NOT NULL
                )
