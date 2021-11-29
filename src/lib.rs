//! This crate provides location for `nom` with support to grapheme clusters.
//!
//! # Examples
//! ```
//! use nom_grapheme_clusters::{Source, Span, SpanContent};
//! use nom::{IResult, bytes::complete::tag, combinator::map};
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! struct ParsedAtn {
//!     span: Span,
//! }
//!
//! fn parse_atn(input: Span) -> IResult<Span, ParsedAtn> {
//!     map(tag(Span::adhoc("atn̩̊")), |span| ParsedAtn { span })(input)
//! }
//!
//! # fn main() {
//! let source = Source::new("file.txt", "atn̩̊a");
//! let span = source.full_span();
//! let (_, parsed) = parse_atn(span).unwrap();
//! assert_eq!(parsed.span.as_str(), "atn̩̊");
//! assert_eq!(parsed.span.start().position(), 0);
//! assert_eq!(parsed.span.start().line(), 0);
//! assert_eq!(parsed.span.start().column(), 0);
//! assert_eq!(parsed.span.len(), 3);
//! assert_eq!(parsed.span.end().position(), 3);
//! assert_eq!(parsed.span.end().line(), 0);
//! assert_eq!(parsed.span.end().column(), 3);
//!
//! let source = Source::new("file2.txt", "atn");
//! let span = source.full_span();
//! let result = parse_atn(span);
//! assert!(result.is_err());
//! println!("{}", result.unwrap_err());
//! # }
//! ```

pub mod source;
mod location;
pub mod span;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
