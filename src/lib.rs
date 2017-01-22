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
//!     pub struct Body {
//!         pub room_id: RoomId,
//!     }
//!
//!     /// This API endpoint's path parameters.
//!     #[derive(Clone, Debug)]
//!     pub struct PathParams {
//!         pub room_alias: RoomAliasId,
//!     }
//!
//!     pub type Request = ruma_api::request::BodyAndPathParams<Body, PathParams>;
//!
//!     /// Details about this API endpoint.
//!     pub struct Endpoint;
//!
//!     impl ruma_api::PathParams for PathParams {
//!         fn request_path(&self) -> String {
//!             format!("/_matrix/client/r0/directory/room/{}", self.room_alias)
//!         }
//!     }
//!
//!     impl ruma_api::Endpoint for Endpoint {
//!         type Request = Request;
//!         type Response = ruma_api::response::StatusOnly;
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
//!     }
//! }
//! # }

#![deny(missing_docs)]

extern crate serde;

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
    /// The endpoint's request.
    type Request: Request;
    /// The endpoint's response.
    type Response: Response;

    /// Information about the endpoint.
    fn info() -> Info;
}

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

/// An endpoint's request.
pub trait Request {}

/// An endpoint's response.
pub trait Response {}

/// Parameters that are included within the path component of the URL for an endpoint.
pub trait PathParams {
    /// Generates the path component of the URL for this endpoint using the supplied parameters.
    fn request_path(&self) -> String;
}

/// Implementations of the `Request` trait covering various combinations of required components
/// among request body, headers, path parameters, and query parameters.
pub mod request {
    use serde::{Deserialize, Serialize};

    use super::{PathParams, Request};

    /// A request with a body, headers, path parameters, and query parameters.
    #[derive(Debug)]
    pub struct All<B, H, P, Q>
    where B: Deserialize + Serialize, P: PathParams, Q: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
        /// The request's headers.
        pub headers: H,
        /// The request's path parameters.
        pub path_params: P,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with only a body.
    #[derive(Debug)]
    pub struct BodyOnly<B> where B: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
    }

    /// A request with only headers.
    #[derive(Debug)]
    pub struct HeadersOnly<H> {
        /// The request's headers.
        pub headers: H,
    }

    /// A request with only path parameters.
    #[derive(Debug)]
    pub struct PathParamsOnly<P> where P: PathParams {
        /// The request's path parameters.
        pub path_params: P,
    }

    /// A request with only query parameters.
    #[derive(Debug)]
    pub struct QueryParamsOnly<Q> where Q: Deserialize + Serialize {
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with no body.
    #[derive(Debug)]
    pub struct NoBody<H, P, Q> where P: PathParams, Q: Deserialize + Serialize {
        /// The request's headers.
        pub headers: H,
        /// The request's path parameters.
        pub path_params: P,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with no headers.
    #[derive(Debug)]
    pub struct NoHeaders<B, P, Q>
    where B: Deserialize + Serialize, P: PathParams, Q: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
        /// The request's path parameters.
        pub path_params: P,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with no path parameters.
    #[derive(Debug)]
    pub struct NoPathParams<B, H, Q> where B: Deserialize + Serialize, Q: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
        /// The request's headers.
        pub headers: H,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with no query parameters.
    #[derive(Debug)]
    pub struct NoQueryParams<B, H, P> where B: Deserialize + Serialize, P: PathParams {
        /// The request's body.
        pub body: B,
        /// The request's headers.
        pub headers: H,
        /// The request's path parameters.
        pub path_params: P,
    }

    /// A request with only a body and headers.
    #[derive(Debug)]
    pub struct BodyAndHeaders<B, H> where B: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
        /// The request's headers.
        pub headers: H,
    }

    /// A request with only a body and path parameters.
    #[derive(Debug)]
    pub struct BodyAndPathParams<B, P> where B: Deserialize + Serialize, P: PathParams {
        /// The request's body.
        pub body: B,
        /// The request's path parameters.
        pub path_params: P,
    }

    /// A request with only a body and query parameters.
    #[derive(Debug)]
    pub struct BodyAndQueryParams<B, Q>
    where B: Deserialize + Serialize, Q: Deserialize + Serialize {
        /// The request's body.
        pub body: B,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with only headers and path parameters.
    #[derive(Debug)]
    pub struct HeadersAndPathParams<H, P> where P: PathParams {
        /// The request's headers.
        pub headers: H,
        /// The request's path parameters.
        pub path_params: P,
    }

    /// A request with only headers and query parameters.
    #[derive(Debug)]
    pub struct HeadersAndQueryParams<H, Q> where Q: Deserialize + Serialize {
        /// The request's headers.
        pub headers: H,
        /// The request's query parameters.
        pub query_params: Q,
    }

    /// A request with only path parameters and query parameters.
    #[derive(Debug)]
    pub struct PathParamsAndQueryParams<P, Q> where P: PathParams, Q: Deserialize + Serialize {
        /// The request's path parameters.
        pub path_params: P,
        /// The request's query parameters.
        pub query_params: Q,
    }

    impl<B, H, P, Q> Request for All<B, H, P, Q>
    where B: Deserialize + Serialize, P: PathParams, Q: Deserialize + Serialize {}
    impl<B> Request for BodyOnly<B> where B: Deserialize + Serialize {}
    impl<H> Request for HeadersOnly<H> {}
    impl<P> Request for PathParamsOnly<P> where P: PathParams {}
    impl<Q> Request for QueryParamsOnly<Q> where Q: Deserialize + Serialize {}
    impl<H, P, Q> Request for NoBody<H, P, Q> where P: PathParams, Q: Deserialize + Serialize {}
    impl<B, P, Q> Request for NoHeaders<B, P, Q>
    where B: Deserialize + Serialize, P: PathParams, Q: Deserialize + Serialize {}
    impl<B, H, Q> Request for NoPathParams<B, H, Q>
    where B: Deserialize + Serialize, Q: Deserialize + Serialize {}
    impl<B, H, P> Request for NoQueryParams<B, H, P>
    where B: Deserialize + Serialize, P: PathParams {}
    impl<B, H> Request for BodyAndHeaders<B, H> where B: Deserialize + Serialize {}
    impl<B, P> Request for BodyAndPathParams<B, P>
    where B: Deserialize + Serialize, P: PathParams {}
    impl<B, Q> Request for BodyAndQueryParams<B, Q>
    where B: Deserialize + Serialize, Q: Deserialize + Serialize {}
    impl<H, P> Request for HeadersAndPathParams<H, P> where P: PathParams {}
    impl<H, Q> Request for HeadersAndQueryParams<H, Q> where Q: Deserialize + Serialize {}
    impl<P, Q> Request for PathParamsAndQueryParams<P, Q>
    where P: PathParams, Q: Deserialize + Serialize {}
}

/// Implementations of the `Response` trait covering various combinations of required components
/// among request body, headers, status code.
pub mod response {
    use serde::{Deserialize, Serialize};

    use super::Response;

    /// A response with a body, headers, path parameters, and query parameters.
    #[derive(Debug)]
    pub struct All<B, H> where B: Deserialize + Serialize {
        /// The response's body.
        pub body: B,
        /// The response's headers.
        pub headers: H,
        /// The response's status code.
        pub status: u16,
    }

    /// A response with no body.
    #[derive(Debug)]
    pub struct NoBody<H> {
        /// The response's headers.
        pub headers: H,
        /// The response's status code.
        pub status: u16,
    }

    /// A response with no headers.
    #[derive(Debug)]
    pub struct NoHeaders<B> where B: Deserialize + Serialize {
        /// The response's body.
        pub body: B,
        /// The response's status code.
        pub status: u16,
    }

    /// A response with only a status code.
    #[derive(Debug)]
    pub struct StatusOnly {
        /// The response's status code.
        pub status: u16,
    }

    impl<B, H> Response for All<B, H> where B: Deserialize + Serialize {}
    impl<H> Response for NoBody<H> {}
    impl<B> Response for NoHeaders<B> where B: Deserialize + Serialize {}
    impl Response for StatusOnly {}
}
