// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Session management for Glean.
//!
//! Sessions provide first-class boundaries for user activity, enabling
//! session-level sampling, explicit start/end events, and per-event session
//! metadata for downstream analysis.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use chrono::{DateTime, FixedOffset, SecondsFormat};
use malloc_size_of_derive::MallocSizeOf;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::metrics::{QuantityMetric, StringMetric};
use crate::storage::INTERNAL_STORAGE;
use crate::{CommonMetricData, Glean, Lifetime};

// Storage key names for session persistence.
const SESSION_SEQ_METRIC_NAME: &str = "session#seq";
const SESSION_ID_METRIC_NAME: &str = "session#id";
const SESSION_INACTIVE_SINCE_METRIC_NAME: &str = "session#inactive_since";
const SESSION_START_TIME_METRIC_NAME: &str = "session#start_time";
const SESSION_EVENT_SEQ_METRIC_NAME: &str = "session#event_seq";

/// How sessions are managed by Glean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default, MallocSizeOf)]
pub enum SessionMode {
    /// Glean automatically manages session boundaries based on client activity.
    /// Sessions end after a configurable inactivity timeout.
    #[default]
    Auto,
    /// A new session starts on every client-active/inactive transition.
    Lifecycle,
    /// Sessions are managed manually by the application.
    ///
    /// `handle_client_active` and `handle_client_inactive` have no effect on
    /// session state.  The application must call `glean_session_start()` and
    /// `glean_session_end()` explicitly.
    ///
    /// Telemetry recorded before the first `glean_session_start()` call is
    /// treated as between-session telemetry: it is not suppressed by session
    /// sampling and carries no session metadata.
    Manual,
}

/// The state of the current session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// No active session.
    Inactive,
    /// A session is currently active.
    Active,
}

/// Session metadata attached to each in-session event.
///
/// Serialized into the event payload for downstream session-level analysis.
///
/// `PartialEq` is derived (using `f64::eq` for `session_sample_rate`).
/// `Eq` is implemented manually — it is sound because `session_sample_rate`
/// is always clamped to `[0.0, 1.0]` and is therefore never NaN.
/// Tests should prefer asserting on individual fields rather than whole-struct
/// equality for clarity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, MallocSizeOf)]
pub struct SessionMetadata {
    /// The unique UUID for this session.
    pub session_id: String,
    /// Monotonically increasing session counter, persisted across restarts.
    pub session_seq: u64,
    /// Per-session event counter, reset at each new session.
    pub event_seq: u64,
    /// The sampling rate in effect for this session.
    pub session_sample_rate: f64,
    /// Wall-clock timestamp at session start (RFC 3339).
    /// Absent on events from before sessions introduced this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub session_start_time: Option<String>,
}

// SAFETY: session_sample_rate is always clamped to [0.0, 1.0] and is never
// NaN, so the derived PartialEq (f64::eq) satisfies the Eq contract.
impl Eq for SessionMetadata {}

/// Describes a single event's relationship to the current session.
///
/// Computed once in `EventMetric::record_sync` and passed to
/// `EventDatabase::record`, collapsing the two-phase sampling gate and
/// metadata-attachment logic into a single value.
#[derive(Debug, Clone, Serialize, Deserialize, MallocSizeOf)]
pub enum EventSessionContext {
    /// The event is out-of-session (always recorded; no session metadata attached).
    ///
    /// Covers two cases:
    /// - The metric was declared `out_of_session = true`.
    /// - The metric is session-scoped but no session is currently active
    ///   (between sessions).
    OutOfSession,
    /// The event belongs to a sampled-in active session.
    ///
    /// The metadata is attached to the resulting `RecordedEvent` for
    /// downstream session-level analysis.
    InSession(SessionMetadata),
}

/// In-memory session state.
///
/// All persistence is handled by free functions in this module.
/// All mutation happens on the Glean dispatcher thread — no internal synchronization needed.
/// Fields are `pub(crate)` to prevent mutation from outside the crate while still
/// allowing `core/mod.rs` to drive the session lifecycle directly.
#[derive(Debug)]
pub struct SessionManager {
    /// How sessions are managed.
    pub(crate) mode: SessionMode,
    /// Current session state.
    pub(crate) state: SessionState,
    /// The current session's UUID, if active.
    pub(crate) session_id: Option<Uuid>,
    /// Monotonically increasing session counter (persisted).
    pub(crate) session_seq: u64,
    /// Per-session event counter.
    /// Uses AtomicU64 so `current_metadata_with_next_seq` can be called
    /// with only `&SessionManager` (via `&Glean`) from `record_sync`.
    pub(crate) event_seq: AtomicU64,
    /// The sample rate as originally provided at initialization (0.0–1.0).
    /// Never mutated after construction; used as the fallback when Remote
    /// Settings has no active override for the session sample rate.
    pub(crate) configured_sample_rate: f64,
    /// The effective sample rate for the *current* session, reflecting any
    /// Remote Settings override applied at session-start time.
    /// Written once per session in `session_start()`; read by metadata helpers.
    pub(crate) sample_rate: f64,
    /// Whether the current session is sampled in.
    pub(crate) sampled_in: bool,
    /// Wall-clock timestamp at session start. `None` between sessions.
    pub(crate) session_start_time: Option<DateTime<FixedOffset>>,
    /// When the session went inactive (for AUTO mode timeout evaluation).
    pub(crate) inactive_since: Option<DateTime<FixedOffset>>,
    /// How long inactivity before a new session is started (AUTO mode).
    /// `Duration::ZERO` means sessions never time out (always resumed).
    pub(crate) inactivity_timeout: Duration,
}

impl SessionManager {
    /// Creates a new `SessionManager`.
    ///
    /// `sample_rate` is clamped to `[0.0, 1.0]`; values outside that range are
    /// silently brought to the nearest bound.  This matches the behaviour of the
    /// remote-settings override path so the two paths are always consistent.
    pub fn new(mode: SessionMode, sample_rate: f64, inactivity_timeout: Duration) -> Self {
        let clamped = sample_rate.clamp(0.0, 1.0);
        Self {
            mode,
            state: SessionState::Inactive,
            session_id: None,
            session_seq: 0,
            event_seq: AtomicU64::new(0),
            configured_sample_rate: clamped,
            sample_rate: clamped,
            sampled_in: true, // true between sessions so recording proceeds normally
            session_start_time: None,
            inactive_since: None,
            inactivity_timeout,
        }
    }

    /// Returns whether the current session is sampled in.
    ///
    /// Returns `true` when no session is active (between sessions),
    /// so telemetry recorded outside of a session is never suppressed.
    pub fn is_sampled_in(&self) -> bool {
        match self.state {
            SessionState::Inactive => true,
            SessionState::Active => self.sampled_in,
        }
    }

    /// Returns whether a session is currently active.
    pub fn is_active(&self) -> bool {
        self.state == SessionState::Active
    }

    /// Returns the current session's UUID, if a session is active.
    pub fn session_id(&self) -> Option<Uuid> {
        self.session_id
    }

    /// Returns whether the current session is sampled in (direct field access).
    ///
    /// Differs from `is_sampled_in` in that it returns the raw field value
    /// without the "inactive → true" override, useful for asserting the exact
    /// sampling decision made at session start.
    pub fn sampled_in(&self) -> bool {
        self.sampled_in
    }

    /// Returns the wall-clock timestamp recorded when the current session started.
    pub fn session_start_time(&self) -> Option<DateTime<FixedOffset>> {
        self.session_start_time
    }

    /// Returns the current session's metadata without incrementing `event_seq`.
    pub fn current_metadata(&self) -> Option<SessionMetadata> {
        if self.state != SessionState::Active {
            return None;
        }
        let id = self.session_id?;
        Some(SessionMetadata {
            session_id: id.to_string(),
            session_seq: self.session_seq,
            event_seq: self.event_seq.load(Ordering::Relaxed),
            session_sample_rate: self.sample_rate,
            session_start_time: self
                .session_start_time
                .map(|t| t.to_rfc3339_opts(SecondsFormat::Millis, true)),
        })
    }

    /// Computes the session context (metadata attachment decision) for a single event.
    ///
    /// **Precondition:** the caller must have already verified via
    /// `MetricType::should_record()` that the event should be recorded.  That
    /// check handles sampling suppression for all metric types; this function
    /// is concerned only with *what context to attach*, not *whether to record*.
    ///
    /// Returns `OutOfSession` when no session is active (between sessions), or
    /// `InSession(meta)` when an active session is present.
    ///
    /// `event_seq` is incremented only for `InSession` results so that
    /// between-session events do not consume sequence numbers.
    pub fn compute_event_context(&self) -> EventSessionContext {
        match self.state {
            SessionState::Inactive => EventSessionContext::OutOfSession,
            SessionState::Active => {
                // should_record() has already ensured sampled_in is true.
                // current_metadata_with_next_seq increments event_seq atomically.
                match self.current_metadata_with_next_seq() {
                    Some(meta) => EventSessionContext::InSession(meta),
                    // Defensive fallback: session_id was None despite Active state.
                    None => EventSessionContext::OutOfSession,
                }
            }
        }
    }

    /// Returns the current session's metadata with an atomically incremented `event_seq`.
    ///
    /// Called from `EventMetric::record_sync` which only holds `&Glean`.
    pub fn current_metadata_with_next_seq(&self) -> Option<SessionMetadata> {
        if self.state != SessionState::Active {
            return None;
        }
        let id = self.session_id?;
        let seq = self.event_seq.fetch_add(1, Ordering::Relaxed);
        Some(SessionMetadata {
            session_id: id.to_string(),
            session_seq: self.session_seq,
            event_seq: seq,
            session_sample_rate: self.sample_rate,
            session_start_time: self
                .session_start_time
                .map(|t| t.to_rfc3339_opts(SecondsFormat::Millis, true)),
        })
    }
}

// ---------------------------------------------------------------------------
// Sampling
// ---------------------------------------------------------------------------

/// Converts a UUID to a deterministic sample value in [0, 1).
///
/// Interprets the first 8 bytes as a big-endian u64 and divides by 2^64.
/// A session is sampled in when `uuid_to_sample_value(uuid) < sample_rate`.
///
/// Dividing by 2^64 (not u64::MAX) guarantees the result is strictly less than
/// 1.0 for all inputs, so `sample_rate = 1.0` always samples every session.
pub(crate) fn uuid_to_sample_value(uuid: &Uuid) -> f64 {
    let bytes = uuid.as_bytes();
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[..8]);
    let n = u64::from_be_bytes(arr);
    (n as f64) / 2.0f64.powi(64)
}

// ---------------------------------------------------------------------------
// Persistence helpers
// ---------------------------------------------------------------------------

fn make_session_seq_metric() -> QuantityMetric {
    QuantityMetric::new(CommonMetricData {
        name: SESSION_SEQ_METRIC_NAME.into(),
        category: String::new(),
        send_in_pings: vec![INTERNAL_STORAGE.into()],
        lifetime: Lifetime::User,
        out_of_session: true,
        ..Default::default()
    })
}

fn make_session_id_metric() -> StringMetric {
    StringMetric::new(CommonMetricData {
        name: SESSION_ID_METRIC_NAME.into(),
        category: String::new(),
        send_in_pings: vec![INTERNAL_STORAGE.into()],
        lifetime: Lifetime::User,
        out_of_session: true,
        ..Default::default()
    })
}

/// Stores the inactive-since timestamp as an RFC 3339 string.
/// An empty string (or absence of the key) means no recorded inactive_since.
fn make_inactive_since_metric() -> StringMetric {
    StringMetric::new(CommonMetricData {
        name: SESSION_INACTIVE_SINCE_METRIC_NAME.into(),
        category: String::new(),
        send_in_pings: vec![INTERNAL_STORAGE.into()],
        lifetime: Lifetime::User,
        out_of_session: true,
        ..Default::default()
    })
}

/// Reads the current session sequence number from storage.
pub(crate) fn read_session_seq(glean: &Glean) -> u64 {
    make_session_seq_metric()
        .get_value(glean, INTERNAL_STORAGE)
        .filter(|&v| v >= 0)
        .map(|v| v as u64)
        .unwrap_or(0)
}

/// Persists the given session sequence number.
///
/// `QuantityMetric` stores `i64`; the cast from `u64` is lossless for any
/// value below `i64::MAX` (~9.2 × 10^18).  Values at or above that threshold
/// (unreachable in practice) would silently truncate, which is preferable to
/// a panic or corrupted sequence.
pub(crate) fn store_session_seq(glean: &Glean, seq: u64) {
    make_session_seq_metric().set_sync(glean, seq as i64);
}

/// Persists the current session ID.
/// Pass an empty string to indicate no active session.
pub(crate) fn persist_session_id(glean: &Glean, id: &str) {
    make_session_id_metric().set_sync(glean, id);
}

/// Clears the persisted session ID.
pub(crate) fn clear_session_id(glean: &Glean) {
    make_session_id_metric().set_sync(glean, "");
}

/// Reads the persisted session ID, if any.
/// Returns `None` if no session ID is stored or if it was cleared.
pub(crate) fn read_session_id(glean: &Glean) -> Option<String> {
    let id = make_session_id_metric().get_value(glean, INTERNAL_STORAGE)?;
    if id.is_empty() {
        None
    } else {
        Some(id)
    }
}

/// Persists the inactive-since timestamp as an RFC 3339 string.
pub(crate) fn persist_inactive_since(glean: &Glean, ts: DateTime<FixedOffset>) {
    make_inactive_since_metric().set_sync(
        glean,
        ts.to_rfc3339_opts(SecondsFormat::Millis, true).as_str(),
    );
}

/// Reads the persisted inactive-since timestamp, if any.
/// Returns `None` if the key is absent or the stored string is empty.
pub(crate) fn read_inactive_since(glean: &Glean) -> Option<DateTime<FixedOffset>> {
    let s = make_inactive_since_metric().get_value(glean, INTERNAL_STORAGE)?;
    if s.is_empty() {
        return None;
    }
    DateTime::parse_from_rfc3339(&s).ok()
}

/// Clears the inactive-since timestamp by writing an empty string.
pub(crate) fn clear_inactive_since(glean: &Glean) {
    make_inactive_since_metric().set_sync(glean, "");
}

// ---------------------------------------------------------------------------
// session_start_time persistence
// ---------------------------------------------------------------------------

fn make_session_start_time_metric() -> StringMetric {
    StringMetric::new(CommonMetricData {
        name: SESSION_START_TIME_METRIC_NAME.into(),
        category: String::new(),
        send_in_pings: vec![INTERNAL_STORAGE.into()],
        lifetime: Lifetime::User,
        out_of_session: true,
        ..Default::default()
    })
}

/// Persists the session start timestamp as an RFC 3339 string.
pub(crate) fn persist_session_start_time(glean: &Glean, ts: DateTime<FixedOffset>) {
    make_session_start_time_metric().set_sync(
        glean,
        ts.to_rfc3339_opts(SecondsFormat::Millis, true).as_str(),
    );
}

/// Reads the persisted session start timestamp, if any.
/// Returns `None` if the key is absent, empty, or unparseable.
pub(crate) fn read_session_start_time(glean: &Glean) -> Option<DateTime<FixedOffset>> {
    let s = make_session_start_time_metric().get_value(glean, INTERNAL_STORAGE)?;
    if s.is_empty() {
        return None;
    }
    DateTime::parse_from_rfc3339(&s).ok()
}

/// Clears the persisted session start timestamp.
pub(crate) fn clear_session_start_time(glean: &Glean) {
    make_session_start_time_metric().set_sync(glean, "");
}

// ---------------------------------------------------------------------------
// session_event_seq persistence
// ---------------------------------------------------------------------------

fn make_session_event_seq_metric() -> QuantityMetric {
    QuantityMetric::new(CommonMetricData {
        name: SESSION_EVENT_SEQ_METRIC_NAME.into(),
        category: String::new(),
        send_in_pings: vec![INTERNAL_STORAGE.into()],
        lifetime: Lifetime::User,
        out_of_session: true,
        ..Default::default()
    })
}

/// Reads the persisted per-session event sequence counter.
///
/// Returns `0` if no value has been stored (e.g. fresh session or after clear).
pub(crate) fn read_session_event_seq(glean: &Glean) -> u64 {
    make_session_event_seq_metric()
        .get_value(glean, INTERNAL_STORAGE)
        .filter(|&v| v >= 0)
        .map(|v| v as u64)
        .unwrap_or(0)
}

/// Persists the per-session event sequence counter.
///
/// Should be called whenever the in-memory `event_seq` changes and persistence
/// is required (i.e. on `session_transition_to_inactive`).  The cast from
/// `u64` is lossless for any value below `i64::MAX`.
pub(crate) fn store_session_event_seq(glean: &Glean, seq: u64) {
    make_session_event_seq_metric().set_sync(glean, seq as i64);
}

/// Clears the persisted event sequence counter (stores 0).
///
/// Called when a session ends so a resumed session from a stale storage entry
/// does not inherit a stale counter.
pub(crate) fn clear_session_event_seq(glean: &Glean) {
    make_session_event_seq_metric().set_sync(glean, 0);
}
