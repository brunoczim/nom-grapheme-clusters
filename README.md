# nom-grapheme-clusters

A library that adds support to strings for (`nom`)[https://crates.io/crates/nom]
such that:

1. An "element" of such string is a grapheme cluster/segment, not a byte nor an
    UTF-8 character. A grapheme cluster is a sequence of one or more characters
    such that they are displayed as what a human would feel like a single
    character should be. For instance, `"m̤̊"` is a single grapheme cluster,
    although it looks like a single written character.

2. Grapheme clusters location at the input string (i.e. the "source code") are
    tracked by their absolute position, line and column. All of the three data
    are counted in terms of grapheme clusters as well. For instance, in the
    string `"m̤̊ef"`, the grapheme cluster "e" is at position `1`, line `0` and
    column `1` (all of them begin at `0`), because "m̤̊" counts as a single thing.

# Main-branch Docs

https://brunoczim.github.io/nom-grapheme-clusters/nom_grapheme_clusters/index.html

# Example
```rust
use nom_grapheme_clusters::{Source, Span, SpanContent, tag_table};
use nom::{IResult, bytes::complete::tag, combinator::map};

#[derive(Debug, Clone)]
struct Tags {
    smth: Span,
    atn: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedAtn {
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedSmth {
    span: Span,
}

fn parse_smth<'a>(
    tags: &'a Tags
) -> impl FnMut(Span) -> IResult<Span, ParsedAtn> + 'a {
    map(tag(&tags.smth), |span| ParsedAtn { span })
}

fn parse_atn<'a>(
    tags: &'a Tags
) -> impl FnMut(Span) -> IResult<Span, ParsedAtn> + 'a {
    map(tag(&tags.atn), |span| ParsedAtn { span })
}

fn main() {
    let tags = tag_table! {
        Tags {
            smth: "smth",
            atn: "atn̩̊",
        }
    };
    let source = Source::new("file.txt", "atn̩̊smtha");
   
    let span0 = source.full_span();
    let (span1, parsed) = parse_atn(&tags)(span0).unwrap();
    assert_eq!(parsed.span.as_str(), "atn̩̊");
    assert_eq!(parsed.span.start().position(), 0);
    assert_eq!(parsed.span.start().line(), 0);
    assert_eq!(parsed.span.start().column(), 0);
    assert_eq!(parsed.span.len(), 3);
    assert_eq!(parsed.span.end().position(), 3);
    assert_eq!(parsed.span.end().line(), 0);
    assert_eq!(parsed.span.end().column(), 3);
   
    let (span2, parsed) = parse_smth(&tags)(span1).unwrap();
    assert_eq!(parsed.span.as_str(), "smth");
    assert_eq!(parsed.span.start().position(), 3);
    assert_eq!(parsed.span.start().line(), 0);
    assert_eq!(parsed.span.start().column(), 3);
    assert_eq!(parsed.span.len(), 4);
    assert_eq!(parsed.span.end().position(), 7);
    assert_eq!(parsed.span.end().line(), 0);
    assert_eq!(parsed.span.end().column(), 7);
   
    let result = parse_atn(&tags)(span2);
    assert!(result.is_err());
    println!("{}", result.unwrap_err());
}
```
