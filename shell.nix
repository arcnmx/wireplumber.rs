{ pkgs ? null, ci ? import <ci> (builtins.removeAttrs args [ "ci" ]), ... }@args: ci.config.shell
