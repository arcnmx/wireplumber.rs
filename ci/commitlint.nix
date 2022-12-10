{ self, lib }: let
  inherit (lib)
    mapAttrsToList genAttrs attrNames
    optional
    optionalString concatStringsSep
    flip
  ;
  levels = {
    error = 2;
    warn = 1;
    disable = 0;
  };
  mkRule = { level ? levels.error, applicable ? true }: value: [
    level
    (if applicable then "always" else "never")
  ] ++ optional (value != null) value;
  types = {
    build = { };
    chore = { };
    docs = { };
    feat = { };
    fix = { };
    perf = { };
    refactor = { };
    revert = { };
    style = { };
    test = { };
  };
  moduleScopes = [
    "core" "dbus" "local"
    "lua" "plugin" "spa"
    "error" "log" "util"
    "pipewire" "registry" "session"
  ];
  scopes = {
    examples = {
      paths = [ "./examples/" ];
    };
    ffi = {
      paths = [ "./sys/" "./Gir.toml" "./src/auto/" ];
      help = "changes to Gir.toml, src/auto, and sys/bindings updates";
    };
    readme = {
      paths = [ "./README.adoc" "./sys/README.adoc" "./Cargo.toml" "./sys/Cargo.toml" ];
      help = "README updates";
    };
    ci = {
      paths = [ "./ci/*.nix" "./flake.nix" ];
      help = "changes to CI-related nix files";
    };
    lock = {
      paths = [ "./flake.lock" "./Cargo.lock" ];
      help = "cargo/flake updates";
    };
    pipewire = {
      paths = [ "./src/pw/" ];
      help = "crate::pw module";
    };
    prelude = {
      paths = [ "./src/prelude.rs" ];
      help = "crate::prelude::*";
    };
  } // flip genAttrs (id: {
    paths = [ "./src/${id}/" ];
    help = "crate::${id} module";
  }) moduleScopes;
in {
  help-adoc = let
    types = mapAttrsToList (id: { help ? null }:
      "* ${id}" + optionalString (help != null) ": ${help}"
    ) self.lib.commitlint.types;
    scopes = mapAttrsToList (id: { help ? toString paths, paths ? [ ] }:
      "* ${id}: ${help}"
    ) self.lib.commitlint.scopes;
  in ''
    = https://commitlint.js.org[commitlint] usage

    Commit messages should follow the https://www.conventionalcommits.org[Conventional Commits] specification.

    == Types

    ${concatStringsSep "\n" types}

    == Scopes

    ${concatStringsSep "\n" scopes}
  '';
  inherit types scopes;
  commitlintrc = {
    extends = [ "@commitlint/config-conventional" ];
    rules = {
      type-enum = mkRule { } (attrNames self.lib.commitlint.types);
      scope-enum = mkRule { } (attrNames self.lib.commitlint.scopes);
      scope-case = mkRule { } "lower-case";
      scope-empty = mkRule { level = levels.warn; applicable = false; } null;
    };
    helpUrl = "https://github.com/${self.lib.owner}/${self.lib.repo}/blob/main/.github/commitlint.adoc";
  };
}
