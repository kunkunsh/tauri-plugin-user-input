use enigo::{Keyboard, Mouse};
use rdev::{exit_grab, listen, EventType};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::time::SystemTime;
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
        let mut rdev_handle = self.rdev_handle.lock().await;
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
            // if let Err(error) = rdev::listen(move |event: rdev::Event| {
            #[cfg(target_os = "macos")]
            rdev::set_is_main_thread(false); // without this line, any key event will crash the app
            if let Err(error) = rdev::grab(move |event: rdev::Event| {
                let event2 = event.clone();
                if window_labels.len() == 0 {
                    match event.event_type {
                        EventType::KeyPress(_key) | EventType::KeyRelease(_key) => {
                            /*  */
                            println!(
                                "name: {:?}, type: {:?}, code: {:#04X?}, scan: {:#06X?}",
                                &event.unicode,
                                &event.event_type,
                                &event.platform_code,
                                &event.position_code
                            );
                            app_handle.emit("user-input", event.clone()).unwrap();
                            app_handle
                                .emit("user-input", InputEvent::from(event.clone()))
                                .unwrap();
                        }
                        EventType::ButtonPress(_button) | EventType::ButtonRelease(_button) => {
                            println!("Button: {:?}", _button);
                            app_handle
                                .emit("user-input", InputEvent::from(event.clone()))
                                .unwrap();
                            app_handle.emit("user-input", event).unwrap();
                        }
                        _ => {}
                    };
                    // app_handle.emit("user-input", event).unwrap();
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
                Some(event2)
            }) {
                println!("Error: {:?}", error)
            }
        });
        *rdev_handle = Some(handle);
        println!("rdev_handle: {:?}", rdev_handle);
        Ok(())
    }

    pub async fn stop_listening(&self) -> Result<(), rdev::GrabError> {
        println!("stop_listening in desktop.rs");
        let is_grabbed = rdev::is_grabbed();
        println!("is_grabbed: {:?}", is_grabbed);
        println!("exit_grab");
        rdev::exit_grab()?;
        let is_grabbed = rdev::is_grabbed();
        println!("is_grabbed: {:?}", is_grabbed);
        let mut rdev_handle = self.rdev_handle.lock().await;
        println!("rdev_handle: {:?}", rdev_handle);
        // rdev::stop_listen();
        let is_grabbed = rdev::is_grabbed();
        println!("is_grabbed: {:?}", is_grabbed);
        if let Some(handle) = rdev_handle.take() {
            handle.abort();
        }
        assert!(rdev_handle.is_none());
        Ok(())
    }

    /* -------------------------------------------------------------------------- */
    /*                                 enigo APIs                                 */
    /* -------------------------------------------------------------------------- */
    pub fn get_enigo(&self) -> Result<enigo::Enigo, String> {
        enigo::Enigo::new(&enigo::Settings::default()).map_err(|err| format!("Error: {:?}", err))
    }

    pub fn key(&self, key: enigo::Key, direction: enigo::Direction) -> Result<(), String> {
        let mut _enigo = self.get_enigo()?;
        _enigo
            .key(key, direction)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn text(&self, text: &str) -> Result<(), String> {
        let mut _enigo = self.get_enigo()?;
        _enigo
            .text(text)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn button(&self, button: enigo::Button, direction: enigo::Direction) -> Result<(), String> {
        let mut _enigo = self.get_enigo()?;
        _enigo
            .button(button, direction)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn move_mouse(&self, x: i32, y: i32, coordinate: enigo::Coordinate) -> Result<(), String> {
        let mut _enigo = self.get_enigo()?;
        _enigo
            .move_mouse(x, y, coordinate)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn scroll(&self, length: i32, axis: enigo::Axis) -> Result<(), String> {
        let mut _enigo = self.get_enigo()?;
        _enigo
            .scroll(length, axis)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }
}
