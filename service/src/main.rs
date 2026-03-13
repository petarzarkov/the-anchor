#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, State,
};

pub struct AppState {
    pub task: Mutex<String>,
}

#[tauri::command]
fn get_task(state: State<'_, AppState>) -> String {
    state.task.lock().unwrap().clone()
}

#[tauri::command]
fn set_task(
    task: String,
    state: State<'_, AppState>,
) -> String {
    let mut t = state.task.lock().unwrap();
    *t = task;
    t.clone()
}

#[tauri::command]
fn clear_task(state: State<'_, AppState>) -> String {
    let mut t = state.task.lock().unwrap();
    *t = String::new();
    t.clone()
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            task: Mutex::new(String::new()),
        })
        .setup(|app| {
            let quit = MenuItem::with_id(
                app, "quit", "Quit", true,
                None::<&str>,
            )?;
            let menu = Menu::with_items(app, &[&quit])?;
            TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .unwrap()
                        .clone(),
                )
                .menu(&menu)
                .tooltip("The Anchor")
                .on_menu_event(|app, event| {
                    if event.id() == "quit" {
                        app.exit(0);
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_task,
            set_task,
            clear_task,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(initial: &str) -> AppState {
        AppState {
            task: Mutex::new(initial.to_string()),
        }
    }

    #[test]
    fn initial_task_is_empty() {
        let state = make_state("");
        assert_eq!(*state.task.lock().unwrap(), "");
    }

    #[test]
    fn set_task_updates_value() {
        let state = make_state("");
        {
            let mut t = state.task.lock().unwrap();
            *t = "Fix DB Migration".to_string();
        }
        assert_eq!(
            *state.task.lock().unwrap(),
            "Fix DB Migration"
        );
    }

    #[test]
    fn clear_task_empties_value() {
        let state = make_state("some task");
        {
            let mut t = state.task.lock().unwrap();
            *t = String::new();
        }
        assert_eq!(*state.task.lock().unwrap(), "");
    }

    #[test]
    fn task_can_be_overwritten() {
        let state = make_state("old task");
        {
            let mut t = state.task.lock().unwrap();
            *t = "new task".to_string();
        }
        assert_eq!(
            *state.task.lock().unwrap(),
            "new task"
        );
    }
}
