//! Errors returned by gateway operations.

#[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
pub use crate::inflater::{CompressionError, CompressionErrorType};

use std::{
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};
use twilight_model::gateway::CloseCode;

/// Received gateway message couldn't be processed.
#[derive(Debug)]
pub struct ProcessError {
    /// Type of error.
    pub(crate) kind: ProcessErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ProcessError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ProcessErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ProcessErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }

    /// Shortcut to create a new error from a message sending error.
    pub(crate) fn from_send(source: SendError) -> Self {
        Self {
            kind: ProcessErrorType::SendingMessage,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ProcessErrorType::Deserializing { event } => {
                f.write_str("gateway event could not be deserialized: source=")?;
                if let Some(source) = &self.source {
                    Display::fmt(source, f)?;
                } else {
                    f.write_str("None")?;
                }
                f.write_str(", event=")?;
                f.write_str(event)
            }
            ProcessErrorType::SendingMessage => {
                f.write_str("failed to send a message over the websocket")
            }
        }
    }
}

impl Error for ProcessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ProcessError`] that occurred.
#[derive(Debug)]
pub enum ProcessErrorType {
    /// Gateway event could not be deserialized.
    Deserializing {
        /// Gateway event.
        ///
        /// Unlike [`ReceiveMessageErrorType::Deserializing`], the event is
        /// never modified.
        event: String,
    },
    /// Message could not be sent over the Websocket connection.
    ///
    /// This may happen when the shard sends heartbeats or attempts to identify
    /// a new gateway session.
    SendingMessage,
}

/// Receiving the next Websocket message failed.
#[derive(Debug)]
pub struct ReceiveMessageError {
    /// Type of error.
    pub(crate) kind: ReceiveMessageErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl ReceiveMessageError {
    /// Whether the error is fatal.
    ///
    /// If the error is fatal then further attempts to use the shard will return
    /// more fatal errors.
    pub const fn is_fatal(&self) -> bool {
        matches!(self.kind(), ReceiveMessageErrorType::FatallyClosed { .. })
    }

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &ReceiveMessageErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ReceiveMessageErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Shortcut to create a new error for a message compression error.
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    pub(crate) fn from_compression(source: CompressionError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::Compression,
            source: Some(Box::new(source)),
        }
    }

    /// Shortcut to create a new error from a fatal close code.
    pub(crate) fn from_fatally_closed(close_code: CloseCode) -> Self {
        Self {
            kind: ReceiveMessageErrorType::FatallyClosed { close_code },
            source: None,
        }
    }

    /// Shortcut to create a new error from a message sending error.
    pub(crate) fn from_send(source: SendError) -> Self {
        Self {
            kind: ReceiveMessageErrorType::SendingMessage,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for ReceiveMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
            ReceiveMessageErrorType::Compression => {
                f.write_str("binary message could not be decompressed")
            }
            ReceiveMessageErrorType::Deserializing { ref event } => {
                f.write_str("gateway event could not be deserialized: source=")?;
                if let Some(source) = &self.source {
                    Display::fmt(source, f)?;
                } else {
                    f.write_str("None")?;
                }
                f.write_str(", event=")?;
                f.write_str(event)
            }
            ReceiveMessageErrorType::FatallyClosed { close_code } => {
                f.write_str("shard fatally closed: ")?;

                Display::fmt(&close_code, f)
            }
            ReceiveMessageErrorType::Io => f.write_str("websocket connection error"),
            ReceiveMessageErrorType::Process => {
                f.write_str("failed to internally process the received message")
            }
            ReceiveMessageErrorType::Reconnect => f.write_str("failed to reconnect to the gateway"),
            ReceiveMessageErrorType::SendingMessage => {
                f.write_str("failed to send a message over the websocket")
            }
        }
    }
}

impl Error for ReceiveMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ReceiveMessageError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ReceiveMessageErrorType {
    /// Binary message could not be decompressed.
    ///
    /// The associated error downcasts to [`CompressionError`].
    #[cfg(any(feature = "zlib-stock", feature = "zlib-simd"))]
    Compression,
    /// Gateway event could not be deserialized.
    Deserializing {
        /// Gateway event.
        ///
        /// Note that the `simd-json` feature will slightly modify the event.
        event: String,
    },
    /// Shard has been closed due to a fatal configuration error.
    FatallyClosed {
        /// Close code of the close message.
        close_code: CloseCode,
    },
    /// WebSocket connection error.
    Io,
    /// Processing the message failed.
    ///
    /// The associated error downcasts to [`ProcessError`].
    Process,
    /// Shard failed to reconnect to the gateway.
    Reconnect,
    /// Message could not be sent over the Websocket connection.
    ///
    /// This may happen when the shard sends heartbeats or attempts to identify
    /// a new gateway session.
    SendingMessage,
}

/// Sending a command failed.
#[derive(Debug)]
pub struct SendError {
    /// Type of error.
    pub(crate) kind: SendErrorType,
    /// Source error if available.
    pub(crate) source: Option<Box<dyn Error + Send + Sync>>,
}

impl SendError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SendErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (SendErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for SendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            SendErrorType::Sending => f.write_str("sending the message over the websocket failed"),
            SendErrorType::Serializing => f.write_str("serializing the value as json failed"),
        }
    }
}

impl Error for SendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`SendError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum SendErrorType {
    /// Sending the payload over the WebSocket failed. This is indicative of a
    /// shutdown shard.
    Sending,
    /// Serializing the payload as JSON failed.
    Serializing,
}

#[cfg(test)]
mod tests {
    use super::{
        ProcessError, ProcessErrorType, ReceiveMessageError, ReceiveMessageErrorType, SendError,
        SendErrorType,
    };
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};
    use twilight_model::gateway::CloseCode;

    assert_fields!(ReceiveMessageErrorType::FatallyClosed: close_code);
    assert_impl_all!(ProcessErrorType: Debug, Send, Sync);
    assert_impl_all!(ProcessError: Error, Send, Sync);
    assert_impl_all!(ReceiveMessageErrorType: Debug, Send, Sync);
    assert_impl_all!(ReceiveMessageError: Error, Send, Sync);
    assert_impl_all!(SendErrorType: Debug, Send, Sync);
    assert_impl_all!(SendError: Error, Send, Sync);

    #[test]
    fn process_error_display() {
        let messages: [(ProcessErrorType, &str); 2] = [
            (
                ProcessErrorType::Deserializing {
                    event: r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-b-0568\",{\"micros\":0.0}]"]}}"#
                        .to_owned(),
                },
                r#"gateway event could not be deserialized: event={"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-b-0568\",{\"micros\":0.0}]"]}}"#,
            ),
            (
                ProcessErrorType::SendingMessage,
                "failed to send a message over the websocket",
            ),
        ];

        for (kind, message) in messages {
            let error = ProcessError { kind, source: None };

            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn receive_message_error_display() {
        let messages: [(ReceiveMessageErrorType, &str); 7] = [
            (
                ReceiveMessageErrorType::Compression,
                "binary message could not be decompressed",
            ),
            (
                ReceiveMessageErrorType::Deserializing {
                    event: r#"{"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-b-0568\",{\"micros\":0.0}]"]}}"#.to_owned(),
                },
                r#"gateway event could not be deserialized: event={"t":null,"s":null,"op":10,"d":{"heartbeat_interval":41250,"_trace":["[\"gateway-prd-us-east1-b-0568\",{\"micros\":0.0}]"]}}"#,
            ),
            (
                ReceiveMessageErrorType::FatallyClosed {
                    close_code: CloseCode::InvalidIntents,
                },
                "shard fatally closed: Invalid Intents",
            ),
            (ReceiveMessageErrorType::Io, "websocket connection error"),
            (
                ReceiveMessageErrorType::Process,
                "failed to internally process the received message",
            ),
            (
                ReceiveMessageErrorType::Reconnect,
                "failed to reconnect to the gateway",
            ),
            (
                ReceiveMessageErrorType::SendingMessage,
                "failed to send a message over the websocket",
            ),
        ];

        for (kind, message) in messages {
            let error = ReceiveMessageError { kind, source: None };

            assert_eq!(error.to_string(), message);
        }
    }

    #[test]
    fn receive_message_error_is_fatal() {
        let fatal = ReceiveMessageError {
            kind: ReceiveMessageErrorType::FatallyClosed {
                close_code: CloseCode::AuthenticationFailed,
            },
            source: None,
        };
        assert!(fatal.is_fatal());
    }

    #[test]
    fn send_error_display() {
        assert_eq!(
            SendError {
                kind: SendErrorType::Sending,
                source: None,
            }
            .to_string(),
            "sending the message over the websocket failed"
        );
        assert_eq!(
            SendError {
                kind: SendErrorType::Serializing,
                source: None,
            }
            .to_string(),
            "serializing the value as json failed"
        );
    }
}
