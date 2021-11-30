//! This module provides means of tracking location in a source code.

use crate::{source::Source, span::Span};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
};

/// The location in a source code.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    /// The source code object.
    source: Source,
    /// The string segment position.
    position: usize,
}

impl Location {
    /// Creates a new location given a source code object and a string segment
    /// position in the object.
    pub(super) fn new_unchecked(source: Source, position: usize) -> Self {
        Self { source, position }
    }

    /// Creates a new location given the source code object and position index.
    ///
    /// # Panics
    /// Panics if `position` is past beyond source length in number of segments.
    pub fn new(source: Source, position: usize) -> Self {
        if source.len() < position {
            panic!(
                "Location position is too big; availabe: {}, given: {}",
                source.len(),
                position,
            );
        }
        Self::new_unchecked(source, position)
    }

    /// This location's position in the source code in terms of grapheme
    /// clusters/segments.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Returns a mutable reference to the location's position.
    pub(super) fn position_mut(&mut self) -> &mut usize {
        &mut self.position
    }

    /// The source code object this location refers to.
    pub fn source(&self) -> &Source {
        &self.source
    }

    /// Finds the line and column (respectively) of this location in the source
    /// code. Line and column count grapheme clusters/segments, not bytes nor
    /// characters.
    pub fn line_column(&self) -> (usize, usize) {
        let line = self.source.line(self.position);
        let line_start = self.source.line_start(line);
        (line, self.position - line_start)
    }

    /// Finds the line of this location in the source code. Line counts grapheme
    /// clusters/segments, not bytes nor characters.
    pub fn line(&self) -> usize {
        self.source.line(self.position)
    }

    /// Finds the column of this location in the source code. Column counts
    /// grapheme clusters/segments, not bytes nor characters.
    pub fn column(&self) -> usize {
        let (_, column) = self.line_column();
        column
    }

    /// Returns the underlying grapheme cluster segment content at this
    /// location.
    pub fn as_str(&self) -> &str {
        &self.source[self.position]
    }

    /// Returns the single segmented pointed by this location.
    pub fn segment(&self) -> LocatedSegment {
        LocatedSegment { location: self.clone() }
    }

    /// Creates a [`Span`] containing the whole line this location is in.
    pub fn line_span(&self) -> Span {
        let line = self.line();
        let init = self.source().line_start(line);
        let end = self
            .source()
            .try_line_start(line + 1)
            .unwrap_or(self.source().len());
        Span::new(Self::new(self.source.clone(), init), end - init)
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let (line, column) = self.line_column();
        fmtr.debug_struct("Location")
            .field("source", &self.source)
            .field("position", &self.position)
            .field("line", &line)
            .field("column", &column)
            .finish()
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let (line, column) = self.line_column();
        write!(fmtr, "in {} ({}, {})", self.source, line + 1, column + 1)
    }
}

impl<T> AsRef<T> for Location
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.as_str().as_ref()
    }
}

/// A grapheme cluster segment with its location in the source code.
#[derive(Clone, Debug)]
pub struct LocatedSegment {
    /// Inner location.
    location: Location,
}

impl LocatedSegment {
    /// Returns the location of this segment.
    pub fn location(&self) -> &Location {
        &self.location
    }

    /// Returns the segment (a single grapheme cluster) as a string.
    pub fn as_str(&self) -> &str {
        self.location.as_str()
    }

    /// Tests whether this segment is a single character.
    pub fn is_single_char(&self) -> bool {
        self.len() == 1
    }

    /// Tests whether this segment is alphabetic. UTF-8 alphabetic characters
    /// with diacritics are also considered alphabetic.
    pub fn is_alphabetic(&self) -> bool {
        self.chars().next().map_or(false, |ch| ch.is_alphabetic())
    }

    /// Tests whether this segment is ASCII alphabetic.
    pub fn is_ascii_alphabetic(&self) -> bool {
        self.is_single_char()
            && self.chars().next().map_or(false, |ch| ch.is_ascii_alphabetic())
    }

    /// Tests whether this segment is numeric. UTF-8 numeric characters
    /// with diacritics are also considered numeric.
    pub fn is_numeric(&self) -> bool {
        self.chars().next().map_or(false, |ch| ch.is_numeric())
    }

    /// Tests whether this segment is ASCII numeric.
    pub fn is_ascii_numeric(&self) -> bool {
        self.is_single_char()
            && self.chars().next().map_or(false, |ch| ch.is_ascii_digit())
    }

    /// Tests whether this segment is alphanumeric. UTF-8 alphanumeric
    /// characters with diacritics are also considered alphanumeric.
    pub fn is_alphanumeric(&self) -> bool {
        self.chars().next().map_or(false, |ch| ch.is_alphanumeric())
    }

    /// Tests whether this segment is ASCII alphanumeric.
    pub fn is_ascii_alphanumeric(&self) -> bool {
        self.is_single_char()
            && self
                .chars()
                .next()
                .map_or(false, |ch| ch.is_ascii_alphanumeric())
    }

    /// Tests whether this segment is an ASCII digit. Digits characters with
    /// diacritics are NOT considered digits. Digit characters are `0-9`,
    /// `a-z`, `A-Z`, depending on the base.
    pub fn is_digit(&self, base: u32) -> bool {
        self.is_single_char()
            && self.chars().next().map_or(false, |ch| ch.is_digit(base))
    }

    /// Converts this grapheme cluster to a digit of given base. Digits with
    /// diacritics are not considered digits. Digit characters are `0-9`, `a-z`,
    /// `A-Z`, depending on the base.
    pub fn to_digit(&self, base: u32) -> Option<u32> {
        self.chars()
            .next()
            .and_then(|ch| ch.to_digit(base))
            .filter(|_| self.is_single_char())
    }

    /// Tests if this segment is only a linefeed character.
    pub fn is_newline(&self) -> bool {
        self == "\n"
    }

    /// Tests whether this segment is a single space.
    pub fn is_space(&self) -> bool {
        self == " "
    }

    /// Tests whether this segment is composed only by UTF-8 whitespace
    /// characters.
    pub fn is_whitespace(&self) -> bool {
        self.chars().all(char::is_whitespace)
    }
}

impl Deref for LocatedSegment {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for LocatedSegment {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", &**self)
    }
}

impl PartialEq for LocatedSegment {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T> PartialEq<T> for LocatedSegment
where
    T: ?Sized,
    str: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        &**self == other
    }
}

impl Eq for LocatedSegment {}

impl PartialOrd for LocatedSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialOrd<T> for LocatedSegment
where
    T: ?Sized,
    str: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl Ord for LocatedSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl Hash for LocatedSegment {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(hasher)
    }
}

impl<T> AsRef<T> for LocatedSegment
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}

impl<T> Borrow<T> for LocatedSegment
where
    T: ?Sized,
    str: Borrow<T>,
{
    fn borrow(&self) -> &T {
        (**self).borrow()
    }
}
