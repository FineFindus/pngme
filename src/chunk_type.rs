use anyhow::ensure;
use std::{fmt::Display, str::FromStr};

/// A Chunktype contains information about the following chunk.
///
/// <http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html>
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    /// Checks whether the chunk is valid.
    ///
    /// A valid chunks only contains bytes in the ASCII range and the reserved bit is valid.
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
            && self
                .bytes
                .iter()
                .all(|&byte| (byte as char).is_alphabetic())
    }

    /// Indicates whether the chunk is `critical`/necessary
    /// to successfully display the image.
    ///
    /// Ancillary chunks may be ignored.
    pub fn is_critical(&self) -> bool {
        self.bytes[0] & 32 == 0
    }

    /// Indicates whether the chunk is part of the PNGSpec
    pub fn is_public(&self) -> bool {
        self.bytes[1] & 32 == 0
    }

    /// Checks whether the reserved bit of the chunk name is set.
    /// If it is set the chunk name is invalid.
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.bytes[2] & 32 == 0
    }

    /// Indicates whether the chunk is safe to copy. Unsafe chunks may not be copied when modifying
    /// an image
    pub fn is_safe_to_copy(&self) -> bool {
        self.bytes[3] & 32 != 0
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(self.bytes.as_slice()))
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = std::io::Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = super::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.len() == 4, "Input must have four bytes");
        ensure!(
            s.chars().all(|char| char.is_alphabetic()),
            "Input must only contain alphabetic ASCII"
        );

        assert_eq!(s.len(), 4);
        let bytes = s.as_bytes().try_into()?;
        Ok(Self { bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
