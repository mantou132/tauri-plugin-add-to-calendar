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
use desktop::AddToCalendar;
#[cfg(mobile)]
use mobile::AddToCalendar;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the add-to-calendar APIs.
pub trait AddToCalendarExt<R: Runtime> {
  fn add_to_calendar(&self) -> &AddToCalendar<R>;
}

impl<R: Runtime, T: Manager<R>> crate::AddToCalendarExt<R> for T {
  fn add_to_calendar(&self) -> &AddToCalendar<R> {
    self.state::<AddToCalendar<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("add-to-calendar")
    .invoke_handler(tauri::generate_handler![commands::add_event])
    .setup(|app, api| {
      #[cfg(mobile)]
      let add_to_calendar = mobile::init(app, api)?;
      #[cfg(desktop)]
      let add_to_calendar = desktop::init(app, api)?;
      app.manage(add_to_calendar);
      Ok(())
    })
    .build()
}
