//! This module defines a tag type for segments. It also implements nom traits
//! in order to be used with [`Span`] and [`SpanContent`].

use crate::{
    span::{Span, SpanContent},
    LocatedSegment,
};
use nom::{
    bytes::complete::tag,
    error::ParseError,
    Compare,
    FindToken,
    InputIter,
    InputLength,
    InputTake,
    InputTakeAtPosition,
    Offset,
    Parser,
    Slice,
};
use std::{
    iter::{Enumerate},
    ops::RangeBounds,
    slice,
};

/// A type usable as tag for a parser without having to create a proper
/// [`Span`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag<'slice, 'seg>(
    /// A sequence of segment contents.
    pub &'slice [&'seg str],
);

impl<'slice, 'seg> Tag<'slice, 'seg> {
    /// Returns the length of the tag in segments/grapheme clusters.
    pub fn len(self) -> usize {
        self.0.len()
    }

    /// Converts this tag parsed into a function (also a parser).
    pub fn into_fn<T, E>(
        self,
    ) -> impl FnMut(T) -> nom::IResult<T, T, E> + 'slice + 'seg
    where
        'slice: 'seg,
        T: nom::InputTake + nom::Compare<Self>,
        E: ParseError<T>,
    {
        move |input| tag(self)(input)
    }

    /// Returns an iterator over the contents of segments of this tag.
    pub fn segments(self) -> SegmentContents<'slice, 'seg> {
        self.into_iter()
    }
}

impl<'slice, 'seg> IntoIterator for Tag<'slice, 'seg> {
    type Item = &'seg str;
    type IntoIter = SegmentContents<'slice, 'seg>;

    fn into_iter(self) -> Self::IntoIter {
        SegmentContents { inner: self.0.iter() }
    }
}

impl<'slice, 'seg, T, E> Parser<T, T, E> for Tag<'slice, 'seg>
where
    T: nom::InputTake + nom::Compare<Self>,
    E: ParseError<T>,
{
    fn parse(&mut self, input: T) -> nom::IResult<T, T, E> {
        tag(*self)(input)
    }
}

impl<'slice, 'seg, T> PartialEq<T> for Tag<'slice, 'seg>
where
    [&'seg str]: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        self.0 == other
    }
}

impl<'slice, 'seg> InputLength for Tag<'slice, 'seg> {
    fn input_len(&self) -> usize {
        self.len()
    }
}

impl<'slice, 'seg, R> Slice<R> for Tag<'slice, 'seg>
where
    R: RangeBounds<usize>,
{
    fn slice(&self, range: R) -> Self {
        Self(
            &self.0[(range.start_bound().cloned(), range.end_bound().cloned())],
        )
    }
}

impl<'slice, 'seg> InputIter for Tag<'slice, 'seg> {
    type Item = &'seg str;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = SegmentContents<'slice, 'seg>;

    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    fn iter_elements(&self) -> Self::IterElem {
        self.segments()
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.iter_elements().position(predicate)
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        if self.input_len() >= count {
            Ok(count)
        } else {
            Err(nom::Needed::new(count - self.input_len()))
        }
    }
}

impl<'slice, 'seg> Offset for Tag<'slice, 'seg> {
    fn offset(&self, second: &Self) -> usize {
        let this = self.0.as_ptr();
        let other = second.0.as_ptr();
        other as usize - this as usize
    }
}

impl<'slice0, 'seg0, 'slice1, 'seg1> Compare<Tag<'slice1, 'seg1>>
    for Tag<'slice0, 'seg0>
{
    fn compare(&self, input: Tag<'slice1, 'seg1>) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment != input_segment {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }

    fn compare_no_case(
        &self,
        input: Tag<'slice1, 'seg1>,
    ) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment.to_lowercase()
                        != input_segment.to_lowercase()
                    {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }
}

impl<'slice0, 'seg0, 'slice1, 'seg1, 'tag> Compare<&'tag Tag<'slice1, 'seg1>>
    for Tag<'slice0, 'seg0>
{
    fn compare(&self, input: &'tag Tag<'slice1, 'seg1>) -> nom::CompareResult {
        self.compare(*input)
    }

    fn compare_no_case(
        &self,
        input: &'tag Tag<'slice1, 'seg1>,
    ) -> nom::CompareResult {
        self.compare_no_case(*input)
    }
}

impl<'slice0, 'seg0, 'slice1, 'seg1, 'tag> Compare<Tag<'slice1, 'seg1>>
    for &'tag Tag<'slice0, 'seg0>
{
    fn compare(&self, input: Tag<'slice1, 'seg1>) -> nom::CompareResult {
        (**self).compare(input)
    }

    fn compare_no_case(
        &self,
        input: Tag<'slice1, 'seg1>,
    ) -> nom::CompareResult {
        (**self).compare_no_case(input)
    }
}

impl<'slice0, 'seg0, 'slice1, 'seg1, 'tag0, 'tag1>
    Compare<&'tag1 Tag<'slice1, 'seg1>> for &'tag0 Tag<'slice0, 'seg0>
{
    fn compare(&self, input: &'tag1 Tag<'slice1, 'seg1>) -> nom::CompareResult {
        (**self).compare(*input)
    }

    fn compare_no_case(
        &self,
        input: &'tag1 Tag<'slice1, 'seg1>,
    ) -> nom::CompareResult {
        (**self).compare_no_case(*input)
    }
}

impl<'slice, 'seg, 'span> Compare<&'span Span> for Tag<'slice, 'seg> {
    fn compare(&self, input: &'span Span) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment != input_segment.as_str() {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }

    fn compare_no_case(&self, input: &'span Span) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment.to_lowercase()
                        != input_segment.as_str().to_lowercase()
                    {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }
}

impl<'slice, 'seg, 'span> Compare<Span> for Tag<'slice, 'seg> {
    fn compare(&self, input: Span) -> nom::CompareResult {
        self.compare(&input)
    }

    fn compare_no_case(&self, input: Span) -> nom::CompareResult {
        self.compare_no_case(&input)
    }
}

impl<'slice, 'seg, 'span> Compare<&'span SpanContent> for Tag<'slice, 'seg> {
    fn compare(&self, input: &'span SpanContent) -> nom::CompareResult {
        self.compare(input.span())
    }

    fn compare_no_case(&self, input: &'span SpanContent) -> nom::CompareResult {
        self.compare_no_case(input.span())
    }
}

impl<'slice, 'seg, 'span> Compare<SpanContent> for Tag<'slice, 'seg> {
    fn compare(&self, input: SpanContent) -> nom::CompareResult {
        self.compare(&input)
    }

    fn compare_no_case(&self, input: SpanContent) -> nom::CompareResult {
        self.compare_no_case(&input)
    }
}

impl<'slice, 'seg> InputTake for Tag<'slice, 'seg> {
    fn take(&self, count: usize) -> Self {
        self.slice(count ..)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count ..), self.slice(.. count))
    }
}

impl<'slice, 'seg> InputTakeAtPosition for Tag<'slice, 'seg> {
    type Item = &'seg str;

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
            None => Ok(self.take_split(self.input_len())),
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
                if self.input_len() > 0 {
                    Ok(self.take_split(self.input_len()))
                } else {
                    Err(nom::Err::Error(E::from_error_kind(self.clone(), e)))
                }
            },
        }
    }
}

impl<'slice, 'seg, 'span> Compare<Tag<'slice, 'seg>> for &'span Span {
    fn compare(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment.as_str() != input_segment {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }

    fn compare_no_case(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        let mut this_iter = self.iter_elements();
        let mut input_iter = input.iter_elements();

        loop {
            match (this_iter.next(), input_iter.next()) {
                (Some(this_segment), Some(input_segment)) => {
                    if this_segment.as_str().to_lowercase()
                        != input_segment.to_lowercase()
                    {
                        break nom::CompareResult::Error;
                    }
                },
                (None, Some(_)) => break nom::CompareResult::Incomplete,
                (_, None) => break nom::CompareResult::Ok,
            }
        }
    }
}

impl<'slice, 'seg> Compare<Tag<'slice, 'seg>> for Span {
    fn compare(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        (&self).compare(input)
    }

    fn compare_no_case(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        (&self).compare_no_case(input)
    }
}

impl<'slice, 'seg, 'span> Compare<Tag<'slice, 'seg>> for &'span SpanContent {
    fn compare(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare(input)
    }

    fn compare_no_case(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare_no_case(input)
    }
}

impl<'slice, 'seg> Compare<Tag<'slice, 'seg>> for SpanContent {
    fn compare(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare(input)
    }

    fn compare_no_case(&self, input: Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare_no_case(input)
    }
}

impl<'slice, 'seg, 'tag, 'span> Compare<&'tag Tag<'slice, 'seg>>
    for &'span Span
{
    fn compare(&self, input: &'tag Tag<'slice, 'seg>) -> nom::CompareResult {
        self.compare(*input)
    }

    fn compare_no_case(
        &self,
        input: &'tag Tag<'slice, 'seg>,
    ) -> nom::CompareResult {
        self.compare_no_case(*input)
    }
}

impl<'slice, 'seg, 'tag> Compare<&'tag Tag<'slice, 'seg>> for Span {
    fn compare(&self, input: &'tag Tag<'slice, 'seg>) -> nom::CompareResult {
        (&self).compare(input)
    }

    fn compare_no_case(
        &self,
        input: &'tag Tag<'slice, 'seg>,
    ) -> nom::CompareResult {
        (&self).compare_no_case(input)
    }
}

impl<'slice, 'seg, 'tag, 'span> Compare<&'tag Tag<'slice, 'seg>>
    for &'span SpanContent
{
    fn compare(&self, input: &'tag Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare(input)
    }

    fn compare_no_case(
        &self,
        input: &'tag Tag<'slice, 'seg>,
    ) -> nom::CompareResult {
        self.span().compare_no_case(input)
    }
}

impl<'slice, 'seg, 'tag> Compare<&'tag Tag<'slice, 'seg>> for SpanContent {
    fn compare(&self, input: &'tag Tag<'slice, 'seg>) -> nom::CompareResult {
        self.span().compare(input)
    }

    fn compare_no_case(
        &self,
        input: &'tag Tag<'slice, 'seg>,
    ) -> nom::CompareResult {
        self.span().compare_no_case(input)
    }
}

impl<'slice, 'seg, 'tok> FindToken<&'tok str> for Tag<'slice, 'seg> {
    fn find_token(&self, token: &'tok str) -> bool {
        self.0.iter().any(|segment| *segment == token)
    }
}

impl<'slice, 'seg, 'tag, 'tok> FindToken<&'tok str>
    for &'tag Tag<'slice, 'seg>
{
    fn find_token(&self, token: &'tok str) -> bool {
        (**self).find_token(token)
    }
}

impl<'slice, 'seg, 'tok> FindToken<&'tok LocatedSegment> for Tag<'slice, 'seg> {
    fn find_token(&self, token: &'tok LocatedSegment) -> bool {
        self.find_token(token.as_str())
    }
}

impl<'slice, 'seg, 'tag, 'tok> FindToken<&'tok LocatedSegment>
    for &'tag Tag<'slice, 'seg>
{
    fn find_token(&self, token: &'tok LocatedSegment) -> bool {
        (**self).find_token(token)
    }
}

impl<'slice, 'seg, 'tok> FindToken<LocatedSegment> for Tag<'slice, 'seg> {
    fn find_token(&self, token: LocatedSegment) -> bool {
        self.find_token(token.as_str())
    }
}

impl<'slice, 'seg, 'tag, 'tok> FindToken<LocatedSegment>
    for &'tag Tag<'slice, 'seg>
{
    fn find_token(&self, token: LocatedSegment) -> bool {
        (**self).find_token(token)
    }
}

/// Iterator over segment contents of a [`Tag`]. See [`Tag::segments`].
#[derive(Debug, Clone)]
pub struct SegmentContents<'slice, 'seg> {
    inner: slice::Iter<'slice, &'seg str>,
}

impl<'slice, 'seg> Iterator for SegmentContents<'slice, 'seg> {
    type Item = &'seg str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().copied()
    }
}
