#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"
DIST_DIR="${REPO_ROOT}/dist"
APP_ID="pixel_paint_rs"
PACKAGE_ID="pixel-paint-rs"
APP_TITLE="Pixel Paint RS"
MAINTAINER="Ksiw <stewhiki@gmail.com>"
URL="https://t.me/mr_Ksiw"
DESCRIPTION="Grid-based drawing app"

version() {
    sed -n 's/^version = "\(.*\)"/\1/p' "${REPO_ROOT}/Cargo.toml" | head -n1
}

host_target() {
    rustc -vV | sed -n 's/^host: //p'
}

target_env_prefix() {
    echo "$1" | tr '[:lower:]-' '[:upper:]_'
}

ensure_clean_dir() {
    local dir="$1"
    rm -rf "${dir}"
    mkdir -p "${dir}"
}

stage_linux_layout() {
    local stage_dir="$1"
    local binary_path="$2"
    install -Dm755 "${binary_path}" "${stage_dir}/usr/bin/${APP_ID}"
    install -Dm644 "${REPO_ROOT}/assets/icon.png" "${stage_dir}/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png"
    install -Dm644 "${REPO_ROOT}/assets/pixel_paint_rs.desktop" "${stage_dir}/usr/share/applications/${APP_ID}.desktop"
    install -Dm644 "${REPO_ROOT}/LICENSE" "${stage_dir}/usr/share/licenses/${PACKAGE_ID}/LICENSE"
    install -Dm644 "${REPO_ROOT}/README.md" "${stage_dir}/usr/share/doc/${PACKAGE_ID}/README.md"
}

build_target_if_possible() {
    local target="$1"
    local linker_cmd="${2:-}"

    if [[ "${target}" == "$(host_target)" ]]; then
        cargo build --release
        return 0
    fi

    if ! rustup target list --installed | grep -qx "${target}"; then
        echo "skip ${target}: rust target not installed" >&2
        return 1
    fi

    if [[ -n "${linker_cmd}" ]] && ! command -v "${linker_cmd}" >/dev/null 2>&1; then
        echo "skip ${target}: missing linker ${linker_cmd}" >&2
        return 1
    fi

    if [[ "${target}" == "x86_64-pc-windows-gnu" ]]; then
        cargo build --release --target "${target}"
        return 0
    fi

    if [[ -n "${linker_cmd}" ]]; then
        local target_env
        target_env="$(target_env_prefix "${target}")"
        env "CARGO_TARGET_${target_env}_LINKER=${linker_cmd}" cargo build --release --target "${target}"
        return 0
    fi

    cargo build --release --target "${target}"
}

binary_path_for() {
    local target="$1"
    if [[ "${target}" == "$(host_target)" ]]; then
        printf '%s/target/release/%s' "${REPO_ROOT}" "${APP_ID}"
    elif [[ "${target}" == "x86_64-pc-windows-gnu" ]]; then
        printf '%s/target/%s/release/%s.exe' "${REPO_ROOT}" "${target}" "${APP_ID}"
    else
        printf '%s/target/%s/release/%s' "${REPO_ROOT}" "${target}" "${APP_ID}"
    fi
}

deb_arch_name() {
    case "$1" in
        x86_64-unknown-linux-gnu) echo "amd64" ;;
        aarch64-unknown-linux-gnu) echo "arm64" ;;
        *) return 1 ;;
    esac
}

arch_pkg_arch_name() {
    case "$1" in
        x86_64-unknown-linux-gnu) echo "x86_64" ;;
        aarch64-unknown-linux-gnu) echo "aarch64" ;;
        *) return 1 ;;
    esac
}

rpm_arch_name() {
    case "$1" in
        x86_64-unknown-linux-gnu) echo "x86_64" ;;
        aarch64-unknown-linux-gnu) echo "aarch64" ;;
        *) return 1 ;;
    esac
}

make_deb_package() {
    local target="$1"
    local binary_path="$2"
    local arch
    arch="$(deb_arch_name "${target}")"
    local ver
    ver="$(version)"
    local stage_dir="${DIST_DIR}/stage/deb-${arch}"
    ensure_clean_dir "${stage_dir}"
    stage_linux_layout "${stage_dir}" "${binary_path}"
    mkdir -p "${stage_dir}/DEBIAN"
    cat > "${stage_dir}/DEBIAN/control" <<EOF
Package: ${PACKAGE_ID}
Version: ${ver}
Section: graphics
Priority: optional
Architecture: ${arch}
Maintainer: ${MAINTAINER}
Homepage: ${URL}
Description: ${DESCRIPTION}
 Grid-based drawing, quick schematic sketching, tabs, JSON save/load,
 session restore, and PNG export of the current visible canvas.
EOF
    dpkg-deb --root-owner-group --build "${stage_dir}" "${DIST_DIR}/${PACKAGE_ID}_${ver}_${arch}.deb"
}

make_arch_package() {
    local target="$1"
    local binary_path="$2"
    local arch
    arch="$(arch_pkg_arch_name "${target}")"
    local ver
    ver="$(version)"
    local pkgrel="1"
    local stage_dir="${DIST_DIR}/stage/arch-${arch}"
    ensure_clean_dir "${stage_dir}"
    stage_linux_layout "${stage_dir}" "${binary_path}"
    local installed_size
    installed_size="$(du -sb "${stage_dir}" | awk '{print $1}')"
    cat > "${stage_dir}/.PKGINFO" <<EOF
pkgname = ${PACKAGE_ID}
pkgbase = ${PACKAGE_ID}
pkgver = ${ver}-${pkgrel}
pkgdesc = ${DESCRIPTION}
url = ${URL}
builddate = $(date +%s)
packager = ${MAINTAINER}
size = ${installed_size}
arch = ${arch}
license = custom
depend = glibc
EOF
    (
        cd "${stage_dir}"
        tar --format=posix --owner=0 --group=0 --numeric-owner -cf - . \
            | zstd -19 -T0 -o "${DIST_DIR}/${PACKAGE_ID}-${ver}-${pkgrel}-${arch}.pkg.tar.zst"
    )
}

make_rpm_package() {
    local target="$1"
    local binary_path="$2"
    if ! command -v rpmbuild >/dev/null 2>&1; then
        echo "skip rpm: rpmbuild not found" >&2
        return 1
    fi
    local arch
    arch="$(rpm_arch_name "${target}")"
    local ver
    ver="$(version)"
    
    # RPM не работает с путями содержащими не-ASCII символы
    # Копируем всё во временную папку без кириллицы
    local tmp_rpm_dir="/tmp/pixel-paint-rs-rpm-$$"
    rm -rf "${tmp_rpm_dir}"
    mkdir -p "${tmp_rpm_dir}"
    
    local rpm_build_dir="${tmp_rpm_dir}/rpm-build"
    mkdir -p "${rpm_build_dir}/SPECS"
    mkdir -p "${rpm_build_dir}/BUILDROOT"
    
    # Копируем бинарник и файлы
    local files_dir="${tmp_rpm_dir}/files"
    mkdir -p "${files_dir}"
    install -m755 "${binary_path}" "${files_dir}/${APP_ID}"
    install -m644 "${REPO_ROOT}/assets/icon.png" "${files_dir}/icon.png"
    install -m644 "${REPO_ROOT}/assets/pixel_paint_rs.desktop" "${files_dir}/${APP_ID}.desktop"
    install -m644 "${REPO_ROOT}/LICENSE" "${files_dir}/LICENSE"
    install -m644 "${REPO_ROOT}/README.md" "${files_dir}/README.md"
    
    cat > "${rpm_build_dir}/SPECS/${PACKAGE_ID}.spec" <<EOF
Name:           ${PACKAGE_ID}
Version:        ${ver}
Release:        1%{?dist}
Summary:        ${DESCRIPTION}

License:        Custom
URL:            ${URL}
BuildArch:      ${arch}

%description
Grid-based drawing app with tabs, JSON save/load, session restore, and PNG export.

%install
mkdir -p %{buildroot}/usr/bin
mkdir -p %{buildroot}/usr/share/icons/hicolor/256x256/apps
mkdir -p %{buildroot}/usr/share/applications
mkdir -p %{buildroot}/usr/share/licenses/${PACKAGE_ID}
mkdir -p %{buildroot}/usr/share/doc/${PACKAGE_ID}
install -m755 ${files_dir}/${APP_ID} %{buildroot}/usr/bin/${APP_ID}
install -m644 ${files_dir}/icon.png %{buildroot}/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png
install -m644 ${files_dir}/${APP_ID}.desktop %{buildroot}/usr/share/applications/${APP_ID}.desktop
install -m644 ${files_dir}/LICENSE %{buildroot}/usr/share/licenses/${PACKAGE_ID}/LICENSE
install -m644 ${files_dir}/README.md %{buildroot}/usr/share/doc/${PACKAGE_ID}/README.md

%files
/usr/bin/${APP_ID}
/usr/share/icons/hicolor/256x256/apps/${APP_ID}.png
/usr/share/applications/${APP_ID}.desktop
/usr/share/licenses/${PACKAGE_ID}/LICENSE
/usr/share/doc/${PACKAGE_ID}/README.md

%changelog
* $(LC_ALL=C date '+%a %b %d %Y') ${MAINTAINER} - ${ver}-1
- Initial package
EOF
    
    if rpmbuild --define "_topdir ${rpm_build_dir}" \
                --define "_rpmdir ${tmp_rpm_dir}" \
                -bb "${rpm_build_dir}/SPECS/${PACKAGE_ID}.spec" 2>/dev/null; then
        mv "${tmp_rpm_dir}/${arch}/${PACKAGE_ID}-${ver}-1"*".rpm" "${DIST_DIR}/" 2>/dev/null || true
    fi
    rm -rf "${tmp_rpm_dir}"
}

make_windows_zip() {
    local binary_path="$1"
    local ver
    ver="$(version)"
    local stage_dir="${DIST_DIR}/stage/windows-x86_64"
    ensure_clean_dir "${stage_dir}"
    install -Dm755 "${binary_path}" "${stage_dir}/${APP_TITLE}/${APP_ID}.exe"
    install -Dm644 "${REPO_ROOT}/LICENSE" "${stage_dir}/${APP_TITLE}/LICENSE"
    install -Dm644 "${REPO_ROOT}/README.md" "${stage_dir}/${APP_TITLE}/README.md"
    (
        cd "${stage_dir}"
        zip -qr "${DIST_DIR}/${PACKAGE_ID}-${ver}-windows-x86_64.zip" "${APP_TITLE}"
    )
}

make_windows_installer() {
    local binary_path="$1"
    if ! command -v makensis >/dev/null 2>&1; then
        echo "skip windows installer: makensis not found" >&2
        return 1
    fi
    local ver
    ver="$(version)"
    makensis \
        -DAPP_VERSION="${ver}" \
        -DAPP_BINARY="${binary_path}" \
        -DAPP_REPO_ROOT="${REPO_ROOT}" \
        -DOUT_FILE="${DIST_DIR}/${PACKAGE_ID}-${ver}-windows-x86_64-setup.exe" \
        "${REPO_ROOT}/packaging/windows/pixel_paint_rs.nsi"
}
