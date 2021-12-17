use crate::{error::Error, check_factory_type};
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{packed::*, prelude::*},
    high_level::{QueryIter, load_cell_type, load_cell_lock},
};
use core::result::Result;

/// This extension checks if at least one input cell has the same lock as factory.
pub struct OnlyOwner;

impl OnlyOwner {
    pub fn handle_creation(nft_type: &Script) -> Result<(), Error> {
        let factory_lock_script: Script = QueryIter::new(load_cell_type, Source::CellDep)
            .position(|type_opt| {
                type_opt.map_or(false, |type_| check_factory_type(&nft_type)(&type_))
                
            })
            .map(|index| load_cell_lock(index, Source::CellDep).map_or(Err(Error::Encoding), Ok))
            .map_or_else(|| Err(Error::Encoding), |lock_| lock_)?;

        let input_cells = QueryIter::new(load_cell_lock, Source::Input)
            .filter(|lock| lock.as_slice() == factory_lock_script.as_slice())
            .count();

        if input_cells < 1 {
            return Err(Error::OnlyOwnerConditionError);
        }

        Ok(())
    }

    pub fn handle_update(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }

    pub fn handle_destroying(_nft_type: &Script) -> Result<(), Error> {
        Ok(())
    }
}
