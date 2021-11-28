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

    /// The string segment position in the source code.
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
    /// code.
    pub fn line_column(&self) -> (usize, usize) {
        let line = self.source.line(self.position);
        let line_start = self.source.line_start(line);
        (line, self.position - line_start)
    }

    /// Finds the line of this location in the source code.
    pub fn line(&self) -> usize {
        self.source.line(self.position)
    }

    /// Finds the column of this location in the source code.
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

    /// Creates a [`Span`](crate::span::Span) containing the whole line this
    /// location is in.
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
