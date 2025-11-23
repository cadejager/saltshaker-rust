#!/usr/bin/env bash

cd "$(dirname "${BASH_SOURCE[0]}")"
if command -v podman >/dev/null 2>&1; then
    podman compose -f compose.dev.yml down -v
    podman rmi container_rust-dev
elif command -v docker >/dev/null 2>&1; then
    docker compose -f compose.dev.yml down --rmi local -v
else
    echo "Neither podman or docker can be found"
    exit 1
fi
