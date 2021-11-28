mod indexing;

use crate::{location::Location, span::Span};
pub use indexing::SourceIndex;
use indexing::{IndexArray, IndexArrayBuilder, IndexArrayIter};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    ops::Index,
    rc::Rc,
};
use unicode_segmentation::UnicodeSegmentation;

/// Inner structure of a source.
#[derive(Debug)]
struct SourceInner {
    /// File name.
    name: Box<str>,
    /// Contents of the source.
    contents: Box<str>,
    /// List of string segmentation in the source.
    segments: IndexArray,
    /// List of newlines in the source.
    newlines: IndexArray,
}

/// A source code object, such as read from a file.
#[derive(Debug, Clone)]
pub struct Source {
    /// The inner structure containing the actual data.
    inner: Rc<SourceInner>,
}

impl Source {
    /// Creates a new source code object given its name and its contentss.
    pub fn new<S0, S1>(name: S0, contents: S1) -> Self
    where
        S0: Into<Box<str>>,
        S1: Into<Box<str>>,
    {
        let name = name.into();
        let contents = contents.into();
        let mut segments = IndexArrayBuilder::new();
        let mut newlines = IndexArrayBuilder::new();

        for (idx, grapheme) in contents.grapheme_indices(true) {
            if grapheme == "\n" {
                newlines.push(segments.len());
            }
            segments.push(idx);
        }
        segments.push(contents.len());

        let segments = segments.into();
        let newlines = newlines.into();
        let inner = SourceInner { name, contents, segments, newlines };
        Self { inner: Rc::new(inner) }
    }

    /// The (file) name of the source.
    pub fn name(&self) -> &str {
        &self.inner.name
    }

    /// The length the source.
    pub fn len(&self) -> usize {
        self.inner.segments.len() - 1
    }

    /// The contentss of the source.
    pub fn contents(&self) -> &str {
        &self.inner.contents
    }

    /// Iterator over the segments of the source.
    pub fn segments(&self) -> SegmentsIter {
        SegmentsIter { inner: self.inner.segments.iter() }
    }

    /// Iterator over the newlines of the source.
    pub fn newlines(&self) -> NewlinesIter {
        NewlinesIter { inner: self.inner.segments.iter() }
    }

    /// Returns the line number where the given position is contained, starting
    /// from `0`.
    pub(super) fn line(&self, position: usize) -> usize {
        match self.inner.newlines.binary_search(position) {
            Ok(n) | Err(n) => n,
        }
    }

    /// Returns the position of the given line number's start. Line number
    /// begins at `0`.
    ///
    /// # Panics
    /// Pancis if the given line does not exist.
    pub(super) fn line_start(&self, line: usize) -> usize {
        if line == 0 {
            0
        } else {
            self.inner.newlines.index(line - 1) + 1
        }
    }

    /// Returns the position of the given line number's start. Line number
    /// begins at `0`, returning `None` on invalid line number.
    pub(super) fn try_line_start(&self, line: usize) -> Option<usize> {
        if line == 0 {
            Some(0)
        } else {
            self.inner.newlines.get(line - 1).map(|position| position + 1)
        }
    }

    /// Indexes this source. It can be a single `usize` or a range of `usize`.
    pub fn get<I>(&self, indexer: I) -> Option<&I::Output>
    where
        I: SourceIndex,
    {
        indexer.get(self)
    }

    /// Returns a span covering the whole source code.
    pub fn full_span(&self) -> Span {
        let start = Location::new_unchecked(self.clone(), 0);
        Span::new_unchecked(start, self.len())
    }
}

impl<I> Index<I> for Source
where
    I: SourceIndex,
{
    type Output = I::Output;

    fn index(&self, indexer: I) -> &Self::Output {
        indexer.index(self)
    }
}

impl PartialEq for Source {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for Source {}

impl PartialOrd for Source {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Source {
    fn cmp(&self, other: &Self) -> Ordering {
        (&*self.inner as *const SourceInner).cmp(&(&*other.inner as *const _))
    }
}

impl Hash for Source {
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        (&*self.inner as *const SourceInner).hash(hasher)
    }
}

impl fmt::Display for Source {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        fmtr.write_str(self.name())
    }
}

/// Iterator over the segments of a source. Double-ended and sized.
#[derive(Debug)]
pub struct SegmentsIter<'src> {
    /// The inner iterator over the indices.
    inner: IndexArrayIter<'src>,
}

impl<'src> Iterator for SegmentsIter<'src> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<'src> DoubleEndedIterator for SegmentsIter<'src> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'array> ExactSizeIterator for SegmentsIter<'array> {}

/// Iterator over the newlines of a source. Double-ended and sized.
#[derive(Debug)]
pub struct NewlinesIter<'src> {
    /// The inner iterator over the indices.
    inner: IndexArrayIter<'src>,
}

impl<'src> Iterator for NewlinesIter<'src> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.inner.len();
        (len, Some(len))
    }
}

impl<'src> DoubleEndedIterator for NewlinesIter<'src> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<'array> ExactSizeIterator for NewlinesIter<'array> {}
