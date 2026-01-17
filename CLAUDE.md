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

- **Entry Point**: `src/main.rs` - Initializes eframe with native options and image loaders
- **State Management**: `src/app/mod.rs` - Single `State` struct implements `eframe::App`, holds `current_route` for navigation
- **Routing**: `src/app/routes.rs` - Enum-based routing (`Home`, `PrintTicket`, `Refund`, `Seats`) with a `router()` function that dispatches to views

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

### Key Patterns

- **Button Component** (`components/menu/button.rs`): Custom struct with icon path, route target, color, and active state - uses `rect_is_clicked()` utility for click detection
- **Asset Embedding**: All SVG assets in `src/assets/` are embedded at compile time via `include_image!()` macro
- **Color Scheme**: Primary blue `rgb(56, 67, 228)`, light background `rgb(246, 246, 246)`

### Special Cases

- `Seats` view renders without the menu component (different layout)
- Header always renders via `TopBottomPanel`, content via `CentralPanel`
