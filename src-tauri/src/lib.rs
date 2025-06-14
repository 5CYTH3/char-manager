// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod char;
mod errs;
mod io;

use char::CharData;
use tauri::{
    menu::{MenuBuilder, SubmenuBuilder},
    AppHandle,
};

#[tauri::command]
fn get_char_infos(name: String, handle: AppHandle) -> Result<CharData, errs::AppError> {
    crate::io::get_char_infos(name, &handle)
}

#[tauri::command]
fn get_chars_infos(handle: AppHandle) -> Result<Vec<CharData>, errs::AppError> {
    crate::io::get_chars_infos(handle)
}

#[tauri::command]
fn get_dummy_data() -> CharData {
    char::dummy_data()
}

#[tauri::command]
fn create_char_file(c_data: CharData, handle: AppHandle) -> Result<(), errs::AppError> {
    crate::io::create_char_file(c_data, handle)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let actions_menu = SubmenuBuilder::new(app, "Actions")
                .text("roll", "Roll Dice")
                .build()?;
            let spell_menu = SubmenuBuilder::new(app, "Spells")
                .text("spells", "Spell List")
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&actions_menu, &spell_menu])
                .build()?;

            app.set_menu(menu)?;
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_char_infos,
            get_chars_infos,
            get_dummy_data,
            create_char_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
