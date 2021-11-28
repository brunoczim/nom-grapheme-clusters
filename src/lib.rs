//! This crate provides location for `nom` with support to grapheme clusters.

pub mod source;
mod location;
pub mod span;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
