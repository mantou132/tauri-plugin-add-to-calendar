import EventKit
import EventKitUI
import SwiftRs
import Tauri
import UIKit
import WebKit

enum Frequency: String, Decodable {
    case daily, weekly, monthly, yearly

    var ek: EKRecurrenceFrequency {
        switch self {
        case .daily: return .daily
        case .weekly: return .weekly
        case .monthly: return .monthly
        case .yearly: return .yearly
        }
    }
}

struct Recurrence: Decodable {
    let frequency: Frequency
    let interval: Int?
    let count: Int?
    let until: Double?
}

struct CalendarEvent: Decodable {
    let title: String
    let start: Double
    let end: Double?
    let allDay: Bool?
    let location: String?
    let notes: String?
    let recurrence: Recurrence?
}

func date(_ ms: Double) -> Date {
    return Date(timeIntervalSince1970: ms / 1000)
}

class AddToCalendarPlugin: Plugin, EKEventEditViewDelegate {
    private let store = EKEventStore()
    private var pendingInvoke: Invoke?

    @objc public func add_event(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(CalendarEvent.self)
        // iOS 17+ runs the editor out of process without calendar access;
        // earlier versions need full access (NSCalendarsUsageDescription).
        if #available(iOS 17.0, *) {
            present(args, invoke)
        } else {
            store.requestAccess(to: .event) { granted, error in
                if granted {
                    self.present(args, invoke)
                } else {
                    invoke.reject(error?.localizedDescription ?? "Calendar access denied")
                }
            }
        }
    }

    private func present(_ args: CalendarEvent, _ invoke: Invoke) {
        DispatchQueue.main.async {
            guard let viewController = self.manager.viewController else {
                invoke.reject("No view controller to present from")
                return
            }
            let start = date(args.start)
            let event = EKEvent(eventStore: self.store)
            event.title = args.title
            event.startDate = start
            event.endDate = args.end.map(date) ?? start.addingTimeInterval(3600)
            event.isAllDay = args.allDay ?? false
            event.location = args.location
            event.notes = args.notes
            if let recurrence = args.recurrence {
                let end = recurrence.count.map { EKRecurrenceEnd(occurrenceCount: $0) }
                    ?? recurrence.until.map { EKRecurrenceEnd(end: date($0)) }
                event.addRecurrenceRule(EKRecurrenceRule(
                    recurrenceWith: recurrence.frequency.ek,
                    interval: recurrence.interval ?? 1,
                    end: end
                ))
            }

            let editor = EKEventEditViewController()
            editor.eventStore = self.store
            editor.event = event
            editor.editViewDelegate = self
            self.pendingInvoke = invoke
            viewController.present(editor, animated: true)
        }
    }

    func eventEditViewController(_ controller: EKEventEditViewController, didCompleteWith action: EKEventEditViewAction) {
        controller.dismiss(animated: true)
        pendingInvoke?.resolve(["saved": action == .saved])
        pendingInvoke = nil
    }
}

@_cdecl("init_plugin_add_to_calendar")
func initPlugin() -> Plugin {
    return AddToCalendarPlugin()
}
