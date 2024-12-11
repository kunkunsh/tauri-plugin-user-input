use std::time::SystemTime;

use rdev::{exit_grab, listen};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tauri::{
    async_runtime::{JoinHandle, Mutex},
    plugin::PluginApi,
    AppHandle, Emitter, Manager, Runtime,
};

use crate::{models::*, Error};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<UserInput<R>> {
    Ok(UserInput {
        app_handle: app.clone(),
        rdev_handle: Mutex::new(None),
    })
}

/// Access to the user-input APIs.
pub struct UserInput<R: Runtime> {
    app_handle: AppHandle<R>,
    rdev_handle: Mutex<Option<JoinHandle<()>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InputEvent {
    pub event_type: String,
    pub time: SystemTime,
}

impl From<rdev::Event> for InputEvent {
    fn from(event: rdev::Event) -> Self {
        let event_type = match event.event_type {
            rdev::EventType::KeyPress(_) => "keypress",
            rdev::EventType::KeyRelease(_) => "keyrelease",
            rdev::EventType::ButtonPress(_) => "buttonpress",
            rdev::EventType::ButtonRelease(_) => "buttonrelease",
            rdev::EventType::MouseMove { .. } => "mousemove",
            rdev::EventType::Wheel { .. } => "wheel",
        }
        .to_string();

        Self {
            event_type,
            time: event.time,
        }
    }
}

impl<R: Runtime> UserInput<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub async fn start_listening(&self, window_labels: Vec<String>) -> Result<(), Error> {
        // skip if rdev_handle is already set
        let rdev_handle = self.rdev_handle.lock().await;
        if rdev_handle.is_some() {
            return Ok(());
        }
        let app_handle = self.app_handle.clone();
        let window_labels = window_labels.clone();
        let handle = tauri::async_runtime::spawn(async move {
            // let callback = move |event: rdev::Event| -> Option<rdev::Event> {
            //     if window_labels.len() == 0 {
            //         app_handle
            //             .emit("user-input", InputEvent::from(event.clone()))
            //             .unwrap();
            //     } else {
            //         for label in &window_labels {
            //             app_handle
            //                 .emit_to(label, "user-input", InputEvent::from(event.clone()))
            //                 .unwrap();
            //         }
            //     }
            //     Some(event)
            // };
            if let Err(error) = rdev::listen(move |event: rdev::Event| {
                println!("Event: {:?}", event);
                if window_labels.len() == 0 {
                    app_handle.emit("user-input", event).unwrap();
                    // app_handle
                    //     .emit("user-input", InputEvent::from(event.clone()))
                    //     .unwrap();
                } else {
                    for label in &window_labels {
                        app_handle
                            .emit_to(label, "user-input", event.clone())
                            .unwrap();
                    }
                }
            }) {
                println!("Error: {:?}", error)
            }
        });
        let mut rdev_handle = self.rdev_handle.lock().await;
        *rdev_handle = Some(handle);
        Ok(())
    }

    pub async fn stop_listening(&self) -> Result<(), rdev::GrabError> {
        let mut rdev_handle = self.rdev_handle.lock().await;
        rdev::exit_grab()?;
        if let Some(handle) = rdev_handle.take() {
            handle.abort();
        }
        assert!(rdev_handle.is_none());
        Ok(())
    }
}
