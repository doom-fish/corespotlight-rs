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
use crate::ffi;
use crate::index::CSSearchableIndex;
use crate::item::CSSearchableItem;
use crate::private::{error_from_payload_json, json_cstring, parse_json_str};
use doom_fish_utils::completion::{AsyncCompletion, AsyncCompletionFuture};
use doom_fish_utils::panic_safe::catch_user_panic;
use std::ffi::{c_char, c_void, CStr};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

type Outcome<T> = Result<T, CoreSpotlightError>;

unsafe fn callback_error(error: *const c_char) -> CoreSpotlightError {
    error_from_payload_json(&CStr::from_ptr(error).to_string_lossy())
}

fn flatten<T>(result: Result<Outcome<T>, String>) -> Outcome<T> {
    result.unwrap_or_else(|message| Err(CoreSpotlightError::bridge(-2, message)))
}

fn start<T>(
    begin: impl FnOnce(*mut c_void) -> Outcome<()>,
) -> AsyncCompletionFuture<Outcome<T>> {
    let (future, ctx) = AsyncCompletion::<Outcome<T>>::create();
    if let Err(error) = begin(ctx) {
        unsafe { AsyncCompletion::<Outcome<T>>::complete_ok(ctx, Err(error)) };
    }
    future
}

extern "C" fn unit_completion_cb(_result: *const c_void, error: *const c_char, user_data: *mut c_void) {
    catch_user_panic("corespotlight async completion", || {
        let outcome = if error.is_null() {
            Ok(())
        } else {
            Err(unsafe { callback_error(error) })
        };
        unsafe { AsyncCompletion::<Outcome<()>>::complete_ok(user_data, outcome) };
    });
}

// ============================================================================
// IndexSearchableItems
// ============================================================================

/// Future for async index_searchable_items
///
/// # Thread Safety
///
/// This future is `Send + Sync` because it wraps `AsyncCompletionFuture<()>` which uses
/// `Arc<Mutex<_>>` for synchronization. The future can be safely moved between threads
/// and shared across threads.
pub struct IndexSearchableItemsFuture {
    inner: AsyncCompletionFuture<Outcome<()>>,
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
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

// ============================================================================
// DeleteSearchableItemsWithIdentifiers
// ============================================================================

/// Future for async delete_searchable_items_with_identifiers
pub struct DeleteSearchableItemsWithIdentifiersFuture {
    inner: AsyncCompletionFuture<Outcome<()>>,
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
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

// ============================================================================
// DeleteSearchableItemsWithDomainIdentifiers
// ============================================================================

/// Future for async delete_searchable_items_with_domain_identifiers
pub struct DeleteSearchableItemsWithDomainIdentifiersFuture {
    inner: AsyncCompletionFuture<Outcome<()>>,
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
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

// ============================================================================
// DeleteAllSearchableItems
// ============================================================================

/// Future for async delete_all_searchable_items
pub struct DeleteAllSearchableItemsFuture {
    inner: AsyncCompletionFuture<Outcome<()>>,
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
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

// ============================================================================
// FetchLastClientState
// ============================================================================

/// Callback for fetch_last_client_state completion
extern "C" fn fetch_last_client_state_cb(
    result: *const c_void,
    error: *const c_char,
    user_data: *mut c_void,
) {
    catch_user_panic("fetch_last_client_state_cb", || {
        let outcome = if !error.is_null() {
            Err(unsafe { callback_error(error) })
        } else if result.is_null() {
            Err(CoreSpotlightError::bridge(
                -2,
                "Core Spotlight returned no client state payload",
            ))
        } else {
            let json = unsafe { CStr::from_ptr(result.cast::<c_char>()) }.to_string_lossy();
            parse_json_str::<Vec<u8>>(&json, "last client state")
        };
        unsafe { AsyncCompletion::<Outcome<Vec<u8>>>::complete_ok(user_data, outcome) };
    });
}

/// Future for async fetch_last_client_state
pub struct FetchLastClientStateFuture {
    inner: AsyncCompletionFuture<Outcome<Vec<u8>>>,
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
        Pin::new(&mut self.inner).poll(cx).map(flatten)
    }
}

// ============================================================================
// AsyncCSSearchableIndex - Async wrapper for CSSearchableIndex
// ============================================================================

/// Async wrapper for [`CSSearchableIndex`]
///
/// Provides async methods for indexing and managing searchable items
/// without blocking. Wraps `CSSearchableIndex` completion-handler APIs and
/// remains executor-agnostic.
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
        let payload = items
            .iter()
            .map(|item| item.as_ptr() as usize as u64)
            .collect::<Vec<_>>();
        IndexSearchableItemsFuture {
            inner: start(|ctx| {
                let items_json = json_cstring(&payload, "searchable item pointers")?;
                unsafe {
                    ffi::corespotlight_index_searchable_items_async(
                        index.as_ptr(),
                        items_json.as_ptr(),
                        30,
                        unit_completion_cb,
                        ctx,
                    );
                }
                Ok(())
            }),
        }
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
        let identifiers: Vec<String> = identifiers.into_iter().map(Into::into).collect();
        DeleteSearchableItemsWithIdentifiersFuture {
            inner: start(|ctx| {
                let identifiers_json = json_cstring(&identifiers, "identifier array")?;
                unsafe {
                    ffi::corespotlight_delete_searchable_items_with_identifiers_async(
                        index.as_ptr(),
                        identifiers_json.as_ptr(),
                        30,
                        unit_completion_cb,
                        ctx,
                    );
                }
                Ok(())
            }),
        }
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
        let domain_identifiers: Vec<String> =
            domain_identifiers.into_iter().map(Into::into).collect();
        DeleteSearchableItemsWithDomainIdentifiersFuture {
            inner: start(|ctx| {
                let domain_identifiers_json =
                    json_cstring(&domain_identifiers, "domain identifier array")?;
                unsafe {
                    ffi::corespotlight_delete_searchable_items_with_domain_identifiers_async(
                        index.as_ptr(),
                        domain_identifiers_json.as_ptr(),
                        30,
                        unit_completion_cb,
                        ctx,
                    );
                }
                Ok(())
            }),
        }
    }

    /// Asynchronously delete all searchable items
    ///
    /// # Errors
    ///
    /// Returns an error if deletion fails.
    pub fn delete_all_searchable_items(
        index: &CSSearchableIndex,
    ) -> DeleteAllSearchableItemsFuture {
        DeleteAllSearchableItemsFuture {
            inner: start(|ctx| {
                unsafe {
                    ffi::corespotlight_delete_all_searchable_items_async(
                        index.as_ptr(),
                        unit_completion_cb,
                        ctx,
                    );
                }
                Ok(())
            }),
        }
    }

    /// Asynchronously fetch the last client state (macOS 13+)
    ///
    /// # Errors
    ///
    /// Returns an error if fetching fails or on unsupported platforms.
    pub fn fetch_last_client_state(index: &CSSearchableIndex) -> FetchLastClientStateFuture {
        FetchLastClientStateFuture {
            inner: start(|ctx| {
                unsafe {
                    ffi::corespotlight_fetch_last_client_state_async(
                        index.as_ptr(),
                        fetch_last_client_state_cb,
                        ctx,
                    );
                }
                Ok(())
            }),
        }
    }
}
