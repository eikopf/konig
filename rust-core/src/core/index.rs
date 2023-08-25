use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexError {
    #[error("expected an index in the range [0, 63]; got {0}")]
    InvalidNumericIndex(u8),
    #[error("expected an algebraic position in the form [a-h][0-8]; got {0:?}")]
    InvalidAlgebraicPosition((char, char)),
    #[error("an unknown index error has occurred")]
    Unknown,
}

#[derive(Debug)]
pub struct Index(u8);

impl TryFrom<u8> for Index {
    type Error = IndexError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value > 63 {
            true => Err(IndexError::InvalidNumericIndex(value)),
            false => Ok(Index(value)),
        }
    }
}

impl TryFrom<(char, char)> for Index {
    type Error = IndexError;

    fn try_from(value: (char, char)) -> Result<Self, Self::Error> {
        match value {
            (rank @ 'a'..='h', file @ '1'..='8') => Ok(Index(
                ((rank as u8 - 'a' as u8) * 8) + (file as u8 - '1' as u8))
            ),
            _ => Err(IndexError::InvalidAlgebraicPosition(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::index::Index;

    #[test]
    fn validate_try_from_u8() {
        assert_eq!(Index::try_from(7).unwrap().0, 7);
        assert_eq!(Index::try_from(63).unwrap().0, 63);
        assert!(Index::try_from(64).is_err());
    }

    #[test]
    fn validate_try_from_char_char_tuple() {
        assert_eq!(Index::try_from(('a', '1')).unwrap().0, 0);
        assert_eq!(Index::try_from(('h', '8')).unwrap().0, 63);
    }
}
