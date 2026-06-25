#![allow(missing_docs)]

use specta::Types;
use tauri_specta::collect_types;

mod commands {
    use specta::specta;
    use tauri::{Runtime, State, Window};

    #[tauri::command]
    #[specta]
    pub fn basic() -> String {
        "Hello, world!".to_string()
    }

    #[tauri::command]
    #[specta]
    pub fn result() -> Result<String, ()> {
        Ok("Hello, world!".to_string())
    }

    #[tauri::command]
    #[specta]
    pub fn value(input: String) -> String {
        input
    }

    #[tauri::command]
    #[specta]
    pub fn state(state: State<String>) -> String {
        state.inner().clone()
    }

    #[tauri::command]
    #[specta]
    pub fn window<R: Runtime>(window: Window<R>) -> String {
        window.label().to_string()
    }

    #[tauri::command]
    #[specta]
    pub fn generic<R: Runtime>(window: Window<R>) -> String {
        window.label().to_string()
    }

    pub mod nested {
        use specta::specta;

        #[tauri::command]
        #[specta]
        pub fn world() -> String {
            "Hello world".to_string()
        }
    }
}

#[test]
fn test_collect_types() {
    let (_commands, _types) = collect_types![];
    let (_commands, _types) = collect_types![commands::basic];
    let (_commands, _types) = collect_types![commands::basic,];
    let (_commands, _types) = collect_types![commands::basic, commands::result];
    let (_commands, _types) = collect_types![commands::generic::<tauri::Wry>];
    let (_commands, _types) = collect_types![commands::generic::<tauri::Wry>,];
    let (_commands, _types) =
        collect_types![commands::basic, commands::nested::world, commands::value];

    let types = Types::default();
    let (_commands, _types) = collect_types![types: types, commands::basic];

    let type_map = Types::default();
    let (_commands, _types) = collect_types![type_map: type_map, commands::basic, commands::result];
}
