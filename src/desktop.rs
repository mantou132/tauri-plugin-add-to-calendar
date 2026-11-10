use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<AddToCalendar<R>> {
    Ok(AddToCalendar(app.clone()))
}

pub struct AddToCalendar<R: Runtime>(#[allow(dead_code)] AppHandle<R>);

impl<R: Runtime> AddToCalendar<R> {
    pub fn add_event(&self, _payload: CalendarEvent) -> crate::Result<AddEventResult> {
        Err(crate::Error::Unsupported)
    }
}
