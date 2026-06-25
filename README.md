
```
mimir/
├── Cargo.toml                  # workspace root
├── crates/
│   ├── mimir-core/             # the ABS-compatible server
│   │   ├── Cargo.toml
│   │   ├── migrations/         # sqlx/sqlite migrations
│   │   └── src/
│   │       ├── main.rs
│   │       ├── api/            # auth.rs, libraries.rs, items.rs, session.rs
│   │       ├── db/             # models, queries
│   │       ├── scanner/        # library scanning, metadata extraction
│   │       └── sync/           # whisper-rs alignment job, sync map generation
│   │
│   ├── mimir-client/           # shared Rust crate (API client + playback state)
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── api.rs          # ABS-compatible endpoint calls
│   │   │   ├── state.rs        # playback state machine
│   │   │   └── ffi.rs          # uniffi exposed surface
│   │   └── mimir-client.udl    # uniffi interface definition
│   │
│   └── mimir-common/           # shared DTOs/types used by both core and client
│       ├── Cargo.toml
│       └── src/lib.rs
│
├── apps/
│   ├── tauri/                  # desktop: Windows/Linux/macOS
│   │   ├── src-tauri/          # Rust backend, Cargo.toml as a workspace member
│   │   └── src/                # Svelte frontend
│   │
│   ├── apple/                  # SwiftUI: macOS + iOS, shared codebase
│   │   ├── Mimir.xcodeproj
│   │   ├── Mimir/              # SwiftUI sources
│   │   └── Generated/          # uniffi-generated Swift bindings (gitignored or checked in)
│   │
│   └── android/                # Kotlin/Compose
│       ├── app/
│       └── generated/          # uniffi-generated Kotlin bindings
│
├── packages/ui
├── scripts/
│   └── gen-bindings.sh         # runs uniffi-bindgen for swift/kotlin targets
│
├── docs/
└── README.md
```
