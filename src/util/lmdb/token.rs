extern crate lmdb_rs as lmdb;

use std::path::PathBuf;
use lmdb::{EnvBuilder, DbFlags, core::MdbError};

use anyhow::Result;

use crate::util::setup_lmdb::stm_data_folder;

pub fn store_token(token: &str) -> Result<()> {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)?;

    let db_handle = env.get_default_db(DbFlags::empty())?;
    let txn = env.new_transaction()?;
    {
        let db = txn.bind(&db_handle);
        db.set(&"token", &token)?;
    }

    txn.commit()?;

    println!("token inserted in lmdb");

    Ok(())
}

pub fn get_token() -> Result<Option<String>> {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)?;

    let db_handle = env.get_default_db(DbFlags::empty())?;

    let reader = env.get_reader()?;

    let db = reader.bind(&db_handle);

    let token = match db.get::<String>(&"token") {
        Ok(token) => Ok(Some(token)),
        Err(e) => {
            match e {
                MdbError::NotFound => Ok(None),
                _ =>  Err(anyhow::Error::new(e).context("Failed to get token from the database")),
            }
        }
    };

    token

}