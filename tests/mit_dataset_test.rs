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

/// Tests that an example dataset from the MIT Arrhythmia database can be loaded and parsed without
/// errors.
#[test]
fn can_parse_mit_dataset() {
    let header_file = read_to_string("data/mit-bih-arrhythmia-database-1.0.0/100.hea").unwrap();
    let data_file = read("data/mit-bih-arrhythmia-database-1.0.0/100.dat").unwrap();
    println!("{:?}", read_header(&header_file));
    println!("{:?}", parse_212_format(&data_file));
}
