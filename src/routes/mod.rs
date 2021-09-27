//! The routes module contains the route-handling logic of the service.
//! It provides functions for receiving HTTP requests from connected clients
//! and sending back properly-formatted HTTP responses.

mod health;
mod subscriptions;

pub use health::health_check;
pub use subscriptions::{create_subscriber, subscribe, SubscribeForm};
