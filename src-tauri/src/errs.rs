use std::io;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}

// Implement std::convert::From for AppError; from io::Error
impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}

// Implement std::convert::From for AppError; from tauri::Error
impl From<tauri::Error> for AppError {
    fn from(error: tauri::Error) -> Self {
        AppError {
            kind: String::from("tauri"),
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(error: serde_json::Error) -> Self {
        AppError {
            kind: String::from("serde_json"),
            message: error.to_string(),
        }
    }
}
