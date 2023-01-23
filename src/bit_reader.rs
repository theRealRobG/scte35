use crate::error::ParseError;
use bitter::{BigEndianReader, BitReader};

pub struct Bits<'a> {
    bits: &'a mut BigEndianReader<'a>,
    non_fatal_errors: Vec<ParseError>,
}

impl<'a> Bits<'a> {
    pub fn new(bits: &'a mut BigEndianReader<'a>) -> Self {
        Self {
            bits,
            non_fatal_errors: vec![],
        }
    }

    pub fn bits_remaining(&self) -> usize {
        self.bits.bits_remaining().unwrap_or(0)
    }

    pub fn u8(&mut self, n: u32) -> u8 {
        self.bits.read_bits(n).unwrap() as u8
    }

    pub fn u16(&mut self, n: u32) -> u16 {
        self.bits.read_bits(n).unwrap() as u16
    }

    pub fn u32(&mut self, n: u32) -> u32 {
        self.bits.read_bits(n).unwrap() as u32
    }

    pub fn u64(&mut self, n: u32) -> u64 {
        self.bits.read_bits(n).unwrap()
    }

    pub fn bool(&mut self) -> bool {
        self.u8(1) == 1
    }

    pub fn byte(&mut self) -> u8 {
        self.u8(8)
    }

    pub fn consume(&mut self, n: u32) {
        self.bits.consume(n)
    }

    pub fn string(
        &mut self,
        n: usize,
        error_description: &'static str,
    ) -> Result<String, ParseError> {
        let mut buf = vec![0; n];
        self.bits.read_bytes(&mut buf);
        std::str::from_utf8(&buf)
            .map(ToString::to_string)
            .map_err(|e| ParseError::Utf8ConversionError {
                error: e,
                description: error_description,
            })
    }

    pub fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut buf = vec![0; n];
        self.bits.read_bytes(&mut buf);
        buf
    }

    pub fn validate(
        &mut self,
        expected_minimum_bits_left: u32,
        description: &'static str,
    ) -> Result<(), ParseError> {
        self.bits.refill_lookahead();
        let actual_bits_left = self.bits_remaining() as u32;
        if actual_bits_left < expected_minimum_bits_left {
            Err(ParseError::UnexpectedEndOfData {
                expected_minimum_bits_left,
                actual_bits_left,
                description,
            })
        } else {
            Ok(())
        }
    }

    pub fn refill_lookahead(&mut self) -> u32 {
        self.bits.refill_lookahead()
    }

    pub fn push_non_fatal_error(&mut self, error: ParseError) {
        self.non_fatal_errors.push(error);
    }

    pub fn get_non_fatal_errors(&self) -> &Vec<ParseError> {
        &self.non_fatal_errors
    }
}
