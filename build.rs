const COMMANDS: &[&str] = &["ping", "start_listening", "stop_listening"];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
