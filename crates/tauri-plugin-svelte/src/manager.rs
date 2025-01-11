// This file was autogenerated and should not be edited manually.
// Check the `codegen` command in the `tauri-store-cli` crate.

use crate::svelte::Svelte;
use tauri::{AppHandle, Manager, Runtime, State, WebviewWindow, Window};
use tauri_store::{Result, Store};

/// Extension for the [`Manager`] trait providing access to the Svelte plugin.
///
/// [`Manager`]: https://docs.rs/tauri/latest/tauri/trait.Manager.html
pub trait ManagerExt<R: Runtime>: Manager<R> {
  /// Returns a handle to the Svelte plugin.
  ///
  /// # Panics
  ///
  /// Panics if the internal [store collection] is not in the [resources table].
  ///
  /// This likely indicates that the method was called before the plugin was properly initialized.
  ///
  /// [store collection]: https://tb.dev.br/tauri-store/rust-docs/tauri_store/struct.StoreCollection.html
  /// [resources table]: https://docs.rs/tauri/latest/tauri/struct.ResourceTable.html
  fn svelte(&self) -> State<Svelte<R>> {
    self.state::<Svelte<R>>()
  }

  /// Calls a closure with a mutable reference to the store with the given id.
  fn with_store<F, T>(&self, id: impl AsRef<str>, f: F) -> Result<T>
  where
    F: FnOnce(&mut Store<R>) -> T,
  {
    self.svelte().with_store(id, f)
  }
}

impl<R: Runtime> ManagerExt<R> for AppHandle<R> {}
impl<R: Runtime> ManagerExt<R> for WebviewWindow<R> {}
impl<R: Runtime> ManagerExt<R> for Window<R> {}
