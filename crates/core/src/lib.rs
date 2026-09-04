//! HiTechCloud Core - Core types, config, and utilities
//!
//! This crate provides the fundamental types and configuration structures
//! used across the HiTechCloud CLI platform.

pub mod config;
pub mod error;
pub mod types;

pub use config::HiTechCloudConfig;
pub use error::{Error, Result};
pub use types::*;
