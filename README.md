# Gigafold

Gigafold is a cross-platform, ultra-high-performance zero-knowledge encrypted desktop vault application. Built entirely with Rust and Tauri, it provides secure virtual file management and seamless cloud synchronization without the operational overhead, kernel dependencies, or elevated privileges required by traditional virtual drive solutions (FUSE/WinFSP).

---

## 1. System Architecture

Gigafold is designed around a decoupled, event-driven architecture that separates the web-standard presentation layer from the high-throughput, memory-safe system core.


```

┌──────────────────────────────────────────────────────────────────┐
│                      Frontend Layer (React)                      │
└─────────────────────────────────┬────────────────────────────────┘
│
Tauri IPC Bridge / VFS
│
┌─────────────────────────────────▼────────────────────────────────┐
│                   Storage Core Layer (Rust)                      │
│                                                                  │
│  ┌───────────────────────┐              ┌─────────────────────┐  │
│  │    Crypto Pipeline    │              │   Indexing Engine   │  │
│  │ (XChaCha20-Poly1305)  │              │    (Embedded sled)  │  │
│  └───────────┬───────────┘              └──────────┬──────────┘  │
└──────────────┼─────────────────────────────────────┼─────────────┘
│                                     │
┌──────────────▼─────────────────────────────────────▼─────────────┐
│                      Storage Abstraction Layer                     │
│                (object_store: Local / Cloud Blobs)               │
└──────────────────────────────────────────────────────────────────┘

```

### 1.1 Core Component Layers
*   **User Interface (`src/`):** A modern desktop interface built with React, TypeScript, and Vite. It handles directory graph traversal, interactive file operations, and real-time state synchronization, interfacing with the backend via asynchronous Tauri IPC bridges.
*   **Virtual Filesystem (`src-tauri/src/fs/`):** A logical VFS layer that translates abstract UI interactions into transactionally sound state modifications without interacting with the OS kernel driver space.
*   **Cryptography Pipeline (`src-tauri/src/crypto.rs`):** The cryptographic foundation handling parameter-hardened key derivation, cryptographic nonce safety, and low-level data transformation primitives.
*   **Storage Core & Pipeline (`src-tauri/src/storage.rs`):** Exposes streaming operations using dynamic trait objects (`&mut dyn Read`) to apply authenticated encryption algorithms dynamically to arbitrarily sized IO blocks.
*   **Metadata Indexing (`src-tauri/src/index/`):** An embedded, crash-safe `sled` key-value engine mapping user-facing virtual logical trees to deterministic cryptographically bound chunk graphs with strict ACID guarantees.

---

## 2. Zero-Knowledge Cryptographic Spec

Gigafold guarantees absolute zero-knowledge confidentiality. The host OS and cloud storage providers read only opaque binary structures and uniform metadata markers.

*   **Key Derivation Function (KDF):** Master encryption keys are derived dynamically from user passphrases using hardware-hardened **Argon2id**. This mitigates off-chip custom ASIC/GPU massive parallel dictionary attacks through strict memory and time boundaries.
*   **Authenticated Encryption:** Payloads undergo encryption using **XChaCha20-Poly1305** streaming profiles. The extended 192-bit nonce profile of XChaCha20 guarantees structural protection against reuse collisions across billions of generated chunks.
*   **Integrity Verification:** Every distinct data segment features an inline Poly1305 MAC tag verified dynamically before reaching the application buffer. Any external file modification or bit-flipping attack triggers immediate decryption faults, isolating the client from corrupt data payloads.

---

## 3. Data Flow Pipelines

### 3.1 Ingestion / Write Operations
1.  **IPC Dispatch:** The Frontend pushes an execution payload specifying the source data pointer across the Tauri bridge boundary.
2.  **Streaming Chunking:** The storage tier splits incoming unstructured file streams into uniform, fixed-size operational segments.
3.  **Dynamic Transformation:** Segments run through the `ChunkEncryptor` stream engine, utilizing an automated lookahead buffer to differentiate standard blocks from the final block structure (`encrypt_next` vs `encrypt_last`).
4.  **Transactional Indexing:** Nonces, authenticated tags, and target checksum signatures write to the local persistent `sled` engine.
5.  **Persistence:** The abstracted `object_store` layer routes the resulting authenticated payload out to the configured target backend.

### 3.2 Recovery / Read Operations
1.  **Graph Target Lookup:** The Virtual Filesystem fetches the targeted chunk topology, specific sequence maps, and stored nonce arrays from `sled`.
2.  **Parallel Egress:** The storage abstraction layer fetches corresponding encrypted data streams concurrently from the object storage targets.
3.  **Stream Authentication:** The `ChunkEncryptor` processes raw blocks via dynamic streaming trait objects, verifying the Poly1305 integrity tags before outputting cleartext.
4.  **IPC Return:** Cleansed plain data streams route straight back through the memory-safe IPC runtime into the frontend layout buffer.

---

## 4. Technical Stack

*   **System Core:** Rust (2021 Edition)
*   **Runtime Framework:** Tauri v2 (Async Tokio Engine)
*   **UI Architecture:** React, TypeScript, Vite
*   **Database Engine:** `sled` embedded persistent key-value store
*   **Cloud Abstraction:** `object_store` multi-backend layer
*   **Cryptographic Primitives:** `argon2`, `chacha20poly1305` (Stream extensions enabled)

---

## 5. Development Roadmap

### Phase I: Foundational Cryptography & Indexing (Complete ✓)
*   [x] Architect Argon2id secure passphrase key derivation manager.
*   [x] Build dynamic XChaCha20-Poly1305 stream-level chunk encryption pipeline.
*   [x] Implement unified unit-testing runtime covering memory-safe cryptographic pipelines.

### Phase II: Storage & Structural Indexing (In Progress ⏳)
*   [ ] Integrate the embedded `sled` database schema to track complex multi-chunk chains.
*   [ ] Build the fixed-size file chunking streaming split mechanics.
*   [ ] Wire up multi-target capabilities via `object_store` (Local FS and AWS S3).

### Phase III: Tauri Bridges & Virtual Frontend
*   [ ] Map out Tauri asynchronous IPC command handlers for secure backend bridging.
*   [ ] Develop the React file explorer tree UI.
*   [ ] Design the background Tokio worker pool to manage vault state tracking and synchronization.

---

## 6. License

This project is licensed under the Apache-2.0 License. See the LICENSE file for details.
