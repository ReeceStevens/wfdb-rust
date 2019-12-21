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
pub fn parse_wfdb(header_path: &str) -> (header::Header, Vec<Vec<i16>>) {
    let header_path = Path::new(header_path);
    let header = header::read_header(&read_to_string(header_path).unwrap());
    let mut signals = vec![];
    let data_directory = header_path.parent().unwrap();
    let mut parsed_files = HashMap::new();
    for signal in &header.signal_specs {
        let signal_path = PathBuf::from(&signal.filename);
        if parsed_files.get(&signal_path) == None {
            let buf = if (&signal_path).is_absolute() {
                read(&signal_path).unwrap()
            } else {
                let full_path = data_directory.join(&signal_path);
                read(&full_path).unwrap()
            };
            parsed_files.insert(signal_path, buf);
        }

        let signal_path = PathBuf::from(&signal.filename);
        parsed_files.entry(signal_path).and_modify(|buf| {
            if let Some(num_samples) = header.record.samples_per_signal {
                let num_bytes = num_samples as f32 * 1.5;
                signals.push(signal::parse_212_format(&buf[..num_bytes as usize]));
                *buf = buf[num_bytes as usize..].to_vec();
            }
        });
    }
    (header, signals)
}
