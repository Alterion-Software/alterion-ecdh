// SPDX-License-Identifier: GPL-3.0
//! # alterion-rsa-key-manager
//!
//! RSA-2048 key store with timed rotation, a 300-second grace window, and OAEP-SHA256 decryption.
//!
//! ## Quick start
//! ```rust,no_run
//! use alterion_rsa_key_manager::{init_key_store, start_rotation, get_current_public_key, decrypt};
//!
//! #[tokio::main]
//! async fn main() {
//!     let store = init_key_store(3600);
//!     start_rotation(store.clone(), 3600);
//!     let (key_id, pem) = get_current_public_key(&store).await;
//! }
//! ```

pub mod keystore;

pub use keystore::{
    KeyStore, KeyEntry, RsaError,
    init_key_store, start_rotation, get_current_public_key, decrypt,
};
