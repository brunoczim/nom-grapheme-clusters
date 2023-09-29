use crate::{source::Source, Span};
use nom::Slice;

#[test]
fn unitary_range_exclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    let span = Span::from_range(
        segments[1].location().clone(),
        segments[2].location().clone(),
    );
    assert_eq!(span.as_str(), "b");
}

#[test]
fn empty_range_exclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    let span = Span::from_range(
        segments[1].location().clone(),
        segments[1].location().clone(),
    );
    assert_eq!(span.as_str(), "");
}

#[test]
#[should_panic]
fn invalid_range_exclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    Span::from_range(
        segments[1].location().clone(),
        segments[0].location().clone(),
    );
}

#[test]
fn unitary_range_inclusive_inclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    let span = Span::from_range_inclusive(
        segments[2].location().clone(),
        segments[2].location().clone(),
    );
    assert_eq!(span.as_str(), "c");
}

#[test]
fn empty_range_inclusive_inclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    let span = Span::from_range_inclusive(
        segments[2].location().clone(),
        segments[1].location().clone(),
    );
    assert_eq!(span.as_str(), "");
}

#[test]
#[should_panic]
fn invalid_range_inclusive_inclusive() {
    let source = Source::new("foo.rs", "abcdef");
    let segments: Vec<_> = source.full_span().segments().collect();
    Span::from_range_inclusive(
        segments[2].location().clone(),
        segments[0].location().clone(),
    );
}

#[test]
fn segments() {
    let source = Source::new("complicated.rs", "av́e\nmař̋ia\ns̋ic̄");
    let span = source.full_span();
    let sliced_span = span.slice(1 .. 8);
    let mut iterator = sliced_span.segments();

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "v́");
    assert_eq!(segment.location().position(), 1);
    assert_eq!(segment.location().line_column(), (0, 1));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "e");
    assert_eq!(segment.location().position(), 2);
    assert_eq!(segment.location().line_column(), (0, 2));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "\n");
    assert_eq!(segment.location().position(), 3);
    assert_eq!(segment.location().line_column(), (0, 3));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "m");
    assert_eq!(segment.location().position(), 4);
    assert_eq!(segment.location().line_column(), (1, 0));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "a");
    assert_eq!(segment.location().position(), 5);
    assert_eq!(segment.location().line_column(), (1, 1));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "ř̋");
    assert_eq!(segment.location().position(), 6);
    assert_eq!(segment.location().line_column(), (1, 2));

    let segment = iterator.next().unwrap();
    assert_eq!(&segment, "i");
    assert_eq!(segment.location().position(), 7);
    assert_eq!(segment.location().line_column(), (1, 3));

    assert_eq!(iterator.next(), None);
}
