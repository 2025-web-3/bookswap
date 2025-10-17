use nanoid::nanoid;

pub mod database;
pub mod error;
pub mod session;
pub mod user;

const _SESSION_ID_ALPHABET: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Generate 32 random HEX chars
pub fn new_hex_id(length: usize) -> String {
    nanoid!(length, &_SESSION_ID_ALPHABET)
}
