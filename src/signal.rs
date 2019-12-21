//! Parsing logic for each signal format type

/// Parse a byte buffer of data in format 212, 12-bit two's complement amplitude.
///
/// Two 12-bit samples in 3 bytes:
///
/// ```text
/// | 8 7 6 5 4 3 2 1 | 12 11 10 9 12 11 10 9 | 8 7 6 5 4 3 2 1 |
/// ```
///
pub fn parse_212_format(buf: &[u8]) -> Vec<i16> {
    let mut output_buf = vec![];
    for idx in (0..buf.len()).step_by(3) {
        if idx + 1 >= buf.len() { break }
        let sample_1_lower = buf[idx] as u16;
        let mut sample_1_upper = ((buf[idx+1] & 0x0F) as u16) << 8;
        if (sample_1_upper & 0x0800) != 0 {
            // Extend two's complement sign bits if last bit is 1
            sample_1_upper |= 0xF000;
        }
        output_buf.push((sample_1_lower | sample_1_upper) as i16);

        if idx + 2 >= buf.len() { break }
        let sample_2_lower = buf[idx+2] as u16;
        let mut sample_2_upper = ((buf[idx+1] & 0xF0) as u16) << 4;
        if (sample_2_upper & 0x0800) != 0 {
            // Extend two's complement sign bits if last bit is 1
            sample_2_upper |= 0xF000;
        }
        output_buf.push((sample_2_lower | sample_2_upper) as i16);
    }
    output_buf
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_byte_parser() {
        let byte_buf = [
            0xF0, 0x68, 0x80,
            0xF0, 0x68, 0x80,
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                -1808, 1664, // 0x8F0, 0x680
                -1808, 1664, // 0x8F0, 0x680
            ]
        );
    }

    #[test]
    fn incomplete_buffer() {
        let byte_buf = [
            0xF0, 0x86,
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                1776, //0x6F0
            ]
        );
    }

    #[test]
    fn negative_values_buffer() {
        let byte_buf = [
            0xFF, 0x8F, 0x80
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                -1, -1920 // 0xFFF, 0x880
            ]
        );
    }
}
