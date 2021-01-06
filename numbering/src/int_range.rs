use std::convert::TryInto;

use num_traits::{
    ops::checked::{CheckedAdd, CheckedSub},
    One, Zero,
};

use crate::{iter::IntRangeIter, OrderedSet};

/// The set of integers between `0` and `len-1`.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IntRange<Idx> {
    len: Idx,
}

impl<Idx> IntRange<Idx>
where
    Idx: Clone + TryInto<usize>,
{
    /// Creates a new `Range` on `{0,...,len-1}`.
    ///
    /// # Panics
    /// Panics if `len` cannot be converted into `usize`.
    ///
    /// # Time Complexity
    /// `O(1)`
    #[inline]
    pub fn new(len: Idx) -> Self {
        assert!(
            len.clone().try_into().is_ok(),
            "The argument `len` cannot be converted into `usize`."
        );
        IntRange { len }
    }
}

impl<Idx> OrderedSet for IntRange<Idx>
where
    Idx: CheckedAdd + CheckedSub + Clone + One + Ord + TryInto<usize> + Zero,
    usize: TryInto<Idx>,
{
    type Element = Idx;
    type Iterator = IntRangeIter<Idx>;

    /// Returns an iterator that enumerates the domain elements in the ascending order of numbering.
    ///
    /// # Time Complexity
    /// `O(1)`
    #[inline]
    fn iter(&self) -> IntRangeIter<Idx> {
        IntRangeIter::new(Idx::zero(), self.len.clone())
    }

    /// Returns the index of the specified element, or `None` if the domain does not contain it.
    ///
    /// # Time Complexity
    /// `O(1)`
    #[inline]
    fn index_of(&self, x: Idx) -> Option<usize> {
        Some(x)
            .filter(|x| Idx::zero() <= *x && *x < self.len)
            .map(|x| x.try_into().ok().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let range = IntRange::new(10);
        assert_eq!(range.index_of(-1), None);
        assert_eq!(range.index_of(0), Some(0));
        assert_eq!(range.index_of(3), Some(3));
        assert_eq!(range.index_of(9), Some(9));
        assert_eq!(range.index_of(10), None);
        assert_eq!(
            range.iter().collect::<Vec<_>>(),
            (0..10).collect::<Vec<_>>()
        );
        assert_eq!(range.len(), 10);
    }
}
