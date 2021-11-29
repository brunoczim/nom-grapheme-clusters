//! This module defines macros.

/// Creates a table of tags to be used e.g. in a parser.
///
/// # Examples
///
/// ## Binding to Variables
///
/// ```
/// use nom_grapheme_clusters::tag_table;
///
/// # fn main() {
/// tag_table! {
///     name = "MyTags";
///     let foo = "Foo";
///     let bar = "Bar";
/// }
///
/// assert_eq!(foo.as_str(), "Foo");
/// assert_eq!(foo.start().position(), 0);
/// assert_eq!(bar.as_str(), "Bar");
/// assert_eq!(bar.start().position(), 3);
/// # }
/// ```
///
/// ## Binding to a Struct
///
/// ```
/// use nom_grapheme_clusters::{Span, tag_table};
///
/// struct MyTags {
///     foo: Span,
///     bar: Span,
/// }
///
/// # fn main() {
/// let tags = tag_table! {
///     MyTags {
///         foo: "Foo",
///         bar: "Bar",
///     }
/// };
///
/// assert_eq!(tags.foo.as_str(), "Foo");
/// assert_eq!(tags.foo.start().position(), 0);
/// assert_eq!(tags.bar.as_str(), "Bar");
/// assert_eq!(tags.bar.start().position(), 3);
/// # }
/// ```
#[macro_export]
macro_rules! tag_table {
    (name = $name:expr; $(let $var:ident = $value:expr;)*) => {
        let ($($var),*) = {
            struct Entry {
                start: usize,
                length: usize,
            }

            let mut contents = String::new();

            $(
                let $var = {
                    let value = $value;
                    let start = contents.len();
                    let length = $crate::source::count_grapheme_clusters(value);
                    contents.push_str(value.as_ref());
                    Entry { start, length }
                };
            )*

            let source = $crate::Source::new($name, contents);

            ($(
                $crate::Span::new(
                    $crate::Location::new(source.clone(), $var.start),
                    $var.length,
                ),
            )*)
        };
    };

    ($(let $var:ident = $value:expr;)*) => {
        tag_table! { name = "<tags>"; $(let $var = $value;)* }
    };

    ($name:ident { $($var:ident : $value:expr),* }) => {{
        tag_table! { name = stringify!($name); $(let $var = $value;)* }
        $name { $($var),* }
    }};

    ($name:ident { $($var:ident : $value:expr,)* }) => {{
        tag_table! { name = stringify!($name); $(let $var = $value;)* }
        $name { $($var),* }
    }};

}