//! Tauri merges `tauri.windows.conf.json` as a JSON Merge Patch, which replaces
//! arrays, so it has to repeat the whole `app.windows` entry. Keep the copies in sync.

use std::path::Path;

use serde_json::Value;

fn windows(file: &str) -> Vec<Value> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(file);
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{file}: {e}"));
    let config: Value = serde_json::from_str(&text).unwrap_or_else(|e| panic!("{file}: {e}"));
    config["app"]["windows"]
        .as_array()
        .unwrap_or_else(|| panic!("{file} has no app.windows array"))
        .clone()
}

#[test]
fn windows_config_differs_from_base_only_in_decorations() {
    let mut base = windows("tauri.conf.json");
    let mut windows_only = windows("tauri.windows.conf.json");
    assert_eq!(base.len(), windows_only.len(), "window count");
    for (base, win) in base.iter_mut().zip(&mut windows_only) {
        assert_eq!(win["decorations"], false, "Windows draws its own title bar");
        // Drop the one intended difference from both sides; the base may omit the key (default true).
        for entry in [&mut *base, &mut *win] {
            entry
                .as_object_mut()
                .expect("window entry is an object")
                .remove("decorations");
        }
        assert_eq!(win, base, "tauri.windows.conf.json window entry drifted");
    }
}
