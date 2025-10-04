pub mod cli;
pub mod handler;
pub mod models;
pub mod server;

// Python bindings (optional feature)
#[cfg(feature = "python")]
pub mod python_bindings;

// Re-export for Rust library users
pub use handler::WindowCapServer;
pub use models::*;

// Re-export xcap types for convenience
pub use xcap::{Monitor, Window};
