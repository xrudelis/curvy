#![feature(backtrace)]

#[macro_use]
extern crate more_asserts;

#[macro_use]
extern crate approx;

pub mod geometry;

#[cfg(test)]
mod tests;

pub mod to_svg;
