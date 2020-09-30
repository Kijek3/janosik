use rusqlite::{params, Connection, Result};
use crate::BoxResult;
use tracing::{debug, error, info, instrument, warn, Level};


pub trait DatabaseConnection {
    // fn connect(&mut self) -> Result<()>;
    fn new() -> Self;
    fn connect(&mut self) -> BoxResult;
    fn set_up(&self) -> Result<()>;
}

pub struct SQLiteConnection;

impl DatabaseConnection for SQLiteConnection {
    fn new() -> Self {
        Self {}
    }

    fn connect(&mut self) -> BoxResult {
        self.mutex.lock().await = Some(Connection::open("janosik.db3")?);
        info!("Database connected");
        Ok(())
    }

    fn set_up(&self) -> Result<()> {
        self.conn.as_ref().unwrap().execute(
            "CREATE TABLE IF NOT EXISTS protip (
                  id              INTEGER PRIMARY KEY,
                  task_id         TEXT NOT NULL,
                  content         TEXT NOT NULL
                  )",
            params![],
        )?;

        info!("Database initialized");
        Ok(())
    }
}