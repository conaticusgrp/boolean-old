use crate::log;
use uuid::Uuid;

pub fn optional_row<T>(row: sqlx::Result<T>) -> Option<T> {
    match row {
        Ok(row) => Some(row),
        Err(err) => {
            let message = err.to_string();
            if !message.contains("RowNotFound") {
                log::error("Database error", message);
            }
            None
        }
    }
}

pub fn empty_rows<T>(rows: sqlx::Result<Vec<T>>) -> Vec<T> {
    match rows {
        Ok(rows) => rows,
        Err(err) => {
            let message = err.to_string();
            if !message.contains("RowNotFound") {
                log::error("Database error", message);
            }
            Vec::new()
        }
    }
}

pub fn new_id() -> String {
    Uuid::new_v4().to_string()
}
