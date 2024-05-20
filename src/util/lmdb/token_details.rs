extern crate lmdb_rs as lmdb;

use std::path::PathBuf;
use lmdb::{EnvBuilder, DbFlags};

use crate::util::setup_lmdb::stm_data_folder;

pub fn store_token_details(token_details: &str) {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)
        .unwrap();

    let db_handle = env.get_default_db(DbFlags::empty()).unwrap();

    let txn = env.new_transaction().unwrap();
    {
        let db = txn.bind(&db_handle);
        db.set(&"token_details", &token_details).unwrap();
    }

    match txn.commit() {
        Ok(_) => (),
        Err(_) => panic!("failed to commit"),
    }

    println!("token inserted in lmdb");
}

pub fn get_token_details() -> String {
    let data_folder = stm_data_folder();

    let data_folder_pathbuf = PathBuf::from(data_folder);

    let env = EnvBuilder::new()
        .open(data_folder_pathbuf.join("data-lmdb"), 0o777)
        .unwrap();

    let db_handle = env.get_default_db(DbFlags::empty()).unwrap();

    let reader = env.get_reader().unwrap();

    let db = reader.bind(&db_handle);
    let token = match db.get::<&str>(&"token_details") {
        Ok(token) => token,
        Err(_) => ""
    };

    token.to_string()

}