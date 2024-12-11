use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::UserInputExt;

#[command]
pub(crate) async fn ping<R: Runtime>(
    app: AppHandle<R>,
    payload: PingRequest,
) -> Result<PingResponse> {
    app.user_input().ping(payload)
}

#[command]
pub(crate) async fn start_listening<R: Runtime>(
    app: AppHandle<R>,
    window_labels: Vec<String>,
) -> Result<()> {
    app.user_input().start_listening(window_labels).await?;
    Ok(())
}

#[command]
pub(crate) async fn stop_listening<R: Runtime>(
    app: AppHandle<R>,
) -> std::result::Result<(), String> {
    app.user_input()
        .stop_listening()
        .await
        .map_err(|_| "Failed to stop listening")?;
    Ok(())
}
