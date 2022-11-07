#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("Input Tidak Valid {0}")] 
    NotNumber(#[from] std::num::ParseIntError),
    #[error("Input Tidak Valid {0}")] 
    InvalidInput(&'static str),
    #[error("missing field: {0}")] 
    MissingField(&'static str)
}