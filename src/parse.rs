//! Exports parse functions related to segments/grapheme clusters.

mod tag;

use crate::LocatedSegment;
use nom::{
    error::{ErrorKind, ParseError},
    FindToken,
};
pub use tag::Tag;

/// Recognizes zero or more UTF-8 alphabetic segments, possibly with diacritics.
pub fn alpha0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_alphabetic())
}

/// Recognizes one or more UTF-8 alphabetic segments, possibly with diacritics.
pub fn alpha1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_alphabetic(),
        ErrorKind::Alpha,
    )
}

/// Recognizes zero or more UTF-8 alphabetic segments without diacritics.
pub fn char_alpha0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| {
        !item.is_alphabetic() || !item.is_single_char()
    })
}

/// Recognizes one or more UTF-8 alphabetic segments without diacritics.
pub fn char_alpha1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_alphabetic() || !item.is_single_char(),
        ErrorKind::Alpha,
    )
}

/// Recognizes zero or more ASCII alphabetic segments without diacritics.
pub fn ascii_alpha0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_ascii_alphabetic())
}

/// Recognizes one or more ASCII alphabetic segments without diacritics.
pub fn ascii_alpha1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_ascii_alphabetic(),
        ErrorKind::Alpha,
    )
}

/// Recognizes zero or more UTF-8 alphanumeric segments, possibly with
/// diacritics.
pub fn alphanumeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_alphanumeric())
}

/// Recognizes one or more UTF-8 alphanumeric segments, possibly with
/// diacritics.
pub fn alphanumeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_alphanumeric(),
        ErrorKind::AlphaNumeric,
    )
}

/// Recognizes zero or more UTF-8 alphanumeric segments without diacritics.
pub fn char_alphanumeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| {
        !item.is_alphanumeric() || !item.is_single_char()
    })
}

/// Recognizes one or more UTF-8 alphanumeric segments without diacritics.
pub fn char_alphanumeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_alphanumeric() || !item.is_single_char(),
        ErrorKind::AlphaNumeric,
    )
}

/// Recognizes zero or more ASCII alphanumeric segments without diacritics.
pub fn ascii_alphanumeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_ascii_alphanumeric())
}

/// Recognizes one or more ASCII alphanumeric segments without diacritics.
pub fn ascii_alphanumeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_ascii_alphanumeric(),
        ErrorKind::AlphaNumeric,
    )
}

/// Recognizes zero or more UTF-8 numeric segments, possibly with diacritics.
pub fn numeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_numeric())
}

/// Recognizes one or more UTF-8 numeric segments, possibly with diacritics.
pub fn numeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_numeric(),
        ErrorKind::Digit,
    )
}

/// Recognizes zero or more UTF-8 numeric segments without diacritics.
pub fn char_numeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| {
        !item.is_numeric() || !item.is_single_char()
    })
}

/// Recognizes one or more UTF-8 numeric segments without diacritics.
pub fn char_numeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_numeric() || !item.is_single_char(),
        ErrorKind::Digit,
    )
}

/// Recognizes zero or more ASCII numeric segments without diacritics.
pub fn ascii_numeric0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_ascii_numeric())
}

/// Recognizes one or more ASCII numeric segments without diacritics.
pub fn ascii_numeric1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_ascii_numeric(),
        ErrorKind::Digit,
    )
}

/// Recognizes zero or more digits in the given base. ASCII characters `0-9`.
/// `a-z`, `A-Z` are considered digits, depending on the base.
pub fn digit0<T, E>(base: u32) -> impl FnMut(T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    move |input| input.split_at_position_complete(|item| item.is_digit(base))
}

/// Recognizes one or more digits in the given base. ASCII characters `0-9`.
/// `a-z`, `A-Z` are considered digits, depending on the base.
pub fn digit1<T, E>(base: u32) -> impl FnMut(T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    move |input| {
        input.split_at_position1_complete(
            |item| item.is_digit(base),
            match base {
                8 => ErrorKind::OctDigit,
                16 => ErrorKind::HexDigit,
                _ => ErrorKind::Digit,
            },
        )
    }
}

/// Recognizes zero or more unicode whitespace graphemes.
pub fn whitespace0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_whitespace())
}

/// Recognizes one or more unicode whitespace graphemes.
pub fn whitespace1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(
        |item| !item.is_whitespace(),
        ErrorKind::Space,
    )
}

/// Recognizes zero or more ASCII spaces.
pub fn space0<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position_complete(|item| !item.is_space())
}

/// Recognizes one or more ASCII spaces.
pub fn space1<T, E>(input: T) -> nom::IResult<T, T, E>
where
    T: nom::InputTakeAtPosition<Item = LocatedSegment>,
    E: ParseError<T>,
{
    input.split_at_position1_complete(|item| !item.is_space(), ErrorKind::Space)
}

/// Recognizes one linefeed (`'\n'`) ASCII character.
pub fn newline<T, E>(input: T) -> nom::IResult<T, T::Item, E>
where
    T: nom::InputIter + nom::InputLength + nom::InputTake,
    for<'tok> T::Item: PartialEq<&'tok str>,
    E: ParseError<T>,
{
    segment("\n")(input)
}

/// Recognizes the sequence `"\r\n"`.
pub fn crlf<T, E>(input: T) -> nom::IResult<T, T, E>
where
    for<'slice, 'seg> T: nom::InputTake + nom::Compare<Tag<'slice, 'seg>>,
    E: ParseError<T>,
{
    match Tag(&["\r", "\n"]).into_fn::<_, nom::error::Error<T>>()(input) {
        Ok(data) => Ok(data),
        Err(nom_err) => Err(nom_err
            .map(|error| E::from_error_kind(error.input, ErrorKind::CrLf))),
    }
}

/// Parses line ending, either a linefeed or a `"\r\n"` sequence.
pub fn line_ending<T, E>(input: T) -> nom::IResult<T, T, E>
where
    for<'slice, 'seg> T: nom::InputTake + nom::Compare<Tag<'slice, 'seg>>,
    E: ParseError<T>,
{
    match input.compare(Tag(&["\n"])) {
        nom::CompareResult::Ok => Ok(input.take_split(1)),
        nom::CompareResult::Error => crlf(input),
        nom::CompareResult::Incomplete => {
            Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::CrLf)))
        },
    }
}

/// Recognizes the given grapheme cluster/segment.
pub fn segment<T, A, E>(
    expected: A,
) -> impl FnMut(T) -> nom::IResult<T, T::Item, E>
where
    T: nom::InputIter + nom::InputLength + nom::InputTake,
    T::Item: PartialEq<A>,
    E: ParseError<T>,
{
    move |input| {
        let mut iterator = input.iter_indices();
        match iterator.next() {
            Some((_, segment)) => {
                if segment == expected {
                    match iterator.next() {
                        Some((index, _)) => Ok((input.take(index), segment)),
                        None => Ok((input.take(input.input_len()), segment)),
                    }
                } else {
                    Err(nom::Err::Error(E::from_error_kind(
                        input,
                        ErrorKind::IsA,
                    )))
                }
            },
            None => {
                Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
            },
        }
    }
}

/// Recognizes any grapheme cluster/segment.
pub fn any_segment<T, E>(input: T) -> nom::IResult<T, LocatedSegment, E>
where
    T: nom::InputIter<Item = LocatedSegment>
        + nom::InputLength
        + nom::InputTake,
    E: ParseError<T>,
{
    let mut iterator = input.iter_indices();
    match iterator.next() {
        Some((_, segment)) => match iterator.next() {
            Some((index, _)) => Ok((input.take(index), segment)),
            None => Ok((input.take(input.input_len()), segment)),
        },
        None => Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof))),
    }
}

/// Recognizes any of the grapheme clusters/segments in the given list.
pub fn one_of<T, L, E>(list: L) -> impl FnMut(T) -> nom::IResult<T, T::Item, E>
where
    T: nom::InputIter + nom::InputLength + nom::InputTake,
    for<'tok> L: FindToken<&'tok T::Item>,
    E: ParseError<T>,
{
    move |input| {
        let mut iterator = input.iter_indices();
        match iterator.next() {
            Some((_, segment)) => {
                if list.find_token(&segment) {
                    match iterator.next() {
                        Some((index, _)) => Ok((input.take(index), segment)),
                        None => Ok((input.take(input.input_len()), segment)),
                    }
                } else {
                    Err(nom::Err::Error(E::from_error_kind(
                        input,
                        ErrorKind::IsNot,
                    )))
                }
            },
            None => {
                Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
            },
        }
    }
}

/// Recognizes a grapheme clusters/segments NOT in the given list.
pub fn none_of<T, L, E>(list: L) -> impl FnMut(T) -> nom::IResult<T, T::Item, E>
where
    T: nom::InputIter + nom::InputLength + nom::InputTake,
    for<'tok> L: FindToken<&'tok T::Item>,
    E: ParseError<T>,
{
    move |input| {
        let mut iterator = input.iter_indices();
        match iterator.next() {
            Some((_, segment)) => {
                if !list.find_token(&segment) {
                    match iterator.next() {
                        Some((index, _)) => Ok((input.take(index), segment)),
                        None => Ok((input.take(input.input_len()), segment)),
                    }
                } else {
                    Err(nom::Err::Error(E::from_error_kind(
                        input,
                        ErrorKind::IsNot,
                    )))
                }
            },
            None => {
                Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
            },
        }
    }
}
