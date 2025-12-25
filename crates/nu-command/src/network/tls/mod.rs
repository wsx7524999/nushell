//! TLS support for networking commands.
//!
//! This module is available when the `network` feature is enabled. It requires
//! either the `native-tls` or `rustls-tls` feature to be selected.
//!
//! If both `native-tls` and `rustls-tls` features are enabled, `rustls-tls` takes precedence.
//!
//! See [`tls`] for how to get a TLS connector.

#[cfg(all(feature = "native-tls", not(feature = "rustls-tls")))]
#[path = "impl_native_tls.rs"]
mod impl_tls;

#[cfg(feature = "rustls-tls")]
#[path = "impl_rustls.rs"]
mod impl_tls;

#[cfg(all(not(feature = "native-tls"), not(feature = "rustls-tls")))]
compile_error!(
    "No TLS backend enabled. Please enable either the `native-tls` or `rustls-tls` feature."
);

pub use impl_tls::*;
