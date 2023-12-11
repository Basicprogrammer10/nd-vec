# nd-vec Changelog

## v0.4.0 &ndash; December 11, 2023

- Add component casting APIs
  - `num_cast` &mdash; Numeric casts like with the `as` keyword
  - `cast` &mdash; Casts with the Into trait
  - `try_cast` &mdash; Casts with the TryInto trait
- Reduce type requirements for Vector::signum from Floats to all Signed types
- Add Vector::distance to calculate the Euclidean Distance between two points
- Add Vector::manhattan_distance to calculate the Manhattan Distance between points
- Add Vector::sum to sum the values of all components
- Add Vector::opposite to create a new vector with all components negated

## v0.3.0 &ndash; November 26, 2023

- Impl assigning operations
  - Add
  - Subtract
  - Divide
  - Remainder
- Allow getting vector as a component slice
- Absolute value function
- Allow accessing components on 2D and 3D vectors with `.x()`, `.y()`, and `.x()` if applicable
- Add the `documentation` attribute to the Cargo.toml

## v0.2.0 &ndash; November 26, 2023

- Cleanup type aliases
- Improved vector macro
- Internal code cleanup

## v0.1.0 &ndash; November 26, 2023

First release of nd-vec.
Moved from my [n-dimensional audio engine](https://github.com/Basicprogrammer10/audio_engine) project.
