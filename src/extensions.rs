use ckb_std::ckb_types::packed::*;
use core::result::Result;
use crate::error::Error;

pub struct OnlyOwner;

impl OnlyOwner {
    pub fn handle_creation(_nft_type: &Script) -> Result<(), Error> {
        //Err(Error::NFTDataInvalid)
        Ok(())
    }

    pub fn handle_update(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_destroying(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
}