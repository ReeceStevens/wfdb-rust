//! `wfdb-rust` is a basic library for parsing WFDB-format datasets from PhysioNet. At this time,
//! it does not intend to be a complete WFDB library; instead, it focuses strictly on reading WFDB
//! datasets (not writing them).
//!
//! The motivation for this library was to find an easier way to parse datasets from PhysioNet in
//! other Rust projects.
extern crate regex;

pub mod header;
pub mod signal;
