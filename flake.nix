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
      url = "github:gtk-rs/gir/0.18";
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
      , wireplumber, pipewire, glib
      , pkg-config
      , wireplumber-gir, gobject-introspection, gir-rs-0_18
      , enableRustdoc ? false
      , enableRust ? true, cargo
      , rustTools ? [ ]
      }: mkShell {
        inherit rustTools;
        strictDeps = true;
        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [
          pkg-config
          gir-rs-0_18
          (writeShellScriptBin "commitlint" ''nix run ''${FLAKE_OPTS-} .#wpdev-commitlint -- "$@"'')
          (writeShellScriptBin "generate" ''nix run ''${FLAKE_OPTS-} .#wpdev-generate -- "$@"'')
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
          wireplumber-gir gobject-introspection
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
      , wireplumber, pipewire, glib
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

        buildInputs = [ wireplumber pipewire glib ];
        nativeBuildInputs = [ pkg-config ];

        cargoBuildFlags = "--workspace --bin wpexec";
        buildFeatures = mapNullable singleton (self.lib.featureForVersion wireplumber.version);

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
        version = "0.18-${builtins.substring 0 8 inputs.gir-src.lastModifiedDate}";

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
      wireplumber-gir = { runCommand, runtimeShell, xmlstarlet, wireplumber }: runCommand "wireplumber-${wireplumber.version}.gir" {
        girName = "share/gir-1.0/Wp-${nixlib.versions.majorMinor wireplumber.version}.gir";
        wireplumber = wireplumber.dev;
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
      readme = { rust'builders, wpdev-readme }: rust'builders.check-generate {
        expected = wpdev-readme;
        src = ./src/README.md;
        meta.name = "diff src/README.md (nix run .#wpdev-generate)";
      };
      readme-sys = { rust'builders, wpdev-sys-readme }: rust'builders.check-generate {
        expected = wpdev-sys-readme;
        src = ./sys/src/README.md;
        meta.name = "diff sys/src/README.md (nix run .#wpdev-generate)";
      };
      commitlint-help = { rust'builders, wpdev-commitlint-help }: rust'builders.check-generate {
        expected = wpdev-commitlint-help;
        src = ./.github/commitlint.adoc;
        meta.name = "diff .github/commitlint.adoc (nix run .#wpdev-generate)";
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
    legacyPackages = {
      source = { rust'builders }: rust'builders.wrapSource self.lib.crate.src;

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

      wpdev-commitlintrc = { writeText, commitlint, nodePackages }: writeText "wireplumber-rust.commitlintrc.json" (builtins.toJSON
        (self.lib.commitlint.commitlintrc // {
          extends = [ "${nodePackages."@commitlint/config-conventional"}/lib/node_modules/@commitlint/config-conventional/." ];
        })
      );
      wpdev-commitlint = { writeShellScriptBin, commitlint, wpdev-commitlintrc }: writeShellScriptBin "commitlint" ''
        exec ${commitlint}/bin/commitlint --config ${wpdev-commitlintrc} "$@"
      '';
      wpdev-generate = {
        rust'builders
      , wpdev-readme, wpdev-sys-readme
      , wpdev-commitlint-help
      , outputHashes
      }: rust'builders.generateFiles {
        name = "readmes";
        paths = {
          "src/README.md" = wpdev-readme;
          "sys/src/README.md" = wpdev-sys-readme;
          ".github/commitlint.adoc" = wpdev-commitlint-help;
          "lock.nix" = outputHashes;
        };
      };
      wpdev-readme = { rust'builders }: rust'builders.adoc2md {
        src = ./README.adoc;
        attributes = let
          inherit (self.lib.crate.package) repository;
        in rec {
          release = self.lib.releaseTag;
          relative-tree = "${repository}/tree/${release}/";
          relative-blob = "${repository}/blob/${release}/";
        };
      };
      wpdev-sys-readme = { rust'builders, wpdev-readme }: rust'builders.adoc2md {
        src = ./sys/README.adoc;
        inherit (wpdev-readme) attributes;
      };
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
      ];
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
