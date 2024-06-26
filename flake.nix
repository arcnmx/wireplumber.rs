{
  description = "wireplumber rust bindings";
  inputs = {
    flakelib.url = "github:flakelib/fl";
    nixpkgs = { };
    rust = {
      url = "github:arcnmx/nixexprs-rust";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    gir-src = {
      url = "github:gtk-rs/gir/0.19";
      flake = false;
    };
    gir-files = {
      url = "github:gtk-rs/gir-files/0.19";
      flake = false;
    };
  };
  outputs = { self, flakelib, nixpkgs, rust, ... }@inputs: let
    nixlib = nixpkgs.lib;
  in flakelib {
    inherit inputs;
    systems = [ "x86_64-linux" "aarch64-linux" ];
    devShells = {
      plain = {
        mkShell, writeShellScriptBin, wpexec
      , wireplumber-0_4, pipewire, glib
      , pkg-config
      , wireplumber-gir, gir-files, gir-rs-0_18
      , enableRustdoc ? false
      , enableRust ? true, cargo
      , rustTools ? [ ]
      }: mkShell {
        inherit rustTools;
        strictDeps = true;
        buildInputs = [ wireplumber-0_4 pipewire glib ];
        nativeBuildInputs = [
          pkg-config
          gir-rs-0_18
          (writeShellScriptBin "commitlint" ''nix run ''${FLAKE_OPTS-} .#wpdev-commitlint -- "$@"'')
        ] ++ nixlib.optional enableRust cargo;
        RUSTDOCFLAGS = rust.lib.rustdocFlags {
          inherit (self.lib) crate;
          enableUnstableRustdoc = enableRustdoc;
          extern = rec {
            glib = let
              version = nixlib.versions.majorMinor self.lib.crate.dependencies.glib.version;
            in if self.lib.crate.dependencies.glib ? git
              then "https://gtk-rs.org/gtk-rs-core/git/docs/"
              else "https://gtk-rs.org/gtk-rs-core/stable/${version}/docs/";
            glib-sys = glib;
            gio = glib;
            gio-sys = glib;
            gobject-sys = glib;
            pipewire = "https://pipewire.pages.freedesktop.org/pipewire-rs/";
            pipewire-sys = pipewire;
            libspa = pipewire;
            libspa-sys = pipewire;
          };
        } ++ nixlib.optionals enableRustdoc self.lib.crate.package.metadata.docs.rs.rustdoc-args;
        WP_GIR = "${wireplumber-gir}/share/gir-1.0/Wp-0.4.gir";
        GIRSPATH = nixlib.makeSearchPathOutput "dev" "share/gir-1.0" [
          wireplumber-gir gir-files
        ];
        inherit (wpexec) LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
      };
      stable = { rust'stable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'stable) mkShell;
        enableRust = false;
      };
      dev = { rust'unstable, outputs'devShells'plain }: outputs'devShells'plain.override {
        inherit (rust'unstable) mkShell;
        enableRust = false;
        enableRustdoc = true;
        rustTools = [ "rust-analyzer" ];
      };
      default = { outputs'devShells }: outputs'devShells.plain;
    };
    packages = {
      wpexec = {
        stdenv, rustPlatform, lib
      , wireplumber-0_4, pipewire, glib
      , pkg-config, libclang
      , buildType ? "release"
      , source
      }: with lib; rustPlatform.buildRustPackage rec {
        pname = "wpexec-rs";
        version = if buildType == "release"
          then self.lib.version
          else self.lastModifiedDate or self.lib.version;

        src = source;
        inherit (self.lib.crate) cargoLock;

        buildInputs = [ wireplumber-0_4 pipewire glib ];
        nativeBuildInputs = [ pkg-config ];

        cargoBuildFlags = "--workspace --bin wpexec";
        buildFeatures = mapNullable singleton (self.lib.featureForVersion wireplumber-0_4.version);

        inherit buildType;
        doCheck = false;

        LIBCLANG_PATH = "${libclang.lib}/lib";
        BINDGEN_EXTRA_CLANG_ARGS = [
          "-I${stdenv.cc.cc}/lib/gcc/${stdenv.hostPlatform.config}/${stdenv.cc.cc.version}/include"
          "-I${stdenv.cc.libc.dev}/include"
        ];

        meta = with lib; {
          description = "A WirePlumber utility ported to Rust";
          homepage = "https://github.com/arcnmx/wireplumber.rs";
          license = lib.licenses.mit;
          maintainers = [ maintainers.arcnmx ];
          platforms = platforms.linux;
          mainProgram = "wpexec";
        };
      };
      gir-rs-0_18 = { rustPlatform, gir-rs, fetchFromGitHub }: rustPlatform.buildRustPackage {
        inherit (gir-rs) postPatch meta pname;
        version = "0.19-${builtins.substring 0 8 inputs.gir-src.lastModifiedDate}";

        src = inputs.gir-src;

        cargoLock = {
          lockFile = "${inputs.gir-src}/Cargo.lock";
          outputHashes = {
            "rustdoc-stripper-0.1.19" = "sha256-QPqDiU8Y1yfoLi5fRvI9Q7YMsAOZ7oywkzAgH8sjCM0=";
          };
        };
        buildType = let
          # work around gir panics like: thread 'main' panicked at 'attempt to subtract with overflow', src/analysis/function_parameters.rs:243:46
          girIsBugged = false;
        in if girIsBugged then "release" else "debug";
        doCheck = false;
      };
      wireplumber-gir = { runCommand, runtimeShell, xmlstarlet, wireplumber-0_4 }: runCommand "wireplumber-${wireplumber-0_4.version}.gir" {
        girName = "share/gir-1.0/Wp-${nixlib.versions.majorMinor wireplumber-0_4.version}.gir";
        wireplumber = wireplumber-0_4.dev;
        nativeBuildInputs = [ xmlstarlet ];
      } ''
        mkdir -p $out/$(dirname $girName)
        ${runtimeShell} ${./ci/wp-gir-filter.sh} < "$wireplumber/$girName" > $out/$girName
      '';
    };
    checks = {
      wpexec = { wptest, runtimeShell, wpexec, lib }: let
        key = placeholder "wpexec";
      in wptest "wpexec" ''
        export PATH="$PATH:${lib.makeBinPath [ wpexec ]}"
        ${runtimeShell} ${./ci/test-wpexec.sh} ${./ci/test-wpexec.lua} ${lib.escapeShellArg key} &&
        touch $out
      '';
      rustfmt = { rust'builders, source }: rust'builders.check-rustfmt-unstable {
        src = source;
        config = ./.rustfmt.toml;
        cargoFmtArgs = [
          "-p" "wireplumber"
          "-p" "wp-examples"
        ];
      };
      docs = { docs }: docs;
      readme-github = { rust'builders, wpdev-readme-github }: rust'builders.check-generate {
        expected = wpdev-readme-github;
        src = ./.github/README.md;
        meta.name = "diff .github/README.md (cargo wp generate)";
      };
      readme-sys-github = { rust'builders, wpdev-readme-sys-github }: rust'builders.check-generate {
        expected = wpdev-readme-sys-github;
        src = ./sys/.github/README.md;
        meta.name = "diff sys/README.md (cargo wp generate)";
      };
      readme-examples-github = { rust'builders, wpdev-readme-examples-github }: rust'builders.check-generate {
        expected = wpdev-readme-examples-github;
        src = ./examples/.github/README.md;
        meta.name = "diff examples/README.md (cargo wp generate)";
      };
      readme-package = { rust'builders, wpdev-readme-package }: rust'builders.check-generate {
        expected = wpdev-readme-package;
        src = ./src/README.md;
        meta.name = "diff src/README.md (cargo wp generate)";
      };
      readme-sys-package = { rust'builders, wpdev-readme-sys-package }: rust'builders.check-generate {
        expected = wpdev-readme-sys-package;
        src = ./sys/src/README.md;
        meta.name = "diff sys/src/README.md (cargo wp generate)";
      };
      readme-attrs = { rust'builders, wpdev-readme-attrs }: rust'builders.check-generate {
        expected = wpdev-readme-attrs;
        src = ./ci/readme/attrs.adoc;
        meta.name = "diff ci/readme/attrs.adoc (cargo wp generate)";
      };
      commitlint-help = { rust'builders, wpdev-commitlint-help }: rust'builders.check-generate {
        expected = wpdev-commitlint-help;
        src = ./.github/commitlint.adoc;
        meta.name = "diff .github/commitlint.adoc (cargo wp generate)";
      };
      commitlintrc = { rust'builders, wpdev-commitlintrc-generate }: rust'builders.check-generate {
        expected = wpdev-commitlintrc-generate;
        src = builtins.path {
          name = "commitlintrc.json";
          path = ./.commitlintrc.json;
        };
        meta.name = "diff .commitlintrc.json (cargo wp generate)";
      };
      release-branch = { rust'builders, source }: let
        inherit (self.lib) releaseTag;
        docs'rs = {
          inherit (self.lib.crate.package) name;
          version = releaseTag;
          baseUrl = rust.lib.escapePattern self.lib.pagesRoot;
        };
        cargo'docs = {
          inherit (docs'rs) name version baseUrl;
        };
      in rust'builders.check-contents {
        name = "wireplumber-release-check" ;
        patterns = [
          { path = "src/lib.rs"; inherit docs'rs; }
          { path = "sys/src/lib.rs"; inherit docs'rs; }
          { path = "src/README.md"; plain = "/tree/${releaseTag}/examples"; }
          { path = "Cargo.toml"; inherit cargo'docs; }
          { path = "sys/Cargo.toml"; cargo'docs = cargo'docs // { name = "wireplumber-sys"; }; }
        ];
        src = source;
      };
    };
    packages = {
      wireplumber-0_4 = { wireplumber, lib }: let
        drv = wireplumber.overrideAttrs (old: rec {
          version = "0.4.17";
          src = old.src.override {
            rev = version;
            hash = "sha256-vhpQT67+849WV1SFthQdUeFnYe/okudTQJoL3y+wXwI=";
          };
        });
      in if nixlib.versionAtLeast wireplumber.version "0.5" then drv else wireplumber;
    };
    legacyPackages = {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;
      source-package = { rust'builders }: rust'builders.wrapSource self.lib.crate.pkgSrc;
      source-package-sys = { rust'builders }: rust'builders.wrapSource self.lib.crate.members.sys.pkgSrc;

      gir-files = { linkFarm }: linkFarm "gir-files-0.19-${builtins.substring 0 8 inputs.gir-files.lastModifiedDate}" [
        {
          name = "share/gir-1.0";
          path = inputs.gir-files;
        }
      ];

      docs = { rust'builders, outputs'devShells'plain, wpexec, source }: let
        shell = outputs'devShells'plain.override { enableRust = false; enableRustdoc = true; };
      in rust'builders.cargoDoc {
        inherit (self.lib) crate;
        src = source;
        enableUnstableRustdoc = true;
        rustdocFlags = shell.RUSTDOCFLAGS;
        cargoDocFlags = [ "--no-deps" "--workspace" ];
        postBuild = ''
          cargo doc --frozen \
            ''${cargoDocFeaturesFlag} \
            ''${cargoDocFlags} \
            --examples --document-private-items
        '';

        inherit (wpexec)
          buildInputs nativeBuildInputs
          LIBCLANG_PATH BINDGEN_EXTRA_CLANG_ARGS;
      };

      wptest = { callPackage }: (import ./examples { inherit callPackage; }).wptest;

      wpdev-commitlintrc = { writeText, nodePackages }: writeText "wireplumber-rust.commitlintrc.json" (builtins.toJSON
        (self.lib.commitlint.commitlintrc // {
          extends = [ "${nodePackages."@commitlint/config-conventional"}/lib/node_modules/@commitlint/config-conventional/lib/index.js" ];
        })
      );
      wpdev-commitlintrc-generate = { writeText }: writeText "wireplumber-rust.commitlintrc.json" (builtins.toJSON
        self.lib.commitlint.commitlintrc
      );
      wpdev-commitlint = { writeShellScriptBin, commitlint, wpdev-commitlintrc }: writeShellScriptBin "commitlint" ''
        exec ${commitlint}/bin/commitlint --config ${wpdev-commitlintrc} "$@"
      '';
      wpdev-generate = {
        rust'builders
      , wpdev-readme-github, wpdev-readme-package, wpdev-readme-sys-github, wpdev-readme-examples-github, wpdev-readme-sys-package
      , wpdev-readme-attrs
      , wpdev-commitlint-help, wpdev-commitlintrc-generate
      , outputHashes
      }: rust'builders.generateFiles {
        name = "readmes";
        paths = {
          ".github/README.md" = wpdev-readme-github;
          "sys/.github/README.md" = wpdev-readme-sys-github;
          "examples/.github/README.md" = wpdev-readme-examples-github;
          "src/README.md" = wpdev-readme-package;
          "sys/src/README.md" = wpdev-readme-sys-package;
          "ci/readme/attrs.adoc" = wpdev-readme-attrs;
          ".github/commitlint.adoc" = wpdev-commitlint-help;
          ".commitlintrc.json" = wpdev-commitlintrc-generate;
          "lock.nix" = outputHashes;
        };
      };
      wpdev-readme-src = { symlinkJoin, linkFarm, wpdev-readme-attrs }: let
        whitelist = [
          "/ci"
          "/ci/readme"
          "/ci/readme/content.adoc"
          "/ci/readme/dev.adoc"
          "/ci/readme/header.adoc"
          "/ci/readme/sys.adoc"
          "/README.adoc"
          "/src"
          "/src/README.adoc"
          "/sys"
          "/sys/README.adoc"
          "/sys/src"
          "/sys/src/README.adoc"
          "/examples"
          "/examples/README.adoc"
        ];
        readme-src = builtins.path {
          path = ./.;
          filter = path: type: let
            path' = nixlib.removePrefix (toString ./.) (toString path);
          in builtins.elem path' whitelist;
        };
        readme-attrs = linkFarm "wireplumber-rust-readme-attrs" [
          {
            name = "ci/readme/attrs.adoc";
            path = wpdev-readme-attrs;
          }
        ];
      in symlinkJoin {
        name = "wireplumber-rust-readme-src";
        paths = [
          readme-src
          readme-attrs
        ];
      };
      wpdev-readme-github = { rust'builders, wpdev-readme-src }: rust'builders.adoc2md {
        src = "${wpdev-readme-src}/README.adoc";
        attributes = {
          readme-inc = "${wpdev-readme-src}/ci/readme/";
          # this file ends up in `.github/README.md`, so its relative links must be adjusted to compensate
          relative-blob = "../";
          profile = "github";
        };
      };
      wpdev-readme-sys-github = { rust'builders, wpdev-readme-src }: rust'builders.adoc2md {
        src = "${wpdev-readme-src}/sys/README.adoc";
        attributes = {
          readme-inc = "${wpdev-readme-src}/ci/readme/";
          # this file ends up in `sys/.github/README.md`, so its relative links must be adjusted to compensate
          relative-blob = "../../";
          profile = "github";
        };
      };
      wpdev-readme-examples-github = { rust'builders, wpdev-readme-src }: rust'builders.adoc2md {
        src = "${wpdev-readme-src}/examples/README.adoc";
        attributes = {
          readme-inc = "${wpdev-readme-src}/ci/readme/";
          # this file ends up in `examples/.github/README.md`, so its relative links must be adjusted to compensate
          relative-blob = "../../";
          profile = "github";
        };
      };
      wpdev-readme-package = { rust'builders, wpdev-readme, wpdev-readme-src }: rust'builders.adoc2md {
        src = "${wpdev-readme-src}/src/README.adoc";
        attributes = let
          inherit (self.lib.asciidocAttributes) repository;
        in rec {
          profile = "package";
          readme-inc = "${wpdev-readme-src}/ci/readme/";
          release = self.lib.releaseTag;
          relative-tree = "${repository}/tree/${release}/";
          relative-blob = "${repository}/blob/${release}/";
        };
      };
      wpdev-readme-sys-package = { rust'builders, wpdev-readme-package, wpdev-readme-src }: rust'builders.adoc2md {
        src = "${wpdev-readme-src}/sys/src/README.adoc";
        inherit (wpdev-readme-package) attributes;
      };
      wpdev-readme-attrs = { writeText }: let
        adocAttr = key: value: ":${key}:" + nixlib.optionalString (value != "") " ${value}";
        attrs = nixlib.mapAttrsToList adocAttr self.lib.asciidocAttributes;
      in writeText "wireplumber-attrs.adoc" ''
        // This file is automatically @generated from flake.nix
        ${nixlib.concatStringsSep "\n" attrs}
      '';
      wpdev-commitlint-help = { writeText }: writeText "commitlint.adoc" self.lib.commitlint.help-adoc;
      outputHashes = { rust'builders }: rust'builders.cargoOutputHashes {
        inherit (self.lib) crate;
      };
    };
    lib = with nixlib; {
      featureVersions = [
        "0.4.3" "0.4.5"
        "0.4.6"
        "0.4.8" "0.4.10"
        "0.4.11" "0.4.12"
        "0.4.15"
        "0.4.16"
      ];
      asciidocAttributes = {
        inherit (self.lib.crate.package) repository;
        wprs-versionreq = (if versions.major self.lib.version == "0" then versions.majorMinor else versions.major) self.lib.version;
        wprs-versionfeature = self.lib.versionFeatureName (last self.lib.featureVersions);
        source-highlighter = "highlight.js";
        release = "main";
        relative-tree = "{relative-blob}";
        url-pw = "https://pipewire.org/";
        url-pw-api = "https://docs.pipewire.org/page_api.html";
        url-wp = "https://pipewire.pages.freedesktop.org/wireplumber/index.html";
        url-docs = "https://arcnmx.github.io/wireplumber.rs/{release}/{crate}/";
        url-docs-modules = "{url-docs}#modules";
        url-crates = "https://crates.io/crates/{crate}";
        badge-crates = "https://img.shields.io/crates/v/{crate}.svg?style=flat-square";
        badge-docs = "https://img.shields.io/badge/API-docs-blue.svg?style=flat-square";
        badge-license = "https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square";
      };
      supportedVersions = version: filter (versionAtLeast version) self.lib.featureVersions;
      versionFeatureName = version: "v" + replaceStrings [ "." ] [ "_" ] version;
      featureForVersion = version: let
        features = self.lib.supportedVersions version;
      in if features == [ ] then null else self.lib.versionFeatureName (last features);
      crate = rust.lib.importCargo {
        path = ./Cargo.toml;
        inherit (import ./lock.nix) outputHashes;
      };
      inherit (self.lib.crate.package) version;
      inherit (self.lib.crate.package.metadata) branches;
      owner = "arcnmx";
      repo = "wireplumber.rs";
      pagesRoot = rust.lib.ghPages {
        inherit (self.lib) owner repo;
      };
      releaseTag = "v${self.lib.version}";

      commitlint = import ./ci/commitlint.nix {
        inherit self;
        lib = nixlib;
      };
    };
    config = rec {
      name = "wireplumber-rust";
      packages.namespace = [ name ];
    };
  };
}
