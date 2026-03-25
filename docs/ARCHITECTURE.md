# Architecture

## Goals

- Keep UI entrypoints short and readable.
- Isolate drawing math from window chrome and file operations.
- Make export logic independent from runtime widget layout.
- Keep domain and persistence free from `egui` where possible.

## Current layout

### `src/app`

- `state.rs` — application state, runtime/editor session state, autosave hooks.

### `src/domain`

- `document.rs` — document-level operations.
- `tab.rs` — tab model.
- `stroke.rs` — drawing primitives and stored strokes.

### `src/persistence`

- `json.rs` — document serialization.
- `session.rs` — autosaved session format and loading/saving.

### `src/ui`

- `main_window/`
  - `mod.rs` — top-level orchestration for the main window.
  - `layout.rs` — menu, toolbar, tabs, status bar, central canvas.
  - `actions.rs` — user actions, shortcuts, open/save/export flows.
  - `export_png.rs` — PNG rendering of the current canvas view.
- `canvas/`
  - `mod.rs` — public canvas API.
  - `constants.rs` — shared canvas sizing constants.
  - `geometry.rs` — world/screen/grid conversions.
  - `render.rs` — grid and stroke rendering.
  - `input.rs` — draw/erase interaction and undo/redo.
- `tabs.rs` — tab strip rendering and rename/delete behavior.
- `toolbar.rs` — drawing tool controls.
- `paint_palette.rs` — shared draw palette for runtime UI and export.

## Structural rules

- UI orchestration files should stay under ~150 lines.
- Rendering and interaction stay split.
- File dialogs and serialization stay out of rendering modules.
- Shared drawing constants/palette live in dedicated modules, not duplicated.
- New features should first choose the correct layer:
  - domain change
  - persistence change
  - canvas rendering/input
  - window-level action/layout

## Next refactor targets

- Split `tabs.rs` into strip/layout and rename editor helpers if it grows further.
- Move localization keys to typed constants if the menu surface grows.
- Introduce integration tests for document/session roundtrips.
