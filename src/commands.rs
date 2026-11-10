use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::AddToCalendarExt;
use crate::Result;

#[command]
pub(crate) async fn add_event<R: Runtime>(
    app: AppHandle<R>,
    payload: CalendarEvent,
) -> Result<AddEventResult> {
    app.add_to_calendar().add_event(payload)
}
