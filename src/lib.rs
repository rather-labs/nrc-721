#![allow(dead_code)]
#![no_std]
extern crate alloc;

pub mod factory;
pub mod nft;
pub mod error;
pub mod helper;
pub mod extensions;

use blake2b_rs::Blake2bBuilder;
use ckb_std::{
    ckb_constants::Source,
    ckb_types::{bytes::Bytes, packed::*, prelude::*},
    high_level::load_input,
};
use core::result::Result;
use error::Error;
use factory::FACTORY_TYPE_ARGS_LEN;
use helper::{count_cells_by_type, load_output_index_by_type, Action};
use nft::{
        NFT_TYPE_ARGS_COLL_ARGS_INDEX, NFT_TYPE_ARGS_COLL_ARGS_LEN,
        NFT_TYPE_ARGS_COLL_CODE_HASH_INDEX, NFT_TYPE_ARGS_COLL_CODE_HASH_LEN,
        NFT_TYPE_ARGS_COLL_TYPE_INDEX, NFT_TYPE_ARGS_COLL_TYPE_LEN, NFT_TYPE_ARGS_LEN,
        NFT_TYPE_ARGS_TOKEN_ID_INDEX, NFT_TYPE_ARGS_TOKEN_ID_LEN,
    };

fn check_nft_type<'a>(nft_type: &'a Script) -> impl Fn(&Script) -> bool + 'a {
    let nft_args: Bytes = nft_type.args().unpack();
    move |type_: &Script| {
        let type_args: Bytes = type_.args().unpack();
        type_.code_hash().as_slice() == nft_type.code_hash().as_slice()
            && type_.hash_type().as_slice() == nft_type.hash_type().as_slice()
            && type_args.len() == NFT_TYPE_ARGS_LEN
            && type_args[0..FACTORY_TYPE_ARGS_LEN] == nft_args[0..FACTORY_TYPE_ARGS_LEN]
    }
}

fn check_factory_type<'a>(nft_type: &'a Script) -> impl Fn(&Script) -> bool + 'a {
    let nft_type_args: Bytes = nft_type.args().unpack();
    move |unknown_type: &Script| {
        unknown_type.code_hash().as_slice()
            == &nft_type_args[NFT_TYPE_ARGS_COLL_CODE_HASH_INDEX
                ..NFT_TYPE_ARGS_COLL_CODE_HASH_INDEX + NFT_TYPE_ARGS_COLL_CODE_HASH_LEN]
            && unknown_type.hash_type().as_slice()
                == &nft_type_args[NFT_TYPE_ARGS_COLL_TYPE_INDEX
                    ..NFT_TYPE_ARGS_COLL_TYPE_INDEX + NFT_TYPE_ARGS_COLL_TYPE_LEN]
            && unknown_type.args().raw_data().as_ref()
                == &nft_type_args[NFT_TYPE_ARGS_COLL_ARGS_INDEX
                    ..NFT_TYPE_ARGS_COLL_ARGS_INDEX + NFT_TYPE_ARGS_COLL_ARGS_LEN]
    }
}

pub fn parse_nft_action(nft_type: &Script) -> Result<Action, Error> {
    let nft_inputs_count = count_cells_by_type(Source::Input, &check_nft_type(nft_type));
    if nft_inputs_count == 0 {
        return Ok(Action::Create);
    }

    let nft_outputs_count = count_cells_by_type(Source::Output, &check_nft_type(nft_type));
    if nft_inputs_count == 1 && nft_outputs_count == 0 {
        return Ok(Action::Destroy);
    }

    if nft_inputs_count == nft_outputs_count {
        return Ok(Action::Update);
    }
    Err(Error::NFTCellsCountError)
}

fn build_type_id(nft_type: &Script) -> Result<[u8; 32], Error> {
    let mut blake2b = Blake2bBuilder::new(32)
        .personal(b"ckb-default-hash")
        .build();
    let first_input = load_input(0, Source::Input)?;
    blake2b.update(first_input.as_slice());

    // Use this cell output index
    let output_index = load_output_index_by_type(nft_type).unwrap();
    blake2b.update(&output_index.to_le_bytes());
    let mut ret = [0; 32];
    blake2b.finalize(&mut ret);
    Ok(ret)
}

pub struct Base;

impl Base {
    pub fn handle_creation(nft_type: &Script) -> Result<(), Error> {
        // Check that TOKEN_ID is the expected hash
        let hash = build_type_id(nft_type)?;
        let nft_args: Bytes = nft_type.args().unpack();
        if nft_args[NFT_TYPE_ARGS_TOKEN_ID_INDEX
            ..NFT_TYPE_ARGS_TOKEN_ID_INDEX + NFT_TYPE_ARGS_TOKEN_ID_LEN]
            != hash[0..32]
        {
            return Err(Error::TypeArgsInvalid);
        }

        // Check the factory dependency exists
        let factory_inputs_count =
            count_cells_by_type(Source::CellDep, &check_factory_type(&nft_type));
        if factory_inputs_count != 1 {
            return Err(Error::FactoryCellsCountError);
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

#[macro_export]
macro_rules! define_script {
	($name:ident($($parent:ident),+) {$($field_name:ident : $field_type:ty),*} $($custom:expr),*) => {
		#[derive(Debug)]
		struct $name { $($field_name: $field_type),* }
		impl $name {
            pub fn validate_nft_args(nft_type: &Script) -> Result<(), Error> {
                let nft_args: Bytes = nft_type.args().unpack();
                if nft_args.len() != nft::NFT_TYPE_ARGS_LEN {
                    return Err(Error::TypeArgsInvalid);
                }
                Ok(())
            }
			fn handle_creation(nft_type: &Script) -> Result<(), Error> {
				let results = [$($parent :: handle_creation(nft_type)),+$(, $custom())*];
                for result in results {
                    match result {
                        Ok(_) => {},
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(())
			}
            fn handle_update(nft_type: &Script) -> Result<(), Error> {
				let results = [$($parent :: handle_update(nft_type)),+$(, $custom())*];
                for result in results {
                    match result {
                        Ok(_) => {},
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(())
			}
            fn handle_destroying(nft_type: &Script) -> Result<(), Error> {
				let results = [$($parent :: handle_destroying(nft_type)),+$(, $custom())*];
                for result in results {
                    match result {
                        Ok(_) => {},
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(())
			}
		}
	};
}
