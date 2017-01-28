//! Crate ruma_api contains core types used to define the requests and responses for each endpoint
//! in the various [Matrix](https://matrix.org) API specifications.
//! These types can be shared by client and server code for all Matrix APIs.
//!  When implementing a new Matrix API, each endpoint should have a type that implements `Endpoint`,
//! plus the associated `Request` and `Response` types.
//!
//! # Example
//!
//! ```rust,no_run
//! # #![feature(try_from)]
//! # extern crate ruma_api;
//! # extern crate ruma_identifiers;
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde_json;
//! #
//! # fn main() {
//! /// PUT /_matrix/client/r0/directory/room/:room_alias
//! pub mod create {
//!     use std::convert::TryFrom;
//!
//!     use ruma_api::{self, Info, Method};
//!     use ruma_identifiers::{RoomAliasId, RoomId};
//!     use serde_json::to_vec;
//!
//!     /// Endpoint for adding an alias to a room.
//!     pub struct Endpoint;
//!
//!     /// Input parameters for a request to this endpoint.
//!     pub struct Request {
//!         pub room_alias: RoomAliasId,
//!         pub room_id: RoomId,
//!     }
//!
//!     #[derive(Serialize)]
//!     struct RequestBody {
//!         pub room_id: RoomId,
//!     }
//!
//!     /// The response from this endpoint.
//!     pub struct Response;
//!
//!     impl ruma_api::Endpoint for Endpoint {
//!         type Request = Request;
//!         type Response = Response;
//!
//!         fn info() -> &'static Info {
//!             &Info {
//!                 description: "Add an alias to a room.",
//!                 name: "create_alias",
//!                 rate_limited: false,
//!                 request_method: Method::Put,
//!                 requires_authentication: true,
//!                 router_path: "/_matrix/client/r0/directory/room/:room_alias",
//!             }
//!         }
//!     }
//!
//!     impl Into<ruma_api::Request> for Request {
//!         fn into(self) -> ruma_api::Request {
//!             let request_body = RequestBody {
//!                 room_id: self.room_id,
//!             };
//!
//!             ruma_api::Request {
//!                 body: to_vec(&request_body).expect("request body should serialize"),
//!                 headers: Vec::new(),
//!                 method: Endpoint::info().request_method,
//!                 path: format!("/_matrix/client/r0/directory/room/{}", self.room_alias),
//!                 query: Vec::new(),
//!             }
//!         }
//!     }
//!
//!     impl TryFrom<ruma_api::Request> for Request {
//!         // TODO
//!     }
//!
//!     impl Into<ruma_api::Response> for Response {
//!         Response
//!     }
//!
//!     impl TryFrom<ruma_api::Response> for Response {
//!         // TODO
//!     }
//! }
//! # }

#![feature(try_from)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

use std::convert::TryFrom;

/// Information about an `Endpoint`.
#[derive(Clone, Copy, Debug)]
pub struct Info {
    /// A human-readable description of the endpoint.
    pub description: &'static str,
    /// A unique identifier for this endpoint.
    pub name: &'static str,
    /// Whether or not this endpoint is rate limited by the server.
    pub rate_limited: bool,
    /// The HTTP method used by this endpoint.
    pub request_method: Method,
    /// Whether or not the server requires an authenticated user for this endpoint.
    pub requires_authentication: bool,
    /// The path of this endpoint's URL, with variable names where path parameters should be filled
    /// in during a request.
    ///
    /// This value is suitable for creating routes with `Router` from the router crate.
    pub router_path: &'static str,
}

/// HTTP request methods used in Matrix APIs.
#[derive(Clone, Copy, Debug)]
pub enum Method {
    /// DELETE
    Delete,
    /// GET
    Get,
    /// POST
    Post,
    /// PUT
    Put,
}

/// An API endpoint.
pub trait Endpoint {
    /// Request data from the client.
    type Request: Into<Request> + TryFrom<Request>;

    /// Response data from the server.
    type Response: Into<Response> + TryFrom<Response>;

    /// General information about the endpoint.
    fn info() -> &'static Info;
}

/// An HTTP request.
///
/// This structure is intentionally abstract so as not to bind `ruma-api` to any particular HTTP
/// library.
/// A library implementing `Endpoint`s must provide conversions between their own request types and
/// `Request`.
/// Programs consuming such a Matrix API library should then provide conversions between their HTTP
/// library of choice and `Request`.
#[derive(Clone, Debug)]
pub struct Request {
    /// The request body.
    pub body: Vec<u8>,
    /// The HTTP request headers.
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    /// The HTTP request method.
    pub method: Method,
    /// The path component of the request's URL.
    pub path: String,
    /// The query string component of the request's URL.
    pub query: Vec<(String, String)>
}

/// An HTTP response.
///
/// This structure is intentionally abstract so as not to bind `ruma-api` to any particular HTTP
/// library.
/// A library implementing `Endpoint`s must provide conversions between their own response types and
/// `Request`.
/// Programs consuming such a Matrix API library should then provide conversions between their HTTP
/// library of choice and `Response`.
#[derive(Clone, Debug)]
pub struct Response {
    /// The request body.
    pub body: Vec<u8>,
    /// The HTTP request headers.
    pub headers: Vec<(Vec<u8>, Vec<u8>)>,
    /// The HTTP status code.
    pub status: u16,
}
