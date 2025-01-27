use crate::error::Code;

use super::codes::NewCode;

/// This enum determines if the error affects the connection or the stream
///
/// The end goal is to make all stream errors from the spec potential connection errors if users of h3 decide to treat them as such
#[derive(Debug, Clone, Hash)]
pub enum ErrorScope {
    /// The error affects the connection
    Connection,
    /// The error affects the stream
    Stream,
}

/// This error type represents an internal error type, which is used
/// to represent errors, which have not yet affected the connection or stream state
///
/// This error type is generated from the error types of h3s submodules or by the modules itself.
///
/// This error type is used in functions which handle a http3 request stream
#[derive(Debug, Clone, Hash)]
pub struct InternalRequestStreamError {
    /// The error scope
    pub(crate) scope: ErrorScope,
    /// The error code
    pub(crate) code: NewCode,
    /// The error message
    pub(crate) message: &'static str,
}

/// This error type represents an internal error type, which is used
/// to represent errors, which have not yet affected the connection state
///
/// This error type is generated from the error types of h3s submodules or by the modules itself.
///
/// This error type is used in functions which handle a http3 connection state
#[derive(Debug, Clone, Hash)]
pub struct InternalConnectionError {
    /// The error code
    pub(super) code: NewCode,
    /// The error message
    pub(super) message: &'static str,
}


impl InternalConnectionError {
    /// Create a new internal connection error
    pub fn new(code: NewCode, message: &'static str) -> Self {
        Self { code, message }
    }
}

impl InternalRequestStreamError {
    /// Create a new internal request stream error
    pub fn new(scope: ErrorScope, code: NewCode, message: &'static str) -> Self {
        Self { scope, code, message }
    }
}