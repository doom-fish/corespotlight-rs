//! Async API for `CoreSpotlight`
//!
//! This module provides async versions of CoreSpotlight index operations when the `async` feature is enabled.
//! The async API is **executor-agnostic** and works with any async runtime (Tokio, async-std, smol, etc.).
//!
//! ## Available Types
//!
//! | Type | Method | Description |
//! |------|--------|-------------|
//! | [`AsyncCSSearchableIndex`] | `index_searchable_items` | Index items asynchronously |
//! | [`AsyncCSSearchableIndex`] | `delete_searchable_items_with_identifiers` | Delete items by identifier |
//! | [`AsyncCSSearchableIndex`] | `delete_searchable_items_with_domain_identifiers` | Delete items by domain |
//! | [`AsyncCSSearchableIndex`] | `delete_all_searchable_items` | Delete all indexed items |
//! | [`AsyncCSSearchableIndex`] | `fetch_last_client_state` | Fetch last client state (macOS 13+) |
//!
//! ## Runtime Agnostic Design
//!
//! This async API uses only `std` types and works with **any** async runtime:
//! - Uses callback-based Swift FFI for true async operations
//! - Uses `std::sync::{Arc, Mutex}` for synchronization
//! - Uses `std::task::{Poll, Waker}` for async primitives
//! - Uses `std::future::Future` trait
//!
//! ## Examples
//!
//! ### Index Items Asynchronously
//!
//! ```no_run
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # pollster::block_on(async {
//! use corespotlight::async_api::AsyncCSSearchableIndex;
//! use corespotlight::CSSearchableIndex;
//!
//! let index = CSSearchableIndex::default_searchable_index()?;
//! let items = vec![]; // Create your items
//! AsyncCSSearchableIndex::index_searchable_items(&index, &items).await?;
//! println!("Items indexed");
//! # Ok(())
//! # })
//! # }
//! ```
//!
//! ### Delete Items Asynchronously
//!
//! ```no_run
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # pollster::block_on(async {
//! use corespotlight::async_api::AsyncCSSearchableIndex;
//! use corespotlight::CSSearchableIndex;
//!
//! let index = CSSearchableIndex::default_searchable_index()?;
//! AsyncCSSearchableIndex::delete_searchable_items_with_identifiers(
//!     &index,
//!     vec!["id1", "id2"]
//! ).await?;
//! println!("Items deleted");
//! # Ok(())
//! # })
//! # }
//! ```

use crate::error::CoreSpotlightError;
use crate::index::CSSearchableIndex;
use crate::item::CSSearchableItem;
use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::panic_safe::catch_user_panic;
use std::ffi::c_void;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// ============================================================================
// IndexSearchableItems
// ============================================================================

/// Callback for index_searchable_items completion
extern "C" fn index_searchable_items_cb(
    _result: *const c_void,
    error: *const i8,
    user_data: *mut c_void,
) {
    catch_user_panic("index_searchable_items_cb", || {
        if error.is_null() {
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_ok(user_data, ()) };
        } else {
            // SAFETY: error is a valid C string pointer or null
            let error_msg = unsafe { error_from_cstr(error) };
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_err(user_data, error_msg) };
        }
    });
}

/// Future for async index_searchable_items
///
/// # Thread Safety
///
/// This future is `Send + Sync` because it wraps `AsyncCompletionFuture<()>` which uses
/// `Arc<Mutex<_>>` for synchronization. The future can be safely moved between threads
/// and shared across threads.
pub struct IndexSearchableItemsFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for IndexSearchableItemsFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexSearchableItemsFuture")
            .finish_non_exhaustive()
    }
}

impl Future for IndexSearchableItemsFuture {
    type Output = Result<(), CoreSpotlightError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(|e| CoreSpotlightError::bridge(-1, e)))
    }
}

// ============================================================================
// DeleteSearchableItemsWithIdentifiers
// ============================================================================

/// Callback for delete_searchable_items_with_identifiers completion
extern "C" fn delete_searchable_items_with_identifiers_cb(
    _result: *const c_void,
    error: *const i8,
    user_data: *mut c_void,
) {
    catch_user_panic("delete_searchable_items_with_identifiers_cb", || {
        if error.is_null() {
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_ok(user_data, ()) };
        } else {
            // SAFETY: error is a valid C string pointer or null
            let error_msg = unsafe { error_from_cstr(error) };
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_err(user_data, error_msg) };
        }
    });
}

/// Future for async delete_searchable_items_with_identifiers
pub struct DeleteSearchableItemsWithIdentifiersFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for DeleteSearchableItemsWithIdentifiersFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteSearchableItemsWithIdentifiersFuture")
            .finish_non_exhaustive()
    }
}

impl Future for DeleteSearchableItemsWithIdentifiersFuture {
    type Output = Result<(), CoreSpotlightError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(|e| CoreSpotlightError::bridge(-1, e)))
    }
}

// ============================================================================
// DeleteSearchableItemsWithDomainIdentifiers
// ============================================================================

/// Callback for delete_searchable_items_with_domain_identifiers completion
extern "C" fn delete_searchable_items_with_domain_identifiers_cb(
    _result: *const c_void,
    error: *const i8,
    user_data: *mut c_void,
) {
    catch_user_panic("delete_searchable_items_with_domain_identifiers_cb", || {
        if error.is_null() {
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_ok(user_data, ()) };
        } else {
            // SAFETY: error is a valid C string pointer or null
            let error_msg = unsafe { error_from_cstr(error) };
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_err(user_data, error_msg) };
        }
    });
}

/// Future for async delete_searchable_items_with_domain_identifiers
pub struct DeleteSearchableItemsWithDomainIdentifiersFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for DeleteSearchableItemsWithDomainIdentifiersFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteSearchableItemsWithDomainIdentifiersFuture")
            .finish_non_exhaustive()
    }
}

impl Future for DeleteSearchableItemsWithDomainIdentifiersFuture {
    type Output = Result<(), CoreSpotlightError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(|e| CoreSpotlightError::bridge(-1, e)))
    }
}

// ============================================================================
// DeleteAllSearchableItems
// ============================================================================

/// Callback for delete_all_searchable_items completion
extern "C" fn delete_all_searchable_items_cb(
    _result: *const c_void,
    error: *const i8,
    user_data: *mut c_void,
) {
    catch_user_panic("delete_all_searchable_items_cb", || {
        if error.is_null() {
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_ok(user_data, ()) };
        } else {
            // SAFETY: error is a valid C string pointer or null
            let error_msg = unsafe { error_from_cstr(error) };
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<()>::complete_err(user_data, error_msg) };
        }
    });
}

/// Future for async delete_all_searchable_items
pub struct DeleteAllSearchableItemsFuture {
    inner: AsyncCompletionFuture<()>,
}

impl std::fmt::Debug for DeleteAllSearchableItemsFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeleteAllSearchableItemsFuture")
            .finish_non_exhaustive()
    }
}

impl Future for DeleteAllSearchableItemsFuture {
    type Output = Result<(), CoreSpotlightError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(|e| CoreSpotlightError::bridge(-1, e)))
    }
}

// ============================================================================
// FetchLastClientState
// ============================================================================

/// Callback for fetch_last_client_state completion
extern "C" fn fetch_last_client_state_cb(
    result: *const c_void,
    error: *const i8,
    user_data: *mut c_void,
) {
    catch_user_panic("fetch_last_client_state_cb", || {
        if error.is_null() {
            if result.is_null() {
                // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
                unsafe { AsyncCompletion::<Vec<u8>>::complete_err(user_data, "Unknown error".into()) };
            } else {
                // SAFETY: result is a valid C string pointer
                let result_ptr = result.cast::<i8>();
                let json_str = unsafe { std::ffi::CStr::from_ptr(result_ptr) }
                    .to_string_lossy()
                    .to_string();
                
                // Parse JSON to get the Vec<u8>
                match serde_json::from_str::<Vec<u8>>(&json_str) {
                    Ok(data) => {
                        // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
                        unsafe { AsyncCompletion::complete_ok(user_data, data) };
                    }
                    Err(e) => {
                        let err_msg = format!("Failed to parse client state JSON: {e}");
                        // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
                        unsafe { AsyncCompletion::<Vec<u8>>::complete_err(user_data, err_msg) };
                    }
                }
                
                // Free the C string
                // SAFETY: result_ptr is a valid C string pointer from the Swift callback
                if !result_ptr.is_null() {
                    unsafe { crate::ffi::cs_string_free(result_ptr.cast_mut()) };
                }
            }
        } else {
            // SAFETY: error is a valid C string pointer or null
            let error_msg = unsafe { error_from_cstr(error) };
            // SAFETY: user_data is a valid AsyncCompletion context pointer created by AsyncCompletion::create()
            unsafe { AsyncCompletion::<Vec<u8>>::complete_err(user_data, error_msg) };
        }
    });
}

/// Future for async fetch_last_client_state
pub struct FetchLastClientStateFuture {
    inner: AsyncCompletionFuture<Vec<u8>>,
}

impl std::fmt::Debug for FetchLastClientStateFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FetchLastClientStateFuture")
            .finish_non_exhaustive()
    }
}

impl Future for FetchLastClientStateFuture {
    type Output = Result<Vec<u8>, CoreSpotlightError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|r| r.map_err(|e| CoreSpotlightError::bridge(-1, e)))
    }
}

// ============================================================================
// AsyncCSSearchableIndex - Async wrapper for CSSearchableIndex
// ============================================================================

/// Async wrapper for [`CSSearchableIndex`]
///
/// Provides async methods for indexing and managing searchable items
/// without blocking. **Executor-agnostic** - works with any async runtime.
#[derive(Debug)]
pub struct AsyncCSSearchableIndex;

impl AsyncCSSearchableIndex {
    /// Asynchronously index searchable items
    ///
    /// # Errors
    ///
    /// Returns an error if indexing fails.
    pub fn index_searchable_items(
        index: &CSSearchableIndex,
        items: &[CSSearchableItem],
    ) -> IndexSearchableItemsFuture {
        let (future, ctx) = AsyncCompletion::create();
        let payload = items
            .iter()
            .map(|item| item.as_ptr() as usize as u64)
            .collect::<Vec<_>>();
        let items_json = match serde_json::to_string(&payload) {
            Ok(json) => json,
            Err(e) => {
                let err_msg = format!("Failed to serialize items: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return IndexSearchableItemsFuture { inner: future };
            }
        };

        let items_cstr = match std::ffi::CString::new(items_json) {
            Ok(cstr) => cstr,
            Err(e) => {
                let err_msg = format!("Failed to create C string: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return IndexSearchableItemsFuture { inner: future };
            }
        };

        // SAFETY: index.as_ptr() is a valid CoreSpotlight index pointer, items_cstr.as_ptr() is a valid C string,
        // and index_searchable_items_cb is a valid callback function that will be called exactly once with ctx as user_data
        unsafe {
            crate::ffi::corespotlight_index_searchable_items_async(
                index.as_ptr(),
                items_cstr.as_ptr(),
                30,
                index_searchable_items_cb,
                ctx,
            );
        }

        // Leak the CString into the callback; it will be freed by the Swift side
        std::mem::forget(items_cstr);
        IndexSearchableItemsFuture { inner: future }
    }

    /// Asynchronously delete searchable items by identifiers
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_searchable_items_with_identifiers<I, S>(
        index: &CSSearchableIndex,
        identifiers: I,
    ) -> DeleteSearchableItemsWithIdentifiersFuture
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let (future, ctx) = AsyncCompletion::create();
        let identifiers: Vec<String> = identifiers.into_iter().map(Into::into).collect();
        let identifiers_json = match serde_json::to_string(&identifiers) {
            Ok(json) => json,
            Err(e) => {
                let err_msg = format!("Failed to serialize identifiers: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return DeleteSearchableItemsWithIdentifiersFuture { inner: future };
            }
        };

        let identifiers_cstr = match std::ffi::CString::new(identifiers_json) {
            Ok(cstr) => cstr,
            Err(e) => {
                let err_msg = format!("Failed to create C string: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return DeleteSearchableItemsWithIdentifiersFuture { inner: future };
            }
        };

        // SAFETY: index.as_ptr() is a valid CoreSpotlight index pointer, identifiers_cstr.as_ptr() is a valid C string,
        // and delete_searchable_items_with_identifiers_cb is a valid callback function that will be called exactly once with ctx as user_data
        unsafe {
            crate::ffi::corespotlight_delete_searchable_items_with_identifiers_async(
                index.as_ptr(),
                identifiers_cstr.as_ptr(),
                30,
                delete_searchable_items_with_identifiers_cb,
                ctx,
            );
        }

        // Leak the CString into the callback; it will be freed by the Swift side
        std::mem::forget(identifiers_cstr);
        DeleteSearchableItemsWithIdentifiersFuture { inner: future }
    }

    /// Asynchronously delete searchable items by domain identifiers
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_searchable_items_with_domain_identifiers<I, S>(
        index: &CSSearchableIndex,
        domain_identifiers: I,
    ) -> DeleteSearchableItemsWithDomainIdentifiersFuture
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let (future, ctx) = AsyncCompletion::create();
        let domain_identifiers: Vec<String> =
            domain_identifiers.into_iter().map(Into::into).collect();
        let domain_identifiers_json = match serde_json::to_string(&domain_identifiers) {
            Ok(json) => json,
            Err(e) => {
                let err_msg = format!("Failed to serialize domain identifiers: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return DeleteSearchableItemsWithDomainIdentifiersFuture { inner: future };
            }
        };

        let domain_identifiers_cstr = match std::ffi::CString::new(domain_identifiers_json) {
            Ok(cstr) => cstr,
            Err(e) => {
                let err_msg = format!("Failed to create C string: {e}");
                // SAFETY: ctx is a valid AsyncCompletion context pointer from AsyncCompletion::create()
                unsafe {
                    AsyncCompletion::<()>::complete_err(ctx, err_msg);
                }
                return DeleteSearchableItemsWithDomainIdentifiersFuture { inner: future };
            }
        };

        // SAFETY: index.as_ptr() is a valid CoreSpotlight index pointer, domain_identifiers_cstr.as_ptr() is a valid C string,
        // and delete_searchable_items_with_domain_identifiers_cb is a valid callback function that will be called exactly once with ctx as user_data
        unsafe {
            crate::ffi::corespotlight_delete_searchable_items_with_domain_identifiers_async(
                index.as_ptr(),
                domain_identifiers_cstr.as_ptr(),
                30,
                delete_searchable_items_with_domain_identifiers_cb,
                ctx,
            );
        }

        // Leak the CString into the callback; it will be freed by the Swift side
        std::mem::forget(domain_identifiers_cstr);
        DeleteSearchableItemsWithDomainIdentifiersFuture { inner: future }
    }

    /// Asynchronously delete all searchable items
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_all_searchable_items(
        index: &CSSearchableIndex,
    ) -> DeleteAllSearchableItemsFuture {
        let (future, ctx) = AsyncCompletion::create();

        // SAFETY: index.as_ptr() is a valid CoreSpotlight index pointer,
        // and delete_all_searchable_items_cb is a valid callback function that will be called exactly once with ctx as user_data
        unsafe {
            crate::ffi::corespotlight_delete_all_searchable_items_async(
                index.as_ptr(),
                delete_all_searchable_items_cb,
                ctx,
            );
        }

        DeleteAllSearchableItemsFuture { inner: future }
    }

    /// Asynchronously fetch the last client state (macOS 13+)
    ///
    /// # Errors
    ///
    /// Returns an error if fetching fails or on unsupported platforms.
    pub fn fetch_last_client_state(index: &CSSearchableIndex) -> FetchLastClientStateFuture {
        let (future, ctx) = AsyncCompletion::create();

        // SAFETY: index.as_ptr() is a valid CoreSpotlight index pointer,
        // and fetch_last_client_state_cb is a valid callback function that will be called exactly once with ctx as user_data
        unsafe {
            crate::ffi::corespotlight_fetch_last_client_state_async(
                index.as_ptr(),
                fetch_last_client_state_cb,
                ctx,
            );
        }

        FetchLastClientStateFuture { inner: future }
    }
}
