use rusqlite::{params, Connection, Result};
#[derive(serde::Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

fn open_db() -> rusqlite::Result<Connection> {
    Connection::open("todos.db")
}

pub fn init_db() -> rusqlite::Result<()> {
    let conn = open_db()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            completed INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

#[tauri::command]
pub fn get_todos() -> Result<Vec<Todo>, String> {
    let conn = open_db().map_err(|e| e.to_string())?; // Open the database connection --> connection is not mutable
    let mut stmt = conn
        .prepare("SELECT id, title, completed FROM todos")
        .map_err(|e| e.to_string())?; // mut -> mutable statement
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get::<_, i32>(2)? == 1,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(todos)
}

#[tauri::command]
pub fn add_todo(title: String) -> Result<(), String> {
    let conn = open_db().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
        params![title, 0],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_todo(id: i32, completed: bool) -> Result<(), String> {
    let conn = open_db().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE todos SET completed = ?1 WHERE id = ?2",
        params![if completed { 1 } else { 0 }, id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_todo(id: i32) -> Result<(), String> {
    let conn = open_db().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
