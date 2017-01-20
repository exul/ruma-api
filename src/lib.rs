//! Crate ruma_api contains core types used to define the requests and responses for each endpoint
//! in the various [Matrix](https://matrix.org) API specifications.
//! These types can be shared by client and server code for all Matrix APIs.
//!
//! When implementing a new Matrix API, each endpoint have a type that implements `Endpoint`, plus
//! any necessary associated types.
//! An implementation of `Endpoint` contains all the information about the HTTP method, the path and
//! input parameters for requests, and the structure of a successful response.
//! Such types can then be used by client code to make requests, and by server code to fulfill
//! those requests.
//!
//! # Example
//!
//! ```rust,no_run
//! # extern crate ruma_api;
//! # extern crate ruma_identifiers;
//! # #[macro_use]
//! # extern crate serde_derive;
//! #
//! # fn main() {
//! /// PUT /_matrix/client/r0/directory/room/:room_alias
//! pub mod create {
//!     use ruma_api;
//!     use ruma_identifiers::{RoomAliasId, RoomId};
//!
//!     /// This API endpoint's body parameters.
//!     #[derive(Clone, Debug, Deserialize, Serialize)]
//!     pub struct RequestBody {
//!         pub room_id: RoomId,
//!     }
//!
//!     /// This API endpoint's path parameters.
//!     #[derive(Clone, Debug)]
//!     pub struct RequestPathParams {
//!         pub room_alias: RoomAliasId,
//!     }
//!
//!     #[derive(Clone, Debug)]
//!     pub enum ResponseStatus {
//!         AlreadyExists,
//!         Created,
//!     }
//!
//!     /// Details about this API endpoint.
//!     pub struct Endpoint;
//!
//!     impl From<ResponseStatus> for u16 {
//!         fn from(response_status: ResponseStatus) -> Self {
//!             match response_status {
//!                 AlreadyExists => 409,
//!                 Created => 200,
//!             }
//!         }
//!     }
//!
//!     impl<'a> ruma_api::Endpoint<'a> for Endpoint {
//!         type RequestBody = RequestBody;
//!         type RequestHeaders = ruma_api::Unused;
//!         type RequestPathParams = RequestPathParams;
//!         type RequestQueryParams = ruma_api::Unused;
//!         type ResponseBody = ruma_api::Unused;
//!         type ResponseHeaders = ruma_api::Unused;
//!         type ResponseStatus = ResponseStatus;
//!
//!         fn info() -> ruma_api::Info {
//!             ruma_api::Info {
//!                 description: "Matrix implementation of room directory.",
//!                 request_method: ruma_api::Method::Put,
//!                 name: "room_directory",
//!                 router_path: "/_matrix/client/r0/directory/room/:room_alias",
//!                 requires_authentication: true,
//!                 rate_limited: false,
//!             }
//!         }
//!
//!         fn request_path(params: Self::RequestPathParams) -> String {
//!             format!("/_matrix/client/r0/directory/room/{}", params.room_alias)
//!         }
//!     }
//! }
//! # }

#![feature(never_type, try_from)]
#![deny(missing_docs)]

extern crate serde;

use std::convert::TryFrom;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
pub trait Endpoint<'a> {
    /// Request parameters supplied via the body of the HTTP request.
    type RequestBody: Deserialize + Serialize;

    /// Request parameters supplied via HTTP header.
    type RequestHeaders: Into<Vec<u8>>;

    /// Request parameters supplied via the URL's path.
    type RequestPathParams;

    /// Request parameters supplied via the URL's query string.
    type RequestQueryParams: Into<Vec<(&'a str, &'a str)>>;

    /// The body of the HTTP response.
    type ResponseBody: Deserialize + Serialize;

    /// Possible HTTP response headers.
    type ResponseHeaders: TryFrom<&'a [u8]>;

    /// Possible HTTP response codes.
    type ResponseStatus: Into<u16>;

    /// Information about the endpoint.
    fn info() -> Info;

    /// Generates the path component of the URL for this endpoint using the supplied parameters.
    fn request_path(params: Self::RequestPathParams) -> String;
}

/// Information about an `Endpoint`.
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

/// Used to indicate that an associated type is not applicable for a particular `Endpoint`.
pub struct Unused;

impl<'a> TryFrom<&'a [u8]> for Unused {
    type Err = !;

    fn try_from(_slice: &[u8]) -> Result<Self, Self::Err> {
        Ok(Unused)
    }
}

impl<'a> From<Unused> for Vec<(&'a str, &'a str)> {
    fn from(_unused: Unused) -> Self {
        Vec::new()
    }
}

impl From<Unused> for Vec<u8> {
    fn from(_unused: Unused) -> Self {
        Vec::new()
    }
}

impl Deserialize for Unused {
    fn deserialize<D>(deserializer: &mut D) -> Result<Unused, D::Error> where D: Deserializer {
        struct Visitor;
        impl ::serde::de::Visitor for Visitor {
            type Value = Unused;
            #[inline]
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Unused, V::Error>
                where V: ::serde::de::SeqVisitor
            {
                visitor.end()?;
                Ok(Unused{})
            }
            #[inline]
            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Unused, V::Error>
                where V: ::serde::de::MapVisitor
            {
                visitor.end()?;
                Ok(Unused{})
            }
        }
        const FIELDS: &'static [&'static str] = &[];
        deserializer.deserialize_struct("Unused", FIELDS, Visitor)
    }
}

impl Serialize for Unused {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        let state = serializer.serialize_struct("Unused", 0)?;
        serializer.serialize_struct_end(state)
    }
}
