// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod char;
mod io;

use std::io::Read;

use char::CharData;
use tauri::{
    path::{self, BaseDirectory},
    AppHandle, Manager,
};
#[tauri::command]
// For now, name must be snakecase but soon characters will be referenced by UUID
// Assume name is a valid character name
fn get_char_infos(name: String, handle: AppHandle) -> char::CharData {
    let data_path_res = handle
        .path()
        .resolve(format!("chars/{name}.json"), BaseDirectory::AppData);
    let data_path = match data_path_res {
        Ok(d) => d,
        Err(_) => todo!("Unresolved path + create a dialog for this error pls"),
    };

    let file_content = std::fs::read_to_string(&data_path).unwrap();

    serde_json::from_str(file_content.as_str()).unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_char_infos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
