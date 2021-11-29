//! This crate provides location for `nom` with support to grapheme clusters.
//!
//! # Examples
//! ```
//! use nom_grapheme_clusters::{Source, Span, SpanContent};
//! use nom::{IResult, bytes::complete::tag, combinator::value};
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct ParsedAtn;
//!
//! fn parse_atn(input: Span) -> IResult<Span, ParsedAtn> {
//!     value(ParsedAtn, tag(Span::adhoc("atn̩̊")))(input)
//! }
//!
//! # fn main() {
//! let source = Source::new("file.txt", "atn̩̊");
//! let span = source.full_span();
//! assert_eq!(parse_atn(span).unwrap().1, ParsedAtn);
//!
//! let source = Source::new("file.txt", "atn");
//! let span = source.full_span();
//! assert!(parse_atn(span).is_err());
//! # }
//! ```

pub mod source;
mod location;
pub mod span;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
