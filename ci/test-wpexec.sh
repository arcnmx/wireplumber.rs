#!/usr/bin/env bash
set -eu
set +o pipefail

WP_SCRIPT="${1-$(dirname "${BASH_SOURCE[0]}")/test-wpexec.lua}"
WP_KEY="${2-placeholder/wpexec}"

WP_JSON="[\"$WP_KEY\"]"

if [[ $# -eq 0 ]]; then
	wpexec --version > /dev/null
fi
timeout 5 wpexec --json "$WP_JSON" "$WP_SCRIPT" | grep -F "$WP_KEY"
