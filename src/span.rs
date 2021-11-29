//! This module provides ways of tracking ranges (spans) in the source code.

#[cfg(test)]
mod test;

use crate::{
    location::{LocatedSegment, Location},
    source::Source,
};
use nom::{
    error::ParseError,
    InputIter,
    InputLength,
    InputTake,
    InputTakeAtPosition,
    Offset,
    Slice,
};
use std::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::{Bound, Deref, RangeBounds},
};

/// A span (a range) in the source code.
///
/// It can be created with the constructor [`Span::new`], or with
/// [`Source::full_span`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    /// Start of the span.
    start: Location,
    /// Length of the span in string segments.
    length: usize,
}

impl Span {
    /// Creates a new span given the start location and length.
    pub(super) fn new_unchecked(start: Location, length: usize) -> Self {
        Self { start, length }
    }

    /// Creates a new span given the start location and length.
    ///
    /// # Panics
    /// Panics if `length` is too big, i.e. `start.position() + length >
    /// start.source().len()`.
    pub fn new(start: Location, length: usize) -> Self {
        if start.source().len() - start.position() < length {
            panic!(
                "Span length is too big; available size: {}, given: {}",
                start.source().len() - start.position(),
                length
            );
        }
        Self::new_unchecked(start, length)
    }

    /// The start location of this span.
    pub fn start(&self) -> Location {
        self.start.clone()
    }

    /// The end location of this span.
    pub fn end(&self) -> Location {
        Location::new_unchecked(
            self.source().clone(),
            self.start.position() + self.length,
        )
    }

    /// The length of this span in string segments.
    pub fn len(&self) -> usize {
        self.length
    }

    /// The source code object this span refers to.
    pub fn source(&self) -> &Source {
        self.start.source()
    }

    /// Gets the string this span includes as a whole.
    pub fn as_str(&self) -> &str {
        let start = self.start.position();
        self.source().get(start .. start + self.len()).unwrap()
    }

    /// Creates a type that, when displayed, shows the span contents, rather
    /// than location.
    pub fn content(&self) -> SpanContent {
        SpanContent { span: self.clone() }
    }

    /// Expands this span in order to contain the whole lines the original span
    /// contains.
    pub fn expand_lines(&self) -> Span {
        let start_line = self.start().line();
        let end_line = self.end().line();
        let init = self.source().line_start(start_line);
        let end = self
            .source()
            .try_line_start(end_line + 1)
            .unwrap_or(self.source().len());
        Self::new(Location::new(self.source().clone(), init), end - init)
    }

    /// Slices this span to the given range. Returns `None` if the range is
    /// invalid.
    pub fn try_slice<R>(&self, range: R) -> Option<Self>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            Bound::Included(&position) => position,
            Bound::Excluded(position) => position.saturating_add(1),
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(position) => position.saturating_add(1),
            Bound::Excluded(&position) => position,
            Bound::Unbounded => self.len(),
        };

        if start <= self.length && end <= self.length && start <= end {
            let start_loc = Location::new_unchecked(
                self.start.source().clone(),
                self.start.position() + start,
            );
            Some(Self::new(start_loc, end - start))
        } else {
            None
        }
    }

    /// Creates an iterator over located grapheme cluster segments, namely
    /// [`LocatedSegment`]s.
    pub fn segments(&self) -> Segments {
        self.clone().into_iter()
    }

    /// Creates an [`IndexedSegments`] iterator, which yields a tuple of
    /// position and a [`LocatedSegment`] in that position. Note that this is
    /// just for convenience with e.g. nom, [`LocatedSegment`] already contains
    /// its position, and so [`Segments`] can be used.
    pub fn indexed_segments(&self) -> IndexedSegments {
        self.segments().indexed()
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.debug_struct("Span")
            .field("source", self.source())
            .field("start", &self.start())
            .field("end", &self.end())
            .field("content", &self.as_str())
            .finish()
    }
}

impl fmt::Display for Span {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        let file = self.source().name();
        let (line_start, col_start) = self.start().line_column();
        let (line_end, col_end) = self.end().line_column();
        write!(
            fmtr,
            "in {} from ({}, {}) to ({}, {})",
            file,
            line_start + 1,
            col_start + 1,
            line_end + 1,
            col_end + 1
        )
    }
}

impl<T> AsRef<T> for Span
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        self.as_str().as_ref()
    }
}

impl IntoIterator for Span {
    type Item = LocatedSegment;
    type IntoIter = Segments;

    fn into_iter(self) -> Self::IntoIter {
        Segments { span: self }
    }
}

impl InputLength for Span {
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<R> Slice<R> for Span
where
    R: RangeBounds<usize>,
{
    fn slice(&self, range: R) -> Self {
        self.try_slice(range).expect("invalid span range")
    }
}

impl InputIter for Span {
    type Item = LocatedSegment;
    type Iter = IndexedSegments;
    type IterElem = Segments;

    fn iter_indices(&self) -> Self::Iter {
        self.indexed_segments()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.segments()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.segments().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::new(count - self.len()))
        }
    }
}

impl InputTake for Span {
    fn take(&self, count: usize) -> Self {
        self.slice(count ..)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(.. count), self.slice(count ..))
    }
}

impl InputTakeAtPosition for Span {
    type Item = LocatedSegment;

    fn split_at_position<P, E>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(predicate) {
            Some(pos) => Ok(self.take_split(pos)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position1<P, E>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(predicate) {
            Some(0) => {
                Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
            },
            Some(pos) => Ok(self.take_split(pos)),
            None => Err(nom::Err::Incomplete(nom::Needed::new(1))),
        }
    }

    fn split_at_position_complete<P, E>(
        &self,
        predicate: P,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(predicate) {
            Some(pos) => Ok(self.take_split(pos)),
            None => Ok(self.take_split(self.len())),
        }
    }

    fn split_at_position1_complete<P, E>(
        &self,
        predicate: P,
        e: nom::error::ErrorKind,
    ) -> nom::IResult<Self, Self, E>
    where
        P: Fn(Self::Item) -> bool,
        E: ParseError<Self>,
    {
        match self.position(predicate) {
            Some(0) => {
                Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
            },
            Some(pos) => Ok(self.take_split(pos)),
            None => {
                if self.len() > 0 {
                    Ok(self.take_split(self.len()))
                } else {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                }
            },
        }
    }
}

impl Offset for Span {
    fn offset(&self, second: &Self) -> usize {
        second.start().position() - self.start().position()
    }
}

/// Iterator over located segments of a [`Span`]. Created by [`Span::segments`]
/// or [`SpanContent::segments`], as well via [`IntoIterator`] trait.
/// Double-ended and sized.
#[derive(Debug, Clone)]
pub struct Segments {
    /// Span being iterated.
    span: Span,
}

impl Segments {
    /// Converts this iterator into an [`IndexedSegments`] iterator, which
    /// yields a tuple of position and a [`LocatedSegment`] in that
    /// position. Note that this is just for convenience with e.g. nom,
    /// [`LocatedSegment`] already contains its position.
    pub fn indexed(self) -> IndexedSegments {
        IndexedSegments { inner: self }
    }
}

impl Iterator for Segments {
    type Item = LocatedSegment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.span.len() > 0 {
            let segment = self.span.start.segment();
            self.span.length -= 1;
            *self.span.start.position_mut() += 1;
            Some(segment)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.span.length, Some(self.span.length))
    }
}

impl DoubleEndedIterator for Segments {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.span.len() > 0 {
            self.span.length -= 1;
            let segment = self.span.end().segment();
            Some(segment)
        } else {
            None
        }
    }
}

impl ExactSizeIterator for Segments {}

/// Iterator over segments of a [`Span`] which also yield postion for
/// convenience. Created by [`Segments::indexed`], [`Span::indexed_segments`] or
/// [`SpanContent::indexed_segments`]. Double-ended and sized.
#[derive(Debug, Clone)]
pub struct IndexedSegments {
    /// Inner iterator over segments.
    inner: Segments,
}

impl Iterator for IndexedSegments {
    type Item = (usize, LocatedSegment);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|segment| (segment.location().position(), segment))
    }
}

impl DoubleEndedIterator for IndexedSegments {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner
            .next_back()
            .map(|segment| (segment.location().position(), segment))
    }
}

impl ExactSizeIterator for IndexedSegments {}

/// A type that, when displayed, shows the span contents, rather than location.
#[derive(Clone, Debug)]
pub struct SpanContent {
    /// The inner span of a source code.
    span: Span,
}

impl SpanContent {
    /// Returns the inner span.
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// Returns the span contents as a string.
    pub fn as_str(&self) -> &str {
        self.span.as_str()
    }

    /// Creates an iterator over located grapheme cluster segments, namely
    /// [`LocatedSegment`]s.
    pub fn segments(&self) -> Segments {
        self.clone().into_iter()
    }

    /// Creates an [`IndexedSegments`] iterator, which yields a tuple of
    /// position and a [`LocatedSegment`] in that position. Note that this is
    /// just for convenience with e.g. nom, [`LocatedSegment`] already contains
    /// its position, and so [`Segments`] can be used.
    pub fn indexed_segments(&self) -> IndexedSegments {
        self.segments().indexed()
    }
}

impl Deref for SpanContent {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SpanContent {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        write!(fmtr, "{}", &**self)
    }
}

impl PartialEq for SpanContent {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl<T> PartialEq<T> for SpanContent
where
    T: ?Sized,
    str: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        &**self == other
    }
}

impl Eq for SpanContent {}

impl PartialOrd for SpanContent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialOrd<T> for SpanContent
where
    T: ?Sized,
    str: PartialOrd<T>,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        (**self).partial_cmp(other)
    }
}

impl Ord for SpanContent {
    fn cmp(&self, other: &Self) -> Ordering {
        (**self).cmp(&**other)
    }
}

impl Hash for SpanContent {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (**self).hash(hasher)
    }
}

impl<T> AsRef<T> for SpanContent
where
    T: ?Sized,
    str: AsRef<T>,
{
    fn as_ref(&self) -> &T {
        (**self).as_ref()
    }
}

impl<T> Borrow<T> for SpanContent
where
    T: ?Sized,
    str: Borrow<T>,
{
    fn borrow(&self) -> &T {
        (**self).borrow()
    }
}

impl IntoIterator for SpanContent {
    type Item = LocatedSegment;
    type IntoIter = Segments;

    fn into_iter(self) -> Self::IntoIter {
        Segments { span: self.span }
    }
}
