//! Pieces pertaining to the HTTP message protocol.
use http::{HeaderMap, Method, StatusCode, Uri, Version};

pub(crate) use self::h1::{dispatch, Conn, ServerTransaction};

pub(crate) mod h1;
pub(crate) mod h2;

/// An Incoming Message head. Includes request/status line, and headers.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MessageHead<S> {
    /// HTTP version of the message.
    pub version: Version,
    /// Subject (request line or status line) of Incoming message.
    pub subject: S,
    /// Headers of the Incoming message.
    pub headers: HeaderMap,
}

/// An incoming request message.
pub type RequestHead = MessageHead<RequestLine>;

#[derive(Debug, Default, PartialEq)]
pub struct RequestLine(pub Method, pub Uri);

/// An incoming response message.
pub type ResponseHead = MessageHead<StatusCode>;

#[derive(Debug)]
pub enum BodyLength {
    /// Content-Length
    Known(u64),
    /// Transfer-Encoding: chunked (if h1)
    Unknown,
}

/// Status of when an Disaptcher future completes.
pub(crate) enum Dispatched {
    /// Dispatcher completely shutdown connection.
    Shutdown,
    /// Dispatcher has pending upgrade, and so did not shutdown.
    Upgrade(::upgrade::Pending),
}

