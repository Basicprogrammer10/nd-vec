#![doc = include_str!("../README.md")]

#[cfg(test)]
mod test;
mod vector;
pub use vector::Vector;

/// Alias for a 2-dimensional vector with the given type.
pub type Vec2<T> = Vector<T, 2>;
/// Alias for a 3-dimensional vector with the given type.
pub type Vec3<T> = Vector<T, 3>;
