//! Stream processing utilities

use futures::Stream;
use hitechcloud_core::StreamEvent;
use std::pin::Pin;

/// Type alias for a stream of events
pub type EventStream = Pin<Box<dyn Stream<Item = StreamEvent> + Send>>;

/// Trait for processing streaming events
pub trait StreamProcessor: Send + Sync {
    /// Process a stream event
    fn process_event(&mut self, event: StreamEvent) -> Option<StreamEvent>;
}
