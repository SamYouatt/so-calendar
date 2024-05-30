-- Create calendars table
CREATE TABLE IF NOT EXISTS calendars(
                    id TEXT PRIMARY KEY NOT NULL,
                    account_id TEXT NOT NULL REFERENCES accounts (id),
                    calendar_id TEXT NOT NULL UNIQUE,
                    primary_calendar INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT)
