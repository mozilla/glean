/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// MetricsPingScheduler facilitates scheduling the periodic assembling of metrics pings,
/// at a given time, trying its best to handle the following cases:
///
/// - ping is overdue (due time already passed) for the current calendar day;
/// - ping is soon to be sent in the current calendar day;
/// - ping was already sent, and must be scheduled for the next calendar day.
class MetricsPingScheduler {
    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/MetricsPingSched"
        static let lastMetricsPingSentDateTime = "last_metrics_ping_iso_datetime"
        static let dueHourOfTheDay = 4
        static let lastVersionOfAppUsed = "last_version_of_app_used"
    }

    private let logger = Logger(tag: Constants.logTag)

    var timer: Timer?

    init() {
        // In testing mode, set the "last seen version" as the same as this one.
        // Otherwise, all we will ever send is pings for the "upgrade" reason.
        if Dispatchers.shared.testingMode {
            _ = isDifferentVersion()
        }
    }

    /// Schedules the metrics ping collection at the due time.
    ///
    /// - parameters:
    ///     * now:  A `Date` representing the current date/time
    ///     * sendTheNextCalendarDay:  Determines whether to schedule collection for the next calendar day
    ///                            or to attempt to schedule it for the current calendar day. If the latter and
    ///                            we're overdue for the expected collection time, the task is scheduled for
    ///                            immediate execution.
    ///     * reason: The reason the ping is being submitted.
    func schedulePingCollection(
        _ now: Date,
        sendTheNextCalendarDay: Bool,
        reason: GleanMetrics.Pings.MetricsReasonCodes
    ) {
        var fireDate = Calendar.current.date(
            bySettingHour: Constants.dueHourOfTheDay,
            minute: 0,
            second: 0,
            of: now
        )!

        // Invalidiate the timer if it's already running
        timer?.invalidate()

        // Setup timer and schedule
        if sendTheNextCalendarDay {
            let tomorrow = Calendar.current.date(
                byAdding: .day,
                value: 1,
                to: fireDate,
                wrappingComponents: false
            )!
            fireDate = tomorrow
        }

        logger.debug(
            "Scheduling the 'metrics' ping for \(fireDate), in \(fireDate - now) seconds."
        )

        // Set the timer to fire at the `fireDate`
        timer = Timer.scheduledTimer(withTimeInterval: fireDate - now, repeats: false) { _ in
            self.logger.debug("MetricsPingScheduler timer fired!")
            // When the timer fires, call `collectPingAndReschedule` with the current
            // date/time.
            self.collectPingAndReschedule(Date(), startupPing: false, reason: reason)
        }
    }

    /// Determines if the application is a differnet version from the last time it was run. This is used to prevent
    /// mixing data from multiple versions of the application in the same ping.
    ///
    /// - returns: `true` if the version is different, `false` if the version is the same.
    func isDifferentVersion() -> Bool {
        // Determine if the version has changed since the last time we ran.
        let currentVersion = AppInfo.displayVersion
        let lastVersion = UserDefaults.standard.string(forKey: Constants.lastVersionOfAppUsed)
        if currentVersion != lastVersion {
            UserDefaults.standard.set(currentVersion, forKey: Constants.lastVersionOfAppUsed)
            return true
        }

        return false
    }

    /// Check if the provided time is after the ping due time.
    ///
    /// - parameters:
    ///     * now: A `Date` representing the current time
    ///     * dueHourOfTheDay: An `Int` representing the due hour of the day, in the [0...23] range
    ///
    /// - returns: `true` if `now` is past the due hour of the day.
    func isAfterDueTime(_ now: Date, dueHourOfTheDay: Int = Constants.dueHourOfTheDay) -> Bool {
        return now > Calendar.current.date(
            bySettingHour: dueHourOfTheDay,
            minute: 0,
            second: 0,
            of: now
        )!
    }

    /// Performs startup checks to decide when to schedule the next metrics ping collection.
    func schedule() {
        let now = Date()

        // If the version of the app is different from the last time we ran the app,
        // schedule the metrics ping for immediate collection. We only need to perform
        // this check at startup (when overduePingAsFirst is true).
        if isDifferentVersion() {
            self.collectPingAndReschedule(
                now,
                startupPing: true,
                reason: .upgrade
            )
            return
        }

        let lastSentDate = getLastCollectedDate()
        logger.debug("The 'metrics' ping was last sent on \(String(describing: lastSentDate))")

        // We expect to cover 3 cases here:
        //
        // (1) - the ping was already collected on the current calendar day; only schedule
        //       one for collecting the next calendar day at the due time;
        // (2) - the ping was NOT collected on the current calendar day, and we're later than
        //       the due time; collect the ping immediately;
        // (3) - the ping was NOT collected on the current calendar day, but we still have
        //       some time to the due time; schedule for sending the current calendar day.

        let alreadySentToday = lastSentDate != nil && Calendar.current.isDateInToday(lastSentDate!)
        if alreadySentToday {
            // The metrics ping was already sent today. Schedule it for the next
            // calendar day. This addresses (1).
            logger.info("The 'metrics' ping was already sent today, \(now).")
            schedulePingCollection(
                now,
                sendTheNextCalendarDay: true,
                reason: .tomorrow
            )
        } else if isAfterDueTime(now) {
            logger.info("The 'metrics' ping is scheduled for immediate collection, \(now)")
            // **IMPORTANT**
            //
            // We want to make sure no other metric API adds data before the ping is collected.
            // `schedule` should be called off of the main thread.  As long as `schedule` is
            // invoked before replaying any queued recording API call, it guarantees this step
            // is completed before anything else is recorded.
            self.collectPingAndReschedule(
                now,
                startupPing: true,
                reason: .overdue
            )
        } else {
            // This covers (3).
            logger.info("The 'metrics' collection is scheduled for today, \(now)")
            schedulePingCollection(
                now,
                sendTheNextCalendarDay: false,
                reason: .today
            )
        }
    }

    /// Triggers the collection of the "metrics" ping and schedules the next collection.
    ///
    /// - parameters:
    ///     * now: A `Date` representing the current time
    ///     * reason: The reason the ping is being submitted.
    func collectPingAndReschedule(
        _ now: Date,
        startupPing: Bool = false,
        reason: GleanMetrics.Pings.MetricsReasonCodes
    ) {
        let reasonString = GleanMetrics.Pings.shared.metrics.reasonCodes[reason.rawValue]
        logger.info("Collecting the 'metrics' ping, now = \(now), startup = \(startupPing), reason = \(reasonString)")
        if startupPing {
            // **IMPORTANT**
            //
            // If this is a `startupPing` we require any metric recording to be
            // batched up and replayed after the startup metrics ping is sent. To guarantee
            // that we synchronously and manually dispatch the 'metrics' ping without
            // going through our public API.
            //
            // * Do not change this line without checking what it implies for the above wall
            // of text. *
            Glean.shared.submitPingByNameSync(pingName: "metrics", reason: reasonString)
        } else {
            GleanMetrics.Pings.shared.metrics.submit(reason: reason)
        }
        // Update the collection date: we don't really care if we have data or not, let's
        // always update the sent date.
        updateSentDate(now)
        // Reschedule the collection.
        schedulePingCollection(
            now,
            sendTheNextCalendarDay: true,
            reason: .reschedule
        )
    }

    /// Get the date the metrics ping was last collected.
    ///
    /// - returns: A `Date` representing the when the metrics ping was last collected, or nil if no metrics
    ///            ping was previously collected.
    func getLastCollectedDate() -> Date? {
        var lastCollectedDate: Date?

        if let loadedDate = UserDefaults.standard.string(forKey: Constants.lastMetricsPingSentDateTime) {
            lastCollectedDate = Date.fromISO8601String(dateString: loadedDate, precision: .millisecond)
        } else {
            logger.error("MetricsPingScheduler last stored ping time was not valid")
        }

        return lastCollectedDate
    }

    /// Update the persisted date when the metrics ping is sent.
    ///
    /// - parameters:
    ///     * date: The `Date` to store.
    func updateSentDate(_ date: Date = Date()) {
        UserDefaults.standard.set(
            date.toISO8601String(precision: .millisecond),
            forKey: Constants.lastMetricsPingSentDateTime
        )
    }
}
