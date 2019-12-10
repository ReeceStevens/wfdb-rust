//! Parsing logic for each signal format type

/// Parse a byte buffer of data in format 212, 12-bit two's complement amplitude.
///
/// Two 12-bit samples in 3 bytes:
///
/// ```text
/// | 1 2 3 4 5 6 7 8 | 9 10 11 12 1 2 3 4 | 5 6 7 8 9 10 11 12 |
/// ```
///
pub fn parse_212_format(buf: &[u8]) -> Vec<i16> {
    let mut output_buf = vec![];
    for idx in (0..buf.len()).step_by(3) {
        if idx + 1 >= buf.len() { break }
        let sample_1_lower = buf[idx].reverse_bits() as u16;
        let mut sample_1_upper = ((buf[idx+1] & 0xF0).reverse_bits() as u16) << 8;
        if (sample_1_upper & 0x0800) != 0 {
            // Extend two's complement sign bits if last bit is 1
            sample_1_upper |= 0xF000;
        }
        output_buf.push((sample_1_lower + sample_1_upper) as i16);

        if idx + 2 >= buf.len() { break }
        let sample_2_lower = (((buf[idx+1] & 0x0F).reverse_bits()) >> 4) as u16;
        let mut sample_2_upper = ((buf[idx+2].reverse_bits()) << 4) as u16;
        if (sample_2_upper & 0x0800) != 0 {
            // Extend two's complement sign bits if last bit is 1
            sample_2_upper |= 0xF000;
        }
        output_buf.push((sample_2_lower + sample_2_upper) as i16);
    }
    output_buf
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn basic_byte_parser() {
        let byte_buf = [
            0b11110000_u8,
            0b01101000_u8,
            0b10000000_u8,
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                1551, 17
            ]
        );
    }

    #[test]
    fn incomplete_buffer() {
        let byte_buf = [
            0b11110000_u8,
            0b01101000_u8,
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                1551
            ]
        );
    }

    #[test]
    fn negative_values_buffer() {
        let byte_buf = [
            0b11111111_u8,
            0b11111000_u8,
            0b10000000_u8,
        ];
        assert_eq!(
            parse_212_format(&byte_buf),
            vec![
                -1, 17
            ]
        );
    }
}
