package com.plugin.add_to_calendar

import android.app.Activity
import android.content.ActivityNotFoundException
import android.content.Intent
import android.provider.CalendarContract
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale
import java.util.TimeZone

@InvokeArg
class Recurrence {
    var frequency: String = ""
    var interval: Int? = null
    var count: Int? = null
    var until: Long? = null

    fun toRRule(): String {
        val parts = mutableListOf("FREQ=${frequency.uppercase(Locale.ROOT)}")
        interval?.let { parts += "INTERVAL=$it" }
        val count = count
        val until = until
        if (count != null) {
            parts += "COUNT=$count"
        } else if (until != null) {
            val format = SimpleDateFormat("yyyyMMdd'T'HHmmss'Z'", Locale.ROOT)
            format.timeZone = TimeZone.getTimeZone("UTC")
            parts += "UNTIL=${format.format(Date(until))}"
        }
        return parts.joinToString(";")
    }
}

@InvokeArg
class CalendarEvent {
    var title: String = ""
    var start: Long = 0
    var end: Long? = null
    var allDay: Boolean? = null
    var location: String? = null
    var notes: String? = null
    var recurrence: Recurrence? = null
}

@TauriPlugin
class AddToCalendarPlugin(private val activity: Activity) : Plugin(activity) {
    // ACTION_INSERT opens the calendar app's editor, so no calendar permission is needed;
    // calendar apps don't report whether the event was saved.
    @Command
    fun add_event(invoke: Invoke) {
        val args = invoke.parseArgs(CalendarEvent::class.java)
        val intent = Intent(Intent.ACTION_INSERT, CalendarContract.Events.CONTENT_URI).apply {
            putExtra(CalendarContract.Events.TITLE, args.title)
            putExtra(CalendarContract.EXTRA_EVENT_BEGIN_TIME, args.start)
            args.end?.let { putExtra(CalendarContract.EXTRA_EVENT_END_TIME, it) }
            args.allDay?.let { putExtra(CalendarContract.EXTRA_EVENT_ALL_DAY, it) }
            args.location?.let { putExtra(CalendarContract.Events.EVENT_LOCATION, it) }
            args.notes?.let { putExtra(CalendarContract.Events.DESCRIPTION, it) }
            args.recurrence?.let { putExtra(CalendarContract.Events.RRULE, it.toRRule()) }
        }
        try {
            activity.startActivity(intent)
            invoke.resolve(JSObject())
        } catch (e: ActivityNotFoundException) {
            invoke.reject("No calendar app found")
        }
    }
}
