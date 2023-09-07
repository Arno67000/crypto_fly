use std::env;
use std::path::Path;

#[tauri::command]
pub fn get_vault() -> bool {
    let current_dir = env::current_dir().unwrap();
    Path::new(&current_dir.join("SecuredVault")).exists()
}

#[tauri::command]
pub fn get_zip() -> bool {
    let current_dir = env::current_dir().unwrap();
    Path::new(&current_dir.join("enc.zip")).exists()
}
