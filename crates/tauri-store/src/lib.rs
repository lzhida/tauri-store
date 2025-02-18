#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod collection;
mod error;
mod event;
mod manager;
pub mod prelude;
mod store;

use tauri::{Manager, Runtime};

pub use collection::{OnLoadFn, StoreCollection, StoreCollectionBuilder};
pub use error::{BoxResult, Error, Result};
pub use event::{
  EventSource, STORE_CONFIG_CHANGE_EVENT, STORE_STATE_CHANGE_EVENT, STORE_UNLOAD_EVENT,
};
pub use manager::ManagerExt;
pub use serde_json::Value as Json;
pub use store::{SaveStrategy, Store, StoreOptions, StoreState};

#[cfg(feature = "derive")]
pub use tauri_store_macros::{Collection, CollectionBuilder};

/// Calls a closure with a mutable reference to the store with the given id.
pub fn with_store<R, M, F, T>(manager: &M, id: impl AsRef<str>, f: F) -> Result<T>
where
  R: Runtime,
  M: Manager<R> + ManagerExt<R>,
  F: FnOnce(&mut Store<R>) -> T,
{
  manager.store_collection().with_store(id, f)
}
