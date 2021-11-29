use crate::source::Source;
use nom::Slice;

#[test]
fn segments() {
    let source = Source::new("complicated.rs", "av́e\nmař̋ia\ns̋ic̄");
    let span = source.full_span();
    let sliced_span = span.slice(1 .. 8);
    eprintln!("{:#?}", sliced_span);
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
