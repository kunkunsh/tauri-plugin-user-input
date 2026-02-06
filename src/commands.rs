use tauri::ipc::Channel;
use tauri::{command, AppHandle, Runtime};

use crate::models::{self, *};
use crate::UserInputExt;

#[command]
pub(crate) async fn set_window_labels<R: Runtime>(
    app: AppHandle<R>,
    labels: Vec<String>,
) -> crate::Result<()> {
    app.user_input().set_window_labels(labels)?;
    Ok(())
}

#[command]
pub(crate) async fn set_event_types<R: Runtime>(
    app: AppHandle<R>,
    event_types: Vec<EventType>,
) -> crate::Result<()> {
    app.user_input().set_event_types(event_types)?;
    Ok(())
}

#[command]
pub(crate) async fn is_listening<R: Runtime>(app: AppHandle<R>) -> crate::Result<bool> {
    Ok(app.user_input().is_listening())
}

#[command]
pub(crate) async fn start_listening<R: Runtime>(
    app: AppHandle<R>,
    on_event: Channel<InputEvent>,
) -> crate::Result<()> {
    app.user_input().start_listening(on_event)?;
    Ok(())
}

#[command]
pub(crate) async fn stop_listening<R: Runtime>(app: AppHandle<R>) -> Result<(), String> {
    app.user_input()
        .stop_listening()
        .map_err(|e| format!("Failed to stop listening: {}", e))?;
    Ok(())
}

#[command]
pub(crate) async fn key<R: Runtime>(
    app: AppHandle<R>,
    key: monio::Key,
    evt_type: models::EventType,
    delay_ms: Option<u64>,
) -> Result<(), String> {
    if let Some(delay_ms) = delay_ms {
        std::thread::sleep(std::time::Duration::from_millis(delay_ms));
    }
    app.user_input()
        .key(key, evt_type)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub(crate) async fn text<R: Runtime>(app: AppHandle<R>, text: &str) -> Result<(), String> {
    app.user_input().text(text)?;
    Ok(())
}

#[command]
pub(crate) async fn button<R: Runtime>(
    app: AppHandle<R>,
    button: enigo::Button,
    direction: enigo::Direction,
) -> Result<(), String> {
    app.user_input().button(button, direction)?;
    Ok(())
}

#[command]
pub(crate) async fn move_mouse<R: Runtime>(
    app: AppHandle<R>,
    x: i32,
    y: i32,
    coordinate: enigo::Coordinate,
) -> Result<(), String> {
    app.user_input().move_mouse(x, y, coordinate)?;
    Ok(())
}

#[command]
pub(crate) async fn scroll<R: Runtime>(
    app: AppHandle<R>,
    length: i32,
    axis: enigo::Axis,
) -> Result<(), String> {
    app.user_input().scroll(length, axis)?;
    Ok(())
}
