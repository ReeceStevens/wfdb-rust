use regex::Regex;

use std::fs::File;
use std::io::Read;

const DEFREQ: f32 = 250_f32;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StorageFormat {
    _8bit_first_difference = 8,
    _16bit_twos_complement = 16,
    _24bit_twos_complement_lsb = 24,
    _32bit_twos_complement_lsb = 32,
    _16bit_twos_complement_msb = 61,
    _8bit_offset_binary = 80,
    _16bit_offset_binary = 160,
    _12bit_twos_complement = 212,
    _10bit_twos_complement_sets_of_11 = 310,
    _10bit_twos_complement_sets_of_4 = 311,
}

#[derive(PartialEq, Debug)]
pub struct RecordLine {
    record_name: String,
    number_of_segments: Option<u32>,
    number_of_signals: u32,
    sampling_frequency: Option<f32>,
    counter_frequency: Option<f32>,
    base_counter_value: Option<f32>,
    samples_per_signal: Option<u32>,
    base_time: Option<String>,
    base_date: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct SignalSpecLine {
    filename: String,
    format: StorageFormat,
    samples_per_frame: Option<u32>,
    skew: Option<u32>,
    byte_offset: Option<u32>,
    adc_gain: Option<f32>,
    baseline: Option<u32>,
    units: Option<String>,
    adc_resolution: Option<u32>,
    adc_zero: Option<u32>,
    initial_value: Option<u32>,
    checksum: Option<i32>,
    block_size: Option<u32>,
    description: Option<String>,
}

#[derive(PartialEq, Debug)]
pub struct Header {
    record: RecordLine,
    signal_specs: Vec<SignalSpecLine>,
}

pub fn parse_record_line(record_line: &str) -> RecordLine {
    let tokens: Vec<&str> = record_line.split_whitespace().collect();
    let record_name;
    let mut number_of_segments = None;

    let record_name_tokens: Vec<&str> = tokens[0].split('/').collect();
    if record_name_tokens.len() > 1 {
        record_name = String::from(record_name_tokens[0]);
        number_of_segments = record_name_tokens[1].parse::<u32>().ok();
    } else {
        record_name = String::from(record_name_tokens[0]);
    }

    let number_of_signals = tokens[1]
        .parse::<u32>()
        .expect("Invalid header: cannot parse number of signals.");

    let mut sampling_frequency = Some(DEFREQ);
    let mut counter_frequency = None;
    let mut base_counter_value = None;
    if tokens.len() > 2 {
        let frequency_tokens: Vec<&str> = tokens[2].split('/').collect();
        if frequency_tokens.len() > 1 {
            let counter_regex = Regex::new(r"(\d+)\((\d+)\)").unwrap();
            if counter_regex.is_match(frequency_tokens[1]) {
                let captures = counter_regex.captures(frequency_tokens[1]).unwrap();
                counter_frequency = Some(
                    captures[1]
                        .parse::<f32>()
                        .expect("Invalid header: counter frequency specified, but not parseable."),
                );
                base_counter_value = Some(
                    captures[2]
                        .parse::<f32>()
                        .expect("Invalid header: base counter value specified, but not parseable"),
                );
            } else {
                counter_frequency = Some(
                    frequency_tokens[0]
                        .parse::<f32>()
                        .expect("Invalid header: counter frequency specified, but not parseable."),
                );
            }
        }
        sampling_frequency = Some(
            frequency_tokens[0]
                .parse::<f32>()
                .expect("Invalid header: Sampling frequency field present, but not parseable"),
        );
    }

    let mut samples_per_signal = None;
    if tokens.len() > 3 {
        samples_per_signal = Some(
            tokens[3]
                .parse::<u32>()
                .expect("Invalid header: samples per signal specified, but not parseable"),
        );
    }

    // TODO: parse date and time

    RecordLine {
        record_name,
        number_of_segments,
        number_of_signals,
        sampling_frequency,
        counter_frequency: if counter_frequency == None {
            sampling_frequency
        } else {
            counter_frequency
        },
        base_counter_value: if base_counter_value == None {
            Some(0_f32)
        } else {
            base_counter_value
        },
        samples_per_signal,
        base_time: None,
        base_date: None,
    }
}

pub fn parse_signal_line(signal_line: &str) -> SignalSpecLine {
    let tokens: Vec<&str> = signal_line.split_whitespace().collect();
    if tokens.len() < 2 {
        panic!("Invalid header: signal specification line missing required fields.");
    }

    let filename = String::from(tokens[0]);
    let format;
    let mut samples_per_frame = None;
    let mut skew = None;
    let mut byte_offset = None;
    let format_regex = Regex::new(r"(\d+)(?:x(\d+)(?::(\d+)(?:\+(\d+)))?)?").unwrap();
    if format_regex.is_match(tokens[1]) {
        let format_captures = format_regex.captures(tokens[1]).unwrap();
        format = match &format_captures[1] {
            "8" => StorageFormat::_8bit_first_difference,
            "16" => StorageFormat::_16bit_twos_complement,
            "24" => StorageFormat::_24bit_twos_complement_lsb,
            "32" => StorageFormat::_32bit_twos_complement_lsb,
            "61" => StorageFormat::_16bit_twos_complement_msb,
            "80" => StorageFormat::_8bit_offset_binary,
            "160" => StorageFormat::_16bit_offset_binary,
            "212" => StorageFormat::_12bit_twos_complement,
            "310" => StorageFormat::_10bit_twos_complement_sets_of_11,
            "311" => StorageFormat::_10bit_twos_complement_sets_of_4,
            _ => panic!("Unknown storage format!"),
        };
        if format_captures.get(2) != None {
            samples_per_frame =
                Some(format_captures[2].parse::<u32>().expect(
                    "Invalid header: samples per frame specified, but could not be parsed",
                ));
        }
        if format_captures.get(3) != None {
            skew = Some(
                format_captures[3]
                    .parse::<u32>()
                    .expect("Invalid header: skew specified, but could not be parsed"),
            );
        }
        if format_captures.get(4) != None {
            byte_offset = Some(
                format_captures[4]
                    .parse::<u32>()
                    .expect("Invalid header: byte offset specified, but could not be parsed"),
            );
        }
    } else {
        panic!("Invalid header: signal format not properly specified.");
    }

    let mut adc_gain = None;
    let mut baseline = None;
    let mut units = Some(String::from("mV"));
    //  TODO: default adc_resolution value depends on storage format.
    //  This is the default for amplitude-format signals (most common)
    let mut adc_resolution = Some(12);
    let mut adc_zero = Some(0);
    let mut initial_value = None;
    let mut checksum = None;
    let mut block_size = None;
    let mut description = None;
    if tokens.len() > 2 {
        let adc_regex = Regex::new(r"(\d+)(?:\((?P<baseline>\d+)\))?(?:/(?P<units>\S+))?").unwrap();
        let adc_tokens = adc_regex.captures(tokens[2]).unwrap();
        adc_gain = Some(
            adc_tokens[1]
                .parse::<f32>()
                .expect("Invalid header: adc gain specified, but not parseable"),
        );
        if adc_tokens.name("baseline") != None {
            baseline = Some(
                adc_tokens
                    .name("baseline")
                    .unwrap()
                    .as_str()
                    .parse::<u32>()
                    .expect("Invalid header: baseline specified, but could not be parsed"),
            );
        }
        if adc_tokens.name("units") != None {
            units = Some(String::from(adc_tokens.name("units").unwrap().as_str()));
        }

        if tokens.len() > 3 {
            adc_resolution = Some(
                tokens[3]
                    .parse::<u32>()
                    .expect("Invalid header: ADC resolution specified but not parseable."),
            );
        }

        if tokens.len() > 4 {
            adc_zero = Some(
                tokens[4]
                    .parse::<u32>()
                    .expect("Invalid header: ADC zero specified but not parseable."),
            );
        }

        if tokens.len() > 5 {
            initial_value = Some(
                tokens[5]
                    .parse::<u32>()
                    .expect("Invalid header: initial value specified but not parseable."),
            );
        }

        if tokens.len() > 6 {
            checksum = Some(
                tokens[6]
                    .parse::<i32>()
                    .expect("Invalid header: checksum specified but not parseable."),
            );
        }

        if tokens.len() > 7 {
            block_size = Some(
                tokens[7]
                    .parse::<u32>()
                    .expect("Invalid header: block size specified but not parseable."),
            );
        }

        if tokens.len() > 8 {
            description = Some(String::from(tokens[8]));
        }
    }

    SignalSpecLine {
        filename,
        format,
        samples_per_frame,
        skew,
        byte_offset,
        adc_gain,
        baseline: if baseline == None { adc_zero } else { baseline },
        units,
        adc_resolution,
        adc_zero,
        initial_value: if initial_value == None {
            adc_zero
        } else {
            initial_value
        },
        checksum,
        block_size,
        description,
    }
}

pub fn read_header(header_string: &str) -> Header {
    let mut header_lines = header_string.lines().filter(|&line| !line.starts_with("#"));
    Header {
        record: parse_record_line(header_lines.next().unwrap()),
        signal_specs: header_lines.map(parse_signal_line).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_header() {
        let basic_header = "100 2 360 650000
            100.dat 212 200 11 1024 995 -22131 0 MLII
            100.dat 212 200 11 1024 1011 20052 0 V5";
        let parsed = read_header(basic_header);
        assert_eq!(
            parsed,
            Header {
                record: RecordLine {
                    record_name: String::from("100"),
                    number_of_segments: None,
                    number_of_signals: 2,
                    sampling_frequency: Some(360_f32),
                    counter_frequency: Some(360_f32),
                    base_counter_value: Some(0.0),
                    samples_per_signal: Some(650000),
                    base_time: None,
                    base_date: None,
                },
                signal_specs: vec![
                    SignalSpecLine {
                        filename: String::from("100.dat"),
                        format: StorageFormat::_12bit_twos_complement,
                        samples_per_frame: None,
                        skew: None,
                        byte_offset: None,
                        adc_gain: Some(200_f32),
                        baseline: Some(1024),
                        units: Some(String::from("mV")),
                        adc_resolution: Some(11),
                        adc_zero: Some(1024),
                        initial_value: Some(995),
                        checksum: Some(-22131),
                        block_size: Some(0),
                        description: Some(String::from("MLII")),
                    },
                    SignalSpecLine {
                        filename: String::from("100.dat"),
                        format: StorageFormat::_12bit_twos_complement,
                        samples_per_frame: None,
                        skew: None,
                        byte_offset: None,
                        adc_gain: Some(200_f32),
                        baseline: Some(1024),
                        units: Some(String::from("mV")),
                        adc_resolution: Some(11),
                        adc_zero: Some(1024),
                        initial_value: Some(1011),
                        checksum: Some(20052),
                        block_size: Some(0),
                        description: Some(String::from("V5")),
                    },
                ],
            }
        )
    }

    #[test]
    fn test_mit_record_line() {
        let basic_record_line = "100 2 360 650000 0:0:0 0/0/0";
        let parsed = parse_record_line(basic_record_line);
        assert_eq!(
            parsed,
            RecordLine {
                record_name: String::from("100"),
                number_of_segments: None,
                number_of_signals: 2,
                sampling_frequency: Some(360_f32),
                counter_frequency: Some(360_f32),
                base_counter_value: Some(0.0),
                samples_per_signal: Some(650000),
                base_time: None,
                base_date: None,
            }
        )
    }

    #[test]
    fn test_custom_mit_record_line() {
        let basic_record_line = "100/4 2 360/24(5) 650000 0:0:0 0/0/0";
        let parsed = parse_record_line(basic_record_line);
        assert_eq!(
            parsed,
            RecordLine {
                record_name: String::from("100"),
                number_of_segments: Some(4),
                number_of_signals: 2,
                sampling_frequency: Some(360.0),
                counter_frequency: Some(24.0),
                base_counter_value: Some(5.0),
                samples_per_signal: Some(650000),
                base_time: None,
                base_date: None,
            }
        )
    }

    #[test]
    fn test_aha_record_line() {
        let basic_record_line = "7001 2 250 525000";
        let parsed = parse_record_line(basic_record_line);
        assert_eq!(
            parsed,
            RecordLine {
                record_name: String::from("7001"),
                number_of_segments: None,
                number_of_signals: 2,
                sampling_frequency: Some(250_f32),
                counter_frequency: Some(250_f32),
                base_counter_value: Some(0.0),
                samples_per_signal: Some(525000),
                base_time: None,
                base_date: None,
            }
        )
    }

    #[test]
    fn test_mit_signal_spec_line() {
        let signal_line = "100.dat 212 200 11 1024 995 -22131 0 MLII";
        assert_eq!(
            parse_signal_line(signal_line),
            SignalSpecLine {
                filename: String::from("100.dat"),
                format: StorageFormat::_12bit_twos_complement,
                samples_per_frame: None,
                skew: None,
                byte_offset: None,
                adc_gain: Some(200_f32),
                baseline: Some(1024),
                units: Some(String::from("mV")),
                adc_resolution: Some(11),
                adc_zero: Some(1024),
                initial_value: Some(995),
                checksum: Some(-22131),
                block_size: Some(0),
                description: Some(String::from("MLII")),
            }
        )
    }

    #[test]
    fn test_custom_mit_signal_spec_line() {
        let signal_line = "100.dat 212x3:2+53 200(2)/cm 11 1024 995 -22131 0 MLII";
        assert_eq!(
            parse_signal_line(signal_line),
            SignalSpecLine {
                filename: String::from("100.dat"),
                format: StorageFormat::_12bit_twos_complement,
                samples_per_frame: Some(3),
                skew: Some(2),
                byte_offset: Some(53),
                adc_gain: Some(200.0),
                baseline: Some(2),
                units: Some(String::from("cm")),
                adc_resolution: Some(11),
                adc_zero: Some(1024),
                initial_value: Some(995),
                checksum: Some(-22131),
                block_size: Some(0),
                description: Some(String::from("MLII")),
            }
        )
    }
}
