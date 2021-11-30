//! This crate provides location for `nom` with support to grapheme clusters.
//!
//! # Examples
//! ```
//! use nom_grapheme_clusters::{Source, Span, SpanContent, parse::Tag};
//! use nom::{IResult, combinator::map};
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct ParsedAtn {
//!     span: Span,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct ParsedSmth {
//!     span: Span,
//! }
//!
//! fn parse_smth(input: Span) -> IResult<Span, ParsedSmth> {
//!     map(Tag(&["s", "m", "t", "h"]), |span| ParsedSmth { span })(input)
//! }
//!
//! fn parse_atn(input: Span) -> IResult<Span, ParsedAtn> {
//!     map(Tag(&["a", "t", "n̩̊"]), |span| ParsedAtn { span })(input)
//! }
//!
//! # fn main() {
//! let source = Source::new("file.txt", "atn̩̊smtha");
//!
//! let span0 = source.full_span();
//! let (span1, parsed) = parse_atn(span0).unwrap();
//! assert_eq!(parsed.span.as_str(), "atn̩̊");
//! assert_eq!(parsed.span.start().position(), 0);
//! assert_eq!(parsed.span.start().line(), 0);
//! assert_eq!(parsed.span.start().column(), 0);
//! assert_eq!(parsed.span.len(), 3);
//! assert_eq!(parsed.span.end().position(), 3);
//! assert_eq!(parsed.span.end().line(), 0);
//! assert_eq!(parsed.span.end().column(), 3);
//!
//! let (span2, parsed) = parse_smth(span1).unwrap();
//! assert_eq!(parsed.span.as_str(), "smth");
//! assert_eq!(parsed.span.start().position(), 3);
//! assert_eq!(parsed.span.start().line(), 0);
//! assert_eq!(parsed.span.start().column(), 3);
//! assert_eq!(parsed.span.len(), 4);
//! assert_eq!(parsed.span.end().position(), 7);
//! assert_eq!(parsed.span.end().line(), 0);
//! assert_eq!(parsed.span.end().column(), 7);
//!
//! let result = parse_atn(span2);
//! assert!(result.is_err());
//! println!("{}", result.unwrap_err());
//! # }
//! ```

#![deny(missing_docs)]

pub mod source;
mod location;
pub mod span;
pub mod parse;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
