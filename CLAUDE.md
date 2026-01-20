# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Development build and run
cargo run

# Release build (optimized with LTO)
cargo build --release

# Check compilation without building
cargo check
```

## Architecture Overview

This is a **railway ticket kiosk** desktop application built with **egui** (immediate mode GUI) and **eframe** for cross-platform native windowing.

### Core Structure

- **Entry Point**: `src/main.rs` - Initializes eframe with native options, image loaders, and cleanup of old update binaries
- **State Management**: `src/app/mod.rs` - Single `State` struct implements `eframe::App`, holds `current_route`, `language`, and `update_status`
- **Routing**: `src/app/routes.rs` - Enum-based routing (`Home`, `PrintTicket`, `Refund`, `Seats`) with a `router()` function that dispatches to views
- **Auto-Updater**: `src/updater/mod.rs` - Secure self-update with Ed25519 signature verification
- **Internationalization**: `src/app/i18n/` - Translation system with `t(lang, key)` function

### Component Pattern

Components follow a consistent `show()` function pattern:
```rust
pub fn show(ctx: &egui::Context, state: &mut State) {
    // Render UI
}
```

### Layout Structure

- **Header** (`components/header/`): Three-column layout (20%/60%/20%) with logo, call center branding, and language/restart controls
- **Menu** (`components/menu/`): Navigation buttons rendered for all routes except `Seats`
- **Views** (`views/`): Page content - each route maps to a view module

### Key Modules

#### Updater (`src/updater/mod.rs`)
Self-update system with:
- `check_for_update(url)` - Checks remote JSON for newer version
- `download_update(info)` - Downloads binary and verifies Ed25519 signature
- `install_and_restart(path)` - Replaces current binary and restarts app
- `cleanup_old_binary()` - Removes `.old` binary from previous update (called at startup)

Public key is embedded from `keys/public.key` at compile time.

#### Internationalization (`src/app/i18n/`)
- `t(lang, key)` - Returns translated string for given language and key
- Supported languages: `Language::Turkmen`, `Language::Russian`
- Translations stored in `phf_map!` static hashmaps for O(1) lookup

#### Constants (`src/app/constants.rs`)
Ant Design-inspired color palette with semantic naming:
- Background: `BG`, `BG_1` through `BG_5`, `BG_DIM`, `BG_LIGHT`
- Foreground: `FG`, `FG_MUTED`, `FG_DISABLED`, `FG_PLACEHOLDER`
- Primary/Secondary: `PRIMARY`, `PRIMARY_HOVER`, `PRIMARY_ACTIVE`, `SECONDARY`, etc.
- Semantic: `SUCCESS`, `WARNING`, `ERROR`, `INFO` (each with `_HOVER`, `_ACTIVE`, `_BG`, `_BORDER`)
- Component aliases: `BTN_PRIMARY_*`, `INPUT_*`, `CARD_*`, `HEADER_BG`, etc.

### Key Patterns

- **Button Component** (`components/menu/button.rs`): Custom struct with icon path, route target, color, and active state - uses `rect_is_clicked()` utility for click detection
- **Asset Embedding**: All SVG assets in `src/assets/` are embedded at compile time via `include_image!()` macro
- **Background Updates**: Update downloads run in background thread, result received via `mpsc` channel in `State.update_receiver`

### State Fields

```rust
pub struct State {
    pub current_route: Route,      // Current page
    pub language: Language,        // Selected UI language
    pub update_status: UpdateStatus,  // Idle | Checking | Downloading
    pub update_receiver: Option<mpsc::Receiver<Option<PathBuf>>>,  // Update result channel
}
```

### Special Cases

- `Seats` view renders without the menu component (different layout)
- Header always renders via `TopBottomPanel`, content via `CentralPanel`
- Old binary cleanup happens once at app startup before UI initialization
