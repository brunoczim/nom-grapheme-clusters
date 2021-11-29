//! This module defines macros.

/// Creates a table of tags to be used e.g. in a parser.
#[macro_export]
macro_rules! tag_table {
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
}
