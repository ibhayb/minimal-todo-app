// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod todo_db_handler;

use todo_db_handler::{add_todo, delete_todo, get_todos, init_db, update_todo};

fn main() {
    todo_db_handler::init_db().expect("DB init failed");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_todos,
            add_todo,
            update_todo,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    minimal_todo_app_lib::run();
}
