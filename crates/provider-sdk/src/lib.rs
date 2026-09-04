//! HiTechCloud Provider SDK - Provider trait and request/response types
//!
//! This crate defines the core Provider trait that all LLM providers must implement,
//! along with the standardized request/response types.

pub mod provider;
pub mod stream;

pub use provider::{Provider, ProviderError, ProviderResult, ResponseStream};
pub use stream::StreamProcessor;
