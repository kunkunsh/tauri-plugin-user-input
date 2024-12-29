const COMMANDS: &[&str] = &[
    "start_listening",
    "stop_listening",
    "set_window_labels",
    "set_event_types",
    "is_listening",
    "key",
    "button",
    "move_mouse",
    "scroll",
    "text",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
