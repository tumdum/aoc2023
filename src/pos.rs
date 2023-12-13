use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Pos<T: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash> {
    pub(crate) x: T,
    pub(crate) y: T,
}

impl<T> Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    pub fn manhattan_dist(&self, other: &Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    #[inline(always)]
    pub fn idx_1d(&self, w: T) -> usize {
        (self.y * w + self.x).try_into().unwrap()
    }
}

impl<T> Add for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    #[inline(always)]
    const fn add(self, rhs: Self) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl<T> Add<&Pos<T>> for Pos<T>
where
    T: Debug
        + Clone
        + Copy
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + Hash
        + Sub
        + num::Signed
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: &Pos<T>) -> Self::Output {
        Pos::new(self.x + rhs.x, self.y + rhs.y)
    }
}
