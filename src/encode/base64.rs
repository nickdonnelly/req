use bytes::{ Bytes };
use super::super::Payload;
use super::{ Encoder, EncodeError };


// Thank christ for vim macros...
const ALPHABET: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];

const MASK_FIRST:  u32 = 0b000000_11111100_00000000_00000000;
const MASK_SECOND: u32 = 0b000000_00000011_11110000_00000000;
const MASK_THIRD:  u32 = 0b000000_00000000_00001111_11000000;
const MASK_FOURTH: u32 = 0b000000_00000000_00000000_00111111;

pub struct Base64Encoder {
    data: Bytes
}

impl Base64Encoder {
    pub fn new<'a>(payload: &'a Payload) -> Self
    {
        Base64Encoder {
            data: Bytes::from(payload.data.as_slice()),
        }
    }

    /// Converts 3 byte triples to base64 characters. Ignored bytes
    /// should be used only if you are parsing the last 3 bytes and 
    /// you dont have a clean divisor of 3. The last `ignored_bytes`
    /// characters are replaced with `'='`. 
    /// Precondition: `0 <= ignored_bytes <= 2`
    /// This takes a 32 bit integer, but the first 8 bits are ignored
    /// and it is treated as a 24 bit integer.
    #[inline(always)]
    pub fn chars_from_tribyte(int24: u32, ignored_bytes: u8) -> [char; 4]
    {
        let e1 = ALPHABET[((int24 & MASK_FIRST) >> 18) as usize];
        let e2= ALPHABET[((int24 & MASK_SECOND) >> 12) as usize];
        let e3 = if ignored_bytes == 2 { '=' } else { ALPHABET[((int24 & MASK_THIRD) >> 6) as usize] };
        let e4 = if ignored_bytes >= 1 { '=' } else { ALPHABET[(int24 & MASK_FOURTH) as usize] };

        [e1, e2, e3, e4]
    }
}

impl Encoder for Base64Encoder {
    fn encode(&self) -> Result<String, EncodeError>
    {
        use std::fmt::Write;

        let mut offset = 0;
        let mut out_buf = String::new();

        while offset + 3 <= self.data.len() {
            let three_bytes = self.data.slice(offset, offset + 3); // grab the next 3 bytes

            let byte_int: u32 = ((three_bytes[0] as u32) << 16)
                              + ((three_bytes[1] as u32) << 8)
                              + three_bytes[2] as u32;


            let encode = Base64Encoder::chars_from_tribyte(byte_int, 0);

            write!(&mut out_buf, "{}{}{}{}", encode[0], encode[1], encode[2], encode[3])
                .expect("Base64Encoder: Failed when writing to output buffer.");

            offset += 3;
        }

        if offset == self.data.len() { // no more data
            Ok(out_buf)
        } else {
        
            let encode = if offset == (self.data.len() - 1) { // we missed 1 byte
                let last_byte = self.data.slice(offset, offset + 1);
                let byte_int: u32 = (last_byte[0] as u32) << 16;

                Base64Encoder::chars_from_tribyte(byte_int, 2)
            } else { // we missed two bytes
                let last_bytes = self.data.slice(offset, offset + 2);
                let byte_int: u32 = ((last_bytes[0] as u32) << 16) + ((last_bytes[1] as u32) << 8);

                Base64Encoder::chars_from_tribyte(byte_int, 1)
            }; 

            write!(&mut out_buf, "{}{}{}{}", encode[0], encode[1], encode[2], encode[3])
                .expect("Base64Encoder: Failed when writing to output buffer.");

            Ok(out_buf)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Base64Encoder;
    use super::super::super::Payload;
    use super::super::Encoder;

    #[test]
    fn test_encode_even_div() 
    {
        let to_enc = b"hello1".to_vec(); 
        let payload = Payload::new(to_enc, "application/octet-stream");

        let enc = Base64Encoder::new(&payload).encode();
        
        assert_eq!(enc.unwrap(), "aGVsbG8x");

    }

    #[test]
    fn test_encode_uneven_div_single() 
    {
        let to_enc = b"hello123".to_vec();
        let payload = Payload::new(to_enc, "application/octet-stream");
        let enc = Base64Encoder::new(&payload).encode();

        assert_eq!(enc.unwrap(), "aGVsbG8xMjM=");
    }

    #[test]
    fn test_encode_uneven_div_double() 
    {
        let to_enc = b"hello12".to_vec();
        let payload = Payload::new(to_enc, "application/octet-stream");
        let enc = Base64Encoder::new(&payload).encode();

        assert_eq!(enc.unwrap(), "aGVsbG8xMg==");
    }

}
