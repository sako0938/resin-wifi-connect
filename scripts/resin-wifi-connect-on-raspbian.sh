#!/bin/bash

set -u

: "${APPNAME:='resin-wifi-connect-on-raspbian'}"

: "${RWC_REPO:='resin-io/resin-wifi-connect'}"

RELEASE_URL="https://api.github.com/repos/$RWC_REPO/releases/latest"

CONFIRMATION=true

usage() {
    cat 1>&2 <<EOF
$APPNAME 1.0.0 (2017-10-05)
Raspbian installer for resin-wifi-connect

USAGE:
    $APPNAME [FLAGS]

FLAGS:
    -y                      Disable confirmation prompt
    -h, --help              Prints help information
EOF
}

main() {
    need_cmd id
    need_cmd curl
    need_cmd systemctl
    need_cmd apt-get
    need_cmd grep

    for arg in "$@"; do
        case "$arg" in
            -h|--help)
                usage
                exit 0
                ;;
            -y)
                CONFIRMATION=false
                ;;
            *)
                ;;
        esac
    done

    check_os_version

    activate_network_manager

    download_rwc
}

check_os_version() {
    local _version=""

    if [ -f /etc/os-release ]; then
        _version=$(grep -oP 'VERSION="\K[^"]+' /etc/os-release)
    fi

    if [ "$_version" != "9 (stretch)" ]; then
        err "Distribution not based on Debian 9 (stretch)"
    fi
}

activate_network_manager() {
    if [ "$(service_load_state NetworkManager)" = "not-found" ]; then
        say 'NetworkManager is not installed'

        confirm_installation

        # Do not install NetworkManager over dhcpcd to avoid clashes

        say 'Downloading NetworkManager...'

        ensure sudo apt-get install -y -d network-manager

        disable_dhcpcd

        say 'Installing NetworkManager...'

        ensure sudo apt-get install -y network-manager

        ensure sudo apt-get clean
    else
        say 'NetworkManager is already installed'

        if [ "$(service_active_state NetworkManager)" = "active" ]; then
            say 'NetworkManager is already active'
        else
            confirm_installation

            disable_dhcpcd

            say 'Activating NetworkManager...'

            ensure sudo systemctl enable NetworkManager

            ensure sudo systemctl start NetworkManager
        fi
    fi

    if [ ! "$(service_active_state NetworkManager)" = "active" ]; then
        err 'Cannot activate NetworkManager'
    fi
}

disable_dhcpcd() {
    if [ "$(service_active_state dhcpcd)" = "active" ]; then
        say 'Deactivating and disabling dhcpcd...'

        ensure sudo systemctl stop dhcpcd

        ensure sudo systemctl disable dhcpcd

        if [ "$(service_active_state dhcpcd)" = "active" ]; then
            err 'Cannot deactivate dhcpcd'
        else
            say 'dhcpcd successfully deactivated and disabled'
        fi
    else
        say 'dhcpcd is not active'
    fi
}

service_load_state() {
    ensure systemctl -p LoadState --value show "$1"
}

service_active_state() {
    ensure systemctl -p ActiveState --value show "$1"
}

confirm_installation() {
    if [ "$CONFIRMATION" = false ]; then
        return
    fi

    printf '\33[1;36m%s:\33[0m ' "$APPNAME"

    read -r -p "Continue to install NetworkManager and disable dhcpcd? [y/N] " response
    response=${response,,}  # convert to lowercase
    if [[ ! $response =~ ^(yes|y)$ ]]; then
        exit 0
    fi
}

download_rwc() {
    local _regex='browser_download_url": "\K.*rpi\.tar\.gz'
    local _arch_url
    local _rwc_version

    say "Retrieving latest release from $RELEASE_URL..."

    _arch_url=$(ensure curl "$RELEASE_URL" -s | grep -hoP "$_regex")

    say "Downloading and extracting $_arch_url..."

    ensure curl -Ls "$_arch_url" | tar -xvz -C .

    _rwc_version=$(ensure ./resin-wifi-connect --version)

    say "Successfully downloaded $_rwc_version"

    say "Run './resin-wifi-connect --help' for available options"
}

say() {
    printf '\33[1m%s:\33[0m %s\n' "$APPNAME" "$1"
}

err() {
    printf '\33[1;31m%s:\33[0m %s\n' "$APPNAME" "$1" >&2
    exit 1
}

need_cmd() {
    if ! command -v "$1" > /dev/null 2>&1; then
        err "need '$1' (command not found)"
    fi
}

ensure() {
    "$@"
    if [ $? != 0 ]; then
        err "command failed: $*";
    fi
}

main "$@" || exit 1
