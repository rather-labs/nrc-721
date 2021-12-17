use crate::error::Error;
use crate::helper::{parse_dyn_vec_len, DYN_MIN_LEN};
use alloc::vec::Vec;
use core::result::Result;
const FIXED_LEN: usize = 0;
// FIXED_LEN + DYN_MIN_LEN * 3
// FIXED_LEN + Factory::name(usize) + Factory::symbol(usize) + Factory::token_uri(usize) size
const FACTORY_DATA_MIN_LEN: usize = FIXED_LEN + DYN_MIN_LEN * 3;
// TYPE_ID_32_BYTES
pub const FACTORY_TYPE_ARGS_LEN: usize = 32;

/// Factory cell data structure
/// This structure contains the following information:
/// 1) name: <size: u16> + <content>
/// 2) symbol: <size: u16> + <content>
/// 3) token_uri: <size: u16> + <content>
/// None of [name, symbol, token_uri] are mandatory, but 0usize must be placed as length in that case
#[derive(Debug, Clone)]
pub struct Factory {
    pub name: Vec<u8>,
    pub symbol: Vec<u8>,
    pub token_uri: Vec<u8>,
}

impl Factory {
    pub fn from_data(raw_data: &[u8]) -> Result<Self, Error> {
        if raw_data.len() < FACTORY_DATA_MIN_LEN {
            return Err(Error::FactoryDataInvalid);
        }

        // name reading
        let name_len = parse_dyn_vec_len(&raw_data[FIXED_LEN..(FIXED_LEN + DYN_MIN_LEN)]);
        // DYN_MIN_LEN * 2: the min length of symbol + the min length of token_uri
        if raw_data.len() < (FIXED_LEN + name_len + (DYN_MIN_LEN * 2)) {
            return Err(Error::FactoryDataInvalid);
        }
        let name = raw_data[FIXED_LEN..(FIXED_LEN + name_len)].to_vec();

        // symbol reading
        let symbol_index = FIXED_LEN + name_len;
        let symbol_len = parse_dyn_vec_len(&raw_data[symbol_index..(symbol_index + DYN_MIN_LEN)]);
        // DYN_MIN_LEN: the min length of token_uri
        if raw_data.len() < symbol_index + symbol_len + DYN_MIN_LEN {
            return Err(Error::FactoryDataInvalid);
        }
        let symbol = raw_data[symbol_index..(symbol_index + symbol_len)].to_vec();

        // token_uri reading
        let token_uri_index = FIXED_LEN + name_len + symbol_len;
        let token_uri_len =
            parse_dyn_vec_len(&raw_data[token_uri_index..(token_uri_index + DYN_MIN_LEN)]);
        let token_uri = raw_data[token_uri_index..(token_uri_index + token_uri_len)].to_vec();

        Ok(Factory {
            name,
            symbol,
            token_uri,
        })
    }

    pub fn immutable_equal(&self, other: &Factory) -> bool {
        self.name == other.name && self.symbol == other.symbol && self.token_uri == other.token_uri
    }
}
