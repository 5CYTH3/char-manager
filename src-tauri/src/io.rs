use crate::{char::CharData, errs::AppError};
use std::{fs::File, io::Read};

use convert_case::Casing;
use tauri::{path::BaseDirectory, AppHandle, Manager};

pub fn create_char_file(c_data: CharData, handle: AppHandle) -> Result<(), AppError> {
    let fmt_name = c_data
        .name
        .to_lowercase()
        .to_case(convert_case::Case::Kebab);

    let chars_dir = handle.path().resolve("chars/", BaseDirectory::AppData)?;

    let data_path = handle
        .path()
        .resolve(format!("chars/{}.json", fmt_name), BaseDirectory::AppData)?;
    // AppData: /var/lib/{app}

    std::fs::create_dir_all(chars_dir)?;
    let mut file = File::create(data_path)?;

    serde_json::to_writer_pretty(&mut file, &c_data).unwrap();
    Ok(())
}

pub fn get_char_infos(name: String, handle: &AppHandle) -> Result<CharData, AppError> {
    let fmt_name = name.to_lowercase().to_case(convert_case::Case::Kebab); // Not sure this is needed

    let data_path = handle
        .path()
        .resolve(format!("chars/{}.json", fmt_name), BaseDirectory::AppData)?;

    let mut f = File::open(data_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;

    let data: CharData = serde_json::from_str(&content)?;
    Ok(data)
}

pub fn get_chars_infos(handle: AppHandle) -> Result<Vec<CharData>, AppError> {
    let data_path = handle.path().resolve("chars/", BaseDirectory::AppData)?;
    let mut characters: Vec<CharData> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(data_path) {
        for entry in entries {
            let f = entry?.path();
            if !f.is_dir() {
                println!("IT IS NOT A DIR");
                // Here we use f.file_stem because f.file_name includes the extension.
                // In the definition of get_char_info we also include the extension. There is a choice to be made here...
                // TODO: Maybe delete the name in get_char_infos
                let name = match f.file_stem() {
                    Some(v) => match v.to_str() {
                        Some(v1) => v1.to_string(),
                        None => {
                            return Err(AppError {
                                kind: "conversion".to_string(),
                                message: ".to_str() conversion failed on name".to_string(),
                            });
                        }
                    },
                    None => {
                        return Err(AppError {
                            kind: "conversion".to_string(),
                            message: "fail on f.file_name()".to_string(),
                        });
                    }
                };
                let char = get_char_infos(name, &handle)?;
                characters.push(char);
            }
        }
    }
    return Ok(characters);
}
