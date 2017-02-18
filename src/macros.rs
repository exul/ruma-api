/// Convenience macro for quickly creating an API endpoint.
#[macro_export]
macro_rules! endpoint {
    (
        description: $description:expr,
        name: $name:expr,
        rate_limited: $rate_limited:expr,
        request_method: $request_method:ident,
        requires_authentication: $requires_authentication:expr,
        router_path: $router_path:expr
    ) => {
        #[doc=$description]
        #[derive(Clone, Copy, Debug)]
        pub struct Endpoint;

        impl $crate::Endpoint for Endpoint {
            type Request = Request;
            type Response = Response;

            fn info() -> $crate::Info {
                $crate::Info {
                    description: $description,
                    name: $name,
                    rate_limited: $rate_limited,
                    request_method: $crate::Method::$request_method,
                    requires_authentication: $requires_authentication,
                    router_path: $router_path,
                }
            }
        }
    }
}
