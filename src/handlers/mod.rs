//! The handlers module contains functions responsible for handling incoming
//! requests and converting the request data into a response that can be sent
//! back to the client.
//!
//! For now, this module exposes the public API endpoints of the application.
//! This will likely change as the needs of the service evolve over time.

mod health;
mod subscriptions;

pub use health::*;
pub use subscriptions::*;
