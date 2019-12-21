//! `wfdb-rust` is a basic library for parsing WFDB-format datasets from PhysioNet. At this time,
//! it does not intend to be a complete WFDB library; instead, it focuses strictly on reading WFDB
//! datasets (not writing them).
//!
//! The motivation for this library was to find an easier way to parse datasets from PhysioNet in
//! other Rust projects.
extern crate regex;

use std::fs::{read_to_string, read};
use std::path::{Path, PathBuf};
use std::collections::HashMap;

pub mod header;
pub mod signal;

/// Parse the WFDB signals based on information in the header. This returns the parsed header as
/// well as a Vec<i16> for each signal listed in the header.
pub fn parse_wfdb(header_path: &Path) -> (header::Header, Vec<Vec<i16>>) {
    let header = header::read_header(&read_to_string(header_path).unwrap());
    let mut signals = vec![];
    let data_directory = header_path.parent().unwrap();
    let mut parsed_files = HashMap::new();
    let mut signals_in_file = HashMap::new();
    let mut signal_index_in_file = HashMap::new();

    for signal in &header.signal_specs {
        signals_in_file.entry(&signal.filename).and_modify(|v| *v += 1).or_insert(1);
    }

    for signal in &header.signal_specs {
        let buf = parsed_files.entry(&signal.filename).or_insert_with(|| {
            let signal_path = PathBuf::from(&signal.filename);
            if (&signal_path).is_absolute() {
                read(&signal_path).unwrap()
            } else {
                let full_path = data_directory.join(&signal_path);
                read(&full_path).unwrap()
            }
        });
        let signal_idx = signal_index_in_file.entry(&signal.filename).and_modify(|v| *v += 1).or_insert(0);
        let num_signals_in_file = signals_in_file.entry(&signal.filename).or_insert(1);

        let new_signal: Vec<i16> = signal::parse_212_format(&buf).into_iter().skip(*signal_idx).step_by(*num_signals_in_file).collect();
        signals.push(new_signal);
    }
    (header, signals)
}
