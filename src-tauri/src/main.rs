// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

use models::{
    folder::{sample_folders, Folder},
    message::{sample_message_by_id, sample_messages_for_folder, Message},
};
use tauri::Manager;

#[tauri::command]
fn list_folders() -> Vec<Folder> {
    sample_folders()
}

#[tauri::command]
fn list_messages(folder_id: Option<String>) -> Vec<Message> {
    let folder = folder_id.unwrap_or_else(|| "inbox".to_string());
    sample_messages_for_folder(&folder)
}

#[tauri::command]
fn get_message(id: String) -> Option<Message> {
    sample_message_by_id(&id)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_folders,
            list_messages,
            get_message
        ])
        .setup(|app| {
            let main_window = app.get_window("main");
            if let Some(window) = main_window {
                window.set_title("Coer Mail Client").ok();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Coer Mail Client");
}
