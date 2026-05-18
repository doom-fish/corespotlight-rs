//! Error types returned by the CoreSpotlight bridge.

use core::fmt;

use serde::{Deserialize, Serialize};

/// Error domain used by bridge-generated `CoreSpotlightError` values.
pub const CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN: &str = "CoreSpotlightBridge";

/// Error type used for failures returned by CoreSpotlight bridge calls.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreSpotlightError {
    /// Mirrors the `NSError.domain` value returned by CoreSpotlight.
    pub domain: String,
    /// Mirrors the `NSError.code` value returned by CoreSpotlight.
    pub code: i64,
    /// Human-readable failure message returned by CoreSpotlight or the bridge.
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorPayload {
    pub domain: String,
    pub code: i64,
    pub message: String,
}

impl CoreSpotlightError {
    pub(crate) fn from_payload(payload: ErrorPayload) -> Self {
        Self {
            domain: payload.domain,
            code: payload.code,
            message: payload.message,
        }
    }

    pub(crate) fn bridge(code: i64, message: impl Into<String>) -> Self {
        Self {
            domain: CORESPOTLIGHT_BRIDGE_ERROR_DOMAIN.into(),
            code,
            message: message.into(),
        }
    }
}

impl fmt::Display for CoreSpotlightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}) [{}]", self.message, self.code, self.domain)
    }
}

impl std::error::Error for CoreSpotlightError {}
