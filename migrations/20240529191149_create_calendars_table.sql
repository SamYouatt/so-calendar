-- Create calendars table
CREATE TABLE IF NOT EXISTS calendars(
                    id INTEGER PRIMARY KEY NOT NULL,
                    account_id INTEGER NOT NULL REFERENCES accounts (id),
                    calendar_id TEXT NOT NULL UNIQUE,
                    primary_calendar INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT)
