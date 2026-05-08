#!/bin/bash

set -euo pipefail

GITHUB_REPO="christianhelle/httpgenerator"
BINARY_NAME="httpgenerator"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
DOCUMENTATION_URL="https://christianhelle.com/httpgenerator/"
REQUESTED_VERSION="${VERSION:-}"

log_info() {
    echo "[INFO] $1" >&2
}

log_success() {
    echo "[OK] $1" >&2
}

log_warning() {
    echo "[WARN] $1" >&2
}

log_error() {
    echo "[ERROR] $1" >&2
}

detect_platform() {
    local os
    local arch
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$os" in
        linux*)
            os="linux"
            ;;
        darwin*)
            os="darwin"
            ;;
        *)
            log_error "Unsupported operating system: $os"
            exit 1
            ;;
    esac

    if [[ "$os" == "darwin" ]] && [[ "$(sysctl -n hw.optional.arm64 2>/dev/null || echo 0)" == "1" ]]; then
        arch="arm64"
    else
        case "$arch" in
            x86_64|amd64)
                arch="x64"
                ;;
            aarch64|arm64)
                arch="arm64"
                ;;
            *)
                log_error "Unsupported architecture: $arch"
                exit 1
                ;;
        esac
    fi

    echo "${os}-${arch}"
}

check_dependencies() {
    local deps=("curl" "tar")

    for dep in "${deps[@]}"; do
        if ! command -v "$dep" >/dev/null 2>&1; then
            log_error "Required dependency '$dep' not found. Please install it first."
            exit 1
        fi
    done
}

get_latest_release() {
    local api_url="https://api.github.com/repos/$GITHUB_REPO/releases/latest"

    log_info "Fetching latest release information..."

    if ! curl -fsSL "$api_url" | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$'; then
        log_error "Failed to fetch release information from GitHub Releases."
        exit 1
    fi
}

ensure_install_directory() {
    if [[ -d "$INSTALL_DIR" ]]; then
        return
    fi

    log_info "Creating installation directory: $INSTALL_DIR"

    if mkdir -p "$INSTALL_DIR" 2>/dev/null; then
        return
    fi

    if command -v sudo >/dev/null 2>&1; then
        sudo mkdir -p "$INSTALL_DIR"
        return
    fi

    log_error "Cannot create directory '$INSTALL_DIR'."
    exit 1
}

install_binary() {
    local source_binary="$1"
    local target_binary="$INSTALL_DIR/$BINARY_NAME"

    log_info "Installing to $INSTALL_DIR..."

    if [[ -w "$INSTALL_DIR" ]]; then
        cp "$source_binary" "$target_binary"
        chmod +x "$target_binary"
        return
    fi

    if ! command -v sudo >/dev/null 2>&1; then
        log_error "Cannot write to '$INSTALL_DIR' and sudo is not available."
        exit 1
    fi

    log_warning "Installing with sudo because '$INSTALL_DIR' is not writable by the current user."
    sudo cp "$source_binary" "$target_binary"
    sudo chmod +x "$target_binary"
}

download_and_install() {
    local platform="$1"
    local version="$2"
    local archive_name="httpgenerator-${version}-${platform}.tar.gz"
    local download_url="https://github.com/$GITHUB_REPO/releases/download/$version/$archive_name"
    local temp_dir
    local source_binary

    temp_dir=$(mktemp -d)

    cleanup() {
        rm -rf "$temp_dir"
    }
    trap cleanup RETURN

    log_info "Downloading $archive_name..."

    if ! curl -fsSL -o "$temp_dir/$archive_name" "$download_url"; then
        log_error "Failed to download '$archive_name' from GitHub Releases."
        if [[ "$platform" == *"-arm64" ]]; then
            log_error "That usually means the current release does not publish a native $platform archive yet."
        fi
        exit 1
    fi

    log_info "Extracting archive..."
    tar -xzf "$temp_dir/$archive_name" -C "$temp_dir"

    source_binary=$(find "$temp_dir" -maxdepth 2 -type f -name "$BINARY_NAME" | head -n 1 || true)

    if [[ -z "$source_binary" ]]; then
        log_error "The downloaded archive did not contain '$BINARY_NAME'."
        exit 1
    fi

    ensure_install_directory
    install_binary "$source_binary"

    log_success "Installed $BINARY_NAME $version."
}

verify_installation() {
    local installed_binary="$INSTALL_DIR/$BINARY_NAME"
    local installed_version

    if [[ ! -x "$installed_binary" ]]; then
        log_warning "Installation completed but '$installed_binary' is not executable."
        return
    fi

    installed_version=$("$installed_binary" --version 2>/dev/null | head -n 1 || echo "unknown")
    log_success "Installation verified: $installed_version"

    if command -v "$BINARY_NAME" >/dev/null 2>&1; then
        log_info "You can now run: $BINARY_NAME --help"
        return
    fi

    log_warning "'$INSTALL_DIR' is not on PATH for the current shell."
    log_info "Run '$installed_binary --help' or add '$INSTALL_DIR' to your PATH."
}

show_usage() {
    cat <<EOF
HTTP File Generator installation script

Usage: $0 [OPTIONS]

Options:
  -d, --dir DIR          Set the installation directory (default: /usr/local/bin)
  -v, --version VERSION  Install a specific release tag instead of the latest GitHub Release
  -h, --help             Show this help message

Environment variables:
  INSTALL_DIR            Installation directory (default: /usr/local/bin)
  VERSION                Release tag to install

Examples:
  curl -fsSL https://christianhelle.com/httpgenerator/install | bash
  INSTALL_DIR=\$HOME/.local/bin curl -fsSL https://christianhelle.com/httpgenerator/install | bash
  curl -fsSL https://christianhelle.com/httpgenerator/install | bash -s -- --version 1.1.0
EOF
}

main() {
    while [[ $# -gt 0 ]]; do
        case "$1" in
            -h|--help)
                show_usage
                exit 0
                ;;
            -d|--dir)
                INSTALL_DIR="$2"
                shift 2
                ;;
            -v|--version)
                REQUESTED_VERSION="$2"
                shift 2
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done

    log_info "Starting HTTP File Generator installation..."
    check_dependencies

    local platform
    local version

    platform=$(detect_platform)
    log_info "Detected platform: $platform"

    if [[ -n "$REQUESTED_VERSION" ]]; then
        version="$REQUESTED_VERSION"
    else
        version=$(get_latest_release)
    fi

    log_info "Installing release: $version"
    download_and_install "$platform" "$version"
    verify_installation

    log_success "Installation complete."
    log_info "Documentation: $DOCUMENTATION_URL"
}

main "$@"
