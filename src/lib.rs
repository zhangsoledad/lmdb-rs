//! Idiomatic and safe APIs for interacting with the
//! [Symas Lightning Memory-Mapped Database (LMDB)](http://symas.com/mdb/).

#![cfg_attr(test, feature(test))]
#![deny(warnings)]
#![doc(html_root_url = "https://docs.rs/lmdb/0.7.1")]

extern crate libc;
extern crate lmdb_sys as ffi;

#[cfg(test)] extern crate rand;
#[cfg(test)] extern crate tempdir;
#[cfg(test)] extern crate test;
#[macro_use] extern crate bitflags;

pub use cursor::{
    Cursor,
    RoCursor,
    RwCursor,
    Iter,
    IterDup,
};
pub use database::Database;
pub use environment::{Environment, EnvironmentBuilder};
pub use error::{Error, Result};
pub use flags::*;
pub use transaction::{
    InactiveTransaction,
    RoTransaction,
    RwTransaction,
    Transaction,
};

macro_rules! lmdb_try {
    ($expr:expr) => ({
        match $expr {
            ::ffi::MDB_SUCCESS => (),
            err_code => return Err(::Error::from_err_code(err_code)),
        }
    })
}

macro_rules! lmdb_try_with_cleanup {
    ($expr:expr, $cleanup:expr) => ({
        match $expr {
            ::ffi::MDB_SUCCESS => (),
            err_code => {
                let _ = $cleanup;
                return Err(::Error::from_err_code(err_code))
            },
        }
    })
}

mod flags;
mod cursor;
mod database;
mod environment;
mod error;
mod transaction;

#[cfg(test)]
mod test_utils {

    use tempdir::TempDir;

    use super::*;

    pub fn get_key(n: u32) -> String {
        format!("key{}", n)
    }

    pub fn get_data(n: u32) -> String {
        format!("data{}", n)
    }

    pub fn setup_bench_db<'a>(num_rows: u32) -> (TempDir, Environment) {
        let dir = TempDir::new("test").unwrap();
        let env = Environment::new().open(dir.path()).unwrap();

        {
            let db = env.open_db(None).unwrap();
            let mut txn = env.begin_rw_txn().unwrap();
            for i in 0..num_rows {
                txn.put(db, &get_key(i), &get_data(i), WriteFlags::empty()).unwrap();
            }
            txn.commit().unwrap();
        }
        (dir, env)
    }
}
