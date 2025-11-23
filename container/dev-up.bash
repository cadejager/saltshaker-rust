#!/usr/bin/env bash

export SSH_PUBKEY="$(cat ~/.ssh/*.pub | head -n 1)"
export SSH_PORT=4358

if command -v podman >/dev/null 2>&1; then
    CONTAINER_MANAGER=podman
elif command -v docker >/dev/null 2>&1; then
    CONTAINER_MANAGER=docker
else
    echo "Neither podman or docker can be found"
    exit 1
fi

cd "$(dirname "${BASH_SOURCE[0]}")"
${CONTAINER_MANAGER} compose -f compose.dev.yml up --build -d
