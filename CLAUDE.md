# Kiosk

Railway ticket kiosk terminal app built with Rust + egui.

## Build & Check

```sh
cargo clippy          # lint (must pass clean, no warnings)
cargo build           # build all crates
cargo run -p ui       # run the kiosk app
```

No test suite yet. Always run `cargo clippy` after changes.

## Architecture

Cargo workspace with three crates:

- `crates/core` (kiosk-core) — error types, logger, config constants, base64 decoder
- `crates/api` (kiosk-api) — ureq-based HTTP client (blocking, no tokio), stations fetch, updater
- `crates/ui` (kiosk-ui) — binary crate, eframe/egui GUI, rendering, state, i18n, theme

Dependency graph: `core` → `api` → `ui`

## Key Conventions

### State
- State lives in `crates/ui/src/state/mod.rs`, sub-modules are `pub mod`
- Data-oriented: pub fields, no getters/setters unless enforcing invariants
- Invariant examples: `calendar::State` keeps `one_way_date` private (setter bumps round_trip); `stations::State` keeps async fields private

### i18n
- Two languages: Turkmen, Russian
- `i18n::Language` enum directly on State (not wrapped)
- Usage: `t(&state.lang, "key")` — translations in `i18n/turkmen.rs` and `i18n/russian.rs` using `phf_map!`

### Theme & Visual Style
- Constants in `crates/ui/src/theme/` (colors, corners, alphabet)
- Named constants for all layout values — no magic numbers
- **Soft & clean design**: no hard borders on cards/panels — use `Shadow` instead of `Stroke` on `Frame` containers
- Page background: `colors::BG_DIM` (#F5F5F5), cards/panels: white with soft shadow
- Standard shadow: `Shadow { offset: [0, 2], blur: 8, spread: 0, color: colors::SHADOW }`
- For interactive elements (buttons) without `Frame`, paint shadow manually: `ui.painter().add(shadow.as_shape(rect, corners::MEDIUM))`
- Text color: use `colors::FG` (soft black #262626) — never `colors::BLACK` for UI text
- `egui::Shadow` fields are integer types: `offset: [i8; 2]`, `blur: u8`, `spread: u8`
- Buttons with colored fills (primary, success, etc.) don't need borders or shadows
- Buttons with white/neutral fills on white backgrounds: use `BG_DIM` fill or shadow for contrast

### Fonts
- Inter (regular) and Inter Bold loaded in `main.rs`
- Default proportional font: `FontFamily::Proportional` → Inter
- Bold: `FontFamily::Name("bold".into())` → InterBold
- For `RichText`: `.family(FontFamily::Name("bold".into()))`
- For painter API: `FontId::new(size, FontFamily::Name("bold".into()))`

### egui Patterns
- Create button widget before `ui.add()` — don't pass `ui` to builder and `ui.add()` in same expression
- `include_bytes!` / `include_image!` paths are relative to the source file
- Modals use `state.modal == Modal::Source` / `Modal::Destination` pattern
- Station selection: set `state.trip.selected = true` to close modal
- For painter-based interactive elements: `allocate_exact_size` → paint shadow → paint fill → paint text → handle click
- `horizontal_centered` vertically centers children but expands to available height — use `ui.set_height()` to constrain
- Font "+" renders ~1.5px below visual center — offset with `rect.center() - vec2(0.0, 1.5)`

### Config
- `crates/core/src/config.rs` — API URLs, device ID, popular station IDs, default station
- `POPULAR_STATION_IDS` and `DEFAULT_SOURCE_STATION` used by UI

## Common Pitfalls
- `ureq::Body.read_json()` requires `mut` binding
- Workspace uses `resolver = "3"` for edition 2024
- UI crate needs explicit `log` dependency for `log::debug!`/`log::error!` macros
- egui `FontDefinitions` font data requires `.into()` for `Arc<FontData>`
- egui has no native bold — use the registered `"bold"` font family
