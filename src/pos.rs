use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{Add, Mul, Sub};

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

    #[inline(always)]
    pub fn row(&self) -> T {
        self.y
    }

    #[inline(always)]
    pub fn col(&self) -> T {
        self.x
    }
}

impl<T> Mul<T> for Pos<T>
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

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T, U> Add<Pos<U>> for Pos<T>
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
        + From<U>
        + TryInto<usize>,
    <T as TryInto<usize>>::Error: Debug,
    U: Debug
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
    <U as TryInto<usize>>::Error: Debug,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Pos<U>) -> Self::Output {
        let x: T = rhs.x.into();
        let y: T = rhs.y.into();
        Pos::new(self.x + x, self.y + y)
    }
}
