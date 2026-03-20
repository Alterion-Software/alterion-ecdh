// SPDX-License-Identifier: GPL-3.0
//! # alterion-ecdh
//!
//! X25519 ECDH key store with timed rotation, a 300-second grace window, and HKDF-SHA256
//! session key derivation — the key exchange layer for the
//! [alterion-enc-pipeline](https://crates.io/crates/alterion-enc-pipeline).
//!
//! ## Example
//!
//! ```rust,no_run
//! use alterion_ecdh::{init_key_store, start_rotation, get_current_public_key, ecdh};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Rotate keys every hour; grace window keeps the previous key live for 5 minutes.
//!     let store = init_key_store(3600);
//!     start_rotation(store.clone(), 3600);
//!
//!     // Serve the current public key to clients so they can build WrappedPackets.
//!     let (key_id, public_key_b64) = get_current_public_key(&store).await;
//!
//!     // On an incoming request: perform ECDH with the client's ephemeral key.
//!     let client_pk: [u8; 32] = [0u8; 32]; // received from client
//!     let (shared_secret, server_pk) = ecdh(&store, &key_id, &client_pk).await.unwrap();
//!     // Pass shared_secret + both public keys to HKDF to derive enc/mac session keys.
//! }
//! ```

pub mod keystore;

pub use keystore::{
    KeyStore, KeyEntry, EcdhError,
    init_key_store, start_rotation, get_current_public_key, ecdh,
};
