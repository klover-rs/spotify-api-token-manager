extern crate lmdb_rs as lmdb;

use std::path::PathBuf;
use lmdb::{EnvBuilder, DbFlags, core::MdbError};
use anyhow::Result;


use crate::util::setup_lmdb::stm_data_folder;

pub fn store_token_details(token_details: &str) -> Result<()> {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)?;

    let db_handle = env.get_default_db(DbFlags::empty())?;

    let txn = env.new_transaction()?;
    {
        let db = txn.bind(&db_handle);
        db.set(&"token_details", &token_details)?;
    }

    txn.commit()?;
    Ok(())

}

pub fn get_token_details() -> Result<Option<String>> {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)?;

    let db_handle = env.get_default_db(DbFlags::empty())?;

    let reader = env.get_reader()?;

    let db = reader.bind(&db_handle);

    let token_details = match db.get::<String>(&"token_details") {
        Ok(token) => Ok(Some(token)),
        Err(e) => {
            match e {
                MdbError::NotFound => Ok(None),
                _ =>  Err(anyhow::Error::new(e).context("Failed to get token_details from the database")),
            }
        }
    };

    token_details

}