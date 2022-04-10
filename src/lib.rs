#![feature(test)]

/// The DSL (Domain Specific Language) module defines interactions with the library in a easier-to-read way for the developer.
pub mod dsl;
/// Holds the core infrastructure of the library.
pub mod model;

/// Test functions
#[cfg(test)]
mod tests;
/// Benchmark functions
#[cfg(test)]
mod benches;