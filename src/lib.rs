mod authenticator;
mod connection;
mod error;
mod result;
mod macros;
mod device_worker;
pub mod device;

pub(crate) const BUF_SIZE: usize = 8192;

pub use {
    coap, // Re-export coap
    error::Error,
    result::Result,
    authenticator::TradfriAuthenticator,
    connection::TradfriConnection,
    device::Device
};
