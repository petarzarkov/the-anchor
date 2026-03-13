#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;
use tauri::State;

pub struct AppState {
    pub count: Mutex<i32>,
}

#[tauri::command]
fn increment(state: State<'_, AppState>) -> i32 {
    let mut n = state.count.lock().unwrap();
    *n += 1;
    *n
}

#[tauri::command]
fn decrement(state: State<'_, AppState>) -> i32 {
    let mut n = state.count.lock().unwrap();
    *n -= 1;
    *n
}

#[tauri::command]
fn reset(state: State<'_, AppState>) -> i32 {
    let mut n = state.count.lock().unwrap();
    *n = 0;
    *n
}

#[tauri::command]
fn random_increment(state: State<'_, AppState>) -> i32 {
    use rand::Rng;
    let amount: i32 = rand::rng().random_range(1..=10);
    let mut n = state.count.lock().unwrap();
    *n += amount;
    *n
}

fn main() {
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            count: Mutex::new(0),
        })
        .invoke_handler(tauri::generate_handler![
            increment,
            decrement,
            reset,
            random_increment,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_state(initial: i32) -> AppState {
        AppState {
            count: Mutex::new(initial),
        }
    }

    #[test]
    fn initial_value_is_zero() {
        let state = make_state(0);
        assert_eq!(*state.count.lock().unwrap(), 0);
    }

    #[test]
    fn increment_increases_count() {
        let state = make_state(0);
        {
            let mut n = state.count.lock().unwrap();
            *n += 1;
        }
        assert_eq!(*state.count.lock().unwrap(), 1);
    }

    #[test]
    fn decrement_decreases_count() {
        let state = make_state(5);
        {
            let mut n = state.count.lock().unwrap();
            *n -= 1;
        }
        assert_eq!(*state.count.lock().unwrap(), 4);
    }

    #[test]
    fn reset_sets_to_zero() {
        let state = make_state(42);
        {
            let mut n = state.count.lock().unwrap();
            *n = 0;
        }
        assert_eq!(*state.count.lock().unwrap(), 0);
    }

    #[test]
    fn count_goes_negative() {
        let state = make_state(0);
        {
            let mut n = state.count.lock().unwrap();
            *n -= 1;
        }
        assert_eq!(*state.count.lock().unwrap(), -1);
    }

    #[test]
    fn random_increment_stays_in_range() {
        let state = make_state(0);
        {
            use rand::Rng;
            let amount: i32 = rand::rng().random_range(1..=10);
            let mut n = state.count.lock().unwrap();
            *n += amount;
            assert!(*n >= 1 && *n <= 10);
        }
    }
}
