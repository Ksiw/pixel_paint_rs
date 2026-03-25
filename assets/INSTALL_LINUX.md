## Linux install

1. Build release:

`cargo build --release`

2. Copy binary:

`install -Dm755 target/release/pixel_paint_rs ~/.local/bin/pixel_paint_rs`

3. Copy icon:

`install -Dm644 assets/icon.png ~/.local/share/icons/hicolor/256x256/apps/pixel_paint_rs.png`

4. Copy desktop entry:

`install -Dm644 assets/pixel_paint_rs.desktop ~/.local/share/applications/pixel_paint_rs.desktop`

5. Refresh desktop database if needed:

`update-desktop-database ~/.local/share/applications 2>/dev/null || true`
