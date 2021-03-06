//! Trait object `Service` instances
//!
//! Dynamically dispatched `Service` objects allow for erasing the underlying
//! `Service` type and using the `Service` instances as opaque handles. This can
//! be useful when the service instance cannot be explicitly named for whatever
//! reason.
//!
//! There are two variants of service objects. `BoxService` requires both the
//! service and the response future to be `Send`. These values can move freely
//! across threads. `UnsyncBoxService` requires both the service and the
//! response future to remain on the current thread. This is useful for
//! representing services that are backed by `Rc` or other non-`Send` types.
//!
//! # Examples
//!
//! ```
//! # extern crate futures;
//! # extern crate tower_service;
//! # extern crate tower_util;
//! # use futures::*;
//! # use futures::future::FutureResult;
//! # use tower_service::Service;
//! # use tower_util::{BoxService, service_fn};
//! // Respond to requests using a closure, but closures cannot be named...
//! # pub fn main() {
//! let svc = service_fn(|mut request: String| {
//!     request.push_str(" response");
//!     Ok(request)
//! });
//!
//! let service: BoxService<String, String, ()> = BoxService::new(svc);
//! # drop(service);
//! }
//! ```

mod sync;
mod unsync;

pub use self::{sync::BoxService, unsync::UnsyncBoxService};
