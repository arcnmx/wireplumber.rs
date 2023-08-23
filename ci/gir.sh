#!/usr/bin/env bash
set -eu

WP_CI="$(dirname "${BASH_SOURCE[0]}")"

GIR_FLAG_COUNT=$#

IFS=':'
for gir in ${GIRSPATH-}; do
	if [[ -z $gir ]]; then
		continue
	fi
	set -- "$@" --girs-directories "$gir"
done
IFS=' '

gir "$@"

if [[ $GIR_FLAG_COUNT -eq 0 ]]; then
	if [[ -d src/auto ]]; then
		sed -i -e '/^\/\/ from [^ ]\+/d' \
			src/auto/*.rs
		patch \
			--no-backup-if-mismatch  \
			-p1 --reverse < $WP_CI/wp-gir.patch
	elif [[ -f tests/abi.rs ]]; then
		sed -i -e '/^\/\/ from [^ ]\+$/d' \
			build{,_version}.rs \
			{src,tests}/*.rs \
			tests/*.{h,c}
		patch \
			--no-backup-if-mismatch  \
			-p3 --reverse < $WP_CI/wp-gir-sys.patch
		mv Cargo.toml Cargo.template.toml
		if [[ -n "${WP_GIR-}" ]]; then
			cp -f "$WP_GIR" src/Wp-0.4.gir
		fi
	fi
fi
