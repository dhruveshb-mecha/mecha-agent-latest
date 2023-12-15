pub mod errors;
extern crate sled;
use anyhow::{bail, Result};
use fs::construct_dir_path;
use lazy_static::lazy_static;
use sled::Db;
use std::sync::{Arc, Mutex};
use tracing::info;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::errors::{KeyValueStoreError, KeyValueStoreErrorCodes};
static DATABASE_STORE_FIILE_PATH: &str = "~/.mecha/agent/db";
// Singleton database connection
lazy_static! {
    static ref DATABASE: Arc<Mutex<Db>> = {
        let file_path = fs::construct_dir_path(DATABASE_STORE_FIILE_PATH.clone()).unwrap();
        Arc::new(Mutex::new(sled::open(&file_path).unwrap()))
    };
}

#[derive(Clone)]
pub struct KeyValueStoreClient;

impl KeyValueStoreClient {
    pub fn new() -> Self {
        KeyValueStoreClient
    }
    pub fn set(&mut self, key: &str, value: &str) -> Result<bool> {
        let trace_id = find_current_trace_id();
        info!(trace_id, target = "key_value_store", task = "set", "init");
        let db = match DATABASE.lock() {
            Ok(d) => d,
            Err(e) => bail!(KeyValueStoreError::new(
                KeyValueStoreErrorCodes::DbAcquireLockError,
                format!("Error acquiring lock on set - {}", e),
                true
            )),
        };
        let _last_inserted = db.insert(key, value);
        info!(
            trace_id,
            target = "key_value_store",
            task = "set",
            "completed"
        );
        Ok(true)
    }
    pub fn get(&self, key: &str) -> Result<Option<String>> {
        let trace_id = find_current_trace_id();
        info!(trace_id, target = "key_value_store", task = "get", "init");
        let db = match DATABASE.lock() {
            Ok(d) => d,
            Err(e) => bail!(KeyValueStoreError::new(
                KeyValueStoreErrorCodes::DbAcquireLockError,
                format!("Error acquiring lock on get - {}", e),
                true
            )),
        };
        let last_inserted = db.get(key);
        match last_inserted {
            Ok(s) => {
                info!(
                    trace_id,
                    target = "key_value_store",
                    task = "get",
                    "completed"
                );
                Ok(s.map(|s| String::from_utf8(s.to_vec()).unwrap()))
            }
            Err(e) => bail!(KeyValueStoreError::new(
                KeyValueStoreErrorCodes::RetrieveValueError,
                format!("Error retrieving value from db - {}", e),
                true
            )),
        }
    }
}
