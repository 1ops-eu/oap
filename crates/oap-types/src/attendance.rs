//! Attendance — captures what actually happened at a session.
//!
//! A confirmed booking does NOT imply attendance.
//! Attendance is a separate, explicit record. (v0.2 core, type defined early)

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;

use crate::common::Metadata;
use crate::ids::{ActorId, AttendanceId, BookingId, SessionId};

/// Attendance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttendanceStatus {
    Attended,
    Late,
    NoShow,
    ExcusedAbsence,
}

impl std::fmt::Display for AttendanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attended => write!(f, "attended"),
            Self::Late => write!(f, "late"),
            Self::NoShow => write!(f, "no_show"),
            Self::ExcusedAbsence => write!(f, "excused_absence"),
        }
    }
}

/// An Attendance record in the OAP protocol.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Attendance {
    pub attendance_id: AttendanceId,
    pub booking_id: BookingId,
    pub session_id: SessionId,
    pub participant_actor_id: ActorId,
    pub status: AttendanceStatus,

    /// Who recorded this attendance.
    pub recorded_by_actor_id: ActorId,

    #[serde(with = "time::serde::rfc3339")]
    pub recorded_at: OffsetDateTime,

    #[serde(default, skip_serializing_if = "Metadata::is_empty")]
    pub metadata: Metadata,
}

/// Request body for recording Attendance.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RecordAttendanceRequest {
    pub booking_id: BookingId,
    pub session_id: SessionId,
    pub participant_actor_id: ActorId,
    pub status: AttendanceStatus,
    pub recorded_by_actor_id: ActorId,

    #[serde(default)]
    pub metadata: Metadata,
}
