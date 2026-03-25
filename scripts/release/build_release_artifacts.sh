#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
source "${SCRIPT_DIR}/common.sh"

NATIVE_ONLY=0
SKIP_BUILD=0

for arg in "$@"; do
    case "${arg}" in
        --native-only) NATIVE_ONLY=1 ;;
        --skip-build) SKIP_BUILD=1 ;;
        *)
            echo "unknown arg: ${arg}" >&2
            echo "usage: $0 [--native-only] [--skip-build]" >&2
            exit 1
            ;;
    esac
done

mkdir -p "${DIST_DIR}/stage"
rm -f "${DIST_DIR}"/*.deb "${DIST_DIR}"/*.zst "${DIST_DIR}"/*.zip "${DIST_DIR}"/*setup.exe 2>/dev/null || true

HOST_TARGET="$(host_target)"

package_linux_target() {
    local target="$1"
    local linker="${2:-}"
    if [[ "${SKIP_BUILD}" -eq 0 ]]; then
        if ! build_target_if_possible "${target}" "${linker}"; then
            return 0
        fi
    fi
    local binary
    binary="$(binary_path_for "${target}")"
    if [[ ! -f "${binary}" ]]; then
        echo "skip ${target}: binary not found at ${binary}" >&2
        return 0
    fi
    make_deb_package "${target}" "${binary}"
    make_arch_package "${target}" "${binary}"
}

package_windows_target() {
    local target="x86_64-pc-windows-gnu"
    if [[ "${SKIP_BUILD}" -eq 0 ]]; then
        if ! build_target_if_possible "${target}" "x86_64-w64-mingw32-gcc"; then
            return 0
        fi
    fi
    local binary
    binary="$(binary_path_for "${target}")"
    if [[ ! -f "${binary}" ]]; then
        echo "skip windows: binary not found at ${binary}" >&2
        return 0
    fi
    make_windows_zip "${binary}"
    make_windows_installer "${binary}" || true
}

package_linux_target "${HOST_TARGET}"

if [[ "${NATIVE_ONLY}" -eq 0 ]]; then
    package_linux_target "aarch64-unknown-linux-gnu" "aarch64-linux-gnu-gcc"
    package_windows_target
fi

echo
echo "Artifacts:"
find "${DIST_DIR}" -maxdepth 1 -type f | sort
