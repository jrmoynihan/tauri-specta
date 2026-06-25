# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **`collect_types!` macro** — Restores the specta v1 workflow for collecting command
  function signatures without registering Tauri invoke handlers. Useful for splitting
  TypeScript/JSDoc exports across multiple files.
- **`BuilderConfiguration::from_collected_types`** — Helper for exporting the
  `(commands, types)` tuple returned by `collect_types!`.

### Migration from specta v1 / tauri-specta v1

`collect_types!` was removed when tauri-specta v2 adopted specta v2's
`collect_functions!` API. It is now available again on tauri-specta with the
following changes:

| specta v1 | tauri-specta v2 |
| --- | --- |
| `let (functions, type_defs) = collect_types![cmd].unwrap()` | `let (commands, types) = collect_types![cmd]` |
| `type_map: my_type_defs` | `types: my_types` (v1 `type_map:` alias still supported) |
| Returns `Result<(...), ExportError>` | Returns `(Vec<Function>, Types)` directly |

Collection is infallible at runtime in specta v2; missing `#[specta::specta]`
annotations or invalid types surface as compile-time errors instead.

Rust `///` doc comments on commands are still captured by `#[specta::specta]` and
preserved in exported bindings. JSDoc exports additionally include generated
`@param` and `@returns` tags derived from the collected argument and return types.
