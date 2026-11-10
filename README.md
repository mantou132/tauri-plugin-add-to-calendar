# Tauri Plugin Add to Calendar

Add events to the system calendar on **iOS** and **Android** in Tauri v2. The plugin opens the system event editor prefilled with your data; the user reviews and saves it there, so the app never reads or writes the calendar directly.

| Platform | Implementation | Permission |
| :--- | :--- | :--- |
| **iOS 17+** | `EKEventEditViewController` | None |
| **iOS 13–16** | `EKEventEditViewController` | Full calendar access (`NSCalendarsUsageDescription`) |
| **Android** | `Intent.ACTION_INSERT` | None |
| **Desktop** | Not supported (rejects) | – |

## Installation

```toml
# src-tauri/Cargo.toml
[target.'cfg(any(target_os = "android", target_os = "ios"))'.dependencies]
tauri-plugin-add-to-calendar = { git = "https://github.com/mantou132/tauri-plugin-add-to-calendar" }
```

```rust
#[cfg(mobile)]
app.handle().plugin(tauri_plugin_add_to_calendar::init())?;
```

```json
// capabilities
"permissions": ["add-to-calendar:default"]
```

```bash
pnpm add tauri-plugin-add-to-calendar-api
```

If your iOS deployment target is below 17, add `NSCalendarsUsageDescription` to `Info.plist`.

## Usage

```ts
import { addEvent } from 'tauri-plugin-add-to-calendar-api';

await addEvent({
  title: 'Flight to Tokyo',
  start: new Date('2026-12-01T09:00').getTime(),
  end: new Date('2026-12-01T13:00').getTime(),
  location: 'SFO',
  notes: 'Terminal 2',
});

// Daily check-in for 30 days
const { saved } = await addEvent({
  title: 'Check in',
  start: new Date('2026-12-01T08:00').getTime(),
  recurrence: { frequency: 'daily', count: 30 },
});
```

`start` / `end` are Unix epoch milliseconds. Without `end`, iOS uses one hour after `start` and Android uses the calendar app's default. `recurrence` repeats the event (`frequency`, optional `interval`, and `count` or `until`); without either it repeats forever.

The system editor adds one event (or one recurring series) at a time. The app can't read the calendar, so it can't detect duplicates. `saved` tells whether the user saved the event: on iOS the promise resolves when the editor closes with `saved` set; Android calendar apps don't report it, so the promise resolves once the editor is shown and `saved` is `undefined`.

## License

MIT or Apache-2.0
