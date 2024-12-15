use crate::char::CharData;
use std::fs;

use tauri::{path::BaseDirectory, AppHandle, Manager};

fn create_char_file(name: String, handle: AppHandle) {
    /*
    let char_data: CharData = CharData {
        id: 0,
        img_path: "".to_string(),
        name,
    };
    */

    let data_path = handle
        .path()
        .resolve(format!("chars/{name}.json"), BaseDirectory::AppData)
        .unwrap();

    // fs::write(data_path, contents);

    let file_content = std::fs::read_to_string(&data_path).unwrap();

    serde_json::from_str(file_content.as_str()).unwrap()
}
