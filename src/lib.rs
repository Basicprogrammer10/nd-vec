#![feature(decl_macro)]

use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{real::Real, Num};

#[cfg(test)]
mod test;

// == 2D Vector ==
pub type Vector2f32 = Vector<f32, 2>;
pub type Vector2f64 = Vector<f64, 2>;
// == 3D Vector ==
pub type Vector3f32 = Vector<f32, 3>;
pub type Vector3f64 = Vector<f64, 3>;

/// A compile-time n-dimensional vector, how fancy!
#[derive(Clone)]
pub struct Vector<T, const N: usize> {
    components: [T; N],
}

/// Create a new vector with the given components.
/// ```rust
/// # use vector::vector;
/// vector!(1, 2, 3);
/// ```
pub macro vector($($x:expr),*) {
    Vector::new([$($x),*])
}

impl<T, const N: usize> Vector<T, N> {
    /// Create a new vector with the given components.
    /// ```rust
    /// # use vector::Vector;
    /// Vector::new([1, 2, 3]);
    /// ```
    pub fn new(components: [T; N]) -> Self {
        Self { components }
    }

    /// Create a new vector with zeroed components.
    pub fn zero() -> Self
    where
        T: Num + Copy,
    {
        Self {
            components: [T::zero(); N],
        }
    }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
    /// Create a new vector with zeroed components.
    fn default() -> Self {
        Self {
            components: [T::default(); N],
        }
    }
}

impl<T: Num + Copy, const N: usize> Vector<T, N> {
    /// Computes the Hadamard product of two vectors (component-wise multiplication).
    pub fn hadamard_product(&self, other: &Self) -> Self {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i] * other.components[i];
        }
        Self { components }
    }
}

impl<T: Num + Copy + Ord, const N: usize> Vector<T, N> {
    /// Takes the minimum of each component of two vectors.
    pub fn min(&self, other: &Self) -> Self {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i].min(other.components[i]);
        }
        Self { components }
    }

    /// Takes the maximum of each component of two vectors.
    pub fn max(&self, other: &Self) -> Self {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i].max(other.components[i]);
        }
        Self { components }
    }

    /// Takes the minimum component of a vector.
    pub fn min_component(&self) -> T {
        self.components.iter().min().copied().unwrap()
    }

    /// Takes the maximum component of a vector.
    pub fn max_component(&self) -> T {
        self.components.iter().max().copied().unwrap()
    }
}

impl<T: Num + Copy + Real, const N: usize> Vector<T, N> {
    /// Calculates the sign of each component of a vector.
    /// This is -1 if the component is negative, 0 if it is zero, and 1 if it is positive.
    pub fn signum(&self) -> Self {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i].signum();
        }
        Self { components }
    }
}

impl<T: Num + Copy + Sum, const N: usize> Vector<T, N> {
    /// Calculates the sum of all squared components.
    /// Used for calculating the magnitude of a vector.
    pub fn magnitude_squared(&self) -> T {
        self.components.into_iter().map(|x| x * x).sum()
    }

    /// Calculates the dot product of two vectors.
    pub fn dot(&self, other: &Self) -> T {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}

impl<T: Num + Copy + Sum + Real, const N: usize> Vector<T, N> {
    /// Calculates the magnitude of a vector.
    /// This is the square root of the sum of all squared components.
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

    /// Normalizes a vector.
    /// This is the vector divided by its magnitude.
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl<T: Num + Copy + Display, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let components = self.components.map(|x| x.to_string()).join(", ");
        f.write_fmt(format_args!("({})", components))
    }
}

impl<T: Num + Copy, const N: usize> Copy for Vector<T, N> {}
impl<T: Num + Copy, const N: usize> Eq for Vector<T, N> {}

impl<T: Num + Copy, const N: usize> FromIterator<T> for Vector<T, N> {
    /// Create a new vector from an iterator.
    /// If the iterator has less than N items, the remaining components will be zeroed.
    /// If the iterator has more than N items, the remaining items will be ignored.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut components = [T::zero(); N];
        for (i, item) in iter.into_iter().take(N).enumerate() {
            components[i] = item;
        }
        Self { components }
    }
}

macro bin_op($trait:tt, $func:ident) {
    impl<T: Num + Copy, const N: usize> $trait for Vector<T, N> {
        type Output = Self;

        fn $func(self, other: Self) -> Self::Output {
            let mut components = [T::zero(); N];
            for (i, e) in components.iter_mut().enumerate() {
                *e = self.components[i].$func(other.components[i]);
            }
            Self { components }
        }
    }

    impl<T: Num + Copy, const N: usize> $trait<T> for Vector<T, N> {
        type Output = Self;

        fn $func(self, other: T) -> Self::Output {
            let mut components = [T::zero(); N];
            for (i, e) in components.iter_mut().enumerate() {
                *e = self.components[i].$func(other);
            }
            Self { components }
        }
    }
}

bin_op!(Add, add);
bin_op!(Sub, sub);
bin_op!(Div, div);
bin_op!(Rem, rem);

impl<T: Num + Copy, const N: usize> Neg for Vector<T, N> {
    type Output = Self;

    /// Negates all components of a vector.
    fn neg(self) -> Self::Output {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = T::zero() - self.components[i];
        }
        Self { components }
    }
}

impl<T: Num + Copy, const N: usize> PartialEq for Vector<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.components
            .iter()
            .zip(other.components.iter())
            .all(|(a, b)| a == b)
    }
}

impl<T: Num + Copy + Send + Sync, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i] * rhs;
        }
        Self { components }
    }
}
