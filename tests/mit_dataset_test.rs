/// Tests using the MIT ECG database samples.
///
/// Note: in order to run these integration tests, the datasets referenced here must be downloaded
/// and put in the data directory. The datasets used for testing here were downloaded from
/// PhysioNet and can be found at https://physionet.org/content/mitdb/1.0.0/.
extern crate glob;

use wfdb_rust::parse_wfdb;

fn get_signal_checksum(signal: &[i16]) -> i16 {
    let mut checksum = 0_i16;
    for val in signal {
        checksum = checksum.wrapping_add(*val);
    }
    checksum
}

/// Tests that an example dataset from the MIT Arrhythmia database can be loaded and parsed without
/// errors.
#[test]
fn parse_mit_dataset() {
    let mit_header_files = glob::glob("data/mit-bih-arrhythmia-database-1.0.0/*.hea").expect("Failed to read glob");
    for header_file in mit_header_files {
        if let Ok(path) = header_file {
            println!("Reading {:?}", path);
            let (header, signals) = parse_wfdb(&path);
            for i in 0..signals.len() {
                assert_eq!(header.signal_specs[i].checksum, Some(get_signal_checksum(&signals[i])));
            }
        }
    }

}
