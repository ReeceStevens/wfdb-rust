/// Tests using the MIT ECG database samples.
///
/// Note: in order to run these integration tests, the datasets referenced here must be downloaded
/// and put in the data directory. The datasets used for testing here were downloaded from
/// PhysioNet and can be found at https://physionet.org/content/mitdb/1.0.0/.
use std::fs::{read_to_string, read};
use std::path::Path;
use std::io::Read;

use wfdb_rust::header::read_header;
use wfdb_rust::signal::parse_212_format;
use wfdb_rust::parse_wfdb;

/// Tests that an example dataset from the MIT Arrhythmia database can be loaded and parsed without
/// errors.
#[test]
fn can_parse_mit_dataset() {
    let (header, signals) = parse_wfdb("data/mit-bih-arrhythmia-database-1.0.0/100.hea");
    println!("{:?}", header);
    println!("{:?}", signals);
}
