use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_add_to_calendar);

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<AddToCalendar<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.add_to_calendar", "AddToCalendarPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_add_to_calendar)?;
    Ok(AddToCalendar(handle))
}

pub struct AddToCalendar<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> AddToCalendar<R> {
    pub fn add_event(&self, payload: CalendarEvent) -> crate::Result<AddEventResult> {
        self.0
            .run_mobile_plugin("add_event", payload)
            .map_err(Into::into)
    }
}
