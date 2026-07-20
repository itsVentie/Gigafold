# Gigafold

Gigafold is a high-performance, encrypted desktop vault application built with Rust and Tauri. It provides zero-knowledge cloud synchronization and a secure virtual file management interface without the need for elevated privileges or kernel-level drivers (FUSE/WinFSP). The system is architected to minimize memory overhead while maintaining strict data integrity and confidentiality.

## 1. System Architecture

Gigafold follows a modular, decoupled design pattern separating the visual presentation layer from the high-performance security core.

### 1.1. Core Layers

* **Frontend Interface (`src/`)**: A cross-platform UI built with React and TypeScript. It handles user interactions, folder navigation, and system status visualization. Communication with the backend is established via secure, asynchronous Tauri IPC bridges.
* **Virtual Filesystem (`src-tauri/src/fs/`)**: A logical VFS layer that translates frontend actions into structural storage operations. It manages the virtual directory tree representation without mounting onto the host OS.
* **Cryptography Engine (`src-tauri/src/crypto/`)**: Performs all client-side encryption and decryption. It handles key generation, cryptographic nonces, and authenticated encryption primitives.
* **Storage Abstraction (`src-tauri/src/storage/`)**: Chunks data streams into fixed-size segments and interfaces with local or remote storage backends (S3, GCS, Local Filesystem) using `object_store`.
* **Metadata Indexing (`src-tauri/src/index/`)**: Uses an embedded `sled` database to map logical file paths to encrypted chunk chains, sizes, and specific cryptographic parameters with atomic guarantees.
* **Data Models (`src-tauri/src/models/`)**: Defines the serialized definitions for internal primitives, configuration shapes, and state tracking structures.

## 2. Security Design

Gigafold operates strictly on a zero-knowledge security boundary:

* **Zero-Knowledge Implementation**: All cryptographic operations occur exclusively within the client-side Rust application runtime. Storage providers only receive opaque, encrypted binary blobs and anonymous hashes.
* **Encryption Standard**: Data payloads are encrypted using XChaCha20-Poly1305. This AEAD construction protects against both unauthorized data access and active tampering or bit-flipping attempts.
* **Key Derivation**: Master encryption keys are derived from user-supplied passwords using the Argon2id parameter profile, establishing heavy resistance against GPU-accelerated brute-force attacks.
* **Integrity Verification**: Each encrypted chunk carries its corresponding Poly1305 authentication tag. Any out-of-band modification results in immediate decryption failure, ensuring malicious or corrupted chunks are rejected.

## 3. Data Lifecycle & Flow

### Write Operations
1. **IPC Call**: Frontend sends a file ingestion request along with the path through the Tauri bridge.
2. **Chunking**: The storage layer splits the target file stream into uniform fixed-size chunks.
3. **Encryption**: The crypto engine encrypts each individual chunk using XChaCha20-Poly1305 with a unique, cryptographically secure random nonce.
4. **Index Commit**: The indexing engine saves the structural metadata (chunk hashes, nonces, sequential offsets) into the local `sled` database.
5. **Persist**: The storage backend pushes the encrypted data segments to the defined storage provider.

### Read Operations
1. **Index Lookup**: The frontend triggers a read request. The system queries `sled` to pull the precise sequence of chunk hashes and nonces tied to the logical file.
2. **Retrieval**: The storage engine downloads the required encrypted segments from the backend.
3. **Decryption**: The crypto engine verifies the integrity tags and decrypts the segments back into plaintext.
4. **Streaming**: The raw data stream is safely returned to the UI layer or exported to the local host environment.

## 4. Technical Specifications

* **Language**: Rust 2021 Edition
* **Frontend Stack**: React, TypeScript, Vite
* **Application Shell**: Tauri v2
* **Async Runtime**: Tokio
* **Storage Layer**: `object_store`
* **Local Index**: `sled` persistent key-value store
* **Key Lifecycle**: Ephemeral; retained purely in-memory during active process sessions

## 5. Implementation Roadmap

### Phase I: Core Cryptography & Indexing (Current)
* Implement the Argon2id key derivation manager.
* Finalize the XChaCha20-Poly1305 chunk encryption pipeline.
* Integrate the embedded `sled` database schema for metadata tracking.

### Phase II: Storage & Chunks
* Create the fixed-size file chunking implementation.
* Integrate the `object_store` multi-backend layer for local and initial S3 targets.

### Phase III: Tauri Integration & UI
* Establish the complete Tauri IPC command system map.
* Build the React virtual file navigation view.
* Write the background synchronization worker thread for status tracking.

## 6. License

This project is licensed under the Apache-2.0 License.