use enigo::{Keyboard, Mouse};
use serde::de::DeserializeOwned;
use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, LazyLock, Mutex},
};
use tauri::{ipc::Channel, plugin::PluginApi, AppHandle, Emitter, Runtime};

struct SafeEnigo(Mutex<enigo::Enigo>);

unsafe impl Send for SafeEnigo {}
unsafe impl Sync for SafeEnigo {}

static ENIGO: LazyLock<SafeEnigo> = LazyLock::new(|| {
    SafeEnigo(Mutex::new(
        enigo::Enigo::new(&enigo::Settings::default()).unwrap(),
    ))
});

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
        hook: Arc::new(Mutex::new(None)),
        window_labels: Arc::new(Mutex::new(vec![])),
        event_types: Arc::new(Mutex::new(HashSet::new())),
        on_event_channels: Arc::new(Mutex::new(HashMap::new())),
    })
}

pub struct UserInput<R: Runtime> {
    app_handle: AppHandle<R>,
    hook: Arc<Mutex<Option<monio::Hook>>>,
    window_labels: Arc<Mutex<Vec<String>>>,
    event_types: Arc<Mutex<HashSet<models::EventType>>>,
    on_event_channels: Arc<Mutex<HashMap<u32, Channel<InputEvent>>>>,
}

fn handle_monio_event<R: Runtime>(
    event: &monio::Event,
    event_types: &HashSet<models::EventType>,
    window_labels: &[String],
    channels: &HashMap<u32, Channel<InputEvent>>,
    app_handle: &AppHandle<R>,
) {
    let evt = InputEvent::from(event.clone());
    if event_types.contains(&evt.event_type) {
        for win_label in window_labels.iter() {
            let _ = app_handle.emit_to(win_label, "user-input", &evt);
        }
        for channel in channels.values() {
            let _ = channel.send(evt.clone());
        }
    }
}

impl<R: Runtime> UserInput<R> {
    pub fn start_listening(&self, on_event: Channel<InputEvent>) -> Result<(), Error> {
        let mut on_event_channels = self.on_event_channels.lock().unwrap();
        let event_id = on_event.id();
        on_event_channels.insert(event_id, on_event.clone());
        drop(on_event_channels);

        let mut hook_guard = self.hook.lock().unwrap();
        if hook_guard.is_some() {
            return Ok(());
        }

        let app_handle = self.app_handle.clone();
        let window_labels_clone = self.window_labels.clone();
        let on_event_channels_clone = self.on_event_channels.clone();
        let event_types_clone = self.event_types.clone();

        let hook = monio::Hook::new();
        match hook.run_async(move |event: &monio::Event| {
            handle_monio_event(
                event,
                &event_types_clone.lock().unwrap(),
                &window_labels_clone.lock().unwrap(),
                &on_event_channels_clone.lock().unwrap(),
                &app_handle,
            );
        }) {
            Ok(_) => println!("Listening started"),
            Err(e) => println!("Error: {:?}", e),
        };

        *hook_guard = Some(hook);
        Ok(())
    }

    pub fn stop_listening(&self) -> Result<(), String> {
        let mut hook_guard = self.hook.lock().unwrap();
        if let Some(hook) = hook_guard.take() {
            hook.stop()
                .map_err(|e| format!("Failed to stop hook: {:?}", e))?;
        }
        let mut on_event_channels = self.on_event_channels.lock().unwrap();
        on_event_channels.clear();
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
        self.hook.lock().unwrap().is_some()
    }
    /* -------------------------------------------------------------------------- */
    /*                                 Key APIs                                   */
    /* -------------------------------------------------------------------------- */
    pub fn key(&self, key: monio::Key, evt_type: models::EventType) -> Result<(), String> {
        match evt_type {
            models::EventType::KeyPress => monio::key_press(key).map_err(|e| format!("{:?}", e)),
            models::EventType::KeyRelease => {
                monio::key_release(key).map_err(|e| format!("{:?}", e))
            }
            models::EventType::KeyClick => monio::key_tap(key).map_err(|e| format!("{:?}", e)),
            _ => Err("Invalid event type for key simulation".to_string()),
        }
    }

    pub fn text(&self, text: &str) -> Result<(), String> {
        let mut _enigo = ENIGO.0.lock().unwrap();
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
        let mut _enigo = ENIGO.0.lock().unwrap();

        _enigo
            .key(enigo::Key::Meta, enigo::Direction::Press)
            .unwrap();
        _enigo
            .key(enigo::Key::Unicode('A'), enigo::Direction::Press)
            .unwrap();
        _enigo
            .key(enigo::Key::Meta, enigo::Direction::Release)
            .unwrap();
        _enigo
            .key(enigo::Key::Unicode('A'), enigo::Direction::Release)
            .unwrap();
    }
}
