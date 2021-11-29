//! This crate provides location for `nom` with support to grapheme clusters.
//!
//! # Examples
//! ```
//! use nom_grapheme_clusters::{Source, Span};
//! use nom::{IResult, bytes::complete::tag, combinator::value};
//!
//! #[derive(Debug, Clone)]
//! struct ParsedAtn;
//!
//! fn parse_atn(input: Span) -> IResult<Span, ParsedAtn> {
//!     value(ParsedAtn, tag("atn̩̊"))(input)
//! }
//!
//! # fn main() {
//! let source = Source::new("file.txt", "atn̩̊");
//! let span = source.full_span();
//! println!("{:#?}", parse_atn(span));
//! # }
//! ```

pub mod source;
mod location;
pub mod span;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
