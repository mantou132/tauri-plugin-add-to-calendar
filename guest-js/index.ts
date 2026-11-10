import { invoke } from '@tauri-apps/api/core';

export interface Recurrence {
  frequency: 'daily' | 'weekly' | 'monthly' | 'yearly';
  /** Repeat every `interval` periods, defaults to 1 */
  interval?: number;
  /** End after this many occurrences; takes precedence over `until` */
  count?: number;
  /** End at this Unix epoch milliseconds; repeats forever without `count` or `until` */
  until?: number;
}

export interface CalendarEvent {
  title: string;
  /** Unix epoch milliseconds */
  start: number;
  /** Unix epoch milliseconds; defaults to one hour after `start` on iOS and the calendar app's default on Android */
  end?: number;
  allDay?: boolean;
  location?: string;
  notes?: string;
  recurrence?: Recurrence;
}

export interface AddEventResult {
  /** Whether the user saved the event; only known on iOS, `undefined` on Android */
  saved?: boolean;
}

/**
 * Open the system event editor prefilled with `event`; the user confirms and saves it there.
 * Resolves when the editor closes on iOS and once it is shown on Android.
 */
export async function addEvent(event: CalendarEvent): Promise<AddEventResult> {
  return await invoke('plugin:add-to-calendar|add_event', { payload: event });
}
