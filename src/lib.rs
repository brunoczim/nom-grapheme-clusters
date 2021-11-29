//! This crate provides location for `nom` with support to grapheme clusters.
//!
//! # Examples
//! ```
//! use nom_grapheme_clusters::{Source, Span, SpanContent, tag_table};
//! use nom::{IResult, bytes::complete::tag, combinator::map};
//!
//! #[derive(Debug, Clone)]
//! struct Tags {
//!     smth: Span,
//!     atn: Span,
//! }
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
//! fn parse_smth(tags: &Tags) -> impl FnMut(Span) -> IResult<Span, ParsedAtn> {
//!     map(tag(tags.smth.clone()), |span| ParsedAtn { span })
//! }
//!
//! fn parse_atn(tags: &Tags) -> impl FnMut(Span) -> IResult<Span, ParsedAtn> {
//!     map(tag(tags.atn.clone()), |span| ParsedAtn { span })
//! }
//!
//! # fn main() {
//! let tags = tag_table! {
//!     Tags {
//!         smth: "smth",
//!         atn: "atn̩̊",
//!     }
//! };
//! let source = Source::new("file.txt", "atn̩̊smtha");
//!
//! let span = source.full_span();
//! let (span, parsed) = parse_atn(&tags)(span).unwrap();
//! assert_eq!(parsed.span.as_str(), "atn̩̊");
//! assert_eq!(parsed.span.start().position(), 0);
//! assert_eq!(parsed.span.start().line(), 0);
//! assert_eq!(parsed.span.start().column(), 0);
//! assert_eq!(parsed.span.len(), 3);
//! assert_eq!(parsed.span.end().position(), 3);
//! assert_eq!(parsed.span.end().line(), 0);
//! assert_eq!(parsed.span.end().column(), 3);
//!
//! let (span, parsed) = parse_smth(&tags)(span).unwrap();
//! assert_eq!(parsed.span.as_str(), "smth");
//! assert_eq!(parsed.span.start().position(), 3);
//! assert_eq!(parsed.span.start().line(), 0);
//! assert_eq!(parsed.span.start().column(), 3);
//! assert_eq!(parsed.span.len(), 4);
//! assert_eq!(parsed.span.end().position(), 7);
//! assert_eq!(parsed.span.end().line(), 0);
//! assert_eq!(parsed.span.end().column(), 7);
//!
//! let result = parse_atn(&tags)(span);
//! assert!(result.is_err());
//! println!("{}", result.unwrap_err());
//! # }
//! ```

#[macro_use]
mod macros;

pub mod source;
mod location;
pub mod span;

pub use location::{LocatedSegment, Location};
pub use source::Source;
pub use span::{Span, SpanContent};
