-- Create calendars table
CREATE TABLE IF NOT EXISTS calendars(
                    id TEXT PRIMARY KEY,
                    calendar_id TEXT NOT NULL UNIQUE,
                    primary_calendar INTEGER NOT NULL,
                    title TEXT NOT NULL,
                    description TEXT)
