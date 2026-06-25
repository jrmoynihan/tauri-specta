#![allow(missing_docs)]

use std::{fs, path::PathBuf};

use specta::{
    Types,
    datatype::{DataType, Primitive},
};
use tauri_specta::{Builder, BuilderConfiguration, collect_commands, collect_types};

mod commands {
    use specta::specta;
    use tauri::{Runtime, State, Window};

    /// Greets the user by name.
    #[tauri::command]
    #[specta]
    pub fn greet(name: String, age: i32) -> bool {
        let _ = (name, age);
        true
    }

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

    #[tauri::command]
    #[specta]
    pub fn excluded_from_collect() -> String {
        "nope".into()
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

fn primitive(dt: &DataType) -> Option<Primitive> {
    match dt {
        DataType::Primitive(p) => Some(p.clone()),
        _ => None,
    }
}

fn temp_export_path(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "tauri-specta-collect-types-{name}-{}",
        std::process::id()
    ))
}

fn configuration_from_collect_types(
    commands: Vec<specta::datatype::Function>,
    types: Types,
) -> BuilderConfiguration {
    BuilderConfiguration::from_collected_types(commands, types)
}

fn assert_collect_types_returns_tuple(result: (Vec<specta::datatype::Function>, Types)) {
    let _ = result;
}

fn assert_collect_commands_returns_commands<R: tauri::Runtime>(
    result: tauri_specta::Commands<R>,
) {
    let _ = result;
}

#[test]
fn collect_types_compiles_for_supported_paths() {
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

#[test]
fn collect_types_returns_type_metadata_not_tauri_handler() {
    let result = collect_types![commands::greet];
    assert_collect_types_returns_tuple(result);

    let commands = collect_commands![commands::greet];
    assert_collect_commands_returns_commands::<tauri::Wry>(commands);
}

#[test]
fn collect_types_collects_only_listed_commands() {
    let (commands, _types) = collect_types![commands::greet];

    assert_eq!(commands.len(), 1);
    assert_eq!(commands[0].name(), "greet");
    assert!(
        !commands
            .iter()
            .any(|command| command.name() == "excluded_from_collect")
    );
}

#[test]
fn collect_types_preserves_function_signatures() {
    let (commands, _types) = collect_types![commands::greet];
    let greet = &commands[0];

    assert_eq!(greet.name(), "greet");
    assert_eq!(greet.args().len(), 2);
    assert_eq!(greet.args()[0].0, "name");
    assert_eq!(greet.args()[1].0, "age");
    assert_eq!(primitive(&greet.args()[1].1), Some(Primitive::i32));
    assert_eq!(
        primitive(greet.result().expect("greet should have a return type")),
        Some(Primitive::bool)
    );
}

#[test]
fn collect_types_preserves_rust_doc_comments() {
    let (commands, _types) = collect_types![commands::greet];

    assert!(
        commands[0].docs.contains("Greets the user by name."),
        "expected Rust `///` docs on the command to be captured by `#[specta::specta]`"
    );
}

#[test]
fn collect_types_omits_tauri_injected_arguments() {
    let (commands, _types) = collect_types![commands::state, commands::window::<tauri::Wry>];

    assert_eq!(commands[0].args().len(), 0, "State<T> should not be exported");
    assert_eq!(
        commands[1].args().len(),
        0,
        "Window<R> should not be exported"
    );
}

#[cfg(all(feature = "typescript", feature = "javascript"))]
mod export {
    use super::*;
    use specta_typescript::{JSDoc, Typescript};
    use tauri_specta::LanguageExt;

    fn read_export(path: &PathBuf) -> String {
        fs::read_to_string(path).expect("exported file should exist")
    }

    #[test]
    fn collect_types_export_matches_collect_commands() {
        let path_from_collect_types = temp_export_path("collect-types");
        let path_from_collect_commands = temp_export_path("collect-commands");

        let (commands, types) = collect_types![commands::greet];
        Typescript::default()
            .export(
                &configuration_from_collect_types(commands, types),
                &path_from_collect_types,
            )
            .expect("collect_types export should succeed");

        Builder::<tauri::Wry>::new()
            .commands(collect_commands![commands::greet])
            .export(Typescript::default(), &path_from_collect_commands)
            .expect("collect_commands export should succeed");

        assert_eq!(
            read_export(&path_from_collect_types),
            read_export(&path_from_collect_commands),
            "collect_types and collect_commands should produce identical typescript exports"
        );

        let _ = fs::remove_file(&path_from_collect_types);
        let _ = fs::remove_file(&path_from_collect_commands);
    }

    #[test]
    fn collect_types_export_only_includes_listed_commands() {
        let path = temp_export_path("partial-export");

        let (commands, types) = collect_types![commands::greet];
        Typescript::default()
            .export(
                &configuration_from_collect_types(commands, types),
                &path,
            )
            .expect("collect_types export should succeed");

        let output = read_export(&path);

        assert!(output.contains("greet"));
        assert!(
            !output.contains("excludedFromCollect"),
            "unlisted commands must not appear in a collect_types export"
        );
        assert!(
            !output.contains("/** Events */"),
            "collect_types exports should not include events unless configured"
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn collect_types_export_preserves_rust_docs_in_typescript() {
        let path = temp_export_path("typescript-docs");

        let (commands, types) = collect_types![commands::greet];
        Typescript::default()
            .export(
                &configuration_from_collect_types(commands, types),
                &path,
            )
            .expect("collect_types export should succeed");

        let output = read_export(&path);

        assert!(
            output.contains("Greets the user by name."),
            "typescript export should preserve Rust doc comments from `#[specta::specta]`"
        );

        let _ = fs::remove_file(path);
    }

    #[test]
    fn collect_types_export_adds_jsdoc_param_and_return_tags() {
        let path = temp_export_path("jsdoc-docs");

        let (commands, types) = collect_types![commands::greet];
        JSDoc::default()
            .export(
                &configuration_from_collect_types(commands, types),
                &path,
            )
            .expect("collect_types jsdoc export should succeed");

        let output = read_export(&path);

        assert!(
            output.contains("Greets the user by name."),
            "jsdoc export should preserve Rust doc comments from `#[specta::specta]`"
        );
        assert!(
            output.contains("@param {string} name"),
            "jsdoc export should add @param tags for command arguments"
        );
        assert!(
            output.contains("@param {number} age"),
            "jsdoc export should add @param tags for command arguments"
        );
        assert!(
            output.contains("@returns {Promise<boolean>}"),
            "jsdoc export should add @returns tags for command results"
        );

        let _ = fs::remove_file(path);
    }
}
