use super::Source;

#[test]
fn index_segments() {
    let source = Source::new("complicated.rs", "av́e\nmař̋ia\ns̋ic̄");

    let mut index = 0;
    assert_eq!(source.get(index), Some("a"));
    index += 1;
    assert_eq!(source.get(index), Some("v́"));
    index += 1;
    assert_eq!(source.get(index), Some("e"));
    index += 1;
    assert_eq!(source.get(index), Some("\n"));
    index += 1;
    assert_eq!(source.get(index), Some("m"));
    index += 1;
    assert_eq!(source.get(index), Some("a"));
    index += 1;
    assert_eq!(source.get(index), Some("ř̋"));
    index += 1;
    assert_eq!(source.get(index), Some("i"));
    index += 1;
    assert_eq!(source.get(index), Some("a"));
    index += 1;
    assert_eq!(source.get(index), Some("\n"));
    index += 1;
    assert_eq!(source.get(index), Some("s̋"));
    index += 1;
    assert_eq!(source.get(index), Some("i"));
    index += 1;
    assert_eq!(source.get(index), Some("c̄"));
    index += 1;
    assert_eq!(source.get(index), None);
}
