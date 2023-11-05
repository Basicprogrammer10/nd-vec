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

#[derive(Clone)]
pub struct Vector<T, const N: usize> {
    components: [T; N],
}

pub macro vector($($x:expr),*) {
    Vector::new([$($x),*])
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(components: [T; N]) -> Self {
        Self { components }
    }

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
    fn default() -> Self {
        Self {
            components: [T::default(); N],
        }
    }
}

impl<T: Num + Copy, const N: usize> Vector<T, N> {
    pub fn hadamard_product(&self, other: &Self) -> Self {
        let mut components = [T::zero(); N];
        for (i, e) in components.iter_mut().enumerate() {
            *e = self.components[i] * other.components[i];
        }
        Self { components }
    }
}

impl<T: Num + Copy + Sum, const N: usize> Vector<T, N> {
    pub fn magnitude_squared(&self) -> T {
        self.components.into_iter().map(|x| x * x).sum()
    }

    pub fn dot(&self, other: &Self) -> T {
        self.components
            .iter()
            .zip(other.components.iter())
            .map(|(a, b)| *a * *b)
            .sum()
    }
}

impl<T: Num + Copy + Sum + Real, const N: usize> Vector<T, N> {
    pub fn magnitude(&self) -> T {
        self.magnitude_squared().sqrt()
    }

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
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut components = [T::zero(); N];
        for (i, item) in iter.into_iter().enumerate() {
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
