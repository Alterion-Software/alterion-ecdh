<div align="center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="assets/logo-dark.png">
        <source media="(prefers-color-scheme: light)" srcset="assets/logo-light.png">
        <img alt="Alterion Logo" src="assets/logo-dark.png" width="400">
    </picture>
</div>

<div align="center">

[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/alterion-rsa-key-manager.svg)](https://crates.io/crates/alterion-rsa-key-manager)
[![Rust](https://img.shields.io/badge/Rust-2024-orange?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![RSA-2048](https://img.shields.io/badge/RSA--2048-OAEP--SHA256-blue?style=flat)](https://docs.rs/rsa)
[![GitHub](https://img.shields.io/badge/GitHub-Alterion--Software-181717?style=flat&logo=github&logoColor=white)](https://github.com/Alterion-Software)

_RSA-2048 key store with timed rotation, a 300-second grace window, and OAEP-SHA256 decryption — designed as the key management layer for the [alterion-enc-pipeline](https://crates.io/crates/alterion-enc-pipeline)._

---

</div>

## What it does

Manages a live RSA-2048 key pair that rotates automatically on a configurable interval. A 300-second grace window keeps the previous key valid after rotation so any in-flight request encrypted just before a rotation still decrypts successfully.

```
┌─────────────────────────────────────────────┐
│                  KeyStore                    │
│  current  ──→  active RSA key pair           │
│  previous ──→  retiring key (≤300s grace)    │
└─────────────────────────────────────────────┘
```

Decryption automatically falls back to the previous key within its grace window and returns `RsaError::KeyExpired` once the window closes.

---

## Quick start

### 1. Add the dependency

```toml
[dependencies]
alterion-rsa-key-manager = "0.1"
```

### 2. Initialise and rotate

```rust
use alterion_rsa_key_manager::{init_key_store, start_rotation};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Rotate every hour; previous key stays live for 5 minutes.
    let store = init_key_store(3600);
    start_rotation(store.clone(), 3600);

    // pass `store` into your application state
    Ok(())
}
```

### 3. Expose the public key to clients

```rust
use actix_web::{get, web, HttpResponse};
use alterion_rsa_key_manager::{KeyStore, get_current_public_key};
use std::sync::Arc;
use tokio::sync::RwLock;

#[get("/api/pubkey")]
async fn public_key_handler(
    store: web::Data<Arc<RwLock<KeyStore>>>,
) -> HttpResponse {
    let (key_id, pem) = get_current_public_key(&store).await;
    HttpResponse::Ok().json(serde_json::json!({ "key_id": key_id, "public_key": pem }))
}
```

### 4. Decrypt an RSA-OAEP-SHA256 ciphertext

```rust
use alterion_rsa_key_manager::decrypt;

let plaintext = decrypt(&store, &key_id, &ciphertext).await?;
// plaintext is Zeroizing<Vec<u8>> — memory is wiped on drop
```

---

## API

| Function | Description |
|---|---|
| `init_key_store(interval_secs)` | Generates the initial RSA-2048 key pair, returns `Arc<RwLock<KeyStore>>` |
| `start_rotation(store, interval_secs)` | Spawns a background task that rotates the key every `interval_secs` seconds |
| `get_current_public_key(store)` | Returns `(key_id, pem)` for the active key |
| `decrypt(store, key_id, cdata)` | RSA-OAEP-SHA256 decrypts `cdata`, falling back to the previous key within its grace window |

### `RsaError`

| Variant | Meaning |
|---|---|
| `KeyExpired` | The `key_id` is unknown or its grace window has closed |
| `DecryptionFailed(String)` | OAEP decryption failed (bad ciphertext or wrong key) |
| `KeyGenerationFailed(String)` | RSA key generation failed at startup or rotation |

---

## Grace window

The previous key remains valid for **300 seconds** after rotation. Pre-fetch a new public key on the client at `rotation_interval − 300` seconds to ensure the cached key is never stale when a rotation occurs.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Open an issue before writing any code.

---

## License

GNU General Public License v3.0 — see [LICENSE](LICENSE).

---

<div align="center">

**Made with ❤️ by the Alterion Software team**

[![Discord](https://img.shields.io/badge/Discord-Join-5865F2?style=flat&logo=discord&logoColor=white)](https://discord.com/invite/3gy9gJyJY8)
[![Website](https://img.shields.io/badge/Website-Coming%20Soon-blue?style=flat&logo=globe&logoColor=white)](.)
[![GitHub](https://img.shields.io/badge/GitHub-Alterion--Software-181717?style=flat&logo=github&logoColor=white)](https://github.com/Alterion-Software)

</div>
