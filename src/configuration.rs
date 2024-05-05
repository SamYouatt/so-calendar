use std::{fs, path::PathBuf};

use rusqlite::Connection;

pub enum ApplicationError {
    FailedToCreateAccounts,
}

pub struct Application {
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
}

impl Application {
    pub fn new() -> Self {
        let data_dir = dirs_next::data_dir()
            .expect("Unable to find data directory")
            .join("so-calendar");

        let db_path = data_dir.join("app.sqlite");

        Self { data_dir, db_path }
    }

    pub fn setup(&self) -> Result<(), ApplicationError> {
        // Create app data directory
        fs::create_dir_all(&self.data_dir).expect("Failed to create data directory");

        // Create required tables
        let db = Connection::open(&self.db_path).unwrap();

        db.execute(
            "CREATE TABLE IF NOT EXISTS accounts(
                    id integer PRIMARY KEY,
                    email text NOT NULL UNIQUE,
                    access_token text NOT NULL,
                    refresh_token text NOT NULL,
                    expires_at text NOT NULL
                )",
            [],
        )
        .map_err(|e| {
            println!("Error creating accounts table: {:?}", e);
            ApplicationError::FailedToCreateAccounts
        })?;

        Ok(())
    }
}
