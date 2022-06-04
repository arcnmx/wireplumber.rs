{ pkgs ? null, ci ? import <ci> (builtins.removeAttrs args [ "ci" "inNixShell" ]), ... }@args: ci.config.jobs.dev.shell
