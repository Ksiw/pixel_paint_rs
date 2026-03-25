# Release packaging

Основной скрипт:

```bash
scripts/release/build_release_artifacts.sh
```

Он складывает артефакты в `dist/`.

## Что делает

- собирает native Linux release
- делает `deb`
- делает `pkg.tar.zst` для Arch
- пытается собрать `aarch64` Linux, если установлен target и есть `aarch64-linux-gnu-gcc`
- пытается собрать Windows `x86_64-pc-windows-gnu`, если есть нужный target и `x86_64-w64-mingw32-gcc`
- делает Windows `zip`
- делает Windows installer через `makensis`, если `makensis` установлен

## Быстрые режимы

Только native Linux:

```bash
scripts/release/build_release_artifacts.sh --native-only
```

Упаковка только из уже существующих бинарников:

```bash
scripts/release/build_release_artifacts.sh --skip-build
```

## Требования для полного прогона

### Debian / Arch x86_64

- `cargo`
- `dpkg-deb`
- `tar`
- `zstd`

### Linux aarch64

- `rustup target add aarch64-unknown-linux-gnu`
- `aarch64-linux-gnu-gcc`

### Windows x86_64 installer

- `rustup target add x86_64-pc-windows-gnu`
- `x86_64-w64-mingw32-gcc`
- `makensis`

## Выходные файлы

Ожидаемые имена:

- `dist/pixel-paint-rs_<version>_amd64.deb`
- `dist/pixel-paint-rs-<version>-1-x86_64.pkg.tar.zst`
- `dist/pixel-paint-rs_<version>_arm64.deb`
- `dist/pixel-paint-rs-<version>-1-aarch64.pkg.tar.zst`
- `dist/pixel-paint-rs-<version>-windows-x86_64.zip`
- `dist/pixel-paint-rs-<version>-windows-x86_64-setup.exe`
