use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::UserInput;
#[cfg(mobile)]
use mobile::UserInput;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the user-input APIs.
pub trait UserInputExt<R: Runtime> {
  fn user_input(&self) -> &UserInput<R>;
}

impl<R: Runtime, T: Manager<R>> crate::UserInputExt<R> for T {
  fn user_input(&self) -> &UserInput<R> {
    self.state::<UserInput<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("user-input")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let user_input = mobile::init(app, api)?;
      #[cfg(desktop)]
      let user_input = desktop::init(app, api)?;
      app.manage(user_input);
      Ok(())
    })
    .build()
}
