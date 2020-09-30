pub(crate) mod sqlite_connection;
pub(crate) mod protip_handler;

use std::fmt;
use tracing::{debug, info, warn};
use crate::database::sqlite_connection::DatabaseConnection;
use tokio::sync::Mutex;


pub struct Database<T: DatabaseConnection> {
    // conn: Option<T>,
    mutex: Mutex<T>,
}

impl<T: DatabaseConnection> Database<T> {
    pub async fn new() -> Self {
        let mutex: Mutex<T> = Mutex::new(T::new());
        {
            let mut db = mutex.lock().await;
            db.connect().unwrap();
            db.set_up().unwrap();
        }

        Database { mutex }
    }

    // fn connect(&mut self) -> Result<()>;
    // fn set_up(&self) -> Result<()>;
}

