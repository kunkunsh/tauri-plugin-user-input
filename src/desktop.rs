use enigo::{Keyboard, Mouse};
use lazy_static::lazy_static;
#[cfg(target_os = "linux")]
use rdev::start_grab_listen as start_grab;
#[cfg(target_os = "macos")]
use rdev::start_grab_listen as start_grab;
use rdev::{EventType, SimulateError};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
    sync::{Arc, Mutex},
    time::SystemTime,
};
use tauri::{
    async_runtime::JoinHandle, ipc::Channel, plugin::PluginApi, AppHandle, Emitter, Manager,
    Runtime,
};

struct SafeEnigo(Mutex<enigo::Enigo>);

unsafe impl Sync for SafeEnigo {}

lazy_static::lazy_static! {
    static ref ENIGO: SafeEnigo = SafeEnigo(Mutex::new(enigo::Enigo::new(&enigo::Settings::default()).unwrap()));
}

use crate::{
    models::{self, *},
    Error,
};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<UserInput<R>> {
    Ok(UserInput {
        app_handle: app.clone(),
        rdev_handle: Mutex::new(None),
        window_labels: Arc::new(Mutex::new(vec![])),
        event_types: Arc::new(Mutex::new(HashSet::new())),
        on_event_channels: Arc::new(Mutex::new(HashMap::new())),
    })
}

/// Access to the user-input APIs.
pub struct UserInput<R: Runtime> {
    app_handle: AppHandle<R>,
    rdev_handle: Mutex<Option<JoinHandle<()>>>,
    window_labels: Arc<Mutex<Vec<String>>>,
    event_types: Arc<Mutex<HashSet<models::EventType>>>,
    on_event_channels: Arc<Mutex<HashMap<u32, Channel<InputEvent>>>>,
}

fn get_enigo() -> Result<enigo::Enigo, String> {
    enigo::Enigo::new(&enigo::Settings::default()).map_err(|err| format!("Error: {:?}", err))
}

impl<R: Runtime> UserInput<R> {
    pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            value: payload.value,
        })
    }

    pub fn start_listening(&self, on_event: Channel<InputEvent>) -> Result<(), Error> {
        let mut on_event_channels = self.on_event_channels.lock().unwrap();
        let event_id = on_event.id();
        on_event_channels.insert(event_id, on_event.clone());
        drop(on_event_channels);

        // skip if rdev_handle is already set
        let mut rdev_handle = self.rdev_handle.lock().unwrap();
        if rdev_handle.is_some() {
            return Ok(());
        }

        let app_handle = self.app_handle.clone();
        let window_labels_clone = self.window_labels.clone();
        let on_event_channels_clone = self.on_event_channels.clone();
        let event_types_clone = self.event_types.clone();
        println!("start listening");
        let handle = tauri::async_runtime::spawn(async move {
            println!("start listening in thread");
            #[cfg(target_os = "macos")]
            rdev::set_is_main_thread(false); // without this line, any key event will crash the app
            if let Err(error) = start_grab(move |event: rdev::Event| {
                println!("event: {:?}", event.clone());
                let event2 = event.clone();
                let evt = InputEvent::from(event.clone());
                let event_types = event_types_clone.lock().unwrap();
                if event_types.contains(&evt.event_type) {
                    let window_labels = window_labels_clone.lock().unwrap();
                    for win_label in window_labels.iter() {
                        app_handle.emit_to(win_label, "user-input", &evt).unwrap();
                    }
                    drop(window_labels);
                    let channels = on_event_channels_clone.lock().unwrap();
                    for channel in channels.values() {
                        channel.send(evt.clone()).unwrap();
                    }
                    drop(channels);
                }
                Some(event2)
            }) {
                println!("Error: {:?}", error)
            }
            #[cfg(target_os = "linux")]
            {
                rdev::enable_grab();
            }
        });

        *rdev_handle = Some(handle);
        Ok(())
    }

    pub fn stop_listening(&self) -> Result<(), rdev::GrabError> {
        let is_grabbed = rdev::is_grabbed();
        if is_grabbed {
            #[cfg(target_os = "macos")]
            rdev::exit_grab()?;

            #[cfg(target_os = "linux")]
            {
                rdev::disable_grab();
                rdev::exit_grab_listen();
            }
        }
        let mut rdev_handle = self.rdev_handle.lock().unwrap();
        if let Some(handle) = rdev_handle.take() {
            handle.abort();
        }
        let mut on_event_channels = self.on_event_channels.lock().unwrap();
        // remove all channels
        on_event_channels.clear();
        assert_eq!(on_event_channels.len(), 0);
        assert!(rdev_handle.is_none());
        Ok(())
    }

    pub fn set_window_labels(&self, labels: Vec<String>) -> Result<(), Error> {
        let mut window_labels = self.window_labels.lock().unwrap();
        *window_labels = labels;
        Ok(())
    }

    pub fn set_event_types(&self, event_types: Vec<models::EventType>) -> Result<(), Error> {
        let mut _event_types = self.event_types.lock().unwrap();
        *_event_types = event_types.into_iter().collect();
        println!("{:?}", *_event_types);
        Ok(())
    }

    pub fn is_listening(&self) -> bool {
        rdev::is_grabbed()
    }
    /* -------------------------------------------------------------------------- */
    /*                                 enigo APIs                                 */
    /* -------------------------------------------------------------------------- */
    /// enigo's key method cause crash on MacOS, so we use rdev to simulate the key event
    pub fn key(&self, key: rdev::Key, evt_type: models::EventType) -> Result<(), SimulateError> {
        match evt_type {
            models::EventType::KeyPress => rdev::simulate(&EventType::KeyPress(key)),
            models::EventType::KeyRelease => rdev::simulate(&EventType::KeyRelease(key)),
            models::EventType::KeyClick => match rdev::simulate(&EventType::KeyPress(key)) {
                Ok(_) => match rdev::simulate(&EventType::KeyRelease(key)) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            },
            _ => Err(SimulateError),
        }
    }
    // pub fn key(&self, key: enigo::Key, direction: enigo::Direction) -> Result<(), String> {
    //     println!("desktop: key: {:?}, direction: {:?}", key, direction);
    //     let mut _enigo = ENIGO.0.lock().unwrap();
    //     _enigo.key(key, direction).unwrap();
    //     // let mut _enigo = get_enigo()?;
    //     // _enigo.key(key, direction).unwrap();
    //     //     .map_err(|err| format!("Error: {:?}", err))?;
    //     Ok(())
    // }

    pub fn text(&self, text: &str) -> Result<(), String> {
        let mut _enigo = get_enigo()?;
        // let mut _enigo = ENIGO.0.lock().unwrap();
        _enigo
            .text(text)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn button(&self, button: enigo::Button, direction: enigo::Direction) -> Result<(), String> {
        let mut _enigo = ENIGO.0.lock().unwrap();
        _enigo
            .button(button, direction)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    // pub fn button(
    //     &self,
    //     button: rdev::Button,
    //     evt_type: models::EventType,
    // ) -> Result<(), rdev::SimulateError> {
    //     match evt_type {
    //         models::EventType::ButtonPress => rdev::simulate(&rdev::EventType::ButtonPress(button)),
    //         models::EventType::ButtonRelease => {
    //             rdev::simulate(&rdev::EventType::ButtonRelease(button))
    //         }
    //         _ => Err(SimulateError),
    //     }
    // }

    pub fn move_mouse(&self, x: i32, y: i32, coordinate: enigo::Coordinate) -> Result<(), String> {
        let mut _enigo = ENIGO.0.lock().unwrap();
        _enigo
            .move_mouse(x, y, coordinate)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }

    pub fn scroll(&self, length: i32, axis: enigo::Axis) -> Result<(), String> {
        let mut _enigo = ENIGO.0.lock().unwrap();
        _enigo
            .scroll(length, axis)
            .map_err(|err| format!("Error: {:?}", err))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key() {
        // let mut _enigo1 = get_enigo().unwrap();
        let mut _enigo = ENIGO.0.lock().unwrap();

        _enigo
            .key(enigo::Key::Meta, enigo::Direction::Press)
            .unwrap();
        // let mut _enigo2 = get_enigo().unwrap();
        _enigo
            .key(enigo::Key::Unicode('A'), enigo::Direction::Press)
            .unwrap();
        // let mut _enigo3 = get_enigo().unwrap();
        _enigo
            .key(enigo::Key::Meta, enigo::Direction::Release)
            .unwrap();
        // let mut _enigo4 = get_enigo().unwrap();
        _enigo
            .key(enigo::Key::Unicode('A'), enigo::Direction::Release)
            .unwrap();
    }
}
