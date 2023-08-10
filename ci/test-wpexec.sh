#!/usr/bin/env bash
set -eu
set +o pipefail

WP_SCRIPT="${1-$(dirname "${BASH_SOURCE[0]}")/test-wpexec.lua}"
WP_KEY="${2-placeholder/wpexec}"

WP_JSON="[\"$WP_KEY\"]"

timeout 5 wpexec --json "$WP_JSON" "$WP_SCRIPT" | grep -F "$WP_KEY"
