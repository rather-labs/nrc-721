use crate::error::Error;
use crate::helper::{parse_dyn_vec_len, DYN_MIN_LEN};
use alloc::vec::Vec;
use core::result::Result;

const FIXED_LEN: usize = 0;
pub const NFT_DATA_MIN_LEN: usize = FIXED_LEN + DYN_MIN_LEN;
// Factory_code_hash<32 bytes>
pub const NFT_TYPE_ARGS_COLL_CODE_HASH_INDEX: usize = 0;
pub const NFT_TYPE_ARGS_COLL_CODE_HASH_LEN: usize = 32;
// Factory_type<uint8>
pub const NFT_TYPE_ARGS_COLL_TYPE_INDEX: usize =
    NFT_TYPE_ARGS_COLL_CODE_HASH_INDEX + NFT_TYPE_ARGS_COLL_CODE_HASH_LEN;
pub const NFT_TYPE_ARGS_COLL_TYPE_LEN: usize = 1;
// Factory_args<32 bytes>
pub const NFT_TYPE_ARGS_COLL_ARGS_INDEX: usize =
    NFT_TYPE_ARGS_COLL_TYPE_INDEX + NFT_TYPE_ARGS_COLL_TYPE_LEN;
pub const NFT_TYPE_ARGS_COLL_ARGS_LEN: usize = 32;
// TOKEN_ID [32 bytes Type-ID]
pub const NFT_TYPE_ARGS_TOKEN_ID_INDEX: usize =
    NFT_TYPE_ARGS_COLL_ARGS_INDEX + NFT_TYPE_ARGS_COLL_ARGS_LEN;
pub const NFT_TYPE_ARGS_TOKEN_ID_LEN: usize = 32;
// complete args length
pub const NFT_TYPE_ARGS_LEN: usize = NFT_TYPE_ARGS_TOKEN_ID_INDEX + NFT_TYPE_ARGS_TOKEN_ID_LEN;

/// NFT cell data structure
/// This structure contains the following information:
/// 1) data: <size: u16> + <vartext>
#[derive(Debug, Clone)]
pub struct Nft {
    pub data: Vec<u8>,
}

impl Nft {
    pub fn from_data(raw_data: &[u8]) -> Result<Self, Error> {
        if raw_data.len() < NFT_DATA_MIN_LEN {
            return Err(Error::NFTDataInvalid);
        }

        // data reading
        let data_len = parse_dyn_vec_len(&raw_data[FIXED_LEN..(FIXED_LEN + DYN_MIN_LEN)]);
        if raw_data.len() < data_len {
            return Err(Error::NFTDataInvalid);
        }
        let data = raw_data[FIXED_LEN..(FIXED_LEN + data_len)].to_vec();

        Ok(Nft { data })
    }
}
